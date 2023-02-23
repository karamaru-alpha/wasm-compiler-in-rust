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
        self.current = self.peek;
        self.peek = self.input.next().unwrap_or('\u{0}');
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();
        let token = match self.current {
            '+' => Token::Plus,
            ';' => Token::Semicolon,
            '(' => Token::Lparen,
            ')' => Token::Rparen,
            '{' => Token::Lbracket,
            '}' => Token::Rbracket,
            ',' => Token::Comma,
            '\u{0}' => Token::Eof,
            c => {
                if c.is_ascii_alphabetic() || c == '_' {
                    self.read_letter()
                } else if c.is_ascii_digit() {
                    self.read_number()
                } else {
                    panic!("unsupported token.");
                }
            }
        };
        self.read_char();
        token
    }

    fn skip_whitespace(&mut self) {
        while let ' ' | '\t' | '\n' | '\r' = self.current {
            self.read_char()
        }
    }

    fn read_letter(&mut self) -> Token {
        let mut letter = self.current.to_string();
        while self.peek.is_ascii_alphabetic() || self.peek == '_' {
            self.read_char();
            letter.push(self.current);
        }

        return match letter.as_str() {
            "fn" => Token::Fn,
            _ => Token::Ident(letter),
        };
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
