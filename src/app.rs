use std::{
    io, process, time::{Duration, Instant}
};

use crate::{themes::model::Theme, ui::{terminal::TerminalUI, trait_def::UI}};

pub struct App<T: Theme> {
    pub is_running: bool,
    start_time: Instant,
    paused_time: Duration,
    pause_time: Instant,
    ui: TerminalUI<T>
}

impl<T: Theme> App<T> {
    pub fn new(theme: T) -> Self {
        Self {
            start_time: Instant::now(),
            is_running: true,
            paused_time: Duration::from_millis(0),
            pause_time: Instant::now(),
            ui: TerminalUI::new(theme),
        }
    }

    pub fn init(&self) -> io::Result<()> {
        self.ui.prepare_screen()
    }

    pub fn print_one(&mut self) -> io::Result<()> {
        let wo_pause = self.start_time.elapsed() - self.paused_time;
        self.ui.print(&wo_pause)
    }

    pub fn toggle_pause(&mut self) -> io::Result<()> {
        if self.is_running {
            self.ui.pause_screen()?;
            self.pause_time = Instant::now();
        } else {
            self.paused_time += self.pause_time.elapsed();

        }

        self.is_running = !self.is_running;
        io::Result::Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        self.ui.quit_screen()?;
        process::exit(0)
    }
}
