use std::time::Duration;

use crate::themes::model::Theme;

#[derive(Default)]
pub struct DefaultTheme {}

impl Theme for DefaultTheme {
    fn new() -> Self {
        DefaultTheme {}
    }

    fn format(&self, dur: &Duration) -> String {
        let millis = dur.as_millis();
        let s = millis as f32 / 1000_f32;
        let m = millis as f32 / (1000_f32 * 60_f32);
        let h = millis as f32 / (1000_f32 * 60_f32 * 60_f32);

        format!(
            "{} : {:0>2} : {:0>2} : {:0>3.0}",
            h as u8,
            m as u8 % 60,
            s as u8 % 60,
            s.fract() * 1000_f32
        )
    }
}
