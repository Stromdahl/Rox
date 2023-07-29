use super::{evaluate, RuntimeError};
use crate::{lexer, parser, expression::{self, LiteralExpression}};

fn parse(source: &str) -> expression::Expr {
    let mut tokens = lexer::Lexer::from_iter(source.chars()).peekable();
    parser::parse(&mut tokens).unwrap()
}

fn assert_literal_number(result: expression::Expr, expect: f64) {
    assert_eq!(result, expression::Expr::Literal(LiteralExpression::number(expect)));
}

fn assert_literal_boolean(result: expression::Expr, expect: bool) {
    assert_eq!(result, expression::Expr::Literal(LiteralExpression::boolean(expect)));
}

#[test]
fn test_interpreter_group() -> Result<(), RuntimeError>{
    let expr = parse("(123)");
    let result = evaluate(expr)?;
    let expect = expression::Expr::Literal(LiteralExpression::number(123_f64));
    assert_eq!(result, expect);
    Ok(())
}

#[test]
fn test_interpreter_expression_add_mult() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("2 * 10 + 5"))?, 25_f64);
    Ok(())
}

#[test]
fn test_interpreter_expression_add_group() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("(10 + 5)"))?, 15_f64);
    Ok(())
}

#[test]
fn test_interpreter_expression_mult_group_add() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("2 * (10 + 5)"))?, 30_f64);
    Ok(())
}

mod binary;
mod unary;
mod runtime_error;
