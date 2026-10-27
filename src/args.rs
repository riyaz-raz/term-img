use clap::{Parser, ValueEnum};

/// Terminal image protocol to force when displaying.
#[derive(Copy, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub enum Protocol {
    /// Detect the best protocol for the current terminal.
    Auto,
    /// Force the Kitty graphics protocol.
    Kitty,
    /// Force the iTerm2 inline image protocol.
    Iterm,
    /// Force half-block character rendering.
    Halfblocks,
}

#[derive(Parser)]
#[command(name = "term-img")]
#[command(
    about = "Display images in your terminal from a path, URL, or stdin",
    version = env!("CARGO_PKG_VERSION"),
    long_about = None,
)]
pub struct Args {
    /// Path to image file, URL, or `-` to read from stdin
    #[arg(default_value = "")]
    pub source: String,

    /// Resize width (in terminal columns). Defaults to the terminal width.
    #[arg(short = 'w', long)]
    pub width: Option<u32>,

    /// Resize height (in terminal rows)
    #[arg(long)]
    pub height: Option<u32>,

    /// Display in full color (24-bit). Use `--no-color` to disable.
    #[arg(
        long,
        default_value_t = true,
        num_args = 0..=1,
        default_missing_value = "true",
        require_equals = true,
        action = clap::ArgAction::Set
    )]
    pub color: bool,

    /// Disable 24-bit true color output
    #[arg(long)]
    pub no_color: bool,

    /// Use a transparent background instead of a checkerboard
    #[arg(long)]
    pub transparent: bool,

    /// Force a specific rendering protocol
    #[arg(long, value_enum, default_value_t = Protocol::Auto)]
    pub protocol: Protocol,

    /// Print status messages to stderr
    #[arg(short, long)]
    pub verbose: bool,
}

// Unit tests live in a separate file so this module stays free of test code.
// Compiled only for `cargo test`; never included in release/debug builds.
#[cfg(test)]
mod tests;
