//! Expression evaluator for rq
//!
//! Evaluates parsed expressions against YAML/JSON/TOML data.

#![allow(dead_code)]

use crate::operators::*;
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Context for expression evaluation
#[derive(Debug, Clone)]
pub struct Context {
    /// The current value being processed
    pub value: Value,
    /// Parent context (for relative lookups)
    pub parent: Option<Box<Context>>,
    /// Variables in scope
    pub variables: std::collections::HashMap<String, Value>,
}

impl Context {
    /// Create a new context
    pub fn new(value: Value) -> Self {
        Self {
            value,
            parent: None,
            variables: std::collections::HashMap::new(),
        }
    }

    /// Create a child context
    pub fn child(&self, value: Value) -> Self {
        Self {
            value,
            parent: Some(Box::new(self.clone())),
            variables: self.variables.clone(),
        }
    }

    /// Set a variable
    pub fn set_variable(&mut self, name: String, value: Value) {
        self.variables.insert(name, value);
    }

    /// Get a variable
    pub fn get_variable(&self, name: &str) -> Option<&Value> {
        self.variables.get(name)
    }
}

/// Expression evaluator
pub struct Evaluator;

impl Evaluator {
    /// Create a new evaluator
    pub fn new() -> Self {
        Self
    }

    /// Evaluate an expression against input data
    pub fn evaluate(&self, expr: &Expression, input: Option<&Value>) -> Result<Value> {
        let ctx = match input {
            Some(v) => Context::new(v.clone()),
            None => Context::new(Value::Null),
        };
        self.eval(expr, &ctx)
    }

    /// Evaluate an expression in a context
    pub fn eval(&self, expr: &Expression, ctx: &Context) -> Result<Value> {
        match expr {
            Expression::Identity => Ok(ctx.value.clone()),
            Expression::Literal(v) => Ok(v.clone()),
            Expression::Empty => Ok(Value::Null),
            Expression::FieldAccess { target, field } => {
                field_access::eval(self, target, field, ctx)
            }
            Expression::IndexAccess { target, index } => {
                index_access::eval(self, target, *index, ctx)
            }
            Expression::Iterator { target } => iterator::eval(self, target, ctx),
            Expression::Pipe { left, right } => pipe::eval(self, left, right, ctx),
            Expression::Comma { left, right } => comma::eval(self, left, right, ctx),
            Expression::Assign { target, value } => assign::eval(self, target, value, ctx),
            Expression::Update { target, value } => update::eval(self, target, value, ctx),
            Expression::Add { left, right } => arithmetic::add(self, left, right, ctx),
            Expression::Subtract { left, right } => arithmetic::sub(self, left, right, ctx),
            Expression::Multiply { left, right } => arithmetic::mul(self, left, right, ctx),
            Expression::Divide { left, right } => arithmetic::div(self, left, right, ctx),
            Expression::Modulo { left, right } => arithmetic::modulo(self, left, right, ctx),
            Expression::Equal { left, right } => comparison::equal(self, left, right, ctx),
            Expression::NotEqual { left, right } => comparison::not_equal(self, left, right, ctx),
            Expression::LessThan { left, right } => comparison::less_than(self, left, right, ctx),
            Expression::LessThanOrEqual { left, right } => {
                comparison::less_than_or_equal(self, left, right, ctx)
            }
            Expression::GreaterThan { left, right } => {
                comparison::greater_than(self, left, right, ctx)
            }
            Expression::GreaterThanOrEqual { left, right } => {
                comparison::greater_than_or_equal(self, left, right, ctx)
            }
            Expression::And { left, right } => logical::and(self, left, right, ctx),
            Expression::Or { left, right } => logical::or(self, left, right, ctx),
            Expression::Not { expr } => logical::not(self, expr, ctx),
            Expression::Select { condition } => select::eval(self, condition, ctx),
            Expression::Keys { target } => keys::eval(self, target, ctx),
            Expression::Length { target } => length::eval(self, target, ctx),
            Expression::Type { target } => crate::operators::type_op::eval(self, target, ctx),
            Expression::Has { target, key } => has::eval(self, target, key, ctx),
            Expression::Sort { target } => sort::eval(self, target, ctx),
            Expression::Reverse { target } => reverse::eval(self, target, ctx),
            Expression::Unique { target } => unique::eval(self, target, ctx),
            Expression::Flatten { target } => flatten::eval(self, target, ctx),
            Expression::GroupBy { target, key_expr } => group_by::eval(self, target, key_expr, ctx),
            Expression::Map { target, expr } => map::eval(self, target, expr, ctx),
            Expression::Filter { target, expr } => filter::eval(self, target, expr, ctx),
            Expression::Recurse => recurse::eval(self, ctx),
            Expression::Group { expr } => self.eval(expr, ctx),
            Expression::Variable { name } => ctx
                .get_variable(name)
                .cloned()
                .ok_or_else(|| anyhow!("Undefined variable: {}", name)),
            Expression::Array { elements } => array::eval(self, elements, ctx),
            Expression::Object { fields } => object::eval(self, fields, ctx),
            Expression::Slice { target, start, end } => {
                slice::eval(self, target, *start, *end, ctx)
            }
            Expression::Alternative { left, right } => alternative::eval(self, left, right, ctx),
            Expression::First { expr } => first::eval(self, expr, ctx),
            Expression::Last { expr } => last::eval(self, expr, ctx),
            Expression::AddOp => add::eval(self, ctx),
            Expression::Env { name } => env::eval(self, name, ctx),
            Expression::ToString { target } => tostring::eval(self, target, ctx),
            Expression::ToNumber { target } => tonumber::eval(self, target, ctx),
            _ => Err(anyhow!("Unsupported expression: {:?}", expr)),
        }
    }

