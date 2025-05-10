# OCR Clipboard Utility

A small Windows-only Rust utility packaged in a single executable file that:

* Reads an image from the clipboard or a file
* Performs OCR (Optical Character Recognition) using embedded Tesseract `eng.traineddata`
* Presents the recognized text in a native Windows dialog
* Offers to copy the text to the clipboard

---

## Features

* **Clipboard & File Input**: Automatically uses an image from the clipboard if available; otherwise, prompts with a file-open dialog.
* **Portable OCR**: Embeds the Tesseract English language model (`eng.traineddata`) in the binary, extracted at runtime.
* **Native UI**: Uses Win32 message boxes for prompts and confirmations.
* **Unicode-safe**: Converts any selected file (including paths with non-ASCII characters) into a temporary ASCII-only PNG to ensure compatibility.

---

## Prerequisites

* **Windows 10/11 (x64)**

* **Rust toolchain** (1.56+)

  ```sh
  rustup install stable
  rustup default stable
  ```

* **vcpkg** (for native Tesseract and Leptonica libraries)

  ```powershell
  git clone https://github.com/microsoft/vcpkg.git
  cd vcpkg
  .\bootstrap-vcpkg.bat
  .\vcpkg integrate install
  .\vcpkg install leptonica:x64-windows-static tesseract:x64-windows-static
  .\vcpkg install leptonica:x64-windows-static-md tesseract:x64-windows-static-md
  ```

* **LLVM (for bindgen)**

  1. Download from [https://github.com/llvm/llvm-project/releases](https://github.com/llvm/llvm-project/releases) (Windows installer).
  2. Install to a known location (e.g., `C:\Program Files\LLVM`).
  3. Set the environment variable:

     ```powershell
     setx LIBCLANG_PATH "C:\Program Files\LLVM\bin"
     ```

* **Tesseract-trained data**
  Download English data:

  ```powershell
  curl -L -o tessdata/eng.traineddata \
    https://github.com/tesseract-ocr/tessdata/raw/main/eng.traineddata
  ```

Ensure folder structure:

```
ocr_clipboard/
├── tessdata/
│   └── eng.traineddata
├── src/
│   └── main.rs
├── Cargo.toml
└── ...
```

---

## Installation & Build

1. **Clone the repository**

   ```sh
   git clone https://github.com/yourusername/ocr_clipboard.git
   cd ocr_clipboard
   ```

2. **Build in debug**

   ```sh
   cargo build
   ```

3. **Build release**

   ```sh
   cargo build --release
   ```

> To suppress the console window when double-clicking the `.exe`, add at the top of `src/main.rs`:
>
> ```rust
> #![windows_subsystem = "windows"]
> ```

---

## Usage

### From CLI

```sh
# Use image path
ocr_clipboard.exe path\to\image.png

# Use clipboard (no arguments)
ocr_clipboard.exe
```

* If the clipboard contains an image, OCR runs immediately.
* Otherwise, a file-open dialog appears to select an image.

### OCR and Copy Flow

1. Recognized text is shown in a Yes/No dialog.
2. Click **Yes** to copy the text to the clipboard.
3. Click **No** to exit without copying.

---

## Developer Setup Notes

* Ensure `vcpkg` is integrated and installed with the required static libraries.
* Ensure `LLVM` is installed and `LIBCLANG_PATH` is set.

---

## Customization

* **Language support**: Embed additional `.traineddata` files into `tessdata/` and adjust the `LepTess::new` call.
* **CLI improvements**: Use the `clap` crate to add flags like `--lang` or `--no-copy`.

---

