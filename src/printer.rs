use std::{
    io::{self, Write},
    time::Duration,
};

use crossterm::{
    cursor::{self, MoveTo},
    execute,
    style::Print,
    terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType},
};

use crate::themes::model::Theme;

pub struct Printer<T: Theme> {
    stdout: io::Stdout,
    theme: T,
}

impl<T: Theme> Printer<T> {
    pub fn new(theme: T) -> Self {
        Self {
            stdout: io::stdout(),
            theme,
        }
    }

    pub fn reset_screen(&self) -> io::Result<()> {
        let mut stdout = io::stdout();

        enable_raw_mode()?;

        execute!(stdout, Clear(ClearType::All), cursor::Hide, MoveTo(0, 3),)?;

        stdout.flush()
    }

    pub fn quit_screen(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::Show,
            MoveTo(0, 3),
            Clear(ClearType::CurrentLine),
            Print(">> quit"),
            MoveTo(0, 4),
        )?;

        disable_raw_mode()?;

        self.stdout.flush()
    }

    pub fn pause_screen(&mut self) -> io::Result<()> {
        execute!(
            self.stdout,
            MoveTo(0, 3),
            Clear(ClearType::CurrentLine),
            Print(">> paused")
        )?;

        self.stdout.flush()
    }

    pub fn print(&mut self, duration: &Duration) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0, 1),
            Clear(ClearType::CurrentLine),
            Print(self.theme.format(&duration)),
            MoveTo(0, 3),
            Clear(ClearType::CurrentLine),
            Print(">> running")
        )?;

        self.stdout.flush()
    }
}
