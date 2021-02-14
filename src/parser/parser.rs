use crate::common::handle_result_error;
use crate::common::MError;
use crate::models::{ArrayType, FieldType, Language, ParseState};
use crate::objects::{Class, Field};

#[derive(Clone, Debug)]
pub struct Parser {
    pub objects: Vec<Class>,
    pub parse_state: ParseState,
    pub index: usize,
    pub current_class: Option<Class>,
    pub current_field_name: Option<String>,
    pub current_languages: Option<Vec<Language>>,
}

impl Parser {
    pub fn new() -> Parser {
        let class: Vec<Class> = Vec::new();
        Parser {
            objects: class,
            parse_state: ParseState::FILES,
            index: 0,
            current_class: None,
            current_field_name: None,
            current_languages: None,
        }
    }

    fn add_class(&mut self, class: &Class) {
        self.objects.push(class.clone());
    }

    fn get_current_class(&self) -> Option<Class> {
        self.current_class.clone()
    }

    fn get_current_field(&self) -> Option<String> {
        self.current_field_name.clone()
    }

    pub fn get_objects(&self) -> Vec<Class> {
        self.objects.clone()
    }

    fn set_current_class(&mut self, class: &Class) {
        self.current_class = Some(class.clone());
    }

    fn set_current_field(&mut self, field: &str) {
        self.current_field_name = Some(field.to_string());
    }

    fn reset_current_class(&mut self) {
        self.current_class = None;
    }

    pub fn reset_current_field(&mut self) {
        self.current_field_name = None;
    }

    pub fn parse(&mut self, content: &[String]) {
        for _n in self.index..content.len() {
            match self.parse_state {
                ParseState::FILES => self.add_language(content),
                ParseState::CLASS => self.handle_class(content),
                ParseState::FieldT => self.handle_field_t(content),
                ParseState::FieldN => self.handle_field_n(content),
            }
        }
    }

    fn add_language(&mut self, tokens: &[String]) {
        if self.check_if_reached_end(tokens.len()) {
            return;
        }
        let mut file_related_tokens: Vec<Language> = Vec::new();

        if tokens.get(self.index).unwrap().as_str() != "[" {
            let mut message = "Expected object to start with `[` but found ".to_string();
            let first_token = tokens.get(self.index).unwrap().as_str();
            message.push_str(first_token);
            handle_result_error(MError::ParseError(message));
        }

        self.index += 1;
        for i in self.index..tokens.len() {
            let mut token = tokens.get(i).unwrap().clone();
            token.retain(|c| c != ',');
            match token.as_str() {
                "rs" => file_related_tokens.push(Language::RUST),
                "cpp" => file_related_tokens.push(Language::CPP),
                "c" => file_related_tokens.push(Language::C),
                "java" => file_related_tokens.push(Language::JAVA),
                "ts" => file_related_tokens.push(Language::TYPESCRIPT),
                "]" => {
                    self.index += 1;
                    break;
                }
                _ => {
                    let mut message = "Unknown language token found. \
                     Expected either `rs`, `ts`, `cpp`, `c`, or `java` but found "
                        .to_string();
                    message.push_str(token.as_str());
                    handle_result_error(MError::ParseError(message));
                }
            }
            self.index += 1;
        }
        file_related_tokens.sort_unstable();
        file_related_tokens.dedup();
        self.current_languages = Some(file_related_tokens);
        self.parse_state = ParseState::CLASS;
    }

    fn handle_class(&mut self, tokens: &[String]) {
        if self.check_if_reached_end(tokens.len()) {
            return;
        }
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => {
                let message = "Token not found for the current class index.".to_string();
                handle_result_error(MError::ParseError(message));
                panic!()
            }
        };

