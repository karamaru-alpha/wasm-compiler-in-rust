#[derive(Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Plus,
    Ident(String),
    Fn,
    Lparen,
    Rparen,
    Lbracket,
    Rbracket,
    Comma,
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
