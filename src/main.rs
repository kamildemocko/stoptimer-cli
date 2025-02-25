mod args;
mod app;
mod themes;
mod ui;

use std::io;
use std::{thread, time::Duration};

use crossterm::event::{self, Event};

use args::parse_requested_theme;
use app::App;
use themes::factory::ThemeFactory;

const TICK: Duration = Duration::from_millis(8);

fn main() -> io::Result<()> {
    let requested_theme = parse_requested_theme();
    let theme = ThemeFactory::create(requested_theme);

    let mut app = App::new(theme);

    app.init()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Char('q') => {
                        app.quit()?;
                    }
                    event::KeyCode::Char('h') => {
                        app.help()?;
                    }
                    event::KeyCode::Char(' ') => {
                        app.toggle_pause()?;
                    }
                    event::KeyCode::Char('s') => {
                        app.split()?;
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
