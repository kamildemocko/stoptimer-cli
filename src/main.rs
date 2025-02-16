use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

const TICK: Duration = Duration::from_millis(8);

fn main() {
    let now = Instant::now();

    println!("start");
    loop {
        let since = now.elapsed();
        println!("{:?}", since);
        _ = io::stdout().flush();

        thread::sleep(TICK);

        print!("\x1b[1A\x1b[2K"); // move up 1 line and clear it and move down
    }
}
