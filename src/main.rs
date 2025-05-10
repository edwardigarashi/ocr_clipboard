
use std::{env, fs, path::PathBuf};
use anyhow::Context;
use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer};
use leptess::LepTess;
use tempfile::{NamedTempFile, TempDir};
use copypasta::{ClipboardContext, ClipboardProvider};
use win_msgbox::{information, question, YesNo,Okay};
use rfd::FileDialog;

const ENG_TRAINEDDATA: &[u8] = include_bytes!("../tessdata/eng.traineddata");

fn main() -> anyhow::Result<()> {
    let mut _temp_image: Option<NamedTempFile> = None;
    let img_path: PathBuf = if let Some(arg) = env::args().nth(1) {
        arg.into()
    } else {
        match Clipboard::new().and_then(|mut cb| cb.get_image()) {
            Ok(img) => {
                // convert BGRA → RGBA
                let (w, h) = (img.width as u32, img.height as u32);
                let mut rgba = Vec::with_capacity(img.bytes.len());
                for chunk in img.bytes.chunks_exact(4) {
                    rgba.push(chunk[2]);
                    rgba.push(chunk[1]);
                    rgba.push(chunk[0]);
                    rgba.push(chunk[3]);
                }
                let buf: image::RgbaImage = ImageBuffer::from_raw(w, h, rgba)
                    .context("invalid image buffer from clipboard")?;
                let tmp = tempfile::Builder::new().suffix(".png").tempfile()?;
                buf.save(tmp.path())?;
                tmp.into_temp_path().to_path_buf()
            }
            Err(_) => {
                if let Some(path) = FileDialog::new()
                    .add_filter("Image", &["png","jpg","jpeg","bmp","tiff","gif"])
                    .set_title("Select an image for OCR")
                    .pick_file()
                {
                    // Load & re-save to ASCII-only temp PNG
                    let dyn_img: DynamicImage = image::open(&path)
                        .context("failed to open chosen image")?;
                    let tmp = tempfile::Builder::new()
                        .prefix("ocr_file_")
                        .suffix(".png")
                        .tempfile()?;
                    dyn_img.save(tmp.path()).context("saving chosen image")?;
                    let path = tmp.path().to_path_buf();
                    _temp_image = Some(tmp);  // guard!
                    path
                } else {
                    information::<Okay>("No image selected. Exiting.").title("OCR").show().map_err(|e| anyhow::anyhow!("copying text: {}", e))?;
                    return Ok(());
                }
            }
        }
    };

    let td = TempDir::new().context("creating temp dir")?;
    let tessdata = td.path().join("tessdata");
    fs::create_dir(&tessdata)?;
    fs::write(tessdata.join("eng.traineddata"), ENG_TRAINEDDATA)
        .context("writing eng.traineddata")?;

    let mut lt = LepTess::new(Some(tessdata.to_str().unwrap()), "eng")
        .context("initializing Tesseract")?;
    lt.set_image(&img_path).context("loading image into Tesseract")?;
    let text = lt.get_utf8_text().context("running OCR")?;

    let prompt = format!("Copy to clipboard?\n\n{}", text);
    if question::<YesNo>(&prompt).title("OCR Recognized Result").show() == Ok(YesNo::Yes) {
        let mut ctx = ClipboardContext::new().map_err(|e| anyhow::anyhow!("opening clipboard: {}", e))?;
        ctx.set_contents(text).map_err(|e| anyhow::anyhow!("copying text: {}", e))?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedded_traineddata_exists() {
        assert!(!ENG_TRAINEDDATA.is_empty(), "Embedded eng.traineddata should not be empty");
    }

    #[test]
    fn test_temp_tessdata_setup() {
        let td = TempDir::new().unwrap();
        let tessdata = td.path().join("tessdata");
        fs::create_dir(&tessdata).unwrap();
        fs::write(tessdata.join("eng.traineddata"), ENG_TRAINEDDATA).unwrap();
        assert!(tessdata.join("eng.traineddata").exists(), "eng.traineddata should be written to temp tessdata");
    }
}