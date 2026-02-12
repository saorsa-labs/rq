//! Pipe operator (|)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate pipe - pass left result to right expression
pub fn eval(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let left_val = evaluator.eval(left, ctx)?;
    let child_ctx = ctx.child(left_val);
    evaluator.eval(right, &child_ctx)
}
