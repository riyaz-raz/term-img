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

// Unit tests live in a separate file so this module stays free of test code.
// Compiled only for `cargo test`; never included in release/debug builds.
#[cfg(test)]
mod tests;
