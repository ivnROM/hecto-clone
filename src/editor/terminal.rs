use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::style::Print;
use crossterm::{queue, Command};
use std::io::{Error, Write, stdout};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

#[derive(Debug, Clone, Copy)]
pub struct Size {
   pub width: u16,
   pub height: u16,
}

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> Result<(), Error> {
        enable_raw_mode()?;
        Self::clear_screen()?;
        Self::move_cursor_to(Position{x: 0, y: 0})
    }

    pub fn terminate() -> Result<(), Error> {
        Self::execute()?;
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::All))
    }

    pub fn clear_line() -> Result<(), Error> {
        Self::queue_command(Clear(ClearType::CurrentLine))
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        Self::queue_command(MoveTo(pos.x, pos.y))
    }

    pub fn show_cursor() -> Result<(), Error> {
        Self::queue_command(Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        Self::queue_command(Hide)
    }

    pub fn print_out(s: &str) -> Result<(), Error> {
        Self::queue_command(Print(s))
    }

    fn queue_command<T: Command>(command: T) -> Result<(), Error> {
        queue!(stdout(), command)
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn size() -> Result<Size, Error> {
        let (h, w) = size()?;
        Ok(Size {height: h, width: w})
    }
}
