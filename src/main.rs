mod args;
mod help;
mod image_loader;

use args::{Args, Protocol};
use clap::Parser;
use viuer::{print, Config};

fn main() {
    std::process::exit(run());
}

fn run() -> i32 {
    let args = Args::parse();

    // Show custom help message if no image source is provided
    if args.source.trim().is_empty() {
        help::print_help();
        return 2;
    }

    if args.width == Some(0) {
        eprintln!("❌ Error: --width must be greater than 0");
        return 2;
    }
    if args.height == Some(0) {
        eprintln!("❌ Error: --height must be greater than 0");
        return 2;
    }

    // Load the image using our image loader module
    let img = match image_loader::load_image(&args.source, args.verbose) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("❌ Error: {err}");
            return 1;
        }
    };

    // Configure how to display
    let mut config = Config {
        width: args.width,
        height: args.height,
        transparent: args.transparent,
        truecolor: args.color && !args.no_color,
        absolute_offset: false,
        ..Default::default()
    };

    match args.protocol {
        Protocol::Auto => {}
        Protocol::Kitty => {
            config.use_kitty = true;
            config.use_iterm = false;
        }
        Protocol::Iterm => {
            config.use_kitty = false;
            config.use_iterm = true;
        }
        Protocol::Halfblocks => {
            config.use_kitty = false;
            config.use_iterm = false;
        }
    }

    if args.verbose {
        let width = args
            .width
            .map(|w| w.to_string())
            .unwrap_or_else(|| "auto".to_string());
        eprintln!("🖼️  Displaying image (width: {width} cols)...");
    }

    // Display the image
    if let Err(err) = print(&img, &config) {
        eprintln!("❌ Error: {err}");
        return 1;
    }

    if args.verbose {
        eprintln!("✅ Done!");
    }

    0
}
