#[derive(Debug)]
pub struct Program {
    pub statements: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub struct Ident(pub String);


#[derive(Debug)]
pub enum Statement {
    Expression(Expression),
}

#[derive(Debug)]
pub enum Expression {
    Ident(Ident),
    Literal(Literal),
    Infix(Infix, Box<Expression>, Box<Expression>),
    Fn {
        ident: Ident,
        args: Vec<Ident>,
        blocks: Vec<Statement>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
}

#[derive(Debug)]
pub enum Infix {
    Plus,
    Minus,
    Asterisk,
    Slash,
}
