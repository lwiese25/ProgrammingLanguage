extern crate asalang;
extern crate nom;
use std::io::Write;

use asalang::*;
use nom::IResult;

macro_rules! test_fragment {
  ($func:ident, $test:tt, $expected:expr) => (
    #[test]
    fn $func() -> Result<(),AsaErrorKind> {
      let tokens = lex($test);
      match program(tokens) {
        Ok((tokens, tree)) => {
          assert_eq!(tokens.is_done(), true); // Check that input token stream is fully parsed
          let mut interpreter = Interpreter::new();
          let result = interpreter.exec(&tree);
          std::io::stdout().flush();
          assert_eq!(result, $expected);
          Ok(())
        },
        Err(e) => Err(AsaErrorKind::Generic(format!("{:?}",e))),
      }
    }
  )
}

// Test interpreter fragments (no main function)
test_fragment!(interpreter_numeric, r#"123"#, Ok(Value::Number(123)));
test_fragment!(interpreter_string, r#""hello""#, Ok(Value::String("hello".to_string())));
test_fragment!(interpreter_bool_true, r#"true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_bool_false, r#"false"#, Ok(Value::Bool(false)));
test_fragment!(interpreter_identifier, r#"x"#, Err(AsaErrorKind::InvalidVariable));
test_fragment!(interpreter_variable_define, r#"let x = 123;"#, Ok(Value::Number(123)));
test_fragment!(interpreter_variable_init, r#"let x = 1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_variable_bool, r#"let bool = true;"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_variable_string, r#"let string = "Hello";"#, Ok(Value::String("Hello".to_string())));
test_fragment!(interpreter_variable_init_no_space, r#"let x=1;"#, Ok(Value::Number(1)));
test_fragment!(interpreter_math, r#"1 + 1"#, Ok(Value::Number(2)));
test_fragment!(interpreter_math_no_space, r#"1-1"#, Ok(Value::Number(0)));
test_fragment!(interpreter_math_multiply, r#"2 + 4"#, Ok(Value::Number(6)));
test_fragment!(interpreter_assign_math, r#"let x = 1 + 1;"#, Ok(Value::Number(2)));
test_fragment!(interpreter_define_full_program, r#"let x = 1 + 1; let y = 5 - 2; let z = x + y;"#, Ok(Value::Number(5)));
test_fragment!(test1, r#"5 - 3"#, Ok(Value::Number(2)));




// equality test
test_fragment!(interpreter_equal_numbers, r#"5 == 5"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_not_equal_numbers, r#"5 != 6"#, Ok(Value::Bool(true)));

// comparative tests
test_fragment!(interpreter_greater_than, r#"10 > 5"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_less_than, r#"2 < 3"#, Ok(Value::Bool(true)));

// greater than/ less than or equal to test
test_fragment!(interpreter_greater_than_or_equal, r#"5 >= 5"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_less_than_or_equal, r#"3 <= 5"#, Ok(Value::Bool(true)));

// boolean tests
test_fragment!(interpreter_boolean_equal, r#"true == true"#, Ok(Value::Bool(true)));
test_fragment!(interpreter_boolean_not_equal, r#"false != true"#, Ok(Value::Bool(true)));

// string comparisons
test_fragment!(interpreter_string_equal, r#""hello" == "hello""#, Ok(Value::Bool(true)));
test_fragment!(interpreter_string_not_equal, r#""hello" != "world""#, Ok(Value::Bool(true)));

// type errors
test_fragment!(interpreter_type_error_comparison, r#"5 > "five""#, Err(AsaErrorKind::TypeError));
test_fragment!(interpreter_type_error_addition, r#""hello" + 5"#, Err(AsaErrorKind::TypeError));

