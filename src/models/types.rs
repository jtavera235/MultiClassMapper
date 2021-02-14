#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Access {
    PRIVATE,
    PUBLIC,
    UNDEFINED,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum FieldType {
    INTEGER,
    STRING,
    BOOL,
    ARRAY(ArrayType),
    CHAR,
    DOUBLE,
    CUSTOM(String),
    LIST(ArrayType),
    UNDEFINED,
}

#[derive(Clone, Debug)]
#[allow(dead_code)]
pub enum ArrayType {
    INTEGER,
    STRING,
    BOOL,
    CHAR,
    DOUBLE,
    CUSTOM(String),
}
