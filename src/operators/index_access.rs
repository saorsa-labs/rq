//! Index access operator (.[index])

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate index access
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    index: isize,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::Sequence(arr) => {
            let idx = if index < 0 {
                arr.len().checked_sub(index.unsigned_abs())
            } else {
                Some(index as usize)
            };

            match idx {
                Some(i) if i < arr.len() => Ok(arr[i].clone()),
                _ => Err(anyhow!("Index {} out of bounds", index)),
            }
        }
        _ => Err(anyhow!("Cannot index non-array")),
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
    fn test_index_access_first() {
        let result = parse_and_eval(".[0]", "[a, b, c]").unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_index_access_last() {
        let result = parse_and_eval(".[2]", "[a, b, c]").unwrap();
        assert_eq!(result, "c");
    }

    #[test]
    fn test_index_access_negative() {
        let result = parse_and_eval(".[-1]", "[a, b, c]").unwrap();
        assert_eq!(result, "c");
    }

    #[test]
    fn test_index_access_negative_first() {
        let result = parse_and_eval(".[-3]", "[a, b, c]").unwrap();
        assert_eq!(result, "a");
    }

    #[test]
    fn test_index_access_out_of_bounds() {
        let result = parse_and_eval(".[5]", "[a, b, c]");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("out of bounds"));
    }

    #[test]
    fn test_index_access_negative_out_of_bounds() {
        let result = parse_and_eval(".[-5]", "[a, b, c]");
        assert!(result.is_err());
    }

    #[test]
    fn test_index_access_on_non_array() {
        let result = parse_and_eval(".[0]", "hello");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot index"));
    }

    #[test]
    fn test_index_access_on_object() {
        let result = parse_and_eval(".[0]", "name: test");
        assert!(result.is_err());
    }

    #[test]
    fn test_index_access_empty_array() {
        let result = parse_and_eval(".[0]", "[]");
        assert!(result.is_err());
    }

    #[test]
    fn test_index_access_nested_array() {
        let result = parse_and_eval(".[0][1]", "[[1, 2], [3, 4]]").unwrap();
        assert_eq!(result, 2);
    }

    #[test]
    fn test_index_access_after_field() {
        let result = parse_and_eval(".items[0]", "items:\n  - first\n  - second").unwrap();
        assert_eq!(result, "first");
    }
}
