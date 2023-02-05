#[derive(Clone, PartialEq, Eq)]
pub enum Token {
    Int(i64),
    Plus,
    Eof,
}
