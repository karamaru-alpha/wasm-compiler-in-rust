#[derive(Clone, PartialEq)]
pub enum Token {
    Int(i64),
    Plus,
    Minus,
    Asterisk,
    Slash,
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
    Product,
}

impl Token {
    pub fn precedence(&self) -> Precedence {
        match self {
            Token::Plus => Precedence::Sum,
            Token::Minus => Precedence::Sum,
            Token::Asterisk => Precedence::Product,
            Token::Slash => Precedence::Product,
            _ => Precedence::Lowest,
        }
    }
}
