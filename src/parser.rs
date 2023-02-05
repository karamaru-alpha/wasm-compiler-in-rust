use crate::{ast, lexer::Lexer, token::Token};

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

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program { statements: vec![] };
        while self.current != Token::Eof {
            program.statements.push(self.parse_statement());
            self.next_token()
        }
        program
    }

    fn parse_statement(&mut self) -> ast::Statement {
        self.parse_expression_statement()
    }

    fn parse_expression_statement(&mut self) -> ast::Statement {
        ast::Statement::Expression(self.parse_expression())
    }

    fn parse_expression(&mut self) -> ast::Expression {
        let left = match self.current {
            Token::Int(val) => ast::Expression::Int(val),
            _ => panic!("unsupported expression."),
        };
        // TODO: 中間演算子
        left
    }
}
