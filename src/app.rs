use std::{
    io, process, time::{self, Duration, Instant}
};

use crate::{themes::model::Theme, ui::{terminal::TerminalUI, trait_def::UI}};

pub struct App<T: Theme> {
    pub is_running: bool,
    start_time: Instant,
    accumulated_time: Duration,
    ui: TerminalUI<T>
}

impl<T: Theme> App<T> {
    pub fn new(theme: T) -> Self {
        Self {
            start_time: Instant::now(),
            is_running: true,
            accumulated_time: Duration::from_millis(0),
            ui: TerminalUI::new(theme),
        }
    }

    pub fn init(&self) -> io::Result<()> {
        self.ui.prepare_screen()
    }

    pub fn print_one(&mut self) -> io::Result<()> {
        self.ui.print(&self.start_time.elapsed())
    }

    pub fn elapsed(&self) -> time::Duration {
        self.start_time.elapsed()
    }

    pub fn toggle_pause(&mut self) -> io::Result<()> {
        self.is_running = !self.is_running;
        self.accumulated_time += self.elapsed();

        self.ui.pause_screen()
    }

    pub fn quit(&mut self) -> io::Result<()> {
        self.ui.quit_screen()?;
        process::exit(0)
    }
}
