# term-img 🖼️

`term-img` is a lightweight, fast, and optimized command-line tool written in Rust to display images directly inside your terminal. It supports loading images from both local file paths and remote URLs.

Under the hood, it utilizes [`viuer`](https://github.com/atanunq/viuer) and the [`image`](https://github.com/image-rs/image) crate to render images using modern terminal protocols (like Kitty, Sixel, or half-block fallback characters) with full 24-bit true color support.

---

## Features

- **Local & Remote Sources:** Display local files or fetch and display images directly from any HTTP/HTTPS URL.
- **Aspect-Ratio Preserved Scaling:** Custom scale your images by specifying width while automatically preserving the aspect ratio.
- **True Color Support:** High-fidelity 24-bit true color printing.
- **Transparent Backgrounds:** Optional support for transparent backgrounds.
- **Highly Optimized Binary:** Configured with Link-Time Optimization (LTO) and symbol stripping for a minimal and fast executable.

---

## Installation

### Prerequisites

Make sure you have Rust and `cargo` installed. If not, install them from [rustup.rs](https://rustup.rs/).

### Build from Source

Clone the repository and build the release binary:

```bash
git clone <repository-url>
cd term-img
cargo build --release
```

The optimized executable will be available at:
`target/release/term-img`

To install it to your cargo bin directory (so you can run `term-img` anywhere):

```bash
cargo install --path .
```

---

## Usage

Run the tool by providing a local file path or a URL as the source:

```bash
term-img <source> [options]
```

### Examples

#### 1. Display a local image
```bash
term-img ./assets/ferris.png
```

#### 2. Display a remote image from a URL
```bash
term-img https://www.rust-lang.org/static/images/rust-logo-blk.svg
```

#### 3. Resize the output width
Resize the display width to 100 columns (the height will be automatically adjusted to preserve the aspect ratio):
```bash
term-img ./photo.jpg --width 100
```
or using the short flag:
```bash
term-img ./photo.jpg -w 100
```

#### 4. Enable transparent background
If your image has transparent parts and you want your terminal's background to show through:
```bash
term-img ./logo.png --transparent
```

---

## Command-Line Options

```text
Display images in your terminal from path or URL

Usage: term-img [OPTIONS] <SOURCE>

Arguments:
  <SOURCE>  Path to image file or URL

Options:
  -w, --width <WIDTH>  Resize width (in terminal columns) [default: 80]
      --color          Display in full color (24-bit) [default: true]
      --transparent    Use transparent background
  -h, --help           Print help
  -V, --version        Print version
```

---

## License

This project is licensed under the MIT License (or whichever license your repository uses). Refer to your project's license guidelines for details.
