use std::io;
use std::time::Duration;

pub trait UI {
    fn prepare_screen(&self) -> io::Result<()>;
    fn quit_screen(&mut self) -> io::Result<()>;
    fn pause_screen(&mut self) -> io::Result<()>;
    fn print(&mut self, duration: &Duration) -> io::Result<()>;
}
