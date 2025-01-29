
use crossterm::event::{read, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
use crossterm::execute;
use std::io::{self, Write};
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use crossterm::cursor::{MoveTo};



pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub fn default() -> Self {            
        Editor{ should_quit: false }             
    }

    pub fn run(&mut self) {
        if let Err(err) = self.repl() {
            panic!("{err:#?}");
        }
        print!("Goodbye.\r\n");
    }


    fn draw_rows() {
        // fix error handling here
        let ( _, rows) = size().unwrap_or_else(|e| {
            eprintln!("error getting terminal size {:?}", e);
            (0, 10)
        });
        let mut stdout = io::stdout();
        for number in 0 .. rows {
            // should print out smth like 1-10?
            execute!(stdout, MoveTo(0, number)).unwrap();
            println!("~");
            stdout.flush().unwrap();
            
        }
    }


    fn repl(&mut self) -> Result<(), std::io::Error> {
        enable_raw_mode()?;
        Self::draw_rows();
        loop {
            
           if let Key(KeyEvent {
            code, modifiers, kind, state
           }) = read()? {
                println!("Code: {code:?} Modifiers: {modifiers:?} Kind : {kind:?} State: {state:?} \r");
                match code {
                    Char('x') if modifiers == KeyModifiers::CONTROL => { 
                        self.should_quit = true; 
                    }
                    _ => (),
                }
            }
            if self.should_quit {
                break;
            }
        }
        disable_raw_mode()?; 
        Ok(())
    }
}