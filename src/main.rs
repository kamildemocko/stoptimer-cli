mod app;
mod printer;
mod themes;

use std::{thread, time::Duration};

use crate::app::App;
use crate::themes::{default::DefaultTheme, model::Theme};

const TICK: Duration = Duration::from_millis(8);

fn main() {
    let theme = DefaultTheme::new();
    let mut app = App::new(theme);

    loop {
        app.printer.print(&app.elapsed());
        thread::sleep(TICK);
    }
}
