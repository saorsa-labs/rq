//! Input parsing for different data formats

use anyhow::{Context, Result};
use serde_yaml::Value;

/// Supported input formats
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputFormat {
    Yaml,
    Json,
    Toml,
}

/// Parser for input documents
pub struct InputParser;

impl InputParser {
    /// Parse input data into a YAML Value
    pub fn parse(data: &str, format: InputFormat) -> Result<Value> {
        match format {
            InputFormat::Yaml => Self::parse_yaml(data),
            InputFormat::Json => Self::parse_json(data),
            InputFormat::Toml => Self::parse_toml(data),
        }
    }

    /// Parse YAML input
    fn parse_yaml(data: &str) -> Result<Value> {
        serde_yaml::from_str(data).context("Failed to parse YAML")
    }

    /// Parse JSON input
    fn parse_json(data: &str) -> Result<Value> {
        let json_value: serde_json::Value =
            serde_json::from_str(data).context("Failed to parse JSON")?;
        Ok(Self::json_to_yaml(json_value))
    }

    /// Parse TOML input
    fn parse_toml(data: &str) -> Result<Value> {
        let toml_value: toml::Value = toml::from_str(data).context("Failed to parse TOML")?;
        Ok(Self::toml_to_yaml(toml_value))
    }

    /// Convert JSON Value to YAML Value
    fn json_to_yaml(value: serde_json::Value) -> Value {
        match value {
            serde_json::Value::Null => Value::Null,
            serde_json::Value::Bool(b) => Value::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Value::Number(i.into())
                } else if let Some(f) = n.as_f64() {
                    Value::Number(serde_yaml::Number::from(f))
                } else {
                    Value::Null
                }
            }
            serde_json::Value::String(s) => Value::String(s),
            serde_json::Value::Array(arr) => {
                Value::Sequence(arr.into_iter().map(Self::json_to_yaml).collect())
            }
            serde_json::Value::Object(obj) => {
                let mut map = serde_yaml::Mapping::new();
                for (k, v) in obj {
                    map.insert(Value::String(k), Self::json_to_yaml(v));
                }
                Value::Mapping(map)
            }
        }
    }

    /// Convert TOML Value to YAML Value
    fn toml_to_yaml(value: toml::Value) -> Value {
        match value {
            toml::Value::String(s) => Value::String(s),
            toml::Value::Integer(i) => Value::Number(i.into()),
            toml::Value::Float(f) => Value::Number(serde_yaml::Number::from(f)),
            toml::Value::Boolean(b) => Value::Bool(b),
            toml::Value::Datetime(dt) => Value::String(dt.to_string()),
            toml::Value::Array(arr) => {
                Value::Sequence(arr.into_iter().map(Self::toml_to_yaml).collect())
            }
            toml::Value::Table(table) => {
                let mut map = serde_yaml::Mapping::new();
                for (k, v) in table {
                    map.insert(Value::String(k), Self::toml_to_yaml(v));
                }
                Value::Mapping(map)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_yaml() {
        let yaml = "name: test\nvalue: 42";
        let result = InputParser::parse(yaml, InputFormat::Yaml).unwrap();

        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_parse_json() {
        let json = r#"{"name": "test", "value": 42}"#;
        let result = InputParser::parse(json, InputFormat::Json).unwrap();

        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_parse_toml() {
        let toml = r#"name = "test"
value = 42"#;
        let result = InputParser::parse(toml, InputFormat::Toml).unwrap();

        assert_eq!(result["name"], "test");
        assert_eq!(result["value"], 42);
    }

    #[test]
    fn test_parse_nested_yaml() {
        let yaml = "root:\n  items:\n    - name: foo\n      value: 1";
        let result = InputParser::parse(yaml, InputFormat::Yaml).unwrap();

        assert!(result["root"]["items"].is_sequence());
        assert_eq!(result["root"]["items"][0]["name"], "foo");
    }
}
