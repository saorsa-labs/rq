//! Last function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate last function
pub fn eval(evaluator: &Evaluator, expr: &Expression, ctx: &Context) -> Result<Value> {
    let val = evaluator.eval(expr, ctx)?;

    match val {
        Value::Sequence(arr) => arr
            .into_iter()
            .last()
            .ok_or_else(|| anyhow!("Cannot get last of empty array")),
        _ => Err(anyhow!(
            "Cannot get last of {}",
            crate::evaluator::helpers::value_type(&val)
        )),
    }
}
