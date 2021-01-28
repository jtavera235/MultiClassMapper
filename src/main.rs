mod deparser;
mod models;
mod objects;
mod parser;

use crate::deparser::DeParser;
use crate::models::Language;
use crate::parser::Parser;
use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn adjust_field_syntax(token: &str) -> String {
    let mut tok = String::from("");
    let mut start_lowercase = false;
    for c in token.chars() {
        if c.is_ascii_uppercase() && start_lowercase {
            tok = camel_to_underscore(token);
            break;
        } else if c == '_' {
            tok = underscore_to_camel(token);
            break;
        }
    }
    tok
}

fn camel_to_underscore(token: &str) -> String {
    println!(
        "the current token {:?} needs to be changed to underscore",
        token
    );
    token.to_string()
}

fn underscore_to_camel(token: &str) -> String {
    println!(
        "the current token {:?} needs to be changed to underscore",
        token
    );
    token.to_string()
}

fn validate_token(token: &str, token_type: String) {
    assert!(token_type.as_str() == "field" || token_type.as_str() == "value");
    if token.contains(' ') {
        panic!(
            "Current {:?} contains an invalid character {:?}",
            token_type, token
        );
    }
}

fn open_file(file: &str) -> File {
    println!("About to open the file {:?}", file);
    let input_path = Path::new(file);
    match File::open(&input_path) {
        Ok(f) => f,
        Err(e) => panic!("Error opening file. Error message: \n {:?}", e),
    }
}

fn create_file(file: &str) -> File {
    println!("Creating the file {:?}", file);
    let input_path = Path::new(file);
    match File::create(&input_path) {
        Ok(f) => f,
        Err(e) => panic!("Error creating file. Error message: \n {:?}", e),
    }
}

fn write_file(file: &mut File, buffer: &str) {
    file.write_all(buffer.as_bytes())
        .expect("unable to write to file");
}

fn get_file_buffer(file: &mut File) -> Vec<String> {
    let mut buffer = String::new();
    match file.read_to_string(&mut buffer) {
        Ok(_s) => (),
        Err(_e) => panic!("Error reading the input file"),
    }
    let mut buffer_vector: Vec<&str> = buffer
        .split(|c| c == '\n' || c == ' ' || c == '\t')
        .collect();
    buffer_vector.retain(|&c| !c.is_empty());
    let strings = buffer_vector.iter().map(|s| s.to_string()).collect();
    strings
}

fn check_if_brackets_align(buf: &[String]) -> bool {
    let mut stack = Vec::new();

    for n in 0..buf.len() {
        let val = match buf.get(n) {
            Some(s) => s,
            None => panic!("Error getting buffer"),
        };
        if val.as_str() == "}" {
            if stack.is_empty() {
                println!("Stack is not aligned, invalid closing and opening brackets");
                return false;
            } else {
                let top_v = match stack.pop() {
                    Some(v) => v,
                    None => panic!("Stack is not aligned. Invalid closing and opening brackets"),
                };
                if top_v != "{" {
                    return false;
                }
            }
        } else if val.as_str() == "{" {
            stack.push(val);
        }
    }
    stack.is_empty()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Incorrect number of command line arguments specified");
        std::process::exit(1);
    }

    let input_string = match args.get(1) {
        Some(s) => s,
        None => panic!("Error reading input file path."),
    };
    let output_string = match args.get(2) {
        Some(s) => s,
        None => panic!("Error reading output file path."),
    };
    let mut input_file = open_file(input_string);
    let mut output_file = create_file(output_string);
    let mut parser = Parser::new();
    let file_content = get_file_buffer(&mut input_file);
    match check_if_brackets_align(&file_content) {
        true => (),
        false => std::process::exit(1),
    }
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
    println!("Successfully mapped classes.")
}
