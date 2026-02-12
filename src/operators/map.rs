//! Map function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate map function
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    expr: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(arr) => {
            let mut result = Vec::new();
            for item in arr {
                let item_ctx = ctx.child(item);
                let mapped = evaluator.eval(expr, &item_ctx)?;
                result.push(mapped);
            }
            Ok(Value::Sequence(result))
        }
        _ => Err(anyhow!(
            "Cannot map over {}",
            crate::evaluator::helpers::value_type(&target_val)
        )),
    }
}
