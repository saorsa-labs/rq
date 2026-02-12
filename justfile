# Show available recipes
default:
    @just --list

# Format code
fmt:
    cargo fmt --all

# Check formatting (CI mode)
fmt-check:
    cargo fmt --all -- --check

# Lint with clippy (zero warnings)
lint:
    cargo clippy --all-features --all-targets -- -D warnings

# Build debug
build:
    cargo build --all-features

# Build release
build-release:
    cargo build --release

# Build with warnings as errors
build-strict:
    RUSTFLAGS="-D warnings" cargo build --all-features

# Run all tests
test:
    cargo nextest run --all-features

# Run tests with output visible
test-verbose:
    cargo nextest run --all-features --no-capture

# Scan for forbidden patterns (.unwrap, .expect, panic!, etc.)
panic-scan:
    @! grep -rn '\.unwrap()\|\.expect(\|panic!(\|todo!(\|unimplemented!(' src/ --include='*.rs' | grep -v '// SAFETY:' | grep -v '#[cfg(test)]' || true

# Build documentation
doc:
    cargo doc --all-features --no-deps

# Clean build artifacts
clean:
    cargo clean

# Full validation (CI equivalent)
check: fmt-check lint build-strict test doc panic-scan

# Quick check (format + lint + test only)
quick-check: fmt-check lint test

# Run coverage measurement
coverage:
	cargo tarpaulin --all-features --out Html --out Stdout

# Run coverage and open report
coverage-open: coverage
	open tarpaulin-report.html

# Run CI pipeline locally
ci: check coverage
