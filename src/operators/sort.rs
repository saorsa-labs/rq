//! Sort function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate sort function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(mut arr) => {
            arr.sort_by(|a, b| helpers::compare_values(a, b).unwrap_or(std::cmp::Ordering::Equal));
            Ok(Value::Sequence(arr))
        }
        _ => Err(anyhow!("Cannot sort {}", helpers::value_type(&target_val))),
    }
}
