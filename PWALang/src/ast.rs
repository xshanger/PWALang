#[derive(Debug, PartialEq)]
pub struct Simulation {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    Property(Property),
    Block(Block),
}

#[derive(Debug, PartialEq)]
pub struct Block {
    pub name: String,
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq)]
pub struct Property {
    pub name: String,
    pub value: Value,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Number(f64),
    String(String),
}

pub enum Expression {
    Number(f64),

    Binary {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },

    Unary {
        operator: UnaryOperator,
        operand: Box<Expression>,
    },
}

pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
}

pub enum UnaryOperator {
    Plus,
    Minus,
}
