use std::{io::{self, Write}, fs::{self}};
use rodio::{source, OutputStream, OutputStreamHandle, Sink, Source};

mod parser;
mod music;

fn main() {
    let (_stream, stream_handle) = OutputStream::try_default()
        .map_err(|_e| format!("Could not create outputstream"))
        .unwrap();
    let mut file_name = String::new();
    
    loop {
        print!("Enter file path: ");
        io::stdout().flush().unwrap();

        match io::stdin().read_line(&mut file_name) {
            Ok(_) => {
                let file_name = file_name.trim();
                match fs::read_to_string(file_name) {
                    Ok(_) => break,
                    Err(e) => println!("Can not access file (check file director): {}", e)
                }
            },
            Err(_) => println!("Failed to read input. Try again."),
        }
    }
    

    let source_code= parser::read_file(&file_name.trim());
    let tokens = parser::tokenize(&source_code);
    println!("{:?}", tokens);

    let melody: Vec<&str> = vec!["D4", "E4", "F4", "G4", "E4", "C4", "D4"];

    music::play_melody(&stream_handle, melody);
}