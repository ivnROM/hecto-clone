mod terminal;
use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers };
use crossterm::terminal::size;
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
        let sucessrows = self.draw_rows();
        let result = self.repl();
        Terminal::terminate().unwrap();
        sucessrows.unwrap();
        result.unwrap();
        print!("El programa finalizó correctamente\r\n");
    }
    
    // read - evaluate - print
    fn repl(&mut self) -> Result<(), std::io::Error> {
        let mut row = 0;
        loop {
            Terminal::move_cursor_to(2, row).unwrap();
            let event = read()?;
            self.evaluate_event(&event);
            self.refresh_screen()?;
            if self.should_quit == true {
                break;
            }
            row += 1;
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

    fn draw_rows(&self) -> Result<(), std::io::Error>{
        let (_x, y) = size()?;
        for i in 0..=y {
            Terminal::move_cursor_to(0, i).unwrap();
            print!("~ ")
        }
        Ok(())
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
        }
        Ok(())
    }

}

