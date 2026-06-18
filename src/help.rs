/// Prints custom user-friendly help text when no source is provided.
pub fn print_help() {
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
