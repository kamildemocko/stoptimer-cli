use std::{
    io, process, thread, time::{Duration, Instant}
};

use crate::{themes::model::Theme, ui::{terminal::TerminalUI, trait_def::UI}};

pub struct App<T: Theme> {
    pub is_running: bool,
    start_time: Instant,
    paused_duration: Duration,
    paused_time: Instant,
    splits: Vec<Duration>,
    ui: TerminalUI<T>
}

impl<T: Theme> App<T> {
    pub fn new(theme: T) -> Self {
        Self {
            start_time: Instant::now(),
            is_running: true,
            paused_duration: Duration::from_millis(0),
            paused_time: Instant::now(),
            splits: vec!(),
            ui: TerminalUI::new(theme),
        }
    }

    pub fn init(&self) -> io::Result<()> {
        self.ui.init_screen()
    }

    pub fn print_one(&mut self) -> io::Result<()> {
        let wo_pause = self.start_time.elapsed() - self.paused_duration;
        self.ui.print(&wo_pause, &self.splits)
    }

    pub fn help(&mut self) -> io::Result<()> {
        self.ui.print_help()?;
        thread::sleep(Duration::from_secs(3));

        io::Result::Ok(())
    }

    pub fn split(&mut self) -> io::Result<()> {
        self.splits.push(self.start_time.elapsed() - self.paused_duration);
        self.ui.add_split();
        self.ui.init_screen()
    }

    pub fn toggle_pause(&mut self) -> io::Result<()> {
        if self.is_running {
            self.ui.pause_screen()?;
            self.paused_time = Instant::now();
        } else {
            self.paused_duration += self.paused_time.elapsed();

        }

        self.is_running = !self.is_running;
        io::Result::Ok(())
    }

    pub fn quit(&mut self) -> io::Result<()> {
        self.ui.quit_screen()?;
        process::exit(0)
    }
}
