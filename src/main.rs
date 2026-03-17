//use std::io;
mod cli;

fn main() {
    println!("Starting the rust todo cli");

    cli::clean_terminal();
    cli::create_terminal_layout();
}



