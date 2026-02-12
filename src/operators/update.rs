//! Update assignment operator (|=)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate update assignment
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    value: &Expression,
    ctx: &Context,
) -> Result<Value> {
    match target {
        Expression::FieldAccess { target: _, field } => {
            // Get current value
            let current = if let Value::Mapping(map) = &ctx.value {
                map.get(Value::String(field.clone()))
                    .cloned()
                    .unwrap_or(Value::Null)
            } else {
                Value::Null
            };

            // Evaluate RHS with current value as context
            let child_ctx = ctx.child(current);
            let new_value = evaluator.eval(value, &child_ctx)?;

            // Return updated object
            let mut result = ctx.value.clone();
            if let Value::Mapping(ref mut map) = result {
                map.insert(Value::String(field.clone()), new_value);
            }
            Ok(result)
        }
        _ => Err(anyhow!("Update target must be a field access")),
    }
}
