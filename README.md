# term-img 🖼️

`term-img` is a lightweight, fast, and optimized command-line tool written in Rust to display images directly inside your terminal. It supports loading images from local file paths, remote URLs, and stdin.

Under the hood, it utilizes [`viuer`](https://github.com/atanunq/viuer) and the [`image`](https://github.com/image-rs/image) crate to render images using modern terminal protocols (like Kitty, iTerm2, or half-block fallback characters) with full 24-bit true color support.

---

## Features

- **Multiple Sources:** Display local files, fetch and display images directly from any HTTP/HTTPS URL, or read an image from stdin.
- **Terminal Auto-fit:** By default the image is scaled to the width of your terminal; override with `--width`/`--height`.
- **Aspect-Ratio Preserved Scaling:** Custom scale your images by specifying width while automatically preserving the aspect ratio.
- **True Color Support:** High-fidelity 24-bit true color printing, with an opt-out via `--no-color`.
- **Protocol Control:** Force Kitty, iTerm2, or half-block rendering with `--protocol`.
- **Transparent Backgrounds:** Optional support for transparent backgrounds.
- **Resilient Fetching:** URL downloads use a request timeout and identify themselves with a user agent.
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

Run the tool by providing a local file path, a URL, or `-` for stdin:

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
term-img https://www.rustacean.net/assets/rustacean-flat-gesture.png
```

#### 3. Read an image from stdin
```bash
cat image.png | term-img -
```

#### 4. Resize the output
Resize the display width to 100 columns (the height is automatically adjusted to preserve the aspect ratio):
```bash
term-img ./photo.jpg --width 100
```
or using the short flag:
```bash
term-img ./photo.jpg -w 100
```

#### 5. Force a rendering protocol
```bash
term-img ./logo.png --protocol kitty
```

#### 6. Transparent background and no true color
If your image has transparent parts and you want your terminal's background to show through:
```bash
term-img ./logo.png --transparent
term-img ./logo.png --no-color
```

---

## Command-Line Options

```text
Display images in your terminal from a path, URL, or stdin

Usage: term-img [OPTIONS] [SOURCE]

Arguments:
  [SOURCE]  Path to image file, URL, or `-` to read from stdin

Options:
  -w, --width <WIDTH>      Resize width (in terminal columns). Defaults to the terminal width.
      --height <HEIGHT>    Resize height (in terminal rows)
      --color[=<COLOR>]    Display in full color (24-bit). Use `--no-color` to disable. [default: true]
      --no-color           Disable 24-bit true color output
      --transparent        Use a transparent background instead of a checkerboard
      --protocol <PROTOCOL>  Force a specific rendering protocol [default: auto] [possible values: auto, kitty, iterm, halfblocks]
  -v, --verbose            Print status messages to stderr
  -h, --help               Print help
  -V, --version            Print version
```

---

## License

This project is licensed under the MIT License — see the [LICENSE](LICENSE) file for details.
