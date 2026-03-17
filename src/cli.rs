use std::fs;
use std::io; 
use std::env;
use crossterm::{cursor, terminal, terminal::WindowSize}


pub fn clean_terminal() {
    terminal::Clear();    
}

pub fn create_terminal_layout(){

    let mut [rows, columns, width, height] = WindowSize();

    let mut column_idx = 0;


    loop {
        println!("=");
        column_idx++;
    }
}
