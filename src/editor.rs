mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use terminal::{Terminal, Position, Size};
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
            //Terminal::print_out("Goodbye\r\n")?;
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        }
        Terminal::show_cursor()?;
        Terminal::execute()?;
        Ok(())
    }

    fn draw_rows() -> Result<(), Error>{
        let Size{height, ..} = Terminal::size()?;
        for i in 0..=height {
            Terminal::clear_line()?;
            Terminal::print_out("~")?;
            if i + 1 < height {
                Terminal::print_out("\r\n")?;
            }
        }
        Self::display_name()?;
        Ok(())
    }

    fn display_name() -> Result<(), Error> {
        let Size{height, width} = Terminal::size()?;
        //let width = width / 2; esto deberia andar????
        let width = width * 2;
        let height = height / 10;
        Terminal::move_cursor_to(Position{x: width, y: height})?;
        Terminal::print_out("Hecto: 0.1.1")?;
        Terminal::move_cursor_to(Position{x: 0, y: 0})?;
        Ok(())
    }
}

