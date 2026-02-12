//! rq - A lightweight and portable command-line YAML, JSON, and TOML processor
//!
//! rq uses jq-like syntax but works with YAML, JSON, and TOML files.
//! It supports reading, querying, updating, and converting between formats.

use anyhow::{Context, Result};
use clap::{Parser, ValueEnum, CommandFactory};
use colored::Colorize;
use std::io::{self, Read};
use std::path::PathBuf;

mod evaluator;
mod operators;
mod output;
mod parser;

use evaluator::Evaluator;
use parser::expression::ExpressionParser;
use parser::input::InputParser;

/// rq - A lightweight command-line YAML, JSON, and TOML processor
#[derive(Parser, Debug)]
#[command(name = "rq")]
#[command(about = "A lightweight and portable command-line YAML, JSON, and TOML processor")]
#[command(version)]
struct Cli {
    /// The expression to evaluate
    #[arg(value_name = "EXPRESSION")]
    expression: Option<String>,

    /// Input file(s) to process
    #[arg(value_name = "FILE")]
    files: Vec<PathBuf>,

    /// Input format
    #[arg(short = 'p', long = "input-format", value_enum)]
    input_format: Option<InputFormat>,

    /// Output format
    #[arg(short = 'o', long = "output-format", value_enum)]
    output_format: Option<OutputFormat>,

    /// Update the file in place
    #[arg(short = 'i', long = "inplace")]
    inplace: bool,

    /// Don't read input, simply evaluate the expression
    #[arg(short = 'n', long = "null-input")]
    null_input: bool,

    /// Pretty print output
    #[arg(short = 'P', long = "pretty-print")]
    pretty_print: bool,

    /// Force print with colors
    #[arg(short = 'C', long = "colors")]
    colors: bool,

    /// Force print without colors
    #[arg(short = 'M', long = "no-colors")]
    no_colors: bool,

    /// Set indent level for output
    #[arg(short = 'I', long = "indent", default_value = "2")]
    indent: usize,

    /// Unwrap scalar values (no quotes for strings)
    #[arg(short = 'r', long = "unwrap-scalar", default_value = "true")]
    unwrap_scalar: bool,

    /// Expression file to load
    #[arg(long = "from-file")]
    from_file: Option<PathBuf>,

    /// Don't print document separators
    #[arg(short = 'N', long = "no-doc")]
    no_doc: bool,

    /// Use NUL char to separate values
    #[arg(short = '0', long = "nul-output")]
    nul_output: bool,

    /// Set exit status if no matches or null/false returned
    #[arg(short = 'e', long = "exit-status")]
    exit_status: bool,

    /// Verbose mode
    #[arg(short = 'v', long = "verbose")]
    verbose: bool,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum InputFormat {
    Auto,
    Yaml,
    Json,
    Toml,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
enum OutputFormat {
    Auto,
    Yaml,
    Json,
    Toml,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    // Setup colors
    if cli.no_colors {
        colored::control::set_override(false);
    } else if cli.colors {
        colored::control::set_override(true);
    }

    // Get expression
    let expression = if let Some(file) = cli.from_file {
        std::fs::read_to_string(file).context("Failed to read expression file")?
    } else if let Some(expr) = cli.expression {
        expr
    } else if cli.null_input {
        String::new()
    } else {
        // No expression provided - show help
        Cli::command().print_help()?;
        println!();
        return Ok(());
    };

    if cli.verbose {
        eprintln!("{} {}", "Expression:".dimmed(), expression);
    }

    // Parse the expression
    let parser = ExpressionParser::new();
    let expr = parser
        .parse(&expression)
        .context("Failed to parse expression")?;

    if cli.verbose {
        eprintln!("{} {:?}", "Parsed:".dimmed(), expr);
    }

    // Read input
    let input_data = if cli.null_input {
        None
    } else if cli.files.is_empty() {
        // Read from stdin
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("Failed to read from stdin")?;
        if buffer.trim().is_empty() {
            None
        } else {
            Some(buffer)
        }
    } else {
        // Read from file(s)
        let mut buffer = String::new();
        for file in &cli.files {
            let content = std::fs::read_to_string(file)
                .with_context(|| format!("Failed to read file: {}", file.display()))?;
            buffer.push_str(&content);
            buffer.push('\n');
        }
        Some(buffer)
    };

    // Parse input
    let input_format = cli.input_format.unwrap_or(InputFormat::Auto);
    let parsed_input = if let Some(data) = input_data {
        let format = detect_format(&data, input_format, cli.files.first())?;
        Some(InputParser::parse(&data, format)?)
    } else {
        None
    };

    // Evaluate expression
    let evaluator = Evaluator::new();
    let result = evaluator.evaluate(&expr, parsed_input.as_ref())?;

    // Determine output format
    let output_format = cli.output_format.unwrap_or({
        if cli.pretty_print {
            OutputFormat::Yaml
        } else {
            match input_format {
                InputFormat::Json => OutputFormat::Json,
                InputFormat::Toml => OutputFormat::Toml,
                _ => OutputFormat::Yaml,
            }
        }
    });

    // Output result
    let output = output::format_output(
        &result,
        output_format,
        output::OutputOptions {
            indent: cli.indent,
            pretty_print: cli.pretty_print,
            unwrap_scalar: cli.unwrap_scalar,
            no_doc: cli.no_doc,
            colors: cli.colors && !cli.no_colors,
        },
    )?;

    // Handle in-place editing
    if cli.inplace && !cli.files.is_empty() {
        let file = &cli.files[0];
        std::fs::write(file, output)
            .with_context(|| format!("Failed to write to file: {}", file.display()))?;
    } else {
        print!("{}", output);
        if !output.ends_with('\n') {
            println!();
        }
    }

    // Handle exit status
    if cli.exit_status {
        let is_empty = match &result {
            serde_yaml::Value::Null => true,
            serde_yaml::Value::Bool(b) => !b,
            serde_yaml::Value::Sequence(arr) => arr.is_empty(),
            serde_yaml::Value::Mapping(map) => map.is_empty(),
            _ => false,
        };
        if is_empty {
            std::process::exit(1);
        }
    }

    Ok(())
}

fn detect_format(
    data: &str,
    format: InputFormat,
    file: Option<&PathBuf>,
) -> Result<parser::input::InputFormat> {
    match format {
        InputFormat::Auto => {
            // Try to detect from file extension
            if let Some(path) = file {
                if let Some(ext) = path.extension() {
                    let ext = ext.to_string_lossy().to_lowercase();
                    match ext.as_str() {
                        "json" => return Ok(parser::input::InputFormat::Json),
                        "toml" => return Ok(parser::input::InputFormat::Toml),
                        "yaml" | "yml" => return Ok(parser::input::InputFormat::Yaml),
                        _ => {}
                    }
                }
            }

            // Try to detect from content
            let trimmed = data.trim_start();
            if trimmed.starts_with('{') || trimmed.starts_with('[') {
                // Try JSON first
                if serde_json::from_str::<serde_json::Value>(data).is_ok() {
                    return Ok(parser::input::InputFormat::Json);
                }
            }

            // Check for TOML
            if trimmed.contains('=')
                && !trimmed.starts_with('-')
                && toml::from_str::<toml::Value>(data).is_ok()
            {
                return Ok(parser::input::InputFormat::Toml);
            }

            // Default to YAML
            Ok(parser::input::InputFormat::Yaml)
        }
        InputFormat::Yaml => Ok(parser::input::InputFormat::Yaml),
        InputFormat::Json => Ok(parser::input::InputFormat::Json),
        InputFormat::Toml => Ok(parser::input::InputFormat::Toml),
    }
}
