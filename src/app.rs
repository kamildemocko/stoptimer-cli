use std::{
    process,
    time::{self, Duration, Instant},
};

use crate::{printer::Printer, themes::model::Theme};

pub struct App<T: Theme> {
    start_time: Instant,
    pub is_running: bool,
    accumulated_time: Duration,
    pub printer: Printer<T>,
}

impl<T: Theme> App<T> {
    pub fn new(theme: T) -> Self {
        Self {
            start_time: Instant::now(),
            is_running: true,
            accumulated_time: Duration::from_millis(0),
            printer: Printer::new(theme),
        }
    }

    pub fn elapsed(&self) -> time::Duration {
        self.start_time.elapsed()
    }

    pub fn toggle_pause(&mut self) {
        self.is_running = !self.is_running;
        self.accumulated_time += self.elapsed();

        self.printer.pause_screen().unwrap();
    }

    pub fn quit(&mut self) {
        self.printer.quit_screen().unwrap();
        process::exit(0)
    }
}
