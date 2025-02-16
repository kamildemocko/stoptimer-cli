use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

use crate::themes::model::Theme;

const TICK: Duration = Duration::from_millis(8);

pub fn loop_print(theme: impl Theme) {
    let now = Instant::now();

    println!(""); // just an empty line

    loop {
        let since = now.elapsed();

        println!("{}", theme.format(&since));
        _ = io::stdout().flush();

        thread::sleep(TICK);

        print!("\x1b[1A\x1b[2K"); // move up 1 line and clear it and move down
    }
}
