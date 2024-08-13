use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::MoveTo;
use crossterm::execute;
use std::io::stdout;

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(0, 0)
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        execute!(stdout(), Clear(ClearType::All))
    } 

    pub fn move_cursor_to(col: u16, row: u16) -> Result<(), std::io::Error> {
        execute!(stdout(), MoveTo(col, row))
    }

    pub fn size() -> Result<(u16, u16), std::io::Error> {
        size()
    }
}
