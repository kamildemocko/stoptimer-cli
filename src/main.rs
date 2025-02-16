use std::{
    io::{self, Write},
    thread,
    time::{Duration, Instant},
};

const TICK: Duration = Duration::from_millis(8);

fn main() {
    let now = Instant::now();
    loop {
        let since = now.elapsed();
        // print!("\x08\r{:?}", since);
        print!("\r{:?}", since);
        _ = io::stdout().flush();

        thread::sleep(TICK);
    }
}
