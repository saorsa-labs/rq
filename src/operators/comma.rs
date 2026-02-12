//! Comma operator (,)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate comma - collect results into an array
pub fn eval(
    _evaluator: &Evaluator,
    _left: &Expression,
    _right: &Expression,
    _ctx: &Context,
) -> Result<Value> {
    // Comma is handled during parsing as array constructor
    // This should not be called directly
    Ok(Value::Null)
}
