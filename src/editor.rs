use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event::Key, KeyEvent, KeyModifiers, KeyCode::Char};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    
    pub fn default() -> Editor {
        Editor{ should_quit: false }
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        println!("FIN. ")
    }
    
    // read - evaluate - print
    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        loop {
            if let Key(KeyEvent{code, modifiers, kind, state}) = read()? {
                println!("Code: {code:?}, Modifiers: {modifiers:?}, Kind: {kind:?}, State: {state:?}\r");
                match code {
                    Char('q') if modifiers == KeyModifiers::CONTROL => {
                        self.should_quit = true
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode().unwrap();
        Ok(())
    }
}

