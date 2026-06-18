use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::path::PathBuf;

/// Loads an image from either a URL or a local file path.
pub fn load_image(source: &str) -> anyhow::Result<image::DynamicImage> {
    if source.starts_with("http://") || source.starts_with("https://") {
        println!("📡 Fetching image from URL...");
        let response = reqwest::blocking::get(source)?;
        let bytes = response.bytes()?;
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        Ok(img)
    } else {
        let path = PathBuf::from(source);
        if !path.exists() {
            anyhow::bail!("File not found: {}", source);
        }
        println!("📁 Loading image from file...");
        let img = ImageReader::open(path)?.decode()?;
        Ok(img)
    }
}
