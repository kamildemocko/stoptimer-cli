use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

const TICK: Duration = Duration::from_millis(8);

trait Theme {
    fn format(&self, dur: &Duration) -> String;
}

struct DefaultTheme {}

impl Theme for DefaultTheme {
    fn format(&self, dur: &Duration) -> String {
        let millis = dur.as_millis();
        let s = millis as f32 / 1000_f32;
        let m = millis as f32 / (1000_f32 * 60_f32);
        let h = millis as f32 / (1000_f32 * 60_f32 * 60_f32);

        format!(
            "{} : {} : {:0>2} : {:0>3.0}",
            h as u8,
            m as u8 % 60,
            s as u8 % 60,
            s.fract() * 1000_f32
        )
    }
}

impl DefaultTheme {
    fn new() -> Self {
        DefaultTheme {}
    }
}

fn print_once(text: String) {
    println!("{:?}", text);
    _ = io::stdout().flush();

    thread::sleep(TICK);

    print!("\x1b[1A\x1b[2K"); // move up 1 line and clear it and move down
}

fn main() {
    let now = Instant::now();
    let theme = DefaultTheme::new();

    println!(""); // just an empty line

    loop {
        let since = now.elapsed();
        print_once(theme.format(&since));
    }
}
