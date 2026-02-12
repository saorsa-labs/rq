//! Logical operators (and, or, not)

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Logical AND
pub fn and(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;

    // Short-circuit: if left is falsy, return left
    if !helpers::is_truthy(&left_val) {
        return Ok(left_val);
    }

    evaluator.eval(right, ctx)
}

/// Logical OR
pub fn or(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;

    // Short-circuit: if left is truthy, return left
    if helpers::is_truthy(&left_val) {
        return Ok(left_val);
    }

    evaluator.eval(right, ctx)
}

/// Logical NOT
pub fn not(evaluator: &Evaluator, expr: &Expression, ctx: &Context) -> Result<Value> {
    let val = evaluator.eval(expr, ctx)?;
    Ok(Value::Bool(!helpers::is_truthy(&val)))
}
