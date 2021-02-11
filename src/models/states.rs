#[derive(Clone, Debug)]
pub enum ParseState {
    FILES,
    CLASS,
    FieldT,
    FieldN,
}
