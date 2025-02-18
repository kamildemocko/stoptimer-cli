use std::time::Duration;

use crate::themes::model::Theme;

#[derive(Default)]
pub struct BinaryTheme {}

impl Theme for BinaryTheme {
    fn new() -> Self {
        BinaryTheme {}
    }

    fn format(&self, dur: &Duration) -> String {

        let millis = dur.as_millis();
        let s = millis as f32 / 1000_f32;
        let m = millis as f32 / (1000_f32 * 60_f32);
        let h = millis as f32 / (1000_f32 * 60_f32 * 60_f32);

        format!(
            "{}-{}-{}-{}",
            BinaryTheme::get_binary_repr6(h as u8 % 60),
            BinaryTheme::get_binary_repr6(m as u8 % 60),
            BinaryTheme::get_binary_repr6(s as u8 % 60),
            BinaryTheme::get_binary_repr4((s.fract() * 10_f32) as u8),
        )
    }
}

impl BinaryTheme {
    fn get_binary_repr6(num: u8) -> String {
        let on = '\u{2591}';
        let off = '\u{2593}';

        let n6 = if (num / 32) % 2 == 0 { on } else { off };
        let n5 = if (num / 16) % 2 == 0 { on } else { off };
        let n4 = if (num / 8) % 2 == 0 { on } else { off };
        let n3 = if (num / 4) % 2 == 0 { on } else { off };
        let n2 = if (num / 2) % 2 == 0 { on } else { off };
        let n1 = if num % 2 == 0 { on } else { off };

        format!("{}{}{}{}{}{}", n6, n5, n4, n3, n2, n1)
    }

    fn get_binary_repr4(num: u8) -> String {
        let on = '\u{2591}';
        let off = '\u{2593}';

        let n4 = if (num / 8) % 2 == 0 { on } else { off };
        let n3 = if (num / 4) % 2 == 0 { on } else { off };
        let n2 = if (num / 2) % 2 == 0 { on } else { off };
        let n1 = if num % 2 == 0 { on } else { off };

        format!("{}{}{}{}", n4, n3, n2, n1)
    }
}
