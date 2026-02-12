//! Array constructor

use crate::evaluator::{Context, Evaluator};
use crate::parser::expression::Expression;
use anyhow::Result;
use serde_yaml::Value;

/// Evaluate array constructor
pub fn eval(evaluator: &Evaluator, elements: &[Expression], ctx: &Context) -> Result<Value> {
    let mut result = Vec::new();
    for expr in elements {
        let val = evaluator.eval(expr, ctx)?;
        result.push(val);
    }
    Ok(Value::Sequence(result))
}
