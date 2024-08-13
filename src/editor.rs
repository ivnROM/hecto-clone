mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use crossterm::terminal::size;
use terminal::Terminal;

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
        print!("El programa finalizó correctamente\r\n");
    }
    
    // read - evaluate - print
    fn repl(&mut self) -> Result<(), std::io::Error> {
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
        if let Key(KeyEvent{code, modifiers, ..}) = event {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                } 
                Char(val) => {
                    print!("{}", val);
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            print!("Goodbye\r\n")
        } else {
            Self::draw_rows()?;
            Terminal::move_cursor_to(0, 0)?;
        }
        Ok(())
    }

    fn draw_rows() -> Result<(), std::io::Error>{
        let y = size()?.1;
        for i in 0..=y {
            print!("~ ");
            if i + 1 < y {
                print!("\r\n");
            }
        }
        Ok(())
    }
}

