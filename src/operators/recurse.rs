//! Recurse function (..)

use crate::evaluator::{Context, Evaluator};
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate recurse function - returns all values recursively
pub fn eval(_evaluator: &Evaluator, ctx: &Context) -> Result<Value> {
    let mut results = Vec::new();
    collect_values(&ctx.value, &mut results);
    Ok(Value::Sequence(results))
}

fn collect_values(value: &Value, results: &mut Vec<Value>) {
    results.push(value.clone());

    match value {
        Value::Sequence(arr) => {
            for item in arr {
                collect_values(item, results);
            }
        }
        Value::Mapping(map) => {
            for (_, v) in map {
                collect_values(v, results);
            }
        }
        _ => {}
    }
}
