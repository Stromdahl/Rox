#![allow(dead_code, unused_imports)]

use crate::token::Token;

#[derive(PartialEq, Debug)]
pub enum Expr {
    Boolean(bool),
    Expression(Box<Expression>),
    Unary(Unary, Box<Expr>),
    Nil,
    True,
    False,
    Number,
    String,
    Binary(Box<Expr>, Binary, Box<Expr>),
    Compare(Box<Expr>, Compare, Box<Expr>),
    Equality(Box<Expr>, Equality, Box<Expr>),
    Grouping(Box<Expr>),
}

#[derive(PartialEq, Debug)]
pub struct Expression {
    expression: Expr,
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
    ExpectUnaryOperator,
    UnexpecedCharacter,
}
