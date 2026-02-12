//! Arithmetic operators (+, -, *, /, %)

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Add two values
pub fn add(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(ai), Some(bi)) = (a.as_i64(), b.as_i64()) {
                Ok(Value::Number((ai + bi).into()))
            } else if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                Ok(Value::Number(serde_yaml::Number::from(af + bf)))
            } else {
                Err(anyhow!("Cannot add numbers"))
            }
        }
        (Value::String(a), Value::String(b)) => Ok(Value::String(format!("{}{}", a, b))),
        (Value::Sequence(a), Value::Sequence(b)) => {
            let mut result = a.clone();
            result.extend(b.clone());
            Ok(Value::Sequence(result))
        }
        _ => Err(anyhow!(
            "Cannot add {:?} and {:?}",
            helpers::value_type(&left_val),
            helpers::value_type(&right_val)
        )),
    }
}

/// Subtract two values
pub fn sub(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(ai), Some(bi)) = (a.as_i64(), b.as_i64()) {
                Ok(Value::Number((ai - bi).into()))
            } else if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                Ok(Value::Number(serde_yaml::Number::from(af - bf)))
            } else {
                Err(anyhow!("Cannot subtract numbers"))
            }
        }
        _ => Err(anyhow!(
            "Cannot subtract {:?} from {:?}",
            helpers::value_type(&right_val),
            helpers::value_type(&left_val)
        )),
    }
}

/// Multiply two values
pub fn mul(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(ai), Some(bi)) = (a.as_i64(), b.as_i64()) {
                Ok(Value::Number((ai * bi).into()))
            } else if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                Ok(Value::Number(serde_yaml::Number::from(af * bf)))
            } else {
                Err(anyhow!("Cannot multiply numbers"))
            }
        }
        _ => Err(anyhow!(
            "Cannot multiply {:?} and {:?}",
            helpers::value_type(&left_val),
            helpers::value_type(&right_val)
        )),
    }
}

/// Divide two values
pub fn div(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                if bf == 0.0 {
                    return Err(anyhow!("Division by zero"));
                }
                Ok(Value::Number(serde_yaml::Number::from(af / bf)))
            } else {
                Err(anyhow!("Cannot divide numbers"))
            }
        }
        _ => Err(anyhow!(
            "Cannot divide {:?} by {:?}",
            helpers::value_type(&left_val),
            helpers::value_type(&right_val)
        )),
    }
}

/// Modulo operation
pub fn modulo(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    match (&left_val, &right_val) {
        (Value::Number(a), Value::Number(b)) => {
            if let (Some(ai), Some(bi)) = (a.as_i64(), b.as_i64()) {
                if bi == 0 {
                    return Err(anyhow!("Modulo by zero"));
                }
                Ok(Value::Number((ai % bi).into()))
            } else if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                if bf == 0.0 {
                    return Err(anyhow!("Modulo by zero"));
                }
                Ok(Value::Number(serde_yaml::Number::from(af % bf)))
            } else {
                Err(anyhow!("Cannot modulo numbers"))
            }
        }
        _ => Err(anyhow!(
            "Cannot modulo {:?} by {:?}",
            helpers::value_type(&left_val),
            helpers::value_type(&right_val)
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluator::Evaluator;
    use crate::parser::expression::ExpressionParser;

    fn parse_and_eval(expr_str: &str, input: &str) -> Result<Value> {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();
        let expr = parser.parse(expr_str)?;
        let input_val = serde_yaml::from_str(input)?;
        evaluator.evaluate(&expr, Some(&input_val))
    }

    // Addition tests
    #[test]
    fn test_add_integers() {
        let result = parse_and_eval("1 + 2", "null").unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_add_floats() {
        let result = parse_and_eval("1.5 + 2.5", "null").unwrap();
        assert_eq!(result, 4.0);
    }

    #[test]
    fn test_add_strings() {
        let result = parse_and_eval("\"hello\" + \" \" + \"world\"", "null").unwrap();
        assert_eq!(result, "hello world");
    }

    #[test]
    fn test_add_arrays() {
        let result = parse_and_eval("[1, 2] + [3, 4]", "null").unwrap();
        let expected: Value = serde_yaml::from_str("[1, 2, 3, 4]").unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_add_mixed_types_error() {
        let result = parse_and_eval("1 + \"hello\"", "null");
        assert!(result.is_err());
    }

    // Subtraction tests
    #[test]
    fn test_sub_integers() {
        let result = parse_and_eval("5 - 3", "null").unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_sub_floats() {
        let result = parse_and_eval("5.5 - 2.5", "null").unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_sub_negative_result() {
        let result = parse_and_eval("3 - 5", "null").unwrap();
        assert_eq!(result, -2);
    }

    #[test]
    fn test_sub_strings_error() {
        let result = parse_and_eval("\"hello\" - \"world\"", "null");
        assert!(result.is_err());
    }

    // Multiplication tests
    #[test]
    fn test_mul_integers() {
        let result = parse_and_eval("4 * 5", "null").unwrap();
        assert_eq!(result, 20);
    }

    #[test]
    fn test_mul_floats() {
        let result = parse_and_eval("2.5 * 4.0", "null").unwrap();
        assert_eq!(result, 10.0);
    }

    #[test]
    fn test_mul_by_zero() {
        let result = parse_and_eval("5 * 0", "null").unwrap();
        assert_eq!(result, 0);
    }

    // Division tests
    #[test]
    fn test_div_integers() {
        let result = parse_and_eval("10 / 2", "null").unwrap();
        assert_eq!(result, 5.0);
    }

    #[test]
    fn test_div_floats() {
        let result = parse_and_eval("7.5 / 2.5", "null").unwrap();
        assert_eq!(result, 3.0);
    }

    #[test]
    fn test_div_by_zero() {
        let result = parse_and_eval("10 / 0", "null");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Division by zero"));
    }

    // Modulo tests
    #[test]
    fn test_modulo_integers() {
        let result = parse_and_eval("10 % 3", "null").unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_modulo_by_zero() {
        let result = parse_and_eval("10 % 0", "null");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Modulo by zero"));
    }

    // Operator precedence tests
    #[test]
    fn test_precedence_mul_before_add() {
        let result = parse_and_eval("2 + 3 * 4", "null").unwrap();
        assert_eq!(result, 14);
    }

    #[test]
    fn test_precedence_div_before_sub() {
        let result = parse_and_eval("10 - 6 / 2", "null").unwrap();
        assert_eq!(result, 7.0);
    }

    #[test]
    fn test_precedence_left_to_right() {
        let result = parse_and_eval("10 - 3 - 2", "null").unwrap();
        assert_eq!(result, 5);
    }

    // Complex expression tests
    #[test]
    fn test_complex_arithmetic() {
        let result = parse_and_eval("(2 + 3) * (4 - 1)", "null").unwrap();
        assert_eq!(result, 15);
    }

    #[test]
    fn test_arithmetic_with_fields() {
        let result = parse_and_eval(".a + .b", "a: 5\nb: 3").unwrap();
        assert_eq!(result, 8);
    }
}
