//! Environment variable function

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;
use std::env;

/// Evaluate env function - read environment variable
pub fn eval(evaluator: &Evaluator, name: &Expression, ctx: &Context) -> Result<Value> {
    let name_val = evaluator.eval(name, ctx)?;

    if let Value::String(name_str) = name_val {
        match env::var(&name_str) {
            Ok(val) => Ok(Value::String(val)),
            Err(_) => Ok(Value::Null),
        }
    } else {
        Ok(Value::Null)
    }
}
