use token::Symbol;

#[derive(Debug, Clone)]
pub enum Literal {
    Integer { value: Symbol },
}

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    Add(Box<Node>, Box<Node>),
    Subtract(Box<Node>, Box<Node>),
    Multiply(Box<Node>, Box<Node>),
    Divide(Box<Node>, Box<Node>),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    BinaryOperation(BinaryOperation),
}

#[derive(Debug, Clone)]
pub enum Node {
    Expression(Expression),
}
