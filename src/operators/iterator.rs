//! Iterator operator (.[])

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate iterator - returns an array of values
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::Sequence(arr) => {
            // Return the array - eval_multi handles the iteration
            Ok(Value::Sequence(arr.clone()))
        }
        Value::Mapping(map) => {
            // Return array of values
            let values: Vec<Value> = map.values().cloned().collect();
            Ok(Value::Sequence(values))
        }
        _ => Err(anyhow!("Cannot iterate over non-array/object")),
    }
}
