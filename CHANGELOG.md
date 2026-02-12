# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
