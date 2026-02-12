//! Field access operator (.field)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate field access
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    field: &str,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::Mapping(map) => {
            let key = Value::String(field.to_string());
            map.get(&key)
                .cloned()
                .ok_or_else(|| anyhow!("Field '{}' not found", field))
        }
        _ => Err(anyhow!("Cannot access field '{}' on non-object", field)),
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
    fn test_field_access_simple() {
        let result = parse_and_eval(".name", "name: test").unwrap();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_field_access_nested() {
        let result = parse_and_eval(".user.name", "user:\n  name: alice").unwrap();
        assert_eq!(result, "alice");
    }

    #[test]
    fn test_field_access_missing() {
        let result = parse_and_eval(".missing", "name: test");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Field 'missing' not found")
        );
    }

    #[test]
    fn test_field_access_on_non_object() {
        let result = parse_and_eval(".name", "[1, 2, 3]");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Cannot access field")
        );
    }

    #[test]
    fn test_field_access_on_string() {
        let result = parse_and_eval(".name", "hello");
        assert!(result.is_err());
    }

    #[test]
    fn test_field_access_number() {
        let result = parse_and_eval(".count", "count: 42").unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_field_access_boolean() {
        let result = parse_and_eval(".active", "active: true").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_field_access_null() {
        let result = parse_and_eval(".value", "value: null").unwrap();
        assert_eq!(result, Value::Null);
    }

    #[test]
    fn test_field_access_deeply_nested() {
        let input = "a:\n  b:\n    c:\n      d: deep";
        let result = parse_and_eval(".a.b.c.d", input).unwrap();
        assert_eq!(result, "deep");
    }

    #[test]
    fn test_field_access_with_hyphens() {
        let result = parse_and_eval(".my-field", "my-field: value").unwrap();
        assert_eq!(result, "value");
    }

    #[test]
    fn test_field_access_with_underscores() {
        let result = parse_and_eval(".my_field", "my_field: value").unwrap();
        assert_eq!(result, "value");
    }
}
