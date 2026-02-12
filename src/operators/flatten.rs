//! Flatten function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate flatten function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(arr) => {
            let mut result = Vec::new();
            for item in arr {
                if let Value::Sequence(inner) = item {
                    result.extend(inner);
                } else {
                    result.push(item);
                }
            }
            Ok(Value::Sequence(result))
        }
        _ => Err(anyhow!(
            "Cannot flatten {}",
            helpers::value_type(&target_val)
        )),
    }
}
