use image::ImageReader;
use std::io::{Cursor, Read};
use std::path::PathBuf;
use std::time::Duration;

const USER_AGENT: &str = concat!("term-img/", env!("CARGO_PKG_VERSION"));
const DEFAULT_TIMEOUT_SECS: u64 = 30;

/// Loads an image from stdin (`-`), a URL, or a local file path.
pub fn load_image(source: &str, verbose: bool) -> anyhow::Result<image::DynamicImage> {
    if source == "-" {
        if verbose {
            eprintln!("📥 Reading image from stdin...");
        }
        let mut bytes = Vec::new();
        std::io::stdin().read_to_end(&mut bytes)?;
        return decode(&bytes);
    }

    if source.starts_with("http://") || source.starts_with("https://") {
        if verbose {
            eprintln!("📡 Fetching image from URL...");
        }
        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(DEFAULT_TIMEOUT_SECS))
            .user_agent(USER_AGENT)
            .build()?;
        let response = client.get(source).send()?.error_for_status()?;
        let bytes = response.bytes()?;
        return decode(&bytes);
    }

    let path = PathBuf::from(source);
    if !path.exists() {
        anyhow::bail!("File not found: {}", source);
    }
    if verbose {
        eprintln!("📁 Loading image from file...");
    }
    let img = ImageReader::open(path)?.with_guessed_format()?.decode()?;
    Ok(img)
}

/// Decodes an in-memory image, guessing the format from its content.
fn decode(bytes: &[u8]) -> anyhow::Result<image::DynamicImage> {
    let img = ImageReader::new(Cursor::new(bytes))
        .with_guessed_format()?
        .decode()?;
    Ok(img)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn png_bytes() -> Vec<u8> {
        let img = image::DynamicImage::ImageRgba8(image::RgbaImage::from_pixel(
            3,
            2,
            image::Rgba([10, 20, 30, 255]),
        ));
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
            .unwrap();
        bytes
    }

    #[test]
    fn decodes_png_from_memory() {
        let bytes = png_bytes();
        let img = decode(&bytes).unwrap();
        assert_eq!(img.width(), 3);
        assert_eq!(img.height(), 2);
    }

    #[test]
    fn missing_file_is_an_error() {
        let err = load_image("/definitely/not/here_12345.png", false).unwrap_err();
        assert!(err.to_string().contains("File not found"));
    }
}
