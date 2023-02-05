use crate::token::Token;

#[derive(Clone)]
pub struct Lexer<'a> {
    input: std::str::Chars<'a>,
    current: char,
    peek: char,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a &str) -> Self {
        let mut lexer = Lexer {
            input: input.chars(),
            current: '\u{0}',
            peek: '\u{0}',
        };
        lexer.read_char();
        lexer.read_char();
        lexer
    }
    fn read_char(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.input.next().unwrap_or('\u{0}');
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current {
            '+' => Token::Plus,
            '\u{0}' => Token::Eof,
            c => {
                if c.is_ascii_digit() {
                    self.read_number()
                } else {
                    panic!("unsupported token.");
                }
            }
        };
        self.read_char();
        return token;
    }

    fn skip_whitespace(&mut self) {
        loop {
            match self.current {
                ' ' | '\t' | '\n' | '\r' => self.read_char(),
                _ => break,
            }
        }
    }

    fn read_number(&mut self) -> Token {
        let mut number = self.current.to_string();
        while self.peek.is_ascii_digit() {
            self.read_char();
            number.push(self.current);
        }
        Token::Int(number.parse().unwrap())
    }
}
