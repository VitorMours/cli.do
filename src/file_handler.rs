use std::io; 
use std::env;
use std::fs;

pub fn ingest_file(filepath: &str) -> Result<String, io::Error>{
    
    match fs::read_to_string(filepath){
        Ok(conteudo) => {
            println!("processando o arquivo {}", filepath);
            return Ok(conteudo)
        }
        Err(e) => {
            println!("O arquivo aparentemente nao existe, e teve o erro: {}", e);
            return Err(e)
        },
    };

}


pub fn process_arguments() {

    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

}
