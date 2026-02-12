//! Keys function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate keys function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::Mapping(map) => {
            let keys: Vec<Value> = map.keys().cloned().collect();
            Ok(Value::Sequence(keys))
        }
        Value::Sequence(arr) => {
            // Return indices as numbers
            let keys: Vec<Value> = (0..arr.len()).map(|i| Value::Number(i.into())).collect();
            Ok(Value::Sequence(keys))
        }
        _ => Err(anyhow!(
            "Cannot get keys of {}",
            crate::evaluator::helpers::value_type(&target_val)
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

    #[test]
    fn test_keys_object() {
        let result = parse_and_eval("keys", "a: 1\nb: 2\nc: 3").unwrap();
        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_keys_array() {
        let result = parse_and_eval("keys", "[a, b, c]").unwrap();
        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0], 0);
        assert_eq!(arr[1], 1);
        assert_eq!(arr[2], 2);
    }

    #[test]
    fn test_keys_empty_object() {
        let result = parse_and_eval("keys", "{}").unwrap();
        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_keys_empty_array() {
        let result = parse_and_eval("keys", "[]").unwrap();
        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 0);
    }

    #[test]
    fn test_keys_on_string() {
        let result = parse_and_eval("keys", "hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_keys_on_number() {
        let result = parse_and_eval("keys", "42");
        assert!(result.is_err());
    }

    #[test]
    fn test_keys_nested() {
        let result = parse_and_eval(".data | keys", "data:\n  x: 1\n  y: 2").unwrap();
        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 2);
    }
}
