use std::{
    io::{self, Write},
    time::Duration,
};

use crate::themes::model::Theme;

pub struct Printer<T: Theme> {
    first_run: bool,
    theme: T,
}

impl<T: Theme> Printer<T> {
    pub fn new(theme: T) -> Self {
        Self {
            first_run: true,
            theme,
        }
    }

    pub fn print(&mut self, duration: &Duration) {
        if self.first_run {
            println!();
            self.first_run = false;
        }

        println!("{}", self.theme.format(&duration));
        _ = io::stdout().flush();
        print!("\x1b[1A\x1b[2K");
    }
}
