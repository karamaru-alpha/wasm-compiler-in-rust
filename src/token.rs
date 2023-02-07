#[derive(Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Plus,
    Semicolon,
    Eof,
}

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
    Lowest,
    Sum,
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Plus => Precedence::Sum,
            _ => Precedence::Lowest,
        }
    }
}
