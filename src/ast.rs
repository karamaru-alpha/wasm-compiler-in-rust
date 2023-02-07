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
