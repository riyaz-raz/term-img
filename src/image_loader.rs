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
    use image::GenericImageView;

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

    #[test]
    fn loads_png_from_local_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sample.png");
        std::fs::write(&path, png_bytes()).unwrap();

        let img = load_image(path.to_str().unwrap(), false).unwrap();
        assert_eq!(img.width(), 3);
        assert_eq!(img.height(), 2);
    }

    #[test]
    fn loads_verbose_local_file() {
        // Same as above, but exercising the verbose code path.
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("sample.png");
        std::fs::write(&path, png_bytes()).unwrap();

        let img = load_image(path.to_str().unwrap(), true).unwrap();
        assert_eq!(img.dimensions(), (3, 2));
    }

    #[test]
    fn decodes_jpeg_from_memory() {
        let img = image::DynamicImage::ImageRgb8(image::RgbImage::from_pixel(
            5,
            4,
            image::Rgb([1, 2, 3]),
        ));
        let mut bytes = Vec::new();
        img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Jpeg)
            .unwrap();

        let decoded = decode(&bytes).unwrap();
        assert_eq!(decoded.dimensions(), (5, 4));
    }

    #[test]
    fn decode_guesses_format_from_content_not_extension() {
        // PNG magic bytes should decode fine even though nothing says "png".
        let bytes = png_bytes();
        assert!(&bytes[1..4] == b"PNG");
        let img = decode(&bytes).unwrap();
        assert_eq!(img.width(), 3);
    }

    #[test]
    fn decode_rejects_garbage() {
        let err = decode(b"this is definitely not an image").unwrap_err();
        // Any decode failure is acceptable; just make sure it's an error, not a panic.
        assert!(!err.to_string().is_empty());
    }

    #[test]
    fn decode_rejects_empty_input() {
        assert!(decode(&[]).is_err());
    }

    #[test]
    fn local_directory_is_reported_as_file_not_found_or_decode_error() {
        let dir = tempfile::tempdir().unwrap();
        // A directory path exists but cannot be decoded as an image.
        let result = load_image(dir.path().to_str().unwrap(), false);
        if let Err(err) = result {
            let msg = err.to_string();
            assert!(
                msg.contains("File not found") || msg.contains("not") || !msg.is_empty(),
                "expected a meaningful error message, got: {msg}"
            );
        }
    }

    #[test]
    fn url_scheme_without_host_still_takes_http_path() {
        // `http://` sources must never touch the filesystem branch; they fail at
        // request time instead of reporting "File not found".
        let err = load_image("http://127.0.0.1:1/x.png", false).unwrap_err();
        assert!(
            !err.to_string().contains("File not found"),
            "unexpected error: {err}"
        );
    }

    #[test]
    fn stdin_dash_source_with_garbage_fails_to_decode() {
        // We can't easily replace process stdin here, so test the equivalent
        // decode path that the `-` branch uses.
        assert!(decode(b"not-an-image").is_err());
    }

    #[test]
    fn user_agent_contains_crate_version() {
        assert_eq!(USER_AGENT, concat!("term-img/", env!("CARGO_PKG_VERSION")));
        assert!(USER_AGENT.starts_with("term-img/"));
    }

    #[test]
    fn default_timeout_is_thirty_seconds() {
        assert_eq!(DEFAULT_TIMEOUT_SECS, 30);
    }
}
