//! Select filter

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate select filter
pub fn eval(evaluator: &Evaluator, condition: &Expression, ctx: &Context) -> Result<Value> {
    let condition_val = evaluator.eval(condition, ctx)?;

    if helpers::is_truthy(&condition_val) {
        Ok(ctx.value.clone())
    } else {
        // Return empty/null for filtered out items
        Ok(Value::Null)
    }
}
