use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use crossterm::event::{read, Event::Key, KeyCode::Char};

pub struct Editor {

}

impl Editor {
    pub fn default() -> Editor {
        Editor{}
    }
    pub fn run(&self) {
        enable_raw_mode().unwrap();
        loop {
            match read() {
                Ok(Key(event)) => {
                    println!("{event:?} \r");
                    if let Char(c) = event.code {
                        if c == 'q' {
                            break;
                        }
                    }
                },
                Err(err) => eprintln!("{err}"),
                _ => ()
            }
        }
        disable_raw_mode().unwrap();
    }
}

