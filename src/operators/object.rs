//! Object constructor

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate object constructor
pub fn eval(
    evaluator: &Evaluator,
    fields: &[(Expression, Expression)],
    ctx: &Context,
) -> Result<Value> {
    let mut result = serde_yaml::Mapping::new();
    for (key_expr, value_expr) in fields {
        let key = evaluator.eval(key_expr, ctx)?;
        let value = evaluator.eval(value_expr, ctx)?;
        result.insert(key, value);
    }
    Ok(Value::Mapping(result))
}