    /// Evaluate an expression and return multiple results (for iterators)
    pub fn eval_multi(&self, expr: &Expression, ctx: &Context) -> Result<Vec<Value>> {
        match expr {
            Expression::Iterator { target } => {
                let target_val = self.eval(target, ctx)?;
                let mut results = vec![];

                match &target_val {
                    Value::Sequence(arr) => {
                        for item in arr {
                            results.push(item.clone());
                        }
                    }
                    Value::Mapping(map) => {
                        for (_, v) in map {
                            results.push(v.clone());
                        }
                    }
                    _ => {}
                }

                Ok(results)
            }
            Expression::Pipe { left, right } => {
                let left_results = self.eval_multi(left, ctx)?;
                let mut results = vec![];
                for val in left_results {
                    let child_ctx = ctx.child(val);
                    match self.eval_multi(right, &child_ctx) {
                        Ok(mut vals) => results.append(&mut vals),
                        Err(_) => {
                            if let Ok(single) = self.eval(right, &child_ctx) {
                                results.push(single);
                            }
                        }
                    }
                }
                Ok(results)
            }
            _ => self.eval(expr, ctx).map(|v| vec![v]),
        }
    }
}

impl Default for Evaluator {
    fn default() -> Self {
        Self::new()
    }
}

/// Helper functions for operators
pub mod helpers {
    use serde_yaml::Value;

