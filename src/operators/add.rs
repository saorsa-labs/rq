//! Add function (sum all elements)

use crate::evaluator::{Context, Evaluator};
use anyhow::{Result, anyhow};
use serde_yaml::Value;

/// Evaluate add function - sum all elements in the array
pub fn eval(_evaluator: &Evaluator, ctx: &Context) -> Result<Value> {
    match &ctx.value {
        Value::Sequence(arr) => {
            let mut sum: f64 = 0.0;
            let mut all_integers = true;

            for item in arr {
                match item {
                    Value::Number(n) => {
                        if let Some(i) = n.as_i64() {
                            sum += i as f64;
                        } else if let Some(f) = n.as_f64() {
                            sum += f;
                            all_integers = false;
                        }
                    }
                    _ => return Err(anyhow!("Cannot add non-numeric value")),
                }
            }

            if all_integers && sum.fract() == 0.0 {
                Ok(Value::Number((sum as i64).into()))
            } else {
                Ok(Value::Number(serde_yaml::Number::from(sum)))
            }
        }
        Value::String(s) => {
            // Concatenate all strings in the array
            match &ctx.value {
                Value::Sequence(arr) => {
                    let mut result = String::new();
                    for item in arr {
                        if let Value::String(s) = item {
                            result.push_str(s);
                        } else {
                            return Err(anyhow!("Cannot add non-string value to string"));
                        }
                    }
                    Ok(Value::String(result))
                }
                _ => Ok(Value::String(s.clone())),
            }
        }
        _ => Err(anyhow!(
            "Cannot add elements of {}",
            crate::evaluator::helpers::value_type(&ctx.value)
        )),
    }
}
