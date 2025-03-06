mod args;
mod app;
mod themes;
mod ui;

use std::io;
use std::time::Instant;
use std::{thread, time::Duration};

use crossterm::event::{self, Event};

use args::ArgParser;
use app::App;
use themes::factory::ThemeFactory;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const TICK: Duration = Duration::from_millis(8);
const DEBOUNCE_DURATION: Duration = Duration::from_millis(150);


fn main() -> io::Result<()> {
    let argp = ArgParser::new();
    if argp.version_requested() {
        println!("{}: {}", NAME, VERSION);
        std::process::exit(0);
    }

    let requested_theme = argp.parse_requested_theme();
    let theme = ThemeFactory::create(requested_theme);

    let mut app = App::new(theme);
    let mut last_key_time = Instant::now();

    app.init()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                let now = Instant::now();
                if now.duration_since(last_key_time) < DEBOUNCE_DURATION { continue }

                match key_event.code {
                    event::KeyCode::Char('q') => {
                        app.quit()?;
                    }
                    event::KeyCode::Char('h') => {
                        app.help()?;
                        last_key_time = Instant::now();
                    }
                    event::KeyCode::Char(' ') => {
                        app.toggle_pause()?;
                        last_key_time = Instant::now();
                    }
                    event::KeyCode::Char('s') => {
                        app.split()?;
                        last_key_time = Instant::now();
                    }
                    _ => {}
                }
            }
        }

        if app.is_running {
            app.print_one()?;
        }

        thread::sleep(TICK);
    }
}
