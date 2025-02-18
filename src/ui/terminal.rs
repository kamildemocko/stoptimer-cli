use std::{io::{self, Write}, time::Duration};

use crossterm::{cursor::{self, MoveTo}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType}};

use crate::themes::model::Theme;

use super::trait_def::UI;

pub struct TerminalUI<T: Theme> {
    stdout: io::Stdout,
    theme: T,
    split_counter: u16,
}

impl<T: Theme> TerminalUI<T> {
    pub fn new(theme: T) -> Self {
            Self {
                stdout: io::stdout(),
                theme,
                split_counter: 0,
            }
        }
}


impl<T: Theme> UI for TerminalUI<T> {
        fn init_screen(&self) -> io::Result<()> {
            let mut stdout = io::stdout();

            enable_raw_mode()?;

            execute!(stdout, Clear(ClearType::All), cursor::Hide, MoveTo(0, 3),)?;

            stdout.flush()
        }

        fn quit_screen(&mut self) -> io::Result<()> {
            execute!(
                self.stdout,
                cursor::Show,
                MoveTo(0, self.split_counter + 3),
                Clear(ClearType::CurrentLine),
                Print(">> quit"),
                MoveTo(0, self.split_counter + 4),
            )?;

            disable_raw_mode()?;

            self.stdout.flush()
        }

        fn add_split(&mut self) -> io::Result<()> {
            self.split_counter += 1;

            io::Result::Ok(())
        }

        fn pause_screen(&mut self) -> io::Result<()> {
            execute!(
                self.stdout,
                MoveTo(0, self.split_counter + 3),
                Clear(ClearType::CurrentLine),
                Print(">> paused")
            )?;

            self.stdout.flush()
        }

        fn print(&mut self, duration: &Duration, splits: &Vec<Duration>) -> io::Result<()> {
            for (i, split) in splits.iter().enumerate() {
                let s = format!("> {}:\t{}\n", i + 1, self.theme.format(split));

                execute!(
                    self.stdout,
                    cursor::MoveTo(0, i as u16),
                    Print(s)
                )?;
            }

            execute!(
                self.stdout,
                cursor::MoveTo(0, self.split_counter + 1),
                Clear(ClearType::CurrentLine),
                Print(self.theme.format(&duration)),
                MoveTo(0, self.split_counter + 3),
                Clear(ClearType::CurrentLine),
                Print(">> running")
            )?;

            self.stdout.flush()
        }
}
