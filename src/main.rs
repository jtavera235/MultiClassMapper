mod deparser;
mod models;
mod objects;
mod parser;
mod user_env;
mod common;

use crate::deparser::DeParser;
use crate::models::Language;
use crate::parser::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use common::handle_result_error;
use common::MError;
use colored::Colorize;


fn open_file(file: &str) -> File {
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

fn create_file(file: &str) -> File {
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

fn write_file(file: &mut File, buffer: &str) {
    file.write_all(buffer.as_bytes())
        .expect("unable to write to file");
}

fn get_file_buffer(file: &mut File, filename: &str) -> Vec<String> {
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

fn check_if_brackets_align(buf: &[String]) {
    let mut stack = Vec::new();

    for n in 0..buf.len() {
        let val = match buf.get(n) {
            Some(s) => s,
            None => {
                let message = "Error occurred while parsing file for bracket verification".to_string();
                handle_result_error(MError::GenError(message));
                panic!()
            },
        };
        if val.as_str() == "}" {
            if stack.is_empty() {
                let message = "Error occurred while checking for bracket verification.
                File contains an extra `}`".to_string();
                handle_result_error(MError::ParseError(message));
            } else {
                let top_v = match stack.pop() {
                    Some(v) => v,
                    None => {
                        let message = "Error occurred while parsing file for bracket verification.
                        File contains an extra `{`".to_string();
                        handle_result_error(MError::ParseError(message));
                        ""
                    },
                };
                if top_v != "{" {
                    let message = "Unknown error occurred while parsing file for bracket verification.
                        Bracket stack is not aligned".to_string();
                    handle_result_error(MError::ParseError(message));
                }
            }
        } else if val.as_str() == "{" {
            stack.push(val);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        let message = "Incorrect number of arguments. Expected 2 arguments.\
         \n Usage: {input file} {output file}".to_string();
        handle_result_error(MError::GenError(message));
    }

    let input_string = match args.get(1) {
        Some(s) => s,
        None => {
            let message = "Error occurred obtaining input file name.".to_string();
            handle_result_error(MError::GenError(message));
            panic!()
        },
    };
    let output_string = match args.get(2) {
        Some(s) => s,
        None => {
            let message = "Error occurred obtaining output file name.".to_string();
            handle_result_error(MError::GenError(message));
            panic!()
        },
    };
    let mut input_file = open_file(input_string);
    let mut output_file = create_file(output_string);
    let mut parser = Parser::new();
    let file_content = get_file_buffer(&mut input_file,
                                       output_string.as_str());
    check_if_brackets_align(&file_content);

    parser.parse(&file_content);
    let mut language = Language::JAVA;
    if output_string.clone().contains(".java") {
        language = Language::JAVA;
    } else if output_string.clone().contains(".ts") {
        language = Language::TYPESCRIPT;
    } else if output_string.clone().contains(".cpp") {
        language = Language::CPP;
    } else if output_string.clone().contains(".c") {
        language = Language::C;
    } else if output_string.clone().contains(".rs") {
        language = Language::RUST;
    }
    let mut deparser = DeParser::new(parser.get_objects(), language);
    deparser.construct();
    write_file(&mut output_file, &deparser.get_output());
    println!("{}", "Successfully mapped objects.".green());
}
