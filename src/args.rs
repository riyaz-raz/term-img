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

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn parses_source_and_defaults() {
        let args = Args::try_parse_from(["term-img", "photo.png"]).unwrap();
        assert_eq!(args.source, "photo.png");
        assert!(args.width.is_none());
        assert!(args.height.is_none());
        assert!(args.color);
        assert!(!args.no_color);
        assert_eq!(args.protocol, Protocol::Auto);
    }

    #[test]
    fn no_color_disables_truecolor() {
        let args = Args::try_parse_from(["term-img", "photo.png", "--no-color"]).unwrap();
        assert!(args.no_color);
        // Mirrors the effective `truecolor` value computed in `main`.
        assert!(!(args.color && !args.no_color));
    }

    #[test]
    fn color_accepts_explicit_false() {
        let args = Args::try_parse_from(["term-img", "photo.png", "--color=false"]).unwrap();
        assert!(!args.color);
    }

    #[test]
    fn color_accepts_bare_flag() {
        let args = Args::try_parse_from(["term-img", "photo.png", "--color"]).unwrap();
        assert!(args.color);
    }

    #[test]
    fn color_does_not_consume_positional() {
        let args = Args::try_parse_from(["term-img", "--color", "photo.png"]).unwrap();
        assert!(args.color);
        assert_eq!(args.source, "photo.png");
    }

    #[test]
    fn parses_protocol() {
        let args = Args::try_parse_from(["term-img", "photo.png", "--protocol", "kitty"]).unwrap();
        assert_eq!(args.protocol, Protocol::Kitty);
    }
}
