use crossterm::terminal::{disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::event::{read, Event::Key, KeyEvent, KeyModifiers, KeyCode::Char};
use crossterm::execute;
use std::io::stdout;

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    
    pub fn default() -> Editor {
        Editor{ should_quit: false }
    }


    pub fn run(&mut self) {
        Self::initialize().unwrap();
        let result = self.repl();
        Self::terminate().unwrap();
        result.unwrap();
        println!("El programa finalizó correctamente");
    }
    
    // read - evaluate - print
    fn repl(&mut self) -> Result<(), std::io::Error> {
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
        Ok(())
    }

    fn initialize() -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::clear_screen()
    }

    fn terminate() -> Result<(), std::io::Error> {
        disable_raw_mode()?;
        Self::clear_screen()
    }

    fn clear_screen() -> Result<(), std::io::Error> {
        let mut stdout = stdout();
        execute!(stdout, Clear(ClearType::All))
    }
}

