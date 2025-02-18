use std::time::Duration;

use crate::themes::model::Theme;

#[derive(Default)]
pub struct CompactTheme {}

impl Theme for CompactTheme {
    fn new() -> Self {
        CompactTheme {}
    }

    fn format(&self, dur: &Duration) -> String {
        let millis = dur.as_millis();
        let s = millis as f32 / 1000_f32;
        let m = millis as f32 / (1000_f32 * 60_f32);
        let h = millis as f32 / (1000_f32 * 60_f32 * 60_f32);

        format!(
            "{}h {}m {}s {:0>3.0}ms",
            h as u8,
            m as u8 % 60,
            s as u8 % 60,
            s.fract() * 1000_f32
        )
    }
}
