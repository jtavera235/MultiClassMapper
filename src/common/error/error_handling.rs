use crate::common::MError;
extern crate colored;

use colored::*;

pub fn handle_result_error(err: MError) {
    println!("{}", "<------ ERROR Occurred ------>\n".yellow());
    match err {
        MError::GenError(s) => {
            let mut message = "Error type: General error\nError message: ".to_string();
            message.push_str(&s);
            println!("{}", message.red());
        },
        MError::ParseError(s) => {
            let mut message = "Error type: Parsing error\nError message: ".to_string();
            message.push_str(&s);
            println!("{}", message.red());
        },
        MError::UserEnvError(s) => {
            let mut message = "Error type: Environment error\nError message: ".to_string();
            message.push_str(&s);
            println!("{}", message.red());
        },
        MError::DeparseError(s) => {
            let mut message = "Error type: Deparsing error\nError message: ".to_string();
            message.push_str(&s);
            println!("{}", message.red());
        },
        MError::ClassError(s) => {
            let mut message = "Error type: Class error\nError message: ".to_string();
            message.push_str(&s);
            println!("{}", message.red());
        },
    }
    println!("{}", "\n<------- End of message ------->".yellow());
    println!("Objects were mapped unsuccessfully");
    std::process::exit(1);
}