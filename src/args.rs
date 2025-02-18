use clap::Parser;

use crate::themes::factory::ThemeType;

#[derive(Parser)]
pub struct Cli {
    #[arg(short, long, default_value="default")]
    theme: String
}

pub fn parse_requested_theme() -> ThemeType {
    let args = Cli::parse();

    match args.theme.as_str() {
        "detail" => ThemeType::Detail,
        "compact" => ThemeType::Compact,
        _ => ThemeType::Default,
    }
}