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
use common::write_file;
use common::create_file;
use common::open_file;
use colored::Colorize;


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
    if args.len() != 2 {
        let message = "Incorrect number of arguments. Expected 1 argument.\
         \n Usage: {input file}".to_string();
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
    let mut input_file = open_file(input_string);
    let mut parser = Parser::new();
    let file_content = get_file_buffer(&mut input_file,
                                       input_string.as_str());
    check_if_brackets_align(&file_content);

    parser.parse(&file_content);
    let mut deparser = DeParser::new(parser.get_objects());
    deparser.construct();
    println!("{}", "Successfully mapped objects.".green());
}
