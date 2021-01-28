use crate::models::{ArrayType, FieldType};
use crate::objects::fields::Field;

#[derive(Clone, Debug)]
pub struct Class {
    pub name: String,
    pub fields: Vec<Field>,
}

impl Class {
    pub fn new(name: String) -> Class {
        Class {
            name,
            fields: Vec::new(),
        }
    }

    pub fn add_field(&mut self, field: &Field) {
        self.fields.push(field.clone());
    }

    pub fn get_name(&self) -> String {
         self.name.clone()
    }

    pub fn get_java_fields(&self) -> String {
        let mut fields = String::new();
        for n in 0..self.fields.len() {
            let field = match self.fields.get(n) {
                Some(f) => f,
                None => panic!(""),
            };
            fields.push('\t');
            match field.get_field_type() {
                FieldType::STRING => fields.push_str("String"),
                FieldType::INTEGER => fields.push_str("int"),
                FieldType::DOUBLE => fields.push_str("double"),
                FieldType::CHAR => fields.push_str("char"),
                FieldType::BOOL => fields.push_str("boolean"),
                FieldType::CUSTOM(name) => fields.push_str(name.as_str()),
                FieldType::ARRAY(ArrayType::INTEGER) => fields.push_str("int[]"),
                FieldType::ARRAY(ArrayType::STRING) => fields.push_str("String[]"),
                FieldType::ARRAY(ArrayType::CHAR) => fields.push_str("char[]"),
                FieldType::ARRAY(ArrayType::DOUBLE) => fields.push_str("double[]"),
                FieldType::ARRAY(ArrayType::BOOL) => fields.push_str("bool[]"),
                FieldType::ARRAY(ArrayType::CUSTOM(name)) => {
                    fields.push_str(name.as_str());
                    fields.push_str("[]")
                }
                FieldType::LIST(ArrayType::INTEGER) => fields.push_str("List<Integer>"),
                FieldType::LIST(ArrayType::STRING) => fields.push_str("List<String>"),
                FieldType::LIST(ArrayType::CHAR) => fields.push_str("List<Character>"),
                FieldType::LIST(ArrayType::DOUBLE) => fields.push_str("List<Double>"),
                FieldType::LIST(ArrayType::BOOL) => fields.push_str("List<Boolean>"),
                FieldType::LIST(ArrayType::CUSTOM(name)) => {
                    fields.push_str("List<");
                    fields.push_str(name.as_str());
                    fields.push('>')
                }
                _ => (),
            }
            fields.push(' ');
            fields.push_str(field.get_name().as_str());
            fields.push_str("; \n");
        }
        fields
    }

    pub fn get_ts_fields(&self) -> String {
        let mut fields = String::new();
        for n in 0..self.fields.len() {
            let field = match self.fields.get(n) {
                Some(f) => f,
                None => panic!(""),
            };
            fields.push('\t');
            fields.push_str(field.get_name().as_str());
            fields.push(':');
            fields.push(' ');
            match field.get_field_type() {
                FieldType::STRING => fields.push_str("string"),
                FieldType::INTEGER => fields.push_str("number"),
                FieldType::DOUBLE => fields.push_str("number"),
                FieldType::CHAR => fields.push_str("string"),
                FieldType::BOOL => fields.push_str("boolean"),
                FieldType::CUSTOM(name) => fields.push_str(name.as_str()),
                FieldType::ARRAY(ArrayType::INTEGER) => fields.push_str("int[]"),
                FieldType::ARRAY(ArrayType::STRING) => fields.push_str("String[]"),
                FieldType::ARRAY(ArrayType::CHAR) => fields.push_str("char[]"),
                FieldType::ARRAY(ArrayType::DOUBLE) => fields.push_str("double[]"),
                FieldType::ARRAY(ArrayType::BOOL) => fields.push_str("bool[]"),
                FieldType::ARRAY(ArrayType::CUSTOM(name)) => {
                    fields.push_str(name.as_str());
                    fields.push_str("[]")
                }
                FieldType::LIST(ArrayType::INTEGER) => fields.push_str("Array<number>"),
                FieldType::LIST(ArrayType::STRING) => fields.push_str("Array<string>"),
                FieldType::LIST(ArrayType::CHAR) => fields.push_str("Array<string>"),
                FieldType::LIST(ArrayType::DOUBLE) => fields.push_str("Array<number>"),
                FieldType::LIST(ArrayType::BOOL) => fields.push_str("Array<boolean>"),
                FieldType::LIST(ArrayType::CUSTOM(name)) => {
                    fields.push_str("Array<");
                    fields.push_str(name.as_str());
                    fields.push('>')
                }
                _ => (),
            }
            fields.push_str("; \n");
        }
        fields
    }

