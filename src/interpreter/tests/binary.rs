use super::{parse, assert_literal_number, evaluate, assert_literal_boolean};

#[test]
fn test_interpreter_binary_add() {
    assert_literal_number(evaluate(parse("42 + 27")), 69_f64);
    assert_literal_number(evaluate(parse("42 + -42")), 0_f64);
}

#[test]
fn test_interpreter_binary_mult() {
    assert_literal_number(evaluate(parse("0.5 * 10")), 5_f64);
}

#[test]
fn test_interpreter_binary_sub() {
    assert_literal_number(evaluate(parse("540 - 120")), 420_f64);
}

#[test]
fn test_interpreter_binary_div_ok() {
    assert_literal_number(evaluate(parse("20 / 2")), 10_f64);
}

#[test]
fn test_interpreter_binary_greater() {
    assert_literal_boolean(evaluate(parse("10 > 5")), true);
    assert_literal_boolean(evaluate(parse("5 > 10")), false);
}

#[test]
fn test_interpreter_binary_greater_equal() {
    assert_literal_boolean(evaluate(parse("10 >= 5")), true);
    assert_literal_boolean(evaluate(parse("10 >= 10")), true);
    assert_literal_boolean(evaluate(parse("5 >= 10")), false);
}

#[test]
fn test_interpreter_binary_less() {
    assert_literal_boolean(evaluate(parse("10 < 5")), false);
    assert_literal_boolean(evaluate(parse("5 < 10")), true);
}

#[test]
fn test_interpreter_binary_less_equal() {
    assert_literal_boolean(evaluate(parse("10 <= 5")), false);
    assert_literal_boolean(evaluate(parse("10 <= 10")), true);
    assert_literal_boolean(evaluate(parse("5 <= 10")), true);
}

#[test]
fn test_interpreter_binary_is_equal() {
    assert_literal_boolean(evaluate(parse("42 == 42")), true);
    assert_literal_boolean(evaluate(parse("42 == 24")), false);

    assert_literal_boolean(evaluate(parse("\"abc\" == \"abc\"")), true);
    assert_literal_boolean(evaluate(parse("\"abc\" == \"cba\"")), false);
}

#[test]
fn test_interpreter_binary_not_equal() {
    assert_literal_boolean(evaluate(parse("42 != 42")), false);
    assert_literal_boolean(evaluate(parse("42 != 24")), true);

    assert_literal_boolean(evaluate(parse("\"abc\" != \"abc\"")), false);
    assert_literal_boolean(evaluate(parse("\"abc\" != \"cba\"")), true);
}
