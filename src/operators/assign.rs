//! Assignment operator (=)

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate assignment
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    value: &Expression,
    ctx: &Context,
) -> Result<Value> {
    // For now, simple field assignment
    match target {
        Expression::FieldAccess { target: _, field } => {
            let new_value = evaluator.eval(value, ctx)?;
            // Return a new object with the field set
            let mut result = ctx.value.clone();
            if let Value::Mapping(ref mut map) = result {
                map.insert(Value::String(field.clone()), new_value);
            }
            Ok(result)
        }
        _ => Err(anyhow!("Assignment target must be a field access")),
    }
}