    pub fn get_c_fields(&self) -> String {
        let mut fields = String::new();
        for n in 0..self.fields.len() {
            let field = match self.fields.get(n) {
                Some(f) => f,
                None => panic!(""),
            };
            fields.push('\t');
            match field.get_field_type() {
                FieldType::STRING => fields.push_str("char*"),
                FieldType::INTEGER => fields.push_str("int"),
                FieldType::DOUBLE => fields.push_str("double"),
                FieldType::CHAR => fields.push_str("char"),
                FieldType::BOOL => fields.push_str("bool"),
                FieldType::CUSTOM(name) => {
                    fields.push_str("struct ");
                    fields.push_str(name.as_str());
                }
                FieldType::ARRAY(ArrayType::INTEGER) => fields.push_str("int*"),
                FieldType::ARRAY(ArrayType::STRING) => fields.push_str("char*"),
                FieldType::ARRAY(ArrayType::CHAR) => fields.push_str("char*"),
                FieldType::ARRAY(ArrayType::DOUBLE) => fields.push_str("double*"),
                FieldType::ARRAY(ArrayType::BOOL) => fields.push_str("bool*"),
                FieldType::ARRAY(ArrayType::CUSTOM(name)) => {
                    fields.push_str(name.as_str());
                    fields.push('*')
                }
                FieldType::LIST(ArrayType::INTEGER) => fields.push_str("int*"),
                FieldType::LIST(ArrayType::STRING) => fields.push_str("char*"),
                FieldType::LIST(ArrayType::CHAR) => fields.push_str("char*"),
                FieldType::LIST(ArrayType::DOUBLE) => fields.push_str("double*"),
                FieldType::LIST(ArrayType::BOOL) => fields.push_str("bool*"),
                FieldType::LIST(ArrayType::CUSTOM(name)) => {
                    fields.push_str(name.as_str());
                    fields.push('*')
                }
                _ => (),
            }
            fields.push(' ');
            fields.push_str(field.get_name().as_str());
            fields.push_str("; \n");
        }
        fields
    }

    pub fn get_cpp_fields(&self) -> String {
        let mut fields = String::new();
        fields.push_str("\n \tpublic: \n \t \t");
        for n in 0..self.fields.len() {
            let field = match self.fields.get(n) {
                Some(f) => f,
                None => panic!(""),
            };
            match field.get_field_type() {
                FieldType::STRING => fields.push_str("string"),
                FieldType::INTEGER => fields.push_str("int"),
                FieldType::DOUBLE => fields.push_str("double"),
                FieldType::CHAR => fields.push_str("char"),
                FieldType::BOOL => fields.push_str("boolean"),
                FieldType::CUSTOM(name) => {
                    fields.push_str(name.as_str());
                }
                FieldType::ARRAY(ArrayType::INTEGER) => fields.push_str("int*"),
                FieldType::ARRAY(ArrayType::STRING) => fields.push_str("string"),
                FieldType::ARRAY(ArrayType::CHAR) => fields.push_str("char*"),
                FieldType::ARRAY(ArrayType::DOUBLE) => fields.push_str("double*"),
                FieldType::ARRAY(ArrayType::BOOL) => fields.push_str("boolean*"),
                FieldType::ARRAY(ArrayType::CUSTOM(name)) => {
                    fields.push_str(name.as_str());
                    fields.push('*')
                }
                FieldType::LIST(ArrayType::INTEGER) => fields.push_str("vector<int>"),
                FieldType::LIST(ArrayType::STRING) => fields.push_str("vector<string>"),
                FieldType::LIST(ArrayType::CHAR) => fields.push_str("vector<char>"),
                FieldType::LIST(ArrayType::DOUBLE) => fields.push_str("vector<double>"),
                FieldType::LIST(ArrayType::BOOL) => fields.push_str("vector<boolean>"),
                FieldType::LIST(ArrayType::CUSTOM(name)) => {
                    fields.push_str("vector<");
                    fields.push_str(name.as_str());
                    fields.push('>')
                }
                _ => (),
            }
            fields.push(' ');
            fields.push_str(field.get_name().as_str());
            fields.push_str(";\n\t\t");
        }
        fields.push('\n');
        fields
    }

    pub fn get_rust_fields(&self) -> String {
        let mut fields = String::new();
        for n in 0..self.fields.len() {
            let field = match self.fields.get(n) {
                Some(f) => f,
                None => panic!(""),
            };
            fields.push_str("\n\t");
            fields.push_str(field.get_name().as_str());
            fields.push_str(": ");
            match field.get_field_type() {
                FieldType::STRING => fields.push_str("String"),
                FieldType::INTEGER => fields.push_str("i32"),
                FieldType::DOUBLE => fields.push_str("f32"),
                FieldType::CHAR => fields.push_str("char"),
                FieldType::BOOL => fields.push_str("bool"),
                FieldType::CUSTOM(name) => {
                    fields.push_str(name.as_str());
                }
                FieldType::ARRAY(ArrayType::INTEGER) => fields.push_str("[i32]"),
                FieldType::ARRAY(ArrayType::STRING) => fields.push_str("[String]"),
                FieldType::ARRAY(ArrayType::CHAR) => fields.push_str("[char]"),
                FieldType::ARRAY(ArrayType::DOUBLE) => fields.push_str("[f64]"),
                FieldType::ARRAY(ArrayType::BOOL) => fields.push_str("[bool]"),
                FieldType::ARRAY(ArrayType::CUSTOM(name)) => {
                    fields.push('[');
                    fields.push_str(name.as_str());
                    fields.push(']');
                }
                FieldType::LIST(ArrayType::INTEGER) => fields.push_str("Vec<i32>"),
                FieldType::LIST(ArrayType::STRING) => fields.push_str("Vec<String>"),
                FieldType::LIST(ArrayType::CHAR) => fields.push_str("Vec<char>"),
                FieldType::LIST(ArrayType::DOUBLE) => fields.push_str("Vec<f64>"),
                FieldType::LIST(ArrayType::BOOL) => fields.push_str("Vec<bool>"),
                FieldType::LIST(ArrayType::CUSTOM(name)) => {
                    fields.push_str("Vec<");
                    fields.push_str(name.as_str());
                    fields.push('>')
                }
                _ => (),
            }
            fields.push(',');
        }
        fields.push('\n');
        fields
    }
}
