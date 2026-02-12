//! Has function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate has function
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    key: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;
    let key_val = evaluator.eval(key, ctx)?;

    match &target_val {
        Value::Mapping(map) => {
            let has_key = map.contains_key(&key_val);
            Ok(Value::Bool(has_key))
        }
        Value::Sequence(arr) => {
            // Check if index exists
            if let Some(idx) = key_val.as_i64() {
                let idx = if idx < 0 {
                    arr.len().checked_sub(idx.unsigned_abs() as usize)
                } else {
                    Some(idx as usize)
                };
                Ok(Value::Bool(idx.map(|i| i < arr.len()).unwrap_or(false)))
            } else {
                Ok(Value::Bool(false))
            }
        }
        _ => Ok(Value::Bool(false)),
    }
}
