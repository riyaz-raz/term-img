use clap::Parser;

#[derive(Parser)]
#[command(name = "term-img")]
#[command(
    about = "Display images in your terminal from path or URL",
    version = "0.1.0",
    long_about = None,
)]
pub struct Args {
    /// Path to image file or URL
    #[arg(default_value = "")]
    pub source: String,

    /// Resize width (in terminal columns)
    #[arg(short, long, default_value_t = 80)]
    pub width: u32,

    /// Display in full color (24-bit)
    #[arg(long, default_value_t = true)]
    pub color: bool,

    /// Use transparent background
    #[arg(long, default_value_t = false)]
    pub transparent: bool,
}
