//! Alternative operator (//)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate alternative - returns left if not null/false, otherwise right
pub fn eval(
    evaluator: &Evaluator,
    left: &Expression,
    right: &Expression,
    ctx: &Context,
) -> Result<Value> {
    match evaluator.eval(left, ctx) {
        Ok(val) => {
            if val.is_null() || val == Value::Bool(false) {
                evaluator.eval(right, ctx)
            } else {
                Ok(val)
            }
        }
        Err(_) => evaluator.eval(right, ctx),
    }
}
