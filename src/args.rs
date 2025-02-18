use std::process;

use clap::Parser;

use crate::themes::factory::ThemeType;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value="default")]
    theme: String,
    #[arg(short, long)]
    list_themes: bool,
}

pub fn parse_requested_theme() -> ThemeType {
    let args = Cli::parse();

    if args.list_themes {
        let available = ThemeType::available_themes();
        println!("Available themes: {}", available.join(", "));
        process::exit(0)
    }

    ThemeType::from_str(&args.theme)
}