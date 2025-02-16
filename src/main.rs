use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

const TICK: Duration = Duration::from_millis(8);

// 0:00:000 (hod:sec:ms)
fn format_stoptimer(dur: &Duration) -> String {
    let millis = dur.as_millis();
    let s = millis as f32 / 1000_f32;

    format!("{} : {:0>2} : {:0>3.0}", 0, s as u8, s.fract() * 1000_f32)
}

fn main() {
    let now = Instant::now();

    println!(""); // just an empty line

    loop {
        let since = now.elapsed();
        println!("{:?}", format_stoptimer(&since));
        _ = io::stdout().flush();

        thread::sleep(TICK);

        print!("\x1b[1A\x1b[2K"); // move up 1 line and clear it and move down
    }
}
