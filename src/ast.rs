#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Ident(String),
    Literal(Literal),
    Infix(Infix, Box<Expression>, Box<Expression>),
    Fn(Vec<Expression>, Vec<Statement>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(Debug)]
pub enum Infix {
    Plus,
}
