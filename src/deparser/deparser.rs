use crate::models::Language;
use crate::objects::{Class, Field};
use crate::common::{handle_result_error, MError, write_file, create_file};
use colored::Colorize;


pub struct DeParser {
    pub objects: Vec<Class>,
}

impl DeParser {
    pub fn new(objects: Vec<Class>) -> DeParser {
        DeParser {
            objects,
        }
    }

    pub fn construct(&mut self) {
        for x in 0..self.objects.len() {
            let current_object = match self.objects.get(x) {
                Some(x) => x,
                None => {
                    let message = "Error trying to find the current objects language.".to_string();
                    handle_result_error(MError::DeparseError(message));
                    panic!()
                },
            };
            for y in 0..current_object.languages.len() {
                let language = match current_object.languages.get(y) {
                    Some(T) => T,
                    None => {
                        let message = "Error trying to find the current objects language.".to_string();
                        handle_result_error(MError::DeparseError(message));
                        panic!()
                    },
                };
                let (output, file_extension) = match language {
                    Language::JAVA => (construct_java_class(current_object), ".java"),
                    Language::TYPESCRIPT => (construct_ts_class(current_object), ".ts"),
                    Language::C => (construct_c_structs(current_object), ".c"),
                    Language::CPP => (construct_cpp_class(current_object), ".cpp"),
                    Language::RUST => (construct_rust_structs(current_object), ".rs"),
                };
                let mut file_name = current_object.get_name();
                file_name.push_str(file_extension);
                let mut output_file = create_file(file_name.as_str());
                print!("{}", "Beginning to write to file ".blue());
                println!("{:?}", file_name);
                write_file(&mut output_file, output.as_str());
                print!("{}", "Finished writing to file ");
                println!("{:?}", file_name);
            }
        }
    }
}

fn construct_java_class(class: &Class) -> String {
    let mut output = String::new();
    output.push_str("class ");
    output.push_str(class.get_name().as_str());
    output.push_str(" { \n");
    let class_fields = class.get_java_fields();
    output.push_str(class_fields.as_str());
    output.push_str("} \n \n");
    output
}

fn construct_ts_class(class: &Class) -> String {
    let mut output = String::new();
    output.push_str("class ");
    output.push_str(class.get_name().as_str());
    output.push_str(" { \n");
    let class_fields = class.get_ts_fields();
    output.push_str(class_fields.as_str());
    output.push_str("} \n \n");
    output
}

fn construct_c_structs(class: &Class) -> String {
    let mut output = String::new();
    output.push_str("typedef struct ");
    output.push_str(class.get_name().as_str());
    output.push_str(" { \n");
    let class_fields = class.get_c_fields();
    output.push_str(class_fields.as_str());
    output.push_str("}; ");
    output.push_str(class.get_name().as_str());
    output.push_str("\n \n");
    output
}

fn construct_cpp_class(class: &Class) -> String {
    let mut output = String::new();
    output.push_str("class ");
    output.push_str(class.get_name().as_str());
    output.push_str(" { \n");
    let class_fields = class.get_cpp_fields();
    output.push_str(class_fields.as_str());
    output.push_str("}; \n \n");
    output
}

fn construct_rust_structs(class: &Class) -> String {
    let mut output = String::new();
    output.push_str("struct ");
    output.push_str(class.get_name().as_str());
    output.push_str(" { ");
    let class_fields = class.get_rust_fields();
    output.push_str(class_fields.as_str());
    output.push_str("}\n \n ");
    output
}
