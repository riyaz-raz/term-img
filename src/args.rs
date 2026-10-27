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

    /// Like `unwrap_err()`, but without requiring `Args: Debug`.
    fn expect_parse_error(argv: &[&str]) -> clap::Error {
        match Args::try_parse_from(argv) {
            Err(err) => err,
            Ok(_) => panic!("expected `{argv:?}` to fail parsing"),
        }
    }

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

    #[test]
    fn parses_all_protocols() {
        for (flag, expected) in [
            ("auto", Protocol::Auto),
            ("iterm", Protocol::Iterm),
            ("halfblocks", Protocol::Halfblocks),
        ] {
            let args = Args::try_parse_from(["term-img", "photo.png", "--protocol", flag]).unwrap();
            assert_eq!(args.protocol, expected, "protocol flag `{flag}`");
        }
    }

    #[test]
    fn invalid_protocol_is_rejected() {
        let err = expect_parse_error(&["term-img", "photo.png", "--protocol", "sixel"]);
        assert_eq!(err.kind(), clap::error::ErrorKind::InvalidValue);
    }

    #[test]
    fn parses_width_and_height() {
        let args =
            Args::try_parse_from(["term-img", "photo.png", "-w", "100", "--height", "42"]).unwrap();
        assert_eq!(args.width, Some(100));
        assert_eq!(args.height, Some(42));
    }

    #[test]
    fn width_accepts_zero_so_main_can_validate_it() {
        // `main` rejects zero widths with a friendly message; clap itself must not.
        let args = Args::try_parse_from(["term-img", "photo.png", "--width", "0"]).unwrap();
        assert_eq!(args.width, Some(0));
    }

    #[test]
    fn non_numeric_width_is_rejected() {
        let err = expect_parse_error(&["term-img", "photo.png", "--width", "wide"]);
        assert_eq!(err.kind(), clap::error::ErrorKind::ValueValidation);
    }

    #[test]
    fn parses_verbose_short_and_long() {
        assert!(
            Args::try_parse_from(["term-img", "p.png", "-v"])
                .unwrap()
                .verbose
        );
        assert!(
            Args::try_parse_from(["term-img", "p.png", "--verbose"])
                .unwrap()
                .verbose
        );
        assert!(!Args::try_parse_from(["term-img", "p.png"]).unwrap().verbose);
    }

    #[test]
    fn parses_transparent() {
        assert!(
            Args::try_parse_from(["term-img", "p.png", "--transparent"])
                .unwrap()
                .transparent
        );
        assert!(
            !Args::try_parse_from(["term-img", "p.png"])
                .unwrap()
                .transparent
        );
    }

    #[test]
    fn source_defaults_to_empty_string() {
        let args = Args::try_parse_from(["term-img"]).unwrap();
        assert_eq!(args.source, "");
    }

    #[test]
    fn bare_color_flag_does_not_swallow_source() {
        // Regression: `--color img.png` used to bind the positional as its value.
        let args = Args::try_parse_from(["term-img", "--color", "img.png"]).unwrap();
        assert!(args.color);
        assert_eq!(args.source, "img.png");
    }

    #[test]
    fn truecolor_expression_matches_main_logic() {
        // Mirrors `config.truecolor = args.color && !args.no_color` from `main`.
        let cases = [
            (vec!["term-img", "p.png"], true),
            (vec!["term-img", "p.png", "--no-color"], false),
            (vec!["term-img", "p.png", "--color=false"], false),
            (vec!["term-img", "p.png", "--color=true"], true),
            (
                vec!["term-img", "p.png", "--color=true", "--no-color"],
                false,
            ),
        ];
        for (argv, expected) in cases {
            let args = Args::try_parse_from(argv.clone()).unwrap();
            assert_eq!(args.color && !args.no_color, expected, "argv: {argv:?}");
        }
    }

    #[test]
    fn protocol_value_enum_round_trips() {
        use clap::ValueEnum;
        let names = Protocol::value_variants()
            .iter()
            .map(|v| v.to_possible_value().expect("each variant has a value"))
            .collect::<Vec<_>>();
        for expected in ["auto", "kitty", "iterm", "halfblocks"] {
            assert!(
                names.iter().any(|pv| pv.get_name() == expected),
                "missing possible value `{expected}`"
            );
        }
    }
}
