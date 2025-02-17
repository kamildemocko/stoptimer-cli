mod app;
mod printer;
mod themes;
mod ui;

use std::io;
use std::{thread, time::Duration};

use crossterm::event::{self, Event};

use crate::app::App;
use crate::themes::{default::DefaultTheme, model::Theme};

const TICK: Duration = Duration::from_millis(8);

fn main() -> io::Result<()> {
    let theme = DefaultTheme::new();
    let mut app = App::new(theme);

    app.init()?;

    loop {
        if event::poll(Duration::from_millis(0))? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Char('q') => {
                        app.quit()?;
                    }
                    event::KeyCode::Char(' ') => {
                        app.toggle_pause()?;
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
