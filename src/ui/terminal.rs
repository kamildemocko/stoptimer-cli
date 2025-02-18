use std::{io::{self, Write}, time::Duration};

use crossterm::{cursor::{self, MoveTo}, execute, style::Print, terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType, size}};

use crate::themes::model::Theme;

use super::trait_def::UI;

pub struct TerminalUI<T: Theme> {
    max_split_rows: u16,
    stdout: io::Stdout,
    theme: T,
    split_counter: u16,
}

impl<T: Theme> TerminalUI<T> {
    pub fn new(theme: T) -> Self {
            let terminal_rows = size().expect("cannot determine terminal rows").1;

            Self {
                max_split_rows: if terminal_rows  > 6 { terminal_rows - 5 } else { 0 },
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

            execute!(stdout, Clear(ClearType::All), cursor::Hide)?;

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

        // add one to split counter, but only until the max  split rows
        fn add_split(&mut self) {
            if self.split_counter >= self.max_split_rows {
                return;
            }

            self.split_counter += 1;
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
            let mut i = 0_u16;

            for split in splits.iter().rev() {
                if i >= self.max_split_rows { break };

                let s = format!(
                    "> {}:\t{}\n", 
                    splits.len() as u16 - i,
                    self.theme.format(split),
                );

                execute!(
                    self.stdout,
                    cursor::MoveTo(0, i),
                    Print(s)
                )?;

                i += 1;
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
