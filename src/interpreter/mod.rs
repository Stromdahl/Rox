
#![allow(dead_code)]
use crate::expression::{Expr, UnaryExpression, LiteralExpression, LiteralOperator, BinaryExpression};

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
enum RuntimeError {
    NumericOperandExpected(LiteralExpression),
    LiteralOperandExpected,
}

fn evaluate(expr: Expr) -> Result<Expr, RuntimeError> {
    let result = match expr {
        Expr::Arithmetic(e) => binary(e)?,
        Expr::Compare(e) => binary(e)?,
        Expr::Equality(e) => binary(e)?,
        Expr::Grouping(e) => evaluate(*e)?,
        Expr::Literal(e) => Expr::Literal(e),
        Expr::Unary(e) => unary(e)?,
    };
    Ok(result)
}

fn unary(expr: UnaryExpression) -> Result<Expr, RuntimeError> {
    let right = evaluate(expr.right().clone())?;
    let literal = match expr.operator() {
        crate::expression::UnaryOperator::Bang => LiteralExpression::boolean(!evaluate_boolean_literal(&right)),
        crate::expression::UnaryOperator::Minus => {
            let number = expect_numeric_literal(&right);
            LiteralExpression::number(-number.unwrap()) // Todo: Fix unwrap
        },
    };
    Ok(Expr::Literal(literal))
}

fn evaluate_boolean_literal(expr: &Expr) -> bool {
    match expr {
        Expr::Literal(l) => {
            match l.value() {
                LiteralOperator::Boolean(b) => *b,
                LiteralOperator::Nil => false,
                LiteralOperator::Number(_) => true,
                LiteralOperator::String(_) => true,
            }
        }
        _ => true,
    }
}

fn expect_numeric_literal(expr: &Expr) -> Result<&f64, RuntimeError> {
    let literal = expr.as_literal().ok_or(RuntimeError::LiteralOperandExpected)?;
    let literal = literal.value()
        .as_number()
        .ok_or(
            RuntimeError::NumericOperandExpected(literal.to_owned())
        )?;
    Ok(literal)
}

fn binary(expr: BinaryExpression) -> Result<Expr, RuntimeError> {
    let result = match expr.operator() {
        crate::expression::BinaryOperator::Mult => mult(expr),
        crate::expression::BinaryOperator::Div => div(expr),
        crate::expression::BinaryOperator::Add => add(expr),
        crate::expression::BinaryOperator::Sub => sub(expr),
        crate::expression::BinaryOperator::Greater => greater(expr),
        crate::expression::BinaryOperator::Less => less(expr),
        crate::expression::BinaryOperator::GreaterEqual => greater_equal(expr),
        crate::expression::BinaryOperator::LessEqual => less_equal(expr),
        crate::expression::BinaryOperator::Equal => equal(expr),
        crate::expression::BinaryOperator::NotEqual => not_equal(expr),
    };
    Ok(Expr::Literal(result?))
}

fn binary_operation<T>(expr: BinaryExpression, operation: &dyn Fn(&f64, &f64) -> T) -> Result<T, RuntimeError> {
    let left = evaluate(expr.left().clone())?;
    let right = evaluate(expr.right().clone())?;

    let left = expect_numeric_literal(&left)?;
    let right = expect_numeric_literal(&right)?;
    Ok(operation(left, right))
}

fn mult(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
    Ok(LiteralExpression::number(binary_operation(expr, &|left, right| left * right )?))
}

fn div(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::number(binary_operation(expr, &|left, right| left / right )?)) 
}

fn add(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::number(binary_operation(expr, &|left, right| left + right )?)) 
}

fn sub(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::number(binary_operation(expr, &|left, right| left - right )?)) 
}

fn greater(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::boolean(binary_operation(expr, &|left, right| left > right )?)) 
}

fn less(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::boolean(binary_operation(expr, &|left, right| left < right )?)) 
}

fn greater_equal(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::boolean(binary_operation(expr, &|left, right| left >= right )?)) 
}

fn less_equal(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
   Ok(LiteralExpression::boolean(binary_operation(expr, &|left, right| left <= right )?)) 
}

fn equal(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
    let left = evaluate(expr.left().clone());
    let right = evaluate(expr.right().clone());
    Ok(LiteralExpression::boolean(left == right))
}

fn not_equal(expr: BinaryExpression) -> Result<LiteralExpression, RuntimeError> {
    let left = evaluate(expr.left().clone());
    let right = evaluate(expr.right().clone());
    Ok(LiteralExpression::boolean(left != right))
}
