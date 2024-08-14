use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::style::Print;
use crossterm::execute;
use std::io::{Error, stdout};

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }

    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::All)) } 

    pub fn clear_line() -> Result<(), Error> {
        execute!(stdout(), Clear(ClearType::CurrentLine)) } 

    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), Error> {
        execute!(stdout(), MoveTo(col, row))
    }

    pub fn show_cursor() -> Result<(), Error> {
        execute!(stdout(), Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        execute!(stdout(), Hide)
    }

    pub fn print_out(s: &str) -> Result<(), Error> {
        execute!(stdout(), Print(s))
    }

    pub fn size() -> Result<(u16, u16), Error> {
        size()
    }
}
