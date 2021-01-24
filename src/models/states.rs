#[derive(Clone, Debug)]
pub enum ParseState {
    CLASS,
    FieldT,
    FieldN,
}