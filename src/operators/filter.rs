//! Filter function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate filter function
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
                let item_ctx = ctx.child(item.clone());
                let condition = evaluator.eval(expr, &item_ctx)?;
                if helpers::is_truthy(&condition) {
                    result.push(item);
                }
            }
            Ok(Value::Sequence(result))
        }
        _ => Err(anyhow!(
            "Cannot filter {}",
            helpers::value_type(&target_val)
        )),
    }
}
