use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor::{MoveTo, Show, Hide};
use crossterm::style::Print;
use crossterm::queue;
use std::io::{Error, Write, stdout};

#[derive(Clone, Copy)]
pub struct Position {
    pub x: u16,
    pub y: u16
}

#[derive(Clone, Copy)]
pub struct Size {
   pub height: u16,
   pub _width: u16,
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
        queue!(stdout(), Clear(ClearType::All)) } 

    pub fn clear_line() -> Result<(), Error> {
        queue!(stdout(), Clear(ClearType::CurrentLine)) 
    }

    pub fn move_cursor_to(pos: Position) -> Result<(), Error> {
        queue!(stdout(), MoveTo(pos.x, pos.y))
    }

    pub fn show_cursor() -> Result<(), Error> {
        queue!(stdout(), Show)
    }

    pub fn hide_cursor() -> Result<(), Error> {
        queue!(stdout(), Hide)
    }

    pub fn print_out(s: &str) -> Result<(), Error> {
        queue!(stdout(), Print(s))
    }

    pub fn execute() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn size() -> Result<Size, Error> {
        let (h, w) = size()?;
        Ok(Size {height: h, _width: w})
    }
}
