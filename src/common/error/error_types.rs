

#[derive(Debug)]
pub enum MError {
    ParseError(String),
    UserEnvError(String),
    GenError(String),
    ClassError(String),
    DeparseError(String),
}