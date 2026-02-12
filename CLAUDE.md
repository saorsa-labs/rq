# CLAUDE.md - rq Project

## Project Overview

`rq` is a Rust implementation of a command-line YAML, JSON, and TOML processor with jq-like syntax. It's inspired by the Go-based `yq` tool by Mike Farah.

## Architecture

```
rq/
├── src/
│   ├── main.rs           # CLI entry point and argument parsing
│   ├── parser/
│   │   ├── mod.rs        # Parser module exports
│   │   ├── input.rs      # Input format parsing (YAML, JSON, TOML)
│   │   └── expression.rs # jq-like expression parser
│   ├── evaluator/
│   │   └── mod.rs        # Expression evaluator and context
│   ├── operators/        # All expression operators
│   │   ├── mod.rs
│   │   ├── field_access.rs
│   │   ├── index_access.rs
│   │   ├── iterator.rs
│   │   ├── pipe.rs
│   │   ├── arithmetic.rs
│   │   ├── comparison.rs
│   │   ├── logical.rs
│   │   ├── select.rs
│   │   ├── keys.rs
│   │   ├── length.rs
│   │   └── ... (more operators)
│   └── output/
│       └── mod.rs        # Output formatting (YAML, JSON, TOML)
├── Cargo.toml
├── justfile
└── README.md
```

## Key Components

### Expression Parser (`src/parser/expression.rs`)
- Hand-written recursive descent parser
- Supports jq-like syntax: `.field`, `.[]`, `|`, `select()`, etc.
- Returns an AST of `Expression` enum variants

### Evaluator (`src/evaluator/mod.rs`)
- Evaluates parsed expressions against YAML/JSON/TOML data
- Uses a `Context` struct for variable scoping
- Returns `serde_yaml::Value` for unified data representation

### Operators (`src/operators/`)
- Each operator is in its own module
- Operators work with `serde_yaml::Value` internally
- Supports arithmetic, comparison, logical, and functional operations

### Output (`src/output/mod.rs`)
- Converts internal `Value` to output format
- Supports YAML, JSON, and TOML output
- Handles colorization and formatting options

## Development Commands

```bash
# Build
just build

# Run tests
just test

# Lint
just lint

# Format
just fmt

# Full validation
just check
```

## Testing

The project includes:
- Unit tests in each module
- Integration tests for end-to-end scenarios
- Property-based tests (planned)

Run tests with:
```bash
cargo test
cargo nextest run  # Faster parallel test runner
```

## Known Limitations

1. Iterator syntax (`.[]`) needs improvement for proper multi-output handling
2. Some advanced jq features not yet implemented (reduce, limit, etc.)
3. XML/CSV/TSV support not yet added
4. YAML anchor/alias preservation not implemented

## Future Enhancements

- [ ] Fix iterator multi-output handling
- [ ] Add more built-in functions (min, max, any, all, etc.)
- [ ] Add regex support (test, match, capture, etc.)
- [ ] Add string manipulation functions (split, join, etc.)
- [ ] Add path manipulation functions (getpath, setpath, delpath)
- [ ] Add try-catch error handling
- [ ] Add variable assignment and scope
- [ ] Add recursive descent operator (`..`)
- [ ] Performance optimizations

## Dependencies

- `clap` - Command line argument parsing
- `serde_yaml` - YAML serialization
- `serde_json` - JSON serialization
- `toml` - TOML serialization
- `colored` - Terminal colors
- `regex` - Pattern matching (planned)
- `anyhow` / `thiserror` - Error handling

## License

MIT OR Apache-2.0
