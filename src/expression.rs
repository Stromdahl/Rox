
#[derive(PartialEq, Debug)]
pub enum Expr {
    Unary(Unary, Box<Expr>),
    Number,
    String(String),
    Binary(Box<Expr>, Binary, Box<Expr>),
    Compare(Box<Expr>, Compare, Box<Expr>),
    Equality(Box<Expr>, Equality, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    expression: Expr,
}

#[derive(PartialEq, Debug)]
pub enum Literal {
    Nil,
    True,
    False,
}

#[derive(PartialEq, Debug)]
pub enum Unary {
    Bang,
    Minus,
}

#[derive(PartialEq, Debug)]
pub enum Binary {
    Mult,
    Div,
    Add,
    Sub,
}

#[derive(PartialEq, Debug)]
pub enum Compare {
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(PartialEq, Debug)]
pub enum Equality {
    Equal,
    NotEqual,
}

#[derive(Debug)]
pub enum Error {
    ExpectExpression,
    ExpectRightParen,
    UnexpecedCharacter,
}
