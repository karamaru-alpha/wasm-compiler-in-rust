use crate::{ast::*, lexer::Lexer, token::Precedence, token::Token};

#[derive(Clone)]
pub struct Parser<'a> {
    lex: Lexer<'a>,
    current: Token,
    peek: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lex: Lexer<'a>) -> Self {
        let mut parser = Self {
            lex,
            current: Token::Eof,
            peek: Token::Eof,
        };
        parser.next_token();
        parser.next_token();
        parser
    }

    fn next_token(&mut self) {
        self.current = self.peek.clone();
        self.peek = self.lex.next_token()
    }

    pub fn parse_program(&mut self) -> Program {
        let mut program = Program { statements: vec![] };
        while self.current != Token::Eof {
            program.statements.push(self.parse_statement());
            self.next_token()
        }

        program
    }

    fn parse_statement(&mut self) -> Statement {
        return match self.current {
            Token::Fn => self.parse_fn_statement(),
            _ => self.parse_expression_statement(),
        };
    }

    fn parse_expression_statement(&mut self) -> Statement {
        let statement = Statement::Expression(self.parse_expression(Precedence::Lowest));
        if self.peek == Token::Semicolon {
            self.next_token();
        }
        statement
    }

    fn parse_expression(&mut self, precedence: Precedence) -> Expression {
        let left = match &self.current {
            Token::Int(val) => Expression::Literal(Literal::Int(*val)),
            Token::Ident(val) => Expression::Ident(Ident(val.parse().unwrap())),
            _ => panic!("unsupported expression."),
        };

        if self.peek != Token::Semicolon && precedence < self.peek.precedence() {
            self.next_token();
            return self.parse_infix_expression(left);
        }

        left
    }

    fn parse_ident(&mut self) -> Ident {
        return match &self.current {
            Token::Ident(val) => Ident(val.parse().unwrap()),
            _ => panic!("unsupported expression."),
        };
    }

    fn parse_fn_statement(&mut self) -> Statement {
        self.next_token();

        match self.current {
            Token::Ident(_) => {}
            _ => panic!("fn ident not found."),
        }

        let ident = self.parse_ident();

        self.next_token();
        if self.current != Token::Lparen {
            panic!("fn lparen not found.");
        }

        self.next_token();
        let mut args = Vec::new();
        while self.current != Token::Rparen {
            args.push(self.parse_expression(Precedence::Lowest));
            self.next_token();
            if self.current == Token::Comma {
                self.next_token();
            }
        }

        self.next_token();
        if self.current != Token::Lbracket {
            panic!("fn lbracket not found.")
        }

        self.next_token();
        let mut blocks = Vec::new();
        while self.current != Token::Rbracket {
            blocks.push(self.parse_statement());
            self.next_token();
        }

        Statement::Fn(ident, args, blocks)
    }

    fn parse_infix_expression(&mut self, left: Expression) -> Expression {
        let infix = match self.current {
            Token::Plus => Infix::Plus,
            _ => panic!("unsupported infix."),
        };
        self.next_token();
        let right = self.parse_expression(self.current.precedence());
        Expression::Infix(infix, Box::from(left), Box::from(right))
    }
}
