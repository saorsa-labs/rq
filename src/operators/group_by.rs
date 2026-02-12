//! Group by function

use crate::evaluator::{Context, Evaluator, helpers};
use crate::parser::expression::Expression;
use anyhow::{Result, anyhow};
use serde_yaml::Value;
use std::collections::HashMap;

/// Evaluate group_by function
pub fn eval(
    evaluator: &Evaluator,
    target: &Expression,
    key_expr: &Expression,
    ctx: &Context,
) -> Result<Value> {
    let target_val = evaluator.eval(target, ctx)?;

    match target_val {
        Value::Sequence(arr) => {
            let mut groups: HashMap<String, Vec<Value>> = HashMap::new();

            for item in arr {
                let item_ctx = ctx.child(item.clone());
                let key_val = evaluator.eval(key_expr, &item_ctx)?;
                let key = helpers::value_to_string(&key_val);

                groups.entry(key).or_default().push(item);
            }

            // Convert to array of {key: ..., value: ...} objects
            let result: Vec<Value> = groups
                .into_iter()
                .map(|(key, values)| {
                    let mut obj = serde_yaml::Mapping::new();
                    obj.insert(Value::String("key".to_string()), Value::String(key));
                    obj.insert(Value::String("value".to_string()), Value::Sequence(values));
                    Value::Mapping(obj)
                })
                .collect();

            Ok(Value::Sequence(result))
        }
        _ => Err(anyhow!("Cannot group {}", helpers::value_type(&target_val))),
    }
}
