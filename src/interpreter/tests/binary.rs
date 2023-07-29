use super::RuntimeError;

use super::{parse, assert_literal_number, evaluate, assert_literal_boolean};

#[test]
fn test_interpreter_binary_add() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("42 + 27"))?, 69_f64);
    assert_literal_number(evaluate(parse("42 + -42"))?, 0_f64);
    Ok(())
}

#[test]
fn test_interpreter_binary_mult() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("0.5 * 10"))?, 5_f64);
    Ok(())
}

#[test]
fn test_interpreter_binary_sub() -> Result<(), RuntimeError> {
    assert_literal_number(evaluate(parse("540 - 120"))?, 420_f64);
    Ok(())
}

#[test]
fn test_interpreter_binary_div_ok() -> Result<(), RuntimeError>{
    assert_literal_number(evaluate(parse("20 / 2"))?, 10_f64);
    Ok(())
}

#[test]
fn test_interpreter_binary_greater() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("10 > 5"))?, true);
    assert_literal_boolean(evaluate(parse("5 > 10"))?, false);
    Ok(())
}

#[test]
fn test_interpreter_binary_greater_equal() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("10 >= 5"))?, true);
    assert_literal_boolean(evaluate(parse("10 >= 10"))?, true);
    assert_literal_boolean(evaluate(parse("5 >= 10"))?, false);
    Ok(())
}

#[test]
fn test_interpreter_binary_less() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("10 < 5"))?, false);
    assert_literal_boolean(evaluate(parse("5 < 10"))?, true);
    Ok(())
}

#[test]
fn test_interpreter_binary_less_equal() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("10 <= 5"))?, false);
    assert_literal_boolean(evaluate(parse("10 <= 10"))?, true);
    assert_literal_boolean(evaluate(parse("5 <= 10"))?, true);
    Ok(())
}

#[test]
fn test_interpreter_binary_is_equal() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("42 == 42"))?, true);
    assert_literal_boolean(evaluate(parse("42 == 24"))?, false);

    assert_literal_boolean(evaluate(parse("\"abc\" == \"abc\""))?, true);
    assert_literal_boolean(evaluate(parse("\"abc\" == \"cba\""))?, false);
    Ok(())
}

#[test]
fn test_interpreter_binary_not_equal() -> Result<(), RuntimeError>{
    assert_literal_boolean(evaluate(parse("42 != 42"))?, false);
    assert_literal_boolean(evaluate(parse("42 != 24"))?, true);

    assert_literal_boolean(evaluate(parse("\"abc\" != \"abc\""))?, false);
    assert_literal_boolean(evaluate(parse("\"abc\" != \"cba\""))?, true);
    Ok(())
}
