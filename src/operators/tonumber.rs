//! ToNumber function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate tonumber function
pub fn eval(evaluator: &Evaluator, target: &Expression, ctx: &Context) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match &target_val {
        Value::Number(_) => Ok(target_val),
        Value::String(s) => {
            if let Ok(i) = s.parse::<i64>() {
                Ok(Value::Number(i.into()))
            } else if let Ok(f) = s.parse::<f64>() {
                Ok(Value::Number(serde_yaml::Number::from(f)))
            } else {
                Err(anyhow!("Cannot parse '{}' as number", s))
            }
        }
        _ => Err(anyhow!(
            "Cannot convert {} to number",
            helpers::value_type(&target_val)
        )),
    }
}
