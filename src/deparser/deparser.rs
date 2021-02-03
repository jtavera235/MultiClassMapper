use crate::models::Language;
use crate::objects::{Class, Field};
use crate::common::{handle_result_error, MError};

pub struct DeParser {
    pub objects: Vec<Class>,
    pub language: Language,
    pub output: String,
}

impl DeParser {
    pub fn new(objects: Vec<Class>, language: Language) -> DeParser {
        DeParser {
            objects,
            language,
            output: String::new(),
        }
    }

    pub fn get_language(&self) -> Language {
        self.language.clone()
    }

    pub fn get_output(&self) -> String {
        self.output.clone()
    }

    pub fn construct(&mut self) {
        match self.get_language() {
            Language::JAVA => self.construct_java_class(),
            Language::TYPESCRIPT => self.construct_ts_class(),
            Language::C => self.construct_c_structs(),
            Language::CPP => self.construct_cpp_class(),
            Language::RUST => self.construct_rust_structs(),
        }
    }

    pub fn construct_java_class(&mut self) {
        for n in 0..self.objects.len() {
            let class = match self.objects.get(n) {
                Some(c) => c,
                None => {
                    let message = "Error trying to construct the current Java class".to_string();
                    handle_result_error(MError::ClassError(message));
                    panic!()
                },
            };
            self.output.push_str("class ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str(" { \n");
            let class_fields = class.get_java_fields();
            self.output.push_str(class_fields.as_str());
            self.output.push_str("} \n \n");
        }
    }

    pub fn construct_ts_class(&mut self) {
        for n in 0..self.objects.len() {
            let class = match self.objects.get(n) {
                Some(c) => c,
                None => {
                    let message = "Error trying to construct the current Typescript class".to_string();
                    handle_result_error(MError::DeparseError(message));
                    panic!()
                },
            };
            self.output.push_str("class ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str(" { \n");
            let class_fields = class.get_ts_fields();
            self.output.push_str(class_fields.as_str());
            self.output.push_str("} \n \n");
        }
    }

    pub fn construct_c_structs(&mut self) {
        for n in 0..self.objects.len() {
            let class = match self.objects.get(n) {
                Some(c) => c,
                None => {
                    let message = "Error trying to construct the current C struct".to_string();
                    handle_result_error(MError::DeparseError(message));
                    panic!()
                },
            };
            self.output.push_str("typedef struct ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str(" { \n");
            let class_fields = class.get_c_fields();
            self.output.push_str(class_fields.as_str());
            self.output.push_str("}; ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str("\n \n");
        }
    }

    pub fn construct_cpp_class(&mut self) {
        for n in 0..self.objects.len() {
            let class = match self.objects.get(n) {
                Some(c) => c,
                None => {
                    let message = "Error trying to construct the current C++ class".to_string();
                    handle_result_error(MError::DeparseError(message));
                    panic!()
                },
            };
            self.output.push_str("class ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str(" { \n");
            let class_fields = class.get_cpp_fields();
            self.output.push_str(class_fields.as_str());
            self.output.push_str("}; \n \n");
        }
    }

    pub fn construct_rust_structs(&mut self) {
        for n in 0..self.objects.len() {
            let class = match self.objects.get(n) {
                Some(c) => c,
                None => {
                    let message = "Error trying to construct the current Rust struct".to_string();
                    handle_result_error(MError::DeparseError(message));
                    panic!()
                },
            };
            self.output.push_str("struct ");
            self.output.push_str(class.get_name().as_str());
            self.output.push_str(" { ");
            let class_fields = class.get_rust_fields();
            self.output.push_str(class_fields.as_str());
            self.output.push_str("}\n \n ");
        }
    }
}