    /// Check if a value is "truthy"
    pub fn is_truthy(value: &Value) -> bool {
        match value {
            Value::Null => false,
            Value::Bool(b) => *b,
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    i != 0
                } else if let Some(f) = n.as_f64() {
                    f != 0.0
                } else {
                    true
                }
            }
            Value::String(s) => !s.is_empty(),
            Value::Sequence(arr) => !arr.is_empty(),
            Value::Mapping(map) => !map.is_empty(),
            _ => true,
        }
    }

    /// Compare two values
    pub fn compare_values(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
        match (a, b) {
            (Value::Null, Value::Null) => Some(std::cmp::Ordering::Equal),
            (Value::Bool(a), Value::Bool(b)) => a.partial_cmp(b),
            (Value::Number(a), Value::Number(b)) => {
                if let (Some(ai), Some(bi)) = (a.as_i64(), b.as_i64()) {
                    ai.partial_cmp(&bi)
                } else if let (Some(af), Some(bf)) = (a.as_f64(), b.as_f64()) {
                    af.partial_cmp(&bf)
                } else {
                    None
                }
            }
            (Value::String(a), Value::String(b)) => a.partial_cmp(b),
            _ => None,
        }
    }

    /// Get the type of a value as a string
    pub fn value_type(value: &Value) -> &'static str {
        match value {
            Value::Null => "null",
            Value::Bool(_) => "boolean",
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Sequence(_) => "array",
            Value::Mapping(_) => "object",
            _ => "unknown",
        }
    }

    /// Convert a value to a string representation
    pub fn value_to_string(value: &Value) -> String {
        match value {
            Value::Null => "null".to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Number(n) => n.to_string(),
            Value::String(s) => s.clone(),
            _ => serde_json::to_string(value).unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parser::expression::ExpressionParser;

    #[test]
    fn test_eval_identity() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".").unwrap();
        let input = serde_yaml::from_str("name: test").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert_eq!(result["name"], "test");
    }

    #[test]
    fn test_eval_field_access() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".name").unwrap();
        let input = serde_yaml::from_str("name: test\nvalue: 42").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert_eq!(result, "test");
    }

    #[test]
    fn test_eval_nested_field() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".user.name").unwrap();
        let input = serde_yaml::from_str("user:\n  name: alice\n  age: 30").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert_eq!(result, "alice");
    }

    #[test]
    fn test_eval_array_index() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".[1]").unwrap();
        let input = serde_yaml::from_str("[a, b, c]").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert_eq!(result, "b");
    }

    #[test]
    fn test_eval_iterator() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".[]").unwrap();
        let input = serde_yaml::from_str("[1, 2, 3]").unwrap();
        let results = evaluator.eval_multi(&expr, &Context::new(input)).unwrap();

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], 1);
        assert_eq!(results[1], 2);
        assert_eq!(results[2], 3);
    }

    #[test]
    fn test_eval_pipe() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse(".items[] | .name").unwrap();
        let input = serde_yaml::from_str(
            r#"
items:
  - name: foo
    value: 1
  - name: bar
    value: 2
"#,
        )
        .unwrap();
        let results = evaluator.eval_multi(&expr, &Context::new(input)).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0], "foo");
        assert_eq!(results[1], "bar");
    }

    #[test]
    fn test_eval_select() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        // Test select with field existence check
        // Items with 'active' field should be selected
        let expr = parser.parse(".[] | select(.active)").unwrap();
        let input = serde_yaml::from_str(
            r#"
- name: alice
  active: true
- name: bob
- name: charlie
  active: true
"#,
        )
        .unwrap();
        let results = evaluator.eval_multi(&expr, &Context::new(input)).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[0]["name"], "alice");
        assert_eq!(results[1]["name"], "charlie");
    }

    #[test]
    fn test_eval_arithmetic() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse("1 + 2 * 3").unwrap();
        let result = evaluator.evaluate(&expr, None).unwrap();

        assert_eq!(result, 7);
    }

    #[test]
    fn test_eval_comparison() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse("5 > 3").unwrap();
        let result = evaluator.evaluate(&expr, None).unwrap();

        assert_eq!(result, true);
    }

    #[test]
    fn test_eval_keys() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse("keys").unwrap();
        let input = serde_yaml::from_str("a: 1\nb: 2\nc: 3").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert!(result.is_sequence());
        let arr = result.as_sequence().unwrap();
        assert_eq!(arr.len(), 3);
    }

    #[test]
    fn test_eval_length() {
        let parser = ExpressionParser::new();
        let evaluator = Evaluator::new();

        let expr = parser.parse("length").unwrap();
        let input = serde_yaml::from_str("[1, 2, 3, 4, 5]").unwrap();
        let result = evaluator.evaluate(&expr, Some(&input)).unwrap();

        assert_eq!(result, 5);
    }
}