        let class = match self.get_current_class() {
            Some(c) => c,
            None => Class::new(token.to_string(), self.current_languages.as_ref().unwrap()),
        };
        self.set_current_class(&class);
        self.index += 1;
        self.parse_state = ParseState::FieldT;
    }

    fn handle_field_t(&mut self, tokens: &[String]) {
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => {
                let message = "Token not found for the current value index.".to_string();
                handle_result_error(MError::ParseError(message));
                panic!()
            }
        };
        if token != "{" && token.contains(':') {
            let mut token_cpy = token.clone();
            token_cpy.truncate(token_cpy.len() - 1);
            self.set_current_field(&token_cpy);
            self.parse_state = ParseState::FieldN;
        } else if token != "{" {
            let mut message = "Unknown token found. Expected field but found '".to_string();
            message.push_str(token);
            message.push('\'');
            handle_result_error(MError::ParseError(message));
        }
        self.index += 1;
    }

    fn handle_field_n(&mut self, tokens: &[String]) {
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => {
                let message = "Token not found for the current field type index.".to_string();
                handle_result_error(MError::ParseError(message));
                panic!()
            }
        };
        /* if token.as_str() == "\n" {
            self.ignore_newline_();
            self.handle_field_n(tokens);
        }

        */
        let mut current_class = match self.get_current_class() {
            Some(c) => c,
            None => {
                let message = "Expected to find a class but none were found. Verify that braces are set correctly.".to_string();
                handle_result_error(MError::ParseError(message));
                panic!()
            }
        };
        let is_last_field = token.contains(',');

        if token == "{" {
            let message = "Cannot have `{` as a field type.".to_string();
            handle_result_error(MError::ParseError(message));
            panic!()
        }

        if token == "}" {
            self.parse_state = ParseState::FILES;
            self.add_class(&current_class);
            self.reset_current_class();
            self.reset_current_field();
        } else {
            let mut token_cpy = token.clone();
            if is_last_field {
                token_cpy.truncate(token_cpy.len() - 1);
            }
            let field_type: FieldType;
            if token_cpy == "String" {
                field_type = FieldType::STRING;
            } else if token_cpy == "int" {
                field_type = FieldType::INTEGER;
            } else if token_cpy == "bool" {
                field_type = FieldType::BOOL;
            } else if token_cpy == "double" {
                field_type = FieldType::DOUBLE;
            } else if token_cpy == "char" {
                field_type = FieldType::CHAR;
            } else if token_cpy.contains('[') && token_cpy.contains(']') {
                let open_bracket_index = token_cpy.find('[').unwrap();
                let closed_bracket_index = token_cpy.find(']').unwrap();
                let array_type = token_cpy
                    .get(open_bracket_index + 1..closed_bracket_index)
                    .unwrap();
                match array_type {
                    "String" => field_type = FieldType::ARRAY(ArrayType::STRING),
                    "int" => field_type = FieldType::ARRAY(ArrayType::INTEGER),
                    "char" => field_type = FieldType::ARRAY(ArrayType::CHAR),
                    "bool" => field_type = FieldType::ARRAY(ArrayType::BOOL),
                    "double" => field_type = FieldType::ARRAY(ArrayType::DOUBLE),
                    _ => field_type = FieldType::ARRAY(ArrayType::CUSTOM(String::from(array_type))),
                }
            } else if token_cpy.contains('<') && token_cpy.contains('>') {
                let open_bracket_index = token_cpy.find('<').unwrap();
                let closed_bracket_index = token_cpy.find('>').unwrap();
                let array_type = token_cpy
                    .get(open_bracket_index + 1..closed_bracket_index)
                    .unwrap();
                match array_type {
                    "String" => field_type = FieldType::LIST(ArrayType::STRING),
                    "int" => field_type = FieldType::LIST(ArrayType::INTEGER),
                    "char" => field_type = FieldType::LIST(ArrayType::CHAR),
                    "bool" => field_type = FieldType::LIST(ArrayType::BOOL),
                    "double" => field_type = FieldType::LIST(ArrayType::DOUBLE),
                    _ => field_type = FieldType::LIST(ArrayType::CUSTOM(String::from(array_type))),
                }
            } else {
                field_type = FieldType::CUSTOM(token_cpy);
            }
            match self.get_current_field() {
                Some(_s) => (),
                None => {
                    let message = "Error getting the value of the current field".to_string();
                    handle_result_error(MError::ParseError(message));
                    panic!()
                }
            }
            let field = Field::new(self.get_current_field().unwrap(), field_type);
            current_class.add_field(&field);
            self.set_current_class(&current_class);
            if is_last_field {
                self.parse_state = ParseState::FieldT;
            }
        }
        self.reset_current_field();
        self.index += 1;
    }

    fn check_if_reached_end(&self, size: usize) -> bool {
        self.index == size
    }
}

pub fn check_if_brackets_align(buf: &[String]) {
    let mut stack = Vec::new();

    for n in 0..buf.len() {
        let val = match buf.get(n) {
            Some(s) => s,
            None => {
                let message =
                    "Error occurred while parsing file for bracket verification".to_string();
                handle_result_error(MError::GenError(message));
                panic!()
            }
        };
        if val.as_str() == "}" {
            if stack.is_empty() {
                let message = "Error occurred while checking for bracket verification.
                File contains an extra `}`"
                    .to_string();
                handle_result_error(MError::ParseError(message));
            } else {
                let top_v = match stack.pop() {
                    Some(v) => v,
                    None => {
                        let message = "Error occurred while parsing file for bracket verification.
                        File contains an extra `{`"
                            .to_string();
                        handle_result_error(MError::ParseError(message));
                        ""
                    }
                };
                if top_v != "{" {
                    let message =
                        "Unknown error occurred while parsing file for bracket verification.
                        Bracket stack is not aligned"
                            .to_string();
                    handle_result_error(MError::ParseError(message));
                }
            }
        } else if val.as_str() == "{" {
            stack.push(val);
        }
    }
}
