//! Output formatting module for rq
//!
//! Handles formatting output in various formats (YAML, JSON, TOML).

#![allow(dead_code)]

use anyhow::{Context, Result, anyhow};
use serde_yaml::Value;

/// Output format options
#[derive(Debug, Clone)]
pub struct OutputOptions {
    /// Indentation level
    pub indent: usize,
    /// Pretty print
    pub pretty_print: bool,
    /// Unwrap scalar values
    pub unwrap_scalar: bool,
    /// Don't print document separators
    pub no_doc: bool,
    /// Use colors
    pub colors: bool,
}

/// Format output value
pub fn format_output(
    value: &Value,
    format: crate::OutputFormat,
    options: OutputOptions,
) -> Result<String> {
    match format {
        crate::OutputFormat::Yaml => format_yaml(value, &options),
        crate::OutputFormat::Json => format_json(value, &options),
        crate::OutputFormat::Toml => format_toml(value, &options),
        crate::OutputFormat::Auto => format_yaml(value, &options),
    }
}

/// Format as YAML
fn format_yaml(value: &Value, options: &OutputOptions) -> Result<String> {
    // Handle unwrapped scalars
    if options.unwrap_scalar {
        match value {
            Value::String(s) => {
                return Ok(s.clone());
            }
            Value::Number(n) => {
                return Ok(n.to_string());
            }
            Value::Bool(b) => {
                return Ok(b.to_string());
            }
            Value::Null => {
                return Ok("null".to_string());
            }
            _ => {}
        }
    }

    let mut output = String::new();

    // Add document separator if not disabled
    if !options.no_doc && !matches!(value, Value::Null) {
        output.push_str("---\n");
    }

    // Serialize YAML
    let yaml_str = serde_yaml::to_string(value).context("Failed to serialize YAML")?;

    output.push_str(&yaml_str);

    // Apply colors if requested
    if options.colors {
        output = colorize_yaml(&output);
    }

    Ok(output)
}

/// Format as JSON
fn format_json(value: &Value, options: &OutputOptions) -> Result<String> {
    // Convert YAML value to JSON value
    let json_value = yaml_to_json(value.clone());

    let output = if options.pretty_print {
        serde_json::to_string_pretty(&json_value).context("Failed to serialize JSON")?
    } else {
        serde_json::to_string(&json_value).context("Failed to serialize JSON")?
    };

    Ok(output)
}

/// Format as TOML
fn format_toml(value: &Value, _options: &OutputOptions) -> Result<String> {
    // Convert YAML value to TOML value
    let toml_value = yaml_to_toml(value.clone())?;

    let output = toml::to_string_pretty(&toml_value).context("Failed to serialize TOML")?;

    Ok(output)
}

/// Convert YAML value to JSON value
fn yaml_to_json(value: Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                serde_json::Value::Number(i.into())
            } else if let Some(f) = n.as_f64() {
                serde_json::Number::from_f64(f)
                    .map(serde_json::Value::Number)
                    .unwrap_or(serde_json::Value::Null)
            } else {
                serde_json::Value::Null
            }
        }
        Value::String(s) => serde_json::Value::String(s),
        Value::Sequence(arr) => {
            serde_json::Value::Array(arr.into_iter().map(yaml_to_json).collect())
        }
        Value::Mapping(map) => {
            let mut obj = serde_json::Map::new();
            for (k, v) in map {
                let key = match k {
                    Value::String(s) => s,
                    _ => serde_yaml::to_string(&k)
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                };
                obj.insert(key, yaml_to_json(v));
            }
            serde_json::Value::Object(obj)
        }
        _ => serde_json::Value::Null,
    }
}

/// Convert YAML value to TOML value
fn yaml_to_toml(value: Value) -> Result<toml::Value> {
    match value {
        Value::Null => Ok(toml::Value::String("null".to_string())),
        Value::Bool(b) => Ok(toml::Value::Boolean(b)),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Ok(toml::Value::Integer(i))
            } else if let Some(f) = n.as_f64() {
                Ok(toml::Value::Float(f))
            } else {
                Err(anyhow!("Invalid number"))
            }
        }
        Value::String(s) => Ok(toml::Value::String(s)),
        Value::Sequence(arr) => {
            let values: Result<Vec<_>> = arr.into_iter().map(yaml_to_toml).collect();
            Ok(toml::Value::Array(values?))
        }
        Value::Mapping(map) => {
            let mut table = toml::map::Map::new();
            for (k, v) in map {
                let key = match k {
                    Value::String(s) => s,
                    _ => serde_yaml::to_string(&k)
                        .unwrap_or_default()
                        .trim()
                        .to_string(),
                };
                table.insert(key, yaml_to_toml(v)?);
            }
            Ok(toml::Value::Table(table))
        }
        _ => Err(anyhow!("Unsupported value type for TOML")),
    }
}

/// Apply colors to YAML output
fn colorize_yaml(yaml: &str) -> String {
    use colored::Colorize;

    let mut result = String::new();
    for line in yaml.lines() {
        let colored_line = if line.starts_with("---") || line.starts_with("...") {
            line.dimmed().to_string()
        } else if line.trim_start().starts_with('#') {
            line.bright_black().to_string()
        } else if line.contains(':') {
            // Key: value line
            let parts: Vec<&str> = line.splitn(2, ':').collect();
            if parts.len() == 2 {
                format!("{}:{}", parts[0].cyan(), parts[1])
            } else {
                line.to_string()
            }
        } else if line.trim().starts_with('-') {
            // Array item
            let trimmed = line.trim_start();
            let indent = &line[..line.len() - trimmed.len()];
            format!("{}{}", indent, trimmed.bright_yellow())
        } else if line.trim() == "true" || line.trim() == "false" {
            line.bright_magenta().to_string()
        } else if line.trim() == "null" {
            line.bright_black().to_string()
        } else {
            line.to_string()
        };
        result.push_str(&colored_line);
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_yaml_simple() {
        let value = serde_yaml::from_str("name: test\nvalue: 42").unwrap();
        let options = OutputOptions {
            indent: 2,
            pretty_print: false,
            unwrap_scalar: false,
            no_doc: false,
            colors: false,
        };
        let output = format_yaml(&value, &options).unwrap();
        assert!(output.contains("name: test"));
        assert!(output.contains("value: 42"));
    }

    #[test]
    fn test_format_json() {
        let value = serde_yaml::from_str("name: test\nvalue: 42").unwrap();
        let options = OutputOptions {
            indent: 2,
            pretty_print: true,
            unwrap_scalar: false,
            no_doc: false,
            colors: false,
        };
        let output = format_json(&value, &options).unwrap();
        assert!(output.contains("\"name\": \"test\""));
        assert!(output.contains("\"value\": 42"));
    }

    #[test]
    fn test_unwrap_scalar() {
        let value = Value::String("hello".to_string());
        let options = OutputOptions {
            indent: 2,
            pretty_print: false,
            unwrap_scalar: true,
            no_doc: false,
            colors: false,
        };
        let output = format_yaml(&value, &options).unwrap();
        assert_eq!(output.trim(), "hello");
    }
}
