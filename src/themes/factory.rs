use super::{impls::{compact::CompactTheme, default::DefaultTheme, details::DetailTheme}, model::Theme};


pub enum ThemeType {
    Default,
    Detail,
    Compact,
}

pub enum ThemeVariant {
    Default(DefaultTheme),
    Detail(DetailTheme),
    Compact(CompactTheme),
}

impl Theme for ThemeVariant {
    fn new() -> Self {
        ThemeVariant::Default(DefaultTheme::new())
    }

    fn format(&self, dur: &std::time::Duration) -> String {
        match self {
            ThemeVariant::Default(theme) => theme.format(dur),
            ThemeVariant::Detail(theme) => theme.format(dur),
            ThemeVariant::Compact(theme) => theme.format(dur),
        }
    }
}

impl ThemeType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "detail" => ThemeType::Detail,
            "compact" => ThemeType::Compact,
            _ => ThemeType::Default,
        }
    }

    pub fn available_themes() -> Vec<String> {
        vec![
            "default".to_string(),
            "detail".to_string(),
            "compact".to_string(),
            ]
    }
}

pub struct ThemeFactory;

impl ThemeFactory {
    pub fn create(theme_type: ThemeType) -> ThemeVariant {
        match theme_type {
            ThemeType::Default => ThemeVariant::Default(DefaultTheme::new()),
            ThemeType::Detail => ThemeVariant::Detail(DetailTheme::new()),
            ThemeType::Compact => ThemeVariant::Compact(CompactTheme::new()),
        }
    }
}
