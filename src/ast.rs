#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug)]
pub struct Ident(pub String);

#[derive(Debug)]
pub enum Statement {
    Fn(Ident, Vec<Expression>, Vec<Statement>),
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal),
    Infix(Infix, Box<Expression>, Box<Expression>),
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(Debug)]
pub enum Infix {
    Plus,
}
