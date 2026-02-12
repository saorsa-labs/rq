//! Length function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate length function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::String(s) => Ok(Value::Number(s.len().into())),
        Value::Sequence(arr) => Ok(Value::Number(arr.len().into())),
        Value::Mapping(map) => Ok(Value::Number(map.len().into())),
        _ => Err(anyhow!(
            "Cannot get length of {}",
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
    fn test_length_string() {
        let result = parse_and_eval("length", "hello").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_length_array() {
        let result = parse_and_eval("length", "[1, 2, 3, 4, 5]").unwrap();
        assert_eq!(result, 5);
    }

    #[test]
    fn test_length_object() {
        let result = parse_and_eval("length", "a: 1\nb: 2\nc: 3").unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_length_empty_string() {
        let result = parse_and_eval("length", "\"\"").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_length_empty_array() {
        let result = parse_and_eval("length", "[]").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_length_empty_object() {
        let result = parse_and_eval("length", "{}").unwrap();
        assert_eq!(result, 0);
    }

    #[test]
    fn test_length_nested_array() {
        let result = parse_and_eval(".items | length", "items:\n  - a\n  - b\n  - c").unwrap();
        assert_eq!(result, 3);
    }

    #[test]
    fn test_length_on_null() {
        let result = parse_and_eval("length", "null");
        assert!(result.is_err());
    }

    #[test]
    fn test_length_on_boolean() {
        let result = parse_and_eval("length", "true");
        assert!(result.is_err());
    }

    #[test]
    fn test_length_unicode_string() {
        // Note: length returns byte count, not character count
        // héllo is 6 bytes in UTF-8 (h=1, é=2, l=1, l=1, o=1)
        let result = parse_and_eval("length", "héllo").unwrap();
        assert_eq!(result, 6);
    }
}
