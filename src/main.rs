mod args;
mod help;
mod image_loader;

use args::Args;
use clap::Parser;
use viuer::{print, Config};

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // Show custom help message if no image source is provided
    if args.source.trim().is_empty() {
        help::print_help();
        return Ok(());
    }

    // Load the image using our image loader module
    let img = image_loader::load_image(&args.source)?;

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
