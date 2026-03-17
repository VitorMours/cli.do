use crossterm::{execute, terminal, terminal::ClearType, cursor::{MoveTo}};
use std::io;


pub fn clean_terminal() {

    // Precisa limpar la de cima
    execute!(io::stdout(), MoveTo(0, 0));
    execute!(io::stdout(), terminal::Clear(ClearType::All));    
}

pub fn create_terminal_layout(){

    let ws = terminal::window_size();
    let columns = ws.unwrap().columns; 
    let mut column_idx = 0;

    loop {
        print!("=");
        column_idx += 1;
        if column_idx == columns { break;}
    }
}
