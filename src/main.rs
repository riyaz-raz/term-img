use clap::Parser;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::path::PathBuf;
use viuer::{print, Config};

#[derive(Parser)]
#[command(name = "term-img")]
#[command(
    about = "Display images in your terminal from path or URL",
    version = "0.1.0"
)]
struct Args {
    /// Path to image file or URL
    source: String,

    /// Resize width (in terminal columns)
    #[arg(short, long, default_value_t = 80)]
    width: u32,

    /// Display in full color (24-bit)
    #[arg(long, default_value_t = true)]
    color: bool,

    /// Use transparent background
    #[arg(long, default_value_t = false)]
    transparent: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Load image from URL or file
    let img = if args.source.starts_with("http://") || args.source.starts_with("https://") {
        println!("📡 Fetching image from URL...");
        let response = reqwest::blocking::get(&args.source)?;
        let bytes = response.bytes()?;
        let img = ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?;
        img
    } else {
        let path = PathBuf::from(&args.source);
        if !path.exists() {
            anyhow::bail!("File not found: {}", args.source);
        }
        println!("📁 Loading image from file...");
        ImageReader::open(path)?.decode()?
    };

    // Configure how to display
    let config = Config {
        width: Some(args.width),
        height: None, // Auto-preserve aspect ratio
        x: 0,
        y: 0,
        transparent: args.transparent,
        truecolor: args.color,
        absolute_offset: false,
        ..Default::default()
    };

    // Display the image
    println!("\n🖼️  Displaying image (width: {} cols):\n", args.width);
    print(&img, &config)?;
    println!("\n✅ Done!");

    Ok(())
}
