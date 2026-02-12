//! ToString function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate tostring function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;
    Ok(Value::String(helpers::value_to_string(&target_val)))
}
