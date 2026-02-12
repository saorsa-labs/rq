//! Slice operator (.[start:end])

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate slice
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    start: Option<isize>,
    end: Option<isize>,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(arr) => {
            let len = arr.len();
            let start_idx = start.map(|s| normalize_index(s, len)).unwrap_or(0);
            let end_idx = end.map(|e| normalize_index(e, len)).unwrap_or(len);

            let start_idx = start_idx.min(len);
            let end_idx = end_idx.min(len);

            if start_idx >= end_idx {
                Ok(Value::Sequence(vec![]))
            } else {
                Ok(Value::Sequence(arr[start_idx..end_idx].to_vec()))
            }
        }
        Value::String(s) => {
            let len = s.chars().count();
            let start_idx = start.map(|s| normalize_index(s, len)).unwrap_or(0);
            let end_idx = end.map(|e| normalize_index(e, len)).unwrap_or(len);

            let start_idx = start_idx.min(len);
            let end_idx = end_idx.min(len);

            if start_idx >= end_idx {
                Ok(Value::String(String::new()))
            } else {
                let result: String = s
                    .chars()
                    .skip(start_idx)
                    .take(end_idx - start_idx)
                    .collect();
                Ok(Value::String(result))
            }
        }
        _ => Err(anyhow!(
            "Cannot slice {}",
            crate::evaluator::helpers::value_type(&target_val)
        )),
    }
}

fn normalize_index(idx: isize, len: usize) -> usize {
    if idx < 0 {
        len.saturating_sub(idx.unsigned_abs())
    } else {
        idx as usize
    }
}
