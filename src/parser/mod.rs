mod parse;

use crate::expression::{Error, Expr};
use crate::token::Token;

pub fn parse<I: Iterator<Item = Token>>(
    tokens: &mut std::iter::Peekable<I>,
) -> Result<Expr, Error> {
    parse::parse_expression(tokens)
}

