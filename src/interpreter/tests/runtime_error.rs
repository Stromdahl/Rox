use super::{parse, evaluate};

#[test]
fn test_interpreter_runtime_error_number_expected() -> Result<(), String>{
    match evaluate(parse("2 + false")) {
        Ok(_) => panic!("Expected runtime error"),
        Err(_) => Ok(()),
    }
}
