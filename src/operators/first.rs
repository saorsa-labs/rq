//! First function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate first function
pub fn eval(evaluator: &Evaluator, expr: &Expression, ctx: &Context) -> Result<Value> {
    let val = evaluator.eval(expr, ctx)?;

    match val {
        Value::Sequence(arr) => arr
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("Cannot get first of empty array")),
        _ => Err(anyhow!(
            "Cannot get first of {}",
            crate::evaluator::helpers::value_type(&val)
        )),
    }
}
