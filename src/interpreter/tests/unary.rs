use super::{parse, evaluate, expression, LiteralExpression};

#[test]
fn test_interpreter_unary_bang_ok() {
    let expr = parse("!true");
    let result = evaluate(expr);
    let expect = expression::Expr::Literal(LiteralExpression::boolean(false));
    assert_eq!(result, expect)
}

#[ignore = "todo"]
fn test_interpreter_unary_bang_not_literal_boolean() {
    let expr = parse("!abc");
    let result = evaluate(expr);
    let expect = expression::Expr::Literal(LiteralExpression::boolean(false));
    assert_eq!(result, expect)
}


#[test]
fn test_interpreter_unary_minus_ok() {
    let expr = parse("-123");
    let result = evaluate(expr);
    let expect = expression::Expr::Literal(LiteralExpression::number(-123_f64));
    assert_eq!(result, expect)
}

#[ignore = "todo"]
fn test_interpreter_unary_minus_not_a_number() {
    let expr = parse("-abc");
    let result = evaluate(expr);
    let expect = expression::Expr::Literal(LiteralExpression::number(-123_f64));
    assert_eq!(result, expect)
}

