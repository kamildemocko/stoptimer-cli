mod themes;

use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use crate::themes::{default::DefaultTheme, model::Theme};

const TICK: Duration = Duration::from_millis(8);

fn print_once(text: String) {
    println!("{}", text);
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
