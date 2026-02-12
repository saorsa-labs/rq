//! Comparison operators (==, !=, <, <=, >, >=)

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Compare two values for equality
pub fn equal(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => ordering == std::cmp::Ordering::Equal,
        None => false,
    };

    Ok(Value::Bool(result))
}

/// Compare two values for inequality
pub fn not_equal(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => ordering != std::cmp::Ordering::Equal,
        None => true,
    };

    Ok(Value::Bool(result))
}

/// Less than comparison
pub fn less_than(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => ordering == std::cmp::Ordering::Less,
        None => false,
    };

    Ok(Value::Bool(result))
}

/// Less than or equal comparison
pub fn less_than_or_equal(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => {
            ordering == std::cmp::Ordering::Less || ordering == std::cmp::Ordering::Equal
        }
        None => false,
    };

    Ok(Value::Bool(result))
}

/// Greater than comparison
pub fn greater_than(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => ordering == std::cmp::Ordering::Greater,
        None => false,
    };

    Ok(Value::Bool(result))
}

/// Greater than or equal comparison
pub fn greater_than_or_equal(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let right_val = evaluator.eval(right, ctx)?;

    let result = match helpers::compare_values(&left_val, &right_val) {
        Some(ordering) => {
            ordering == std::cmp::Ordering::Greater || ordering == std::cmp::Ordering::Equal
        }
        None => false,
    };

    Ok(Value::Bool(result))
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

    // Equality tests
    #[test]
    fn test_equal_integers() {
        let result = parse_and_eval("5 == 5", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_equal_integers() {
        let result = parse_and_eval("5 == 3", "null").unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_equal_strings() {
        let result = parse_and_eval("\"hello\" == \"hello\"", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_equal_strings() {
        let result = parse_and_eval("\"hello\" == \"world\"", "null").unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_equal_booleans() {
        let result = parse_and_eval("true == true", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_equal_null() {
        let result = parse_and_eval("null == null", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_equal_different_types() {
        let result = parse_and_eval("5 == \"5\"", "null").unwrap();
        assert_eq!(result, false);
    }

    // Inequality tests
    #[test]
    fn test_not_equal_operator() {
        let result = parse_and_eval("5 != 3", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_not_equal_same() {
        let result = parse_and_eval("5 != 5", "null").unwrap();
        assert_eq!(result, false);
    }

    // Less than tests
    #[test]
    fn test_less_than_true() {
        let result = parse_and_eval("3 < 5", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_less_than_false() {
        let result = parse_and_eval("5 < 3", "null").unwrap();
        assert_eq!(result, false);
    }

    #[test]
    fn test_less_than_equal() {
        let result = parse_and_eval("5 < 5", "null").unwrap();
        assert_eq!(result, false);
    }

    // Less than or equal tests
    #[test]
    fn test_less_than_or_equal_true() {
        let result = parse_and_eval("3 <= 5", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_less_than_or_equal_equal() {
        let result = parse_and_eval("5 <= 5", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_less_than_or_equal_false() {
        let result = parse_and_eval("5 <= 3", "null").unwrap();
        assert_eq!(result, false);
    }

    // Greater than tests
    #[test]
    fn test_greater_than_true() {
        let result = parse_and_eval("5 > 3", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_greater_than_false() {
        let result = parse_and_eval("3 > 5", "null").unwrap();
        assert_eq!(result, false);
    }

    // Greater than or equal tests
    #[test]
    fn test_greater_than_or_equal_true() {
        let result = parse_and_eval("5 >= 3", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_greater_than_or_equal_equal() {
        let result = parse_and_eval("5 >= 5", "null").unwrap();
        assert_eq!(result, true);
    }

    // Float comparisons
    #[test]
    fn test_equal_floats() {
        let result = parse_and_eval("3.14 == 3.14", "null").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_less_than_floats() {
        let result = parse_and_eval("3.14 < 3.15", "null").unwrap();
        assert_eq!(result, true);
    }

    // String comparisons
    #[test]
    fn test_less_than_strings() {
        let result = parse_and_eval("\"apple\" < \"banana\"", "null").unwrap();
        assert_eq!(result, true);
    }

    // Comparison with fields
    #[test]
    fn test_comparison_with_fields() {
        let result = parse_and_eval(".value > 10", "value: 15").unwrap();
        assert_eq!(result, true);
    }

    #[test]
    fn test_comparison_with_fields_false() {
        let result = parse_and_eval(".value > 10", "value: 5").unwrap();
        assert_eq!(result, false);
    }

    // Chained comparisons
    #[test]
    fn test_chained_comparison() {
        let result = parse_and_eval("5 > 3 and 3 > 1", "null").unwrap();
        assert_eq!(result, true);
    }

    // Array comparisons (should fail - arrays not comparable)
    #[test]
    fn test_compare_arrays() {
        let result = parse_and_eval("[1, 2] == [1, 2]", "null").unwrap();
        assert_eq!(result, false); // Arrays are not comparable, returns false
    }
}
