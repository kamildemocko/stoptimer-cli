mod printer;
mod themes;

use crate::printer::loop_print;
use crate::themes::{default::DefaultTheme, model::Theme};

fn main() {
    let theme = DefaultTheme::new();

    loop_print(theme);
}
