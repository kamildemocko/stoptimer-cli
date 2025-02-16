use std::time::{self, Instant};

use crate::{printer::Printer, themes::model::Theme};

pub struct App<T: Theme> {
    start_time: Instant,
    is_running: bool,
    pub printer: Printer<T>,
}

impl<T: Theme> App<T> {
    pub fn new(theme: T) -> Self {
        Self {
            start_time: Instant::now(),
            is_running: true,
            printer: Printer::new(theme),
        }
    }

    pub fn elapsed(&self) -> time::Duration {
        self.start_time.elapsed()
    }

    pub fn toggle_pause() {
        todo!()
    }
}
