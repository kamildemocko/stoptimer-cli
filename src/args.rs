use std::process;

use clap::Parser;

use crate::themes::factory::ThemeType;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value="default")]
    theme: String,
    #[arg(short, long)]
    list_themes: bool,
    #[arg(short, long)]
    version: bool,
}

pub struct ArgParser{
    cli: Cli,
}

impl ArgParser {
    pub fn new() -> Self {
        ArgParser{ cli: Cli::parse() }
    }

    pub fn version_requested(&self) -> bool {
        self.cli.version
    }

    pub fn parse_requested_theme(&self) -> ThemeType {
        if self.cli.list_themes {
            let available = ThemeType::available_themes();
            println!("Available themes: {}", available.join(", "));
            process::exit(0)
        }

        ThemeType::from_str(&self.cli.theme)
    }
}