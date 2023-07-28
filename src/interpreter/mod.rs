
#![allow(dead_code)]
use crate::expression::{Expr, UnaryExpression, LiteralExpression, LiteralOperator, BinaryExpression};

#[cfg(test)]
mod tests;

fn evaluate(expr: Expr) -> Expr {
    println!("expr: {expr:?}");
    match expr {
        Expr::Arithmetic(e) => binary(e),
        Expr::Compare(e) => binary(e),
        Expr::Equality(e) => binary(e),
        Expr::Grouping(e) => evaluate(*e),
        Expr::Literal(e) => Expr::Literal(e),
        Expr::Unary(e) => unary(e),
    }
}

fn unary(expr: UnaryExpression) -> Expr {
    let right = evaluate(expr.right().clone());
    let literal = match expr.operator() {
        crate::expression::UnaryOperator::Bang => LiteralExpression::boolean(!evaluate_boolean_literal(&right)),
        crate::expression::UnaryOperator::Minus => {
            let number = expect_numeric_literal(&right);
            LiteralExpression::number(-number)
        },
    };
    Expr::Literal(literal)
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

fn expect_numeric_literal(expr: &Expr) -> f64 {
    match expr {
        Expr::Literal(l) => {
            match l.value() {
                LiteralOperator::Number(n) => *n,
                _ => todo!("Handle not a number")
            }
        }
        _ => todo!("Handle literal expected!")
    }
}

fn binary(expr: BinaryExpression) -> Expr {
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
    Expr::Literal(result)
}

fn binary_operation<T>(expr: BinaryExpression, operation: &dyn Fn(f64, f64) -> T) -> T {
    let left = evaluate(expr.left().clone());
    let right = evaluate(expr.right().clone());

    let left = expect_numeric_literal(&left);
    let right = expect_numeric_literal(&right);
    operation(left, right)
}

fn mult(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::number(binary_operation(expr, &|left, right| left * right ))
}

fn div(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::number(binary_operation(expr, &|left, right| left / right ))
}

fn add(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::number(binary_operation(expr, &|left, right| left + right ))
}

fn sub(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::number(binary_operation(expr, &|left, right| left - right ))
}

fn greater(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::boolean(binary_operation(expr, &|left, right| left > right ))
}

fn less(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::boolean(binary_operation(expr, &|left, right| left < right ))
}

fn greater_equal(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::boolean(binary_operation(expr, &|left, right| left >= right ))
}

fn less_equal(expr: BinaryExpression) -> LiteralExpression {
    LiteralExpression::boolean(binary_operation(expr, &|left, right| left <= right ))
}

fn equal(expr: BinaryExpression) -> LiteralExpression {
    let left = evaluate(expr.left().clone());
    let right = evaluate(expr.right().clone());
    LiteralExpression::boolean(left == right)
}

fn not_equal(expr: BinaryExpression) -> LiteralExpression {
    let left = evaluate(expr.left().clone());
    let right = evaluate(expr.right().clone());
    LiteralExpression::boolean(left != right)
}
