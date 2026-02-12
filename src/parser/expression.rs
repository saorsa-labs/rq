//! Expression parser for rq
//!
//! Parses jq-like expressions into an AST for evaluation.

#![allow(dead_code)]

use anyhow::{Context, Result, anyhow};
use std::iter::Peekable;
use std::str::Chars;

/// Represents a parsed expression
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    /// Identity (.) - returns the input unchanged
    Identity,

    /// Literal value
    Literal(serde_yaml::Value),

    /// Field access (.field or .["field"])
    FieldAccess {
        target: Box<Expression>,
        field: String,
    },

    /// Array index access (.[index])
    IndexAccess {
        target: Box<Expression>,
        index: isize,
    },

    /// Array/Object iterator (.[])
    Iterator { target: Box<Expression> },

    /// Pipe (|) - pass result to next expression
    Pipe {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Comma (,) - collect multiple results
    Comma {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Assignment (=)
    Assign {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Update assignment (|=)
    Update {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Addition (+)
    Add {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Subtraction (-)
    Subtract {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Multiplication (*)
    Multiply {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Division (/)
    Divide {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Modulo (%)
    Modulo {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Equality (==)
    Equal {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Inequality (!=)
    NotEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Less than (<)
    LessThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Less than or equal (<=)
    LessThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Greater than (>)
    GreaterThan {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Greater than or equal (>=)
    GreaterThanOrEqual {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Logical AND
    And {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Logical OR
    Or {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Logical NOT
    Not { expr: Box<Expression> },

    /// Select filter
    Select { condition: Box<Expression> },

    /// Keys function
    Keys { target: Box<Expression> },

    /// Length function
    Length { target: Box<Expression> },

    /// Type function
    Type { target: Box<Expression> },

    /// Has function
    Has {
        target: Box<Expression>,
        key: Box<Expression>,
    },

    /// Sort function
    Sort { target: Box<Expression> },

    /// Reverse function
    Reverse { target: Box<Expression> },

    /// Unique function
    Unique { target: Box<Expression> },

    /// Flatten function
    Flatten { target: Box<Expression> },

    /// Group by function
    GroupBy {
        target: Box<Expression>,
        key_expr: Box<Expression>,
    },

    /// Map function
    Map {
        target: Box<Expression>,
        expr: Box<Expression>,
    },

    /// Filter function
    Filter {
        target: Box<Expression>,
        expr: Box<Expression>,
    },

    /// Recurse function (..)
    Recurse,

    /// Parenthesized expression
    Group { expr: Box<Expression> },

    /// Variable reference ($name)
    Variable { name: String },

    /// Array constructor
    Array { elements: Vec<Expression> },

    /// Object constructor
    Object {
        fields: Vec<(Expression, Expression)>,
    },

    /// Conditional (if-then-else)
    IfThenElse {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Box<Expression>,
    },

    /// Array slice (.[start:end])
    Slice {
        target: Box<Expression>,
        start: Option<isize>,
        end: Option<isize>,
    },

    /// Alternative operator (//)
    Alternative {
        left: Box<Expression>,
        right: Box<Expression>,
    },

    /// Try-catch
    Try {
        expr: Box<Expression>,
        catch: Option<Box<Expression>>,
    },

    /// Path expression
    Path { expr: Box<Expression> },

    /// Get path
    GetPath {
        target: Box<Expression>,
        path: Box<Expression>,
    },

    /// Set path
    SetPath {
        target: Box<Expression>,
        path: Box<Expression>,
        value: Box<Expression>,
    },

    /// Delete path
    DelPath {
        target: Box<Expression>,
        path: Box<Expression>,
    },

    /// Range function
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
    },

    /// Limit function
    Limit {
        n: Box<Expression>,
        expr: Box<Expression>,
    },

    /// First function
    First { expr: Box<Expression> },

    /// Last function
    Last { expr: Box<Expression> },

    /// Nth function
    Nth {
        n: Box<Expression>,
        expr: Box<Expression>,
    },

    /// Empty - produces no output
    Empty,

    /// Error function
    Error { message: Option<Box<Expression>> },

    /// Debug function
    Debug { expr: Box<Expression> },

    /// Env function - read environment variable
    Env { name: Box<Expression> },

    /// Split function
    Split {
        target: Box<Expression>,
        separator: Box<Expression>,
    },

    /// Join function
    Join {
        target: Box<Expression>,
        separator: Box<Expression>,
    },

    /// Contains function
    Contains {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Inside function
    Inside {
        target: Box<Expression>,
        container: Box<Expression>,
    },

    /// Startswith function
    StartsWith {
        target: Box<Expression>,
        prefix: Box<Expression>,
    },

    /// Endswith function
    EndsWith {
        target: Box<Expression>,
        suffix: Box<Expression>,
    },

    /// Ltrimstr function
    Ltrimstr {
        target: Box<Expression>,
        prefix: Box<Expression>,
    },

    /// Rtrimstr function
    Rtrimstr {
        target: Box<Expression>,
        suffix: Box<Expression>,
    },

    /// Format function
    Format {
        target: Box<Expression>,
        fmt: String,
    },

    /// Tostring function
    ToString { target: Box<Expression> },

    /// Tonumber function
    ToNumber { target: Box<Expression> },

    /// Floor function
    Floor { target: Box<Expression> },

    /// Ceil function
    Ceil { target: Box<Expression> },

    /// Sqrt function
    Sqrt { target: Box<Expression> },

    /// Add function (sum all elements)
    AddOp,

    /// Min function
    Min { target: Box<Expression> },

    /// Max function
    Max { target: Box<Expression> },

    /// Min_by function
    MinBy {
        target: Box<Expression>,
        key: Box<Expression>,
    },

    /// Max_by function
    MaxBy {
        target: Box<Expression>,
        key: Box<Expression>,
    },

    /// Any function
    Any { target: Box<Expression> },

    /// All function
    All { target: Box<Expression> },

    /// Indices function
    Indices {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Index function
    Index {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Rindex function
    Rindex {
        target: Box<Expression>,
        value: Box<Expression>,
    },

    /// Inside function (string)
    InsideString {
        target: Box<Expression>,
        substr: Box<Expression>,
    },

    /// Test function (regex)
    Test {
        target: Box<Expression>,
        pattern: Box<Expression>,
    },

    /// Match function (regex)
    Match {
        target: Box<Expression>,
        pattern: Box<Expression>,
    },

    /// Capture function (regex)
    Capture {
        target: Box<Expression>,
        pattern: Box<Expression>,
    },

    /// Scan function
    Scan {
        target: Box<Expression>,
        pattern: Box<Expression>,
    },

    /// Splits function
    Splits {
        target: Box<Expression>,
        pattern: Box<Expression>,
    },

    /// Sub function
    Sub {
        target: Box<Expression>,
        pattern: Box<Expression>,
        replacement: Box<Expression>,
    },

    /// Gsub function
    Gsub {
        target: Box<Expression>,
        pattern: Box<Expression>,
        replacement: Box<Expression>,
    },
}

/// Parser for jq-like expressions
pub struct ExpressionParser;

impl ExpressionParser {
    /// Create a new expression parser
    pub fn new() -> Self {
        Self
    }

    /// Parse an expression string into an AST
    pub fn parse(&self, input: &str) -> Result<Expression> {
        let mut chars = input.chars().peekable();
        self.parse_expression(&mut chars)
    }

    /// Parse the main expression (handles pipes)
    fn parse_expression(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let left = self.parse_or(chars)?;

        self.skip_whitespace(chars);

        if self.peek_char(chars) == Some('|') {
            // Check it's not ||
            if self.peek_chars(chars, 2).as_deref() != Some("||") {
                chars.next(); // consume |
                let right = self.parse_expression(chars)?;
                return Ok(Expression::Pipe {
                    left: Box::new(left),
                    right: Box::new(right),
                });
            }
        }

        Ok(left)
    }

    /// Parse OR expression (||)
    fn parse_or(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut left = self.parse_and(chars)?;

        loop {
            self.skip_whitespace(chars);
            if self.peek_chars(chars, 2).as_deref() == Some("||") {
                chars.next();
                chars.next();
                let right = self.parse_and(chars)?;
                left = Expression::Or {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse AND expression (and)
    fn parse_and(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut left = self.parse_comparison(chars)?;

        loop {
            self.skip_whitespace(chars);
            if self.peek_keyword(chars, "and") {
                self.consume_keyword(chars, "and")?;
                let right = self.parse_comparison(chars)?;
                left = Expression::And {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse comparison expressions
    fn parse_comparison(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let left = self.parse_additive(chars)?;

        self.skip_whitespace(chars);

        let op = match self.peek_chars(chars, 2).as_deref() {
            Some("==") => {
                chars.next();
                chars.next();
                "=="
            }
            Some("!=") => {
                chars.next();
                chars.next();
                "!="
            }
            Some("<=") => {
                chars.next();
                chars.next();
                "<="
            }
            Some(">=") => {
                chars.next();
                chars.next();
                ">="
            }
            _ => match self.peek_char(chars) {
                Some('<') => {
                    chars.next();
                    "<"
                }
                Some('>') => {
                    chars.next();
                    ">"
                }
                _ => return Ok(left),
            },
        };

        let right = self.parse_additive(chars)?;

        Ok(match op {
            "==" => Expression::Equal {
                left: Box::new(left),
                right: Box::new(right),
            },
            "!=" => Expression::NotEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
            "<" => Expression::LessThan {
                left: Box::new(left),
                right: Box::new(right),
            },
            "<=" => Expression::LessThanOrEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
            ">" => Expression::GreaterThan {
                left: Box::new(left),
                right: Box::new(right),
            },
            ">=" => Expression::GreaterThanOrEqual {
                left: Box::new(left),
                right: Box::new(right),
            },
            _ => unreachable!(),
        })
    }

    /// Parse additive expressions (+, -)
    fn parse_additive(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut left = self.parse_multiplicative(chars)?;

        loop {
            self.skip_whitespace(chars);
            match self.peek_char(chars) {
                Some('+') => {
                    chars.next();
                    let right = self.parse_multiplicative(chars)?;
                    left = Expression::Add {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some('-') => {
                    // Check for // (alternative operator)
                    if self.peek_chars(chars, 2).as_deref() == Some("//") {
                        break;
                    }
                    chars.next();
                    let right = self.parse_multiplicative(chars)?;
                    left = Expression::Subtract {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse multiplicative expressions (*, /, %)
    fn parse_multiplicative(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut left = self.parse_alternative(chars)?;

        loop {
            self.skip_whitespace(chars);
            match self.peek_char(chars) {
                Some('*') => {
                    chars.next();
                    let right = self.parse_alternative(chars)?;
                    left = Expression::Multiply {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some('/') => {
                    chars.next();
                    let right = self.parse_alternative(chars)?;
                    left = Expression::Divide {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                Some('%') => {
                    chars.next();
                    let right = self.parse_alternative(chars)?;
                    left = Expression::Modulo {
                        left: Box::new(left),
                        right: Box::new(right),
                    };
                }
                _ => break,
            }
        }

        Ok(left)
    }

    /// Parse alternative expressions (//)
    fn parse_alternative(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut left = self.parse_assignment(chars)?;

        loop {
            self.skip_whitespace(chars);
            if self.peek_chars(chars, 2).as_deref() == Some("//") {
                chars.next();
                chars.next();
                let right = self.parse_assignment(chars)?;
                left = Expression::Alternative {
                    left: Box::new(left),
                    right: Box::new(right),
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    /// Parse assignment expressions (=, |=, +=, etc.)
    fn parse_assignment(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let left = self.parse_comma(chars)?;

        self.skip_whitespace(chars);

        // Check for various assignment operators
        if self.peek_chars(chars, 2).as_deref() == Some("|=") {
            chars.next();
            chars.next();
            let value = self.parse_assignment(chars)?;
            return Ok(Expression::Update {
                target: Box::new(left),
                value: Box::new(value),
            });
        }

        // Check for = but not ==
        if self.peek_chars(chars, 2).as_deref() == Some("==") {
            // This is == comparison, not assignment
            return Ok(left);
        }

        match self.peek_char(chars) {
            Some('=') => {
                chars.next();
                let value = self.parse_assignment(chars)?;
                Ok(Expression::Assign {
                    target: Box::new(left),
                    value: Box::new(value),
                })
            }
            _ => Ok(left),
        }
    }

    /// Parse comma expressions (,)
    fn parse_comma(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut elements = vec![self.parse_unary(chars)?];

        loop {
            self.skip_whitespace(chars);
            if self.peek_char(chars) == Some(',') {
                chars.next();
                elements.push(self.parse_unary(chars)?);
            } else {
                break;
            }
        }

        if elements.len() == 1 {
            Ok(elements.into_iter().next().unwrap())
        } else {
            Ok(Expression::Array { elements })
        }
    }

    /// Parse unary expressions (!, -)
    fn parse_unary(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        self.skip_whitespace(chars);

        match self.peek_char(chars) {
            Some('!') => {
                chars.next();
                let expr = self.parse_unary(chars)?;
                Ok(Expression::Not {
                    expr: Box::new(expr),
                })
            }
            Some('-') => {
                // Could be negative number or subtraction
                if let Some(c) = self.peek_chars(chars, 2).and_then(|s| s.chars().nth(1)) {
                    if c.is_ascii_digit() {
                        return self.parse_primary(chars);
                    }
                }
                chars.next();
                let expr = self.parse_unary(chars)?;
                Ok(Expression::Subtract {
                    left: Box::new(Expression::Literal(serde_yaml::Value::Number(0.into()))),
                    right: Box::new(expr),
                })
            }
            _ => self.parse_primary(chars),
        }
    }

    /// Parse primary expressions (literals, identifiers, function calls, etc.)
    fn parse_primary(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        self.skip_whitespace(chars);

        match self.peek_char(chars) {
            Some('.') => self.parse_dot_expression(chars),
            Some('"') | Some('\'') => self.parse_string_literal(chars),
            Some('[') => self.parse_array_constructor(chars),
            Some('{') => self.parse_object_constructor(chars),
            Some('(') => self.parse_group(chars),
            Some('$') => self.parse_variable(chars),
            Some(c) if c.is_ascii_digit() => self.parse_number_literal(chars),
            Some(c) if c.is_alphabetic() || c == '_' => self.parse_identifier_or_function(chars),
            Some(_) => Err(anyhow!("Unexpected character in expression")),
            None => Ok(Expression::Empty),
        }
    }

    /// Parse dot expressions (.field, .[], .[index], etc.)
    fn parse_dot_expression(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume .

        self.skip_whitespace(chars);

        // Check for recursive descent (..)
        if self.peek_char(chars) == Some('.') {
            chars.next();
            return Ok(Expression::Recurse);
        }

        // Check for just . (identity)
        if self
            .peek_char(chars)
            .map(|c| !c.is_alphanumeric() && c != '_' && c != '[')
            .unwrap_or(true)
        {
            return Ok(Expression::Identity);
        }

        // Check for array access .[index]
        if self.peek_char(chars) == Some('[') {
            let mut expr = self.parse_bracket_access(Expression::Identity, chars)?;

            // Parse chained access (.[0][1], .[0].field)
            loop {
                self.skip_whitespace(chars);
                match self.peek_char(chars) {
                    Some('.') => {
                        chars.next();
                        if self.peek_char(chars) == Some('[') {
                            expr = self.parse_bracket_access(expr, chars)?;
                        } else {
                            let field = self.parse_field_name(chars)?;
                            expr = Expression::FieldAccess {
                                target: Box::new(expr),
                                field,
                            };
                        }
                    }
                    Some('[') => {
                        expr = self.parse_bracket_access(expr, chars)?;
                    }
                    _ => break,
                }
            }

            return Ok(expr);
        }

        // Parse field name
        let field = self.parse_field_name(chars)?;
        let mut expr = Expression::FieldAccess {
            target: Box::new(Expression::Identity),
            field,
        };

        // Parse chained access (.field.subfield[0])
        loop {
            self.skip_whitespace(chars);
            match self.peek_char(chars) {
                Some('.') => {
                    chars.next();
                    let field = self.parse_field_name(chars)?;
                    expr = Expression::FieldAccess {
                        target: Box::new(expr),
                        field,
                    };
                }
                Some('[') => {
                    expr = self.parse_bracket_access(expr, chars)?;
                }
                _ => break,
            }
        }

        Ok(expr)
    }

    /// Parse bracket access ([index], ["key"], [:], etc.)
    fn parse_bracket_access(
        &self,
        target: Expression,
        chars: &mut Peekable<Chars>,
    ) -> Result<Expression> {
        chars.next(); // consume [
        self.skip_whitespace(chars);

        // Check for iterator []
        if self.peek_char(chars) == Some(']') {
            chars.next();
            return Ok(Expression::Iterator {
                target: Box::new(target),
            });
        }

        // Check for string key ["field"]
        if let Some(c) = self.peek_char(chars) {
            if c == '"' || c == '\'' {
                let key = self.parse_string_literal(chars)?;
                self.skip_whitespace(chars);
                if self.peek_char(chars) != Some(']') {
                    return Err(anyhow!("Expected ] after string key"));
                }
                chars.next();

                // Convert to field access
                if let Expression::Literal(serde_yaml::Value::String(s)) = key {
                    return Ok(Expression::FieldAccess {
                        target: Box::new(target),
                        field: s,
                    });
                }
                return Err(anyhow!("String key must be a string literal"));
            }
        }

        // Check for slice [:] or [start:end]
        if self.peek_char(chars) == Some(':') {
            chars.next();
            self.skip_whitespace(chars);
            let end = if self.peek_char(chars) == Some(']') {
                None
            } else {
                Some(self.parse_signed_integer(chars)?)
            };
            if self.peek_char(chars) != Some(']') {
                return Err(anyhow!("Expected ] after slice"));
            }
            chars.next();
            return Ok(Expression::Slice {
                target: Box::new(target),
                start: None,
                end,
            });
        }

        // Parse index or start of slice
        let start = self.parse_signed_integer(chars)?;
        self.skip_whitespace(chars);

        if self.peek_char(chars) == Some(':') {
            chars.next();
            self.skip_whitespace(chars);
            let end = if self.peek_char(chars) == Some(']') {
                None
            } else {
                Some(self.parse_signed_integer(chars)?)
            };
            if self.peek_char(chars) != Some(']') {
                return Err(anyhow!("Expected ] after slice"));
            }
            chars.next();
            return Ok(Expression::Slice {
                target: Box::new(target),
                start: Some(start),
                end,
            });
        }

        // Simple index access
        if self.peek_char(chars) != Some(']') {
            return Err(anyhow!("Expected ] after index"));
        }
        chars.next();

        Ok(Expression::IndexAccess {
            target: Box::new(target),
            index: start,
        })
    }

    /// Parse a field name (identifier after dot)
    fn parse_field_name(&self, chars: &mut Peekable<Chars>) -> Result<String> {
        let mut name = String::new();

        while let Some(c) = self.peek_char(chars) {
            if c.is_alphanumeric() || c == '_' || c == '-' {
                name.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            return Err(anyhow!("Expected field name after ."));
        }

        Ok(name)
    }

    /// Parse a signed integer
    fn parse_signed_integer(&self, chars: &mut Peekable<Chars>) -> Result<isize> {
        self.skip_whitespace(chars);

        let negative = if self.peek_char(chars) == Some('-') {
            chars.next();
            true
        } else {
            false
        };

        let num = self.parse_integer(chars)?;
        Ok(if negative { -num } else { num })
    }

    /// Parse an integer
    fn parse_integer(&self, chars: &mut Peekable<Chars>) -> Result<isize> {
        let mut num_str = String::new();

        while let Some(c) = self.peek_char(chars) {
            if c.is_ascii_digit() {
                num_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        num_str.parse::<isize>().context("Invalid integer")
    }

    /// Parse string literal
    fn parse_string_literal(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let quote = chars.next().unwrap();
        let mut value = String::new();

        while let Some(c) = chars.next() {
            if c == quote {
                return Ok(Expression::Literal(serde_yaml::Value::String(value)));
            }
            if c == '\\' {
                match chars.next() {
                    Some('n') => value.push('\n'),
                    Some('t') => value.push('\t'),
                    Some('r') => value.push('\r'),
                    Some('\\') => value.push('\\'),
                    Some('"') => value.push('"'),
                    Some('\'') => value.push('\''),
                    Some(c) => value.push(c),
                    None => return Err(anyhow!("Unterminated string escape")),
                }
            } else {
                value.push(c);
            }
        }

        Err(anyhow!("Unterminated string literal"))
    }

    /// Parse number literal
    fn parse_number_literal(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let mut num_str = String::new();
        let mut is_float = false;

        // Handle negative sign
        if self.peek_char(chars) == Some('-') {
            num_str.push(chars.next().unwrap());
        }

        while let Some(c) = self.peek_char(chars) {
            if c.is_ascii_digit() {
                num_str.push(c);
                chars.next();
            } else if c == '.' && !is_float {
                is_float = true;
                num_str.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if is_float {
            let num: f64 = num_str.parse().context("Invalid float literal")?;
            Ok(Expression::Literal(serde_yaml::Value::Number(
                serde_yaml::Number::from(num),
            )))
        } else {
            let num: i64 = num_str.parse().context("Invalid integer literal")?;
            Ok(Expression::Literal(serde_yaml::Value::Number(num.into())))
        }
    }

    /// Parse array constructor
    fn parse_array_constructor(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume [
        self.skip_whitespace(chars);

        if self.peek_char(chars) == Some(']') {
            chars.next();
            return Ok(Expression::Array { elements: vec![] });
        }

        let mut elements = vec![];
        loop {
            // Parse element without comma handling at the top level
            // We need to parse at a level that doesn't include comma
            // parse_unary is the level before comma
            elements.push(self.parse_unary(chars)?);
            self.skip_whitespace(chars);

            match self.peek_char(chars) {
                Some(',') => {
                    chars.next();
                    self.skip_whitespace(chars);
                    if self.peek_char(chars) == Some(']') {
                        break;
                    }
                }
                Some(']') => break,
                _ => return Err(anyhow!("Expected , or ] in array constructor")),
            }
        }

        if self.peek_char(chars) != Some(']') {
            return Err(anyhow!("Expected ] to close array constructor"));
        }
        chars.next();

        Ok(Expression::Array { elements })
    }

    /// Parse object constructor
    fn parse_object_constructor(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume {
        self.skip_whitespace(chars);

        if self.peek_char(chars) == Some('}') {
            chars.next();
            return Ok(Expression::Object { fields: vec![] });
        }

        let mut fields = vec![];
        loop {
            // Parse key at unary level to avoid comma handling
            let key = self.parse_unary(chars)?;
            self.skip_whitespace(chars);

            if self.peek_char(chars) != Some(':') {
                return Err(anyhow!("Expected : after object key"));
            }
            chars.next();

            // Parse value at unary level
            let value = self.parse_unary(chars)?;
            fields.push((key, value));

            self.skip_whitespace(chars);
            match self.peek_char(chars) {
                Some(',') => {
                    chars.next();
                    self.skip_whitespace(chars);
                    if self.peek_char(chars) == Some('}') {
                        break;
                    }
                }
                Some('}') => break,
                _ => return Err(anyhow!("Expected , or }} in object constructor")),
            }
        }

        if self.peek_char(chars) != Some('}') {
            return Err(anyhow!("Expected }} to close object constructor"));
        }
        chars.next();

        Ok(Expression::Object { fields })
    }

    /// Parse group expression
    fn parse_group(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume (
        let expr = self.parse_expression(chars)?;
        self.skip_whitespace(chars);

        if self.peek_char(chars) != Some(')') {
            return Err(anyhow!("Expected ) to close group"));
        }
        chars.next();

        // Check for chained access after group
        let mut result = expr;
        loop {
            self.skip_whitespace(chars);
            match self.peek_char(chars) {
                Some('.') => {
                    chars.next();
                    let field = self.parse_field_name(chars)?;
                    result = Expression::FieldAccess {
                        target: Box::new(result),
                        field,
                    };
                }
                Some('[') => {
                    result = self.parse_bracket_access(result, chars)?;
                }
                _ => break,
            }
        }

        Ok(Expression::Group {
            expr: Box::new(result),
        })
    }

    /// Parse variable reference
    fn parse_variable(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume $
        let mut name = String::new();

        while let Some(c) = self.peek_char(chars) {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            return Err(anyhow!("Expected variable name after $"));
        }

        Ok(Expression::Variable { name })
    }

    /// Parse identifier or function call
    fn parse_identifier_or_function(&self, chars: &mut Peekable<Chars>) -> Result<Expression> {
        let name = self.parse_identifier(chars)?;

        // Check for boolean literals
        match name.as_str() {
            "true" => return Ok(Expression::Literal(serde_yaml::Value::Bool(true))),
            "false" => return Ok(Expression::Literal(serde_yaml::Value::Bool(false))),
            "null" => return Ok(Expression::Literal(serde_yaml::Value::Null)),
            _ => {}
        }

        self.skip_whitespace(chars);

        // Check for function call with parentheses
        if self.peek_char(chars) == Some('(') {
            return self.parse_function_call(name, chars);
        }

        // Check if it's a built-in function without parentheses (e.g., "keys", "length")
        match self.parse_bare_function(&name) {
            Some(expr) => Ok(expr),
            None => {
                // It's just an identifier - treat as field access on identity
                Ok(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: name,
                })
            }
        }
    }

    /// Parse a bare function name (without parentheses)
    fn parse_bare_function(&self, name: &str) -> Option<Expression> {
        match name {
            "keys" => Some(Expression::Keys {
                target: Box::new(Expression::Identity),
            }),
            "length" => Some(Expression::Length {
                target: Box::new(Expression::Identity),
            }),
            "type" => Some(Expression::Type {
                target: Box::new(Expression::Identity),
            }),
            "sort" => Some(Expression::Sort {
                target: Box::new(Expression::Identity),
            }),
            "reverse" => Some(Expression::Reverse {
                target: Box::new(Expression::Identity),
            }),
            "unique" => Some(Expression::Unique {
                target: Box::new(Expression::Identity),
            }),
            "flatten" => Some(Expression::Flatten {
                target: Box::new(Expression::Identity),
            }),
            "add" => Some(Expression::AddOp),
            "recurse" | ".." => Some(Expression::Recurse),
            _ => None,
        }
    }

    /// Parse an identifier
    fn parse_identifier(&self, chars: &mut Peekable<Chars>) -> Result<String> {
        let mut name = String::new();

        while let Some(c) = self.peek_char(chars) {
            if c.is_alphanumeric() || c == '_' {
                name.push(c);
                chars.next();
            } else {
                break;
            }
        }

        if name.is_empty() {
            return Err(anyhow!("Expected identifier"));
        }

        Ok(name)
    }

    /// Parse function call
    fn parse_function_call(&self, name: String, chars: &mut Peekable<Chars>) -> Result<Expression> {
        chars.next(); // consume (
        self.skip_whitespace(chars);

        let mut args = vec![];
        if self.peek_char(chars) != Some(')') {
            loop {
                args.push(self.parse_expression(chars)?);
                self.skip_whitespace(chars);

                match self.peek_char(chars) {
                    Some(',') => {
                        chars.next();
                        self.skip_whitespace(chars);
                    }
                    Some(')') => break,
                    _ => return Err(anyhow!("Expected , or ) in function call")),
                }
            }
        }

        if self.peek_char(chars) != Some(')') {
            return Err(anyhow!("Expected ) to close function call"));
        }
        chars.next();

        // Handle built-in functions
        match name.as_str() {
            "select" => {
                if args.len() != 1 {
                    return Err(anyhow!("select requires exactly 1 argument"));
                }
                Ok(Expression::Select {
                    condition: Box::new(args.into_iter().next().unwrap()),
                })
            }
            "keys" => {
                if args.is_empty() {
                    Ok(Expression::Keys {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Keys {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("keys takes 0 or 1 arguments"))
                }
            }
            "length" => {
                if args.is_empty() {
                    Ok(Expression::Length {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Length {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("length takes 0 or 1 arguments"))
                }
            }
            "type" => {
                if args.is_empty() {
                    Ok(Expression::Type {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Type {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("type takes 0 or 1 arguments"))
                }
            }
            "has" => {
                if args.len() != 2 {
                    return Err(anyhow!("has requires exactly 2 arguments"));
                }
                let mut args = args.into_iter();
                Ok(Expression::Has {
                    target: Box::new(args.next().unwrap()),
                    key: Box::new(args.next().unwrap()),
                })
            }
            "sort" => {
                if args.is_empty() {
                    Ok(Expression::Sort {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Sort {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("sort takes 0 or 1 arguments"))
                }
            }
            "reverse" => {
                if args.is_empty() {
                    Ok(Expression::Reverse {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Reverse {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("reverse takes 0 or 1 arguments"))
                }
            }
            "unique" => {
                if args.is_empty() {
                    Ok(Expression::Unique {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Unique {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("unique takes 0 or 1 arguments"))
                }
            }
            "flatten" => {
                if args.is_empty() {
                    Ok(Expression::Flatten {
                        target: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Flatten {
                        target: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("flatten takes 0 or 1 arguments"))
                }
            }
            "group_by" => {
                if args.len() != 2 {
                    return Err(anyhow!("group_by requires exactly 2 arguments"));
                }
                let mut args = args.into_iter();
                Ok(Expression::GroupBy {
                    target: Box::new(args.next().unwrap()),
                    key_expr: Box::new(args.next().unwrap()),
                })
            }
            "map" => {
                if args.len() != 2 {
                    return Err(anyhow!("map requires exactly 2 arguments"));
                }
                let mut args = args.into_iter();
                Ok(Expression::Map {
                    target: Box::new(args.next().unwrap()),
                    expr: Box::new(args.next().unwrap()),
                })
            }
            "filter" => {
                if args.len() != 2 {
                    return Err(anyhow!("filter requires exactly 2 arguments"));
                }
                let mut args = args.into_iter();
                Ok(Expression::Filter {
                    target: Box::new(args.next().unwrap()),
                    expr: Box::new(args.next().unwrap()),
                })
            }
            "first" => {
                if args.is_empty() {
                    Ok(Expression::First {
                        expr: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::First {
                        expr: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("first takes 0 or 1 arguments"))
                }
            }
            "last" => {
                if args.is_empty() {
                    Ok(Expression::Last {
                        expr: Box::new(Expression::Identity),
                    })
                } else if args.len() == 1 {
                    Ok(Expression::Last {
                        expr: Box::new(args.into_iter().next().unwrap()),
                    })
                } else {
                    Err(anyhow!("last takes 0 or 1 arguments"))
                }
            }
            "add" => {
                if args.is_empty() {
                    Ok(Expression::AddOp)
                } else {
                    Err(anyhow!("add takes no arguments"))
                }
            }
            "env" => {
                if args.len() != 1 {
                    return Err(anyhow!("env requires exactly 1 argument"));
                }
                Ok(Expression::Env {
                    name: Box::new(args.into_iter().next().unwrap()),
                })
            }
            _ => Err(anyhow!("Unknown function: {}", name)),
        }
    }

    /// Skip whitespace characters
    fn skip_whitespace(&self, chars: &mut Peekable<Chars>) {
        while let Some(c) = self.peek_char(chars) {
            if c.is_whitespace() {
                chars.next();
            } else {
                break;
            }
        }
    }

    /// Peek at the next character without consuming it
    fn peek_char(&self, chars: &mut Peekable<Chars>) -> Option<char> {
        chars.peek().copied()
    }

    /// Peek at the next n characters
    fn peek_chars(&self, chars: &mut Peekable<Chars>, n: usize) -> Option<String> {
        let s: String = chars.clone().take(n).collect();
        if s.len() == n { Some(s) } else { None }
    }

    /// Check if the next characters match a keyword
    fn peek_keyword(&self, chars: &mut Peekable<Chars>, keyword: &str) -> bool {
        let s: String = chars.clone().take(keyword.len()).collect();
        s == keyword
    }

    /// Consume a keyword
    fn consume_keyword(&self, chars: &mut Peekable<Chars>, keyword: &str) -> Result<()> {
        for c in keyword.chars() {
            match chars.next() {
                Some(actual) if actual == c => continue,
                _ => return Err(anyhow!("Expected keyword: {}", keyword)),
            }
        }
        Ok(())
    }
}

impl Default for ExpressionParser {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_identity() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".").unwrap();
        assert_eq!(expr, Expression::Identity);
    }

    #[test]
    fn test_parse_field_access() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".name").unwrap();
        assert_eq!(
            expr,
            Expression::FieldAccess {
                target: Box::new(Expression::Identity),
                field: "name".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_nested_field() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".user.name").unwrap();
        assert_eq!(
            expr,
            Expression::FieldAccess {
                target: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "user".to_string(),
                }),
                field: "name".to_string(),
            }
        );
    }

    #[test]
    fn test_parse_array_index() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".[1]").unwrap();
        assert_eq!(
            expr,
            Expression::IndexAccess {
                target: Box::new(Expression::Identity),
                index: 1,
            }
        );
    }

    #[test]
    fn test_parse_iterator() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".[]").unwrap();
        assert_eq!(
            expr,
            Expression::Iterator {
                target: Box::new(Expression::Identity),
            }
        );
    }

    #[test]
    fn test_parse_pipe() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".items | .name").unwrap();
        assert_eq!(
            expr,
            Expression::Pipe {
                left: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "items".to_string(),
                }),
                right: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "name".to_string(),
                }),
            }
        );
    }

    #[test]
    fn test_parse_assignment() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".name = \"test\"").unwrap();
        assert_eq!(
            expr,
            Expression::Assign {
                target: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "name".to_string(),
                }),
                value: Box::new(Expression::Literal(serde_yaml::Value::String(
                    "test".to_string()
                ))),
            }
        );
    }

    #[test]
    fn test_parse_literal() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("42").unwrap();
        assert_eq!(
            expr,
            Expression::Literal(serde_yaml::Value::Number(42.into()))
        );
    }

    #[test]
    fn test_parse_string() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("\"hello\"").unwrap();
        assert_eq!(
            expr,
            Expression::Literal(serde_yaml::Value::String("hello".to_string()))
        );
    }

    #[test]
    fn test_parse_array() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("[1, 2, 3]").unwrap();
        assert_eq!(
            expr,
            Expression::Array {
                elements: vec![
                    Expression::Literal(serde_yaml::Value::Number(1.into())),
                    Expression::Literal(serde_yaml::Value::Number(2.into())),
                    Expression::Literal(serde_yaml::Value::Number(3.into())),
                ],
            }
        );
    }

    #[test]
    fn test_parse_object() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(r#"{"name": "test", "value": 42}"#).unwrap();
        assert_eq!(
            expr,
            Expression::Object {
                fields: vec![
                    (
                        Expression::Literal(serde_yaml::Value::String("name".to_string())),
                        Expression::Literal(serde_yaml::Value::String("test".to_string())),
                    ),
                    (
                        Expression::Literal(serde_yaml::Value::String("value".to_string())),
                        Expression::Literal(serde_yaml::Value::Number(42.into())),
                    ),
                ],
            }
        );
    }

    #[test]
    fn test_parse_select() {
        let parser = ExpressionParser::new();
        // Test select with parentheses and simple condition
        let expr = parser.parse("select(.active)").unwrap();
        assert_eq!(
            expr,
            Expression::Select {
                condition: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "active".to_string(),
                }),
            }
        );
    }

    #[test]
    fn test_parse_keys_function() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("keys").unwrap();
        assert_eq!(
            expr,
            Expression::Keys {
                target: Box::new(Expression::Identity),
            }
        );
    }

    #[test]
    fn test_parse_length_function() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("length").unwrap();
        assert_eq!(
            expr,
            Expression::Length {
                target: Box::new(Expression::Identity),
            }
        );
    }

    #[test]
    fn test_parse_field_comparison() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".active > 3").unwrap();
        assert_eq!(
            expr,
            Expression::GreaterThan {
                left: Box::new(Expression::FieldAccess {
                    target: Box::new(Expression::Identity),
                    field: "active".to_string(),
                }),
                right: Box::new(Expression::Literal(serde_yaml::Value::Number(3.into()))),
            }
        );
    }

    #[test]
    fn test_parse_arithmetic() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("1 + 2 * 3").unwrap();
        assert_eq!(
            expr,
            Expression::Add {
                left: Box::new(Expression::Literal(serde_yaml::Value::Number(1.into()))),
                right: Box::new(Expression::Multiply {
                    left: Box::new(Expression::Literal(serde_yaml::Value::Number(2.into()))),
                    right: Box::new(Expression::Literal(serde_yaml::Value::Number(3.into()))),
                }),
            }
        );
    }

    #[test]
    fn test_parse_comparison() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("5 > 3").unwrap();
        assert_eq!(
            expr,
            Expression::GreaterThan {
                left: Box::new(Expression::Literal(serde_yaml::Value::Number(5.into()))),
                right: Box::new(Expression::Literal(serde_yaml::Value::Number(3.into()))),
            }
        );
    }

    #[test]
    fn test_parse_group() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("(1 + 2) * 3").unwrap();
        // (1 + 2) * 3 should parse as Multiply(Group(Add(1, 2)), 3)
        // The outer Group is not needed since Multiply is the top-level operation
        assert_eq!(
            expr,
            Expression::Multiply {
                left: Box::new(Expression::Group {
                    expr: Box::new(Expression::Add {
                        left: Box::new(Expression::Literal(serde_yaml::Value::Number(1.into()))),
                        right: Box::new(Expression::Literal(serde_yaml::Value::Number(2.into()))),
                    }),
                }),
                right: Box::new(Expression::Literal(serde_yaml::Value::Number(3.into()))),
            }
        );
    }

    #[test]
    fn test_parse_recurse() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("..").unwrap();
        assert_eq!(expr, Expression::Recurse);
    }

    #[test]
    fn test_parse_slice() {
        let parser = ExpressionParser::new();
        let expr = parser.parse(".[1:3]").unwrap();
        assert_eq!(
            expr,
            Expression::Slice {
                target: Box::new(Expression::Identity),
                start: Some(1),
                end: Some(3),
            }
        );
    }

    #[test]
    fn test_parse_empty() {
        let parser = ExpressionParser::new();
        let expr = parser.parse("empty").unwrap();
        assert_eq!(
            expr,
            Expression::FieldAccess {
                target: Box::new(Expression::Identity),
                field: "empty".to_string(),
            }
        );
    }
}
