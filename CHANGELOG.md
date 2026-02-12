# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.3] - 2026-02-12

### Fixed
- Show help when no expression provided instead of hanging on stdin

## [0.1.2] - 2026-02-12

### Changed
- Renamed crate to rq-cli to avoid crates.io conflict with existing 'rq' package

## [0.1.1] - 2026-02-12

### Changed
- Renamed crate to rq-cli to avoid crates.io conflict with existing 'rq' package

## [0.1.0] - 2025-02-12

### Added
- Initial implementation of rq
- Multi-format support: YAML, JSON, and TOML input/output
- jq-like expression syntax for querying and transforming data
- Built-in functions: keys, length, sort, reverse, unique, flatten, map, filter, select
- Arithmetic operators: +, -, *, /, %
- Comparison operators: ==, !=, <, <=, >, >=
- Logical operators: and, or, not
- Assignment operators: =, |=
- In-place file editing support
- Pretty print output
- Colored output support
- Exit status based on query results
