//! Unique function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;
use std::collections::HashSet;

/// Evaluate unique function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(arr) => {
            let mut seen = HashSet::new();
            let mut unique = Vec::new();

            for item in arr {
                // Use JSON string representation for comparison
                let key = serde_json::to_string(&item).unwrap_or_default();
                if seen.insert(key) {
                    unique.push(item);
                }
            }

            Ok(Value::Sequence(unique))
        }
        _ => Err(anyhow!(
            "Cannot get unique of {}",
            helpers::value_type(&target_val)
        )),
    }
}
