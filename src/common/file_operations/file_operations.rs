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

pub fn get_file_buffer(file: &mut File, filename: &str) -> Vec<String> {
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_s) => (),
        Err(_e) => {
            let mut message = "Error reading file: ".to_string();
            message.push_str(filename);
            handle_result_error(MError::GenError(message))
        },
    }
    let mut buffer_vector: Vec<&str> = buffer
        .split(|c| c == '\n' || c == ' ' || c == '\t')
        .collect();
    buffer_vector.retain(|&c| !c.is_empty());
    let strings = buffer_vector.iter().map(|s| s.to_string()).collect();
    strings
}