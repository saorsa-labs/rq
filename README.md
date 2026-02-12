# rq-cli - Rust Query Tool

A lightweight and portable command-line YAML, JSON, and TOML processor written in Rust. `rq` uses jq-like syntax but works seamlessly with multiple data formats.

## Features

- **Multi-format support**: YAML, JSON, and TOML input/output
- **jq-like syntax**: Familiar expression syntax for querying and transforming data
- **Type conversion**: Convert between YAML, JSON, and TOML formats
- **In-place editing**: Modify files directly with `-i` flag
- **Pipes and filters**: Chain operations with the `|` operator
- **Built-in functions**: `keys`, `length`, `sort`, `reverse`, `unique`, `flatten`, `map`, `filter`, `select`, and more
- **Assignment operators**: Update values with `=` and `|=`
- **Arithmetic operations**: `+`, `-`, `*`, `/`, `%`
- **Comparison operators**: `==`, `!=`, `<`, `<=`, `>`, `>=`
- **Logical operators**: `and`, `or`, `not`

## Installation

### From Source

```bash
git clone https://github.com/saorsa-labs/rq-cli
cd rq-cli
cargo build --release
```

The binary will be available at `target/release/rq`.

## Quick Start

### Basic Usage

```bash
# Read a value from YAML
echo 'name: test' | rq '.name'
# Output: test

# Read nested values
echo 'user:
  name: alice
  age: 30' | rq '.user.name'
# Output: alice

# Access array elements
echo '["a", "b", "c"]' | rq -p json '.[1]'
# Output: b

# Get array length
echo '[1, 2, 3, 4, 5]' | rq -p json 'length'
# Output: 5
```

### Format Conversion

```bash
# Convert JSON to YAML
echo '{"name": "test"}' | rq -p json -o yaml '.'

# Convert YAML to JSON
echo 'name: test' | rq -o json '.'

# Convert to TOML
echo '{"name": "test"}' | rq -p json -o toml '.'
```

### Updating Values

```bash
# Simple assignment
echo 'name: test' | rq '.name = "updated"'

# Update in place
rq -i '.version = "1.0.1"' config.yaml

# Update using current value
echo 'count: 5' | rq '.count |= . + 1'
```

### Working with Arrays

```bash
# Map over array
echo '[1, 2, 3]' | rq -p json 'map(., . * 2)'

# Filter array
echo '[1, 2, 3, 4, 5]' | rq -p json 'filter(., . > 2)'

# Sort array
echo '[3, 1, 4, 1, 5]' | rq -p json 'sort'

# Get unique values
echo '[1, 2, 2, 3, 3, 3]' | rq -p json 'unique'

# Reverse array
echo '[1, 2, 3]' | rq -p json 'reverse'

# Flatten nested arrays
echo '[[1, 2], [3, 4]]' | rq -p json 'flatten'
```

### Selection and Filtering

```bash
# Select with condition
echo '[{"name": "foo", "active": true}, {"name": "bar", "active": false}]' | \
  rq -p json '.[] | select(.active == true) | .name'

# Get keys of object
echo '{"a": 1, "b": 2}' | rq -p json 'keys'

# Check if field exists
echo '{"name": "test"}' | rq -p json 'has(., "name")'
```

### Pipes

```bash
# Chain multiple operations
echo '{"items": [{"name": "foo"}, {"name": "bar"}]}' | \
  rq -p json '.items | map(., .name) | sort'
```

### Environment Variables

```bash
# Read environment variable
NAME=world echo '{}' | rq '.message = env("NAME")'
```

## Command Line Options

```
$ rq --help
A lightweight and portable command-line YAML, JSON, and TOML processor

Usage: rq [OPTIONS] [EXPRESSION] [FILE]...

Arguments:
  [EXPRESSION]  The expression to evaluate
  [FILE]...     Input file(s) to process

Options:
  -p, --input-format <INPUT_FORMAT>    Input format [possible values: auto, yaml, json, toml]
  -o, --output-format <OUTPUT_FORMAT>  Output format [possible values: auto, yaml, json, toml]
  -i, --inplace                        Update the file in place
  -n, --null-input                     Don't read input, simply evaluate the expression
  -P, --pretty-print                   Pretty print output
  -C, --colors                         Force print with colors
  -M, --no-colors                      Force print without colors
  -I, --indent <INDENT>                Set indent level for output [default: 2]
  -r, --unwrap-scalar                  Unwrap scalar values (no quotes for strings)
      --from-file <FROM_FILE>          Expression file to load
  -N, --no-doc                         Don't print document separators
  -0, --nul-output                     Use NUL char to separate values
  -e, --exit-status                    Set exit status if no matches or null/false returned
  -v, --verbose                        Verbose mode
  -h, --help                           Print help
  -V, --version                        Print version
```

## Expression Syntax

### Identity

- `.` - The input value

### Field Access

- `.field` - Access field
- `.["field"]` - Access field with special characters
- `.field.nested` - Nested field access

### Array Operations

- `.[0]` - Array index
- `.[-1]` - Negative indexing
- `.[1:3]` - Slice
- `.[]` - Iterator (all elements)

### Operators

- `|` - Pipe (chain operations)
- `,` - Comma (array construction)
- `=` - Assignment
- `|=` - Update assignment
- `//` - Alternative (default value)

### Arithmetic

- `+`, `-`, `*`, `/`, `%`

### Comparison

- `==`, `!=`, `<`, `<=`, `>`, `>=`

### Logical

- `and`, `or`, `not`

## Built-in Functions

| Function | Description |
|----------|-------------|
| `keys` | Get object keys or array indices |
| `length` | Get length of string, array, or object |
| `type` | Get value type |
| `has(key)` | Check if object has key |
| `sort` | Sort array |
| `reverse` | Reverse array or string |
| `unique` | Get unique values |
| `flatten` | Flatten nested arrays |
| `group_by(expr)` | Group array by expression |
| `map(array, expr)` | Map expression over array |
| `filter(array, expr)` | Filter array by expression |
| `select(condition)` | Select if condition is true |
| `first` | Get first element |
| `last` | Get last element |
| `add` | Sum all numbers in array |
| `env(name)` | Get environment variable |
| `tostring` | Convert to string |
| `tonumber` | Convert to number |

## Examples

### Kubernetes Config

```bash
# Get all container images from a deployment
rq '.spec.template.spec.containers[].image' deployment.yaml

# Update replica count
rq -i '.spec.replicas = 5' deployment.yaml
```

### CI/CD Configuration

```bash
# Update version in package.json
rq -i -p json '.version = "1.2.3"' package.json

# Get all dependencies
rq -p json 'keys(.dependencies)' package.json
```

### Log Processing

```bash
# Extract error messages
cat logs.json | rq -p json '.[] | select(.level == "error") | .message'
```

## Differences from yq

This is a Rust reimplementation inspired by [mikefarah/yq](https://github.com/mikefarah/yq). Key differences:

- Written in Rust for performance and safety
- Simplified feature set (core functionality)
- No XML/CSV/TSV support (yet)
- No advanced YAML features like anchors/aliases preservation

## License

MIT OR Apache-2.0

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
