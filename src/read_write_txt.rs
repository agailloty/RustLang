use std::fs::File;
use std::io::{self, Read};


pub fn read_txt(filepath : String) {
    let mut file = match File::open(&filepath) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error opening file: {}", error);
            return;
        }
    };

    let mut contents = String::new();
    if let Err(error) = file.read_to_string(&mut contents) {
        eprintln!("Error reading file: {}", error);
    } else {
        println!("File contents:\n{}", contents);
    }
}