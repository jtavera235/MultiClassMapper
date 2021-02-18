mod common;
mod deparser;
mod models;
mod objects;
mod parser;
mod user_env;

use crate::deparser::DeParser;
use crate::parser::Parser;
use std::env;
use std::thread;
use std::time::Instant;

use colored::Colorize;
use common::get_file_buffer;
use common::handle_result_error;
use common::open_file;
use common::MError;
use parser::check_if_brackets_align;
use std::sync::{Arc, Mutex};

fn ensure_input_is_text_file(file: &str) -> bool {
    let file_length = file.len();
    let file_extension_start_index = file_length - 4;
    file[file_extension_start_index..] == *".txt"
}

fn main() {
    let start_time = Instant::now();
    let args: Vec<String> = env::args().collect();

    let mut children = vec![];
    let args_mutex = Mutex::new(args.clone());
    let args_arc = Arc::new(args_mutex);

    for n in 1..args.len() {
        let args = Arc::clone(&args_arc);
        children.push(thread::spawn(move || {
            let args = match args.lock() {
                Ok(e) => e,
                Err(_er) => {
                    let message =
                        "An error occurred obtaining command line arguments in the thread."
                            .to_string();
                    handle_result_error(MError::GenError(message));
                    panic!()
                }
            };
            println!("{}{:?}", "Starting thread ".cyan(), n);
            let input_string = match args.get(n as usize) {
                Some(s) => s,
                None => {
                    let message = "Error occurred obtaining input file name.".to_string();
                    handle_result_error(MError::GenError(message));
                    panic!()
                }
            };
            if !ensure_input_is_text_file(input_string) {
                let message = "Input file must have a .txt extension to be analyzed.".to_string();
                handle_result_error(MError::GenError(message));
                panic!()
            }
            let mut input_file = open_file(input_string);
            let mut parser = Parser::new();
            let file_content = get_file_buffer(&mut input_file, input_string.as_str());
            check_if_brackets_align(&file_content);

            parser.parse(&file_content);
            let mut deparser = DeParser::new(parser.get_objects());
            deparser.construct();
        }));
    }

    for child in children {
        let _ = child.join();
    }

    let end = start_time.elapsed();
    println!(
        "{} in {:?} (milliseconds)",
        "Successfully mapped objects".green(),
        end
    );
}
