use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use crossterm::cursor;
use crossterm::execute;
use std::io::stdout;

pub struct Terminal{}

impl Terminal {
    pub fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    pub fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }

    pub fn draw_rows() {
        let (x, y) = size().unwrap();
        println!("x = {x}, y = {y}");
        cursor::MoveTo(1, 1);
    }
}
