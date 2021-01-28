use crate::models::{ArrayType, FieldType, ParseState};
use crate::objects::{Class, Field};

#[derive(Clone, Debug)]
pub struct Parser {
    pub objects: Vec<Class>,
    pub parse_state: ParseState,
    pub index: usize,
    pub current_class: Option<Class>,
    pub current_field_name: Option<String>,
}

impl Parser {
    pub fn new() -> Parser {
        let class: Vec<Class> = Vec::new();
        Parser {
            objects: class,
            parse_state: ParseState::CLASS,
            index: 0,
            current_class: None,
            current_field_name: None,
        }
    }

    pub fn add_class(&mut self, class: &Class) {
        self.objects.push(class.clone());
    }

    pub fn get_current_class(&self) -> Option<Class> {
        self.current_class.clone()
    }

    pub fn get_current_field(&self) -> Option<String> {
        self.current_field_name.clone()
    }

    pub fn get_objects(&self) -> Vec<Class> {
        self.objects.clone()
    }

    pub fn set_current_class(&mut self, class: &Class) {
        self.current_class = Some(class.clone());
    }

    pub fn set_current_field(&mut self, field: &str) {
        self.current_field_name = Some(field.to_string());
    }

    pub fn reset_current_class(&mut self) {
        self.current_class = None;
    }

    pub fn reset_current_field(&mut self) {
        self.current_field_name = None;
    }

    pub fn parse(&mut self, content: &[String]) {
        for _n in self.index..content.len() {
            match self.parse_state {
                ParseState::CLASS => self.handle_class(content),
                ParseState::FieldT => self.handle_field_t(content),
                ParseState::FieldN => self.handle_field_n(content),
            }
        }
    }

    pub fn handle_class(&mut self, tokens: &[String]) {
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => panic!("No value found in tokens"),
        };

        let class = match self.get_current_class() {
            Some(c) => c,
            None => Class::new(token.to_string()),
        };
        self.set_current_class(&class);
        self.index += 1;
        self.parse_state = ParseState::FieldT;
    }

    pub fn handle_field_t(&mut self, tokens: &[String]) {
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => panic!("No value found in tokens"),
        };
        if token != "{" && token.contains(':') {
            let mut token_cpy = token.clone();
            token_cpy.truncate(token_cpy.len() - 1);
            self.set_current_field(&token_cpy);
            self.parse_state = ParseState::FieldN;
        } else if token != "{" {
            panic!("Unknown field name specified {:?}", token);
        }
        self.index += 1;
    }

    pub fn handle_field_n(&mut self, tokens: &[String]) {
        let token = match tokens.get(self.index) {
            Some(t) => t,
            None => panic!("No value found in tokens"),
        };
        let mut current_class = match self.get_current_class() {
            Some(c) => c,
            None => panic!("There should be a current class"),
        };
        let is_last_field = token.contains(',');

        if token == "{" {
            panic!("Cannot have an opening brace as a field.");
        }

        if token == "}" {
            self.parse_state = ParseState::CLASS;
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
                None => panic!("Error occurred while parsing."),
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
}
