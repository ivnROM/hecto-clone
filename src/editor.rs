mod terminal;

use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use terminal::Terminal;

pub struct Editor {
    should_quit: bool,
}

impl Editor { 
    pub fn default() -> Editor {
        Editor{ should_quit: false }
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
            Terminal::draw_rows();
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit == true {
                break;
            }
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
                    print!("{}\r\n", val);
                }
                _ => (),
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Programa finalizado exitosamente\r\n");
        }
        Ok(())
    }

}

