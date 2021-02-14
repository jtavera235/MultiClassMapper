use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use colored::Colorize;
use crate::common::{handle_result_error, MError};

pub fn write_file(file: &mut File, buffer: &str) {
    file.write_all(buffer.as_bytes())
        .expect("unable to write to file");
}

pub fn open_file(file: &str) -> File {
    println!("{} {:?}", "About to open the file".yellow(), file);
    let input_path = Path::new(file);
    match File::open(&input_path) {
        Ok(f) => f,
        Err(e) => {
            let mut message = "Error opening file: ".to_string();
            message.push_str(file);
            handle_result_error(MError::GenError(message));
            Err(e).unwrap()
        },
    }
}

pub fn create_file(file: &str) -> File {
    println!("{} {:?}", "Creating the file".yellow(), file);
    let input_path = Path::new(file);
    match File::create(&input_path) {
        Ok(f) => f,
        Err(e) => {
            let mut message = "Error creating file: ".to_string();
            message.push_str(file);
            handle_result_error(MError::GenError(message));
            Err(e).unwrap()
        },
    }
}