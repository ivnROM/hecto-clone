mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use terminal::Terminal;
use std::io::Error;

pub struct Editor {
    should_quit: bool,
}

impl Editor { 
    pub const fn default() -> Self {
        Self { should_quit: false }
    }

    pub fn run(&mut self) {
        Terminal::initialize().unwrap();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
        Terminal::print_out("El programa finalizó correctamente\r\n").unwrap();
    }
    
    // read - evaluate - print
    fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit == true {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }

    fn evaluate_event(&mut self, event: &Event) {
        Terminal::clear_line().unwrap();
        if let Key(KeyEvent{code, modifiers, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                } 
                Char(val) => {
                    Terminal::print_out(&val.to_string()).unwrap();
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::hide_cursor()?;
        if self.should_quit {
            Terminal::clear_screen()?;
            Terminal::print_out("Goodbye\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Terminal::show_cursor()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error>{
        let y = Terminal::size()?.1;
        for i in 0..=y {
            Terminal::print_out("~")?;
            if i + 1 < y {
                Terminal::print_out("\r\n")?;
            }
        }
        Ok(())
    }
}

