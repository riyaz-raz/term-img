use clap::Parser;
use image::io::Reader as ImageReader;
use std::io::Cursor;
use std::path::PathBuf;
use viuer::{print, Config};

#[derive(Parser)]
#[command(name = "term-img")]
#[command(
    about = "Display images in your terminal from path or URL",
    version = "0.1.0",
    long_about = None,
)]
struct Args {
    /// Path to image file or URL
    #[arg(default_value = "")]
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

    // Show help if no source provided
    if args.source.trim().is_empty() {
        print_help();
        return Ok(());
    }

    // Load image from URL or file
    let img = if args.source.starts_with("http://") || args.source.starts_with("https://") {
        println!("📡 Fetching image from URL...");
        let response = reqwest::blocking::get(&args.source)?;

        if !response.status().is_success() {
            anyhow::bail!(
                "❌ HTTP error: {} - Failed to fetch {}",
                response.status(),
                args.source
            );
        }

        let bytes = response.bytes()?;

        if bytes.is_empty() {
            anyhow::bail!("❌ Downloaded file is empty: {}", args.source);
        }

        ImageReader::new(Cursor::new(bytes))
            .with_guessed_format()?
            .decode()?
    } else {
        let path = PathBuf::from(&args.source);
        if !path.exists() {
            anyhow::bail!("❌ File not found: {}", args.source);
        }

        if !path.is_file() {
            anyhow::bail!("❌ Path is a directory, not a file: {}", args.source);
        }

        println!("📁 Loading image from file...");
        ImageReader::open(path)?.decode()?
    };

    // Configure and display
    let config = Config {
        width: Some(args.width),
        height: None,
        x: 0,
        y: 0,
        transparent: args.transparent,
        truecolor: args.color,
        absolute_offset: false,
        ..Default::default()
    };

    println!("\n🖼️  Displaying image (width: {} cols):\n", args.width);
    print(&img, &config)?;
    println!("\n✅ Done!");

    Ok(())
}

fn print_help() {
    eprintln!("🖼️  term-img - Display images in your terminal");
    eprintln!("");
    eprintln!("❌ Error: No image source provided");
    eprintln!("");
    eprintln!("💡 Usage: term-img <image_path_or_url> [options]");
    eprintln!("");
    eprintln!("📁 Local file examples:");
    eprintln!("   term-img ~/Pictures/photo.jpg");
    eprintln!("   term-img ./image.png");
    eprintln!("   term-img C:\\Users\\User\\Pictures\\image.jpg");
    eprintln!("");
    eprintln!("🌐 URL examples:");
    eprintln!("   term-img https://example.com/image.jpg");
    eprintln!("   term-img https://www.rustacean.net/assets/rustacean-flat-gesture.png");
    eprintln!("");
    eprintln!("⚙️  Options:");
    eprintln!("   -w, --width <cols>  Set display width in columns (default: 80)");
    eprintln!("   --color             Enable true color (default: true)");
    eprintln!("   --transparent       Use transparent background");
    eprintln!("   -h, --help          Print help");
    eprintln!("   -V, --version       Print version");
    eprintln!("");
    eprintln!("📖 For more help: term-img --help");
}
