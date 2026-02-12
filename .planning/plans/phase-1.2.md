# Phase 1.2: Comprehensive Unit Tests

## Objective
Add comprehensive unit tests for all operators, parsers, and edge cases to achieve >80% code coverage.

## Current Coverage
Need to measure baseline coverage first.

## Tasks

### Task 1: Add Coverage Measurement
- Install cargo-tarpaulin (or use cargo-llvm-cov)
- Create coverage script in justfile
- Set up coverage reporting

### Task 2: Test All Operators
**Files to test**: `src/operators/*.rs`

For each operator, test:
- Normal case
- Edge cases (empty input, null, wrong types)
- Error handling

Operators needing tests:
- [ ] field_access - all edge cases
- [ ] index_access - negative indices, out of bounds
- [ ] iterator - empty arrays, objects
- [ ] arithmetic - overflow, division by zero
- [ ] comparison - different types
- [ ] logical - short-circuit behavior
- [ ] select - various conditions
- [ ] keys - different input types
- [ ] length - strings, arrays, objects
- [ ] sort - already sorted, reverse sorted
- [ ] reverse - arrays, strings
- [ ] unique - duplicates, empty
- [ ] flatten - nested arrays
- [ ] group_by - various key types
- [ ] map - empty arrays
- [ ] filter - all filtered, none filtered
- [ ] slice - negative indices, out of bounds
- [ ] alternative - null/false handling
- [ ] first/last - empty arrays
- [ ] add - sum, string concat
- [ ] env - missing vars
- [ ] tostring/tonumber - various types

### Task 3: Test Input/Output Formats
**Files**: `src/parser/input.rs`, `src/output/mod.rs`

Test all format conversions:
- [ ] YAML -> JSON
- [ ] YAML -> TOML
- [ ] JSON -> YAML
- [ ] JSON -> TOML
- [ ] TOML -> YAML
- [ ] TOML -> JSON
- [ ] Edge cases: empty documents, null values
- [ ] Error handling: malformed input

### Task 4: Test Expression Parser
**File**: `src/parser/expression.rs`

Add tests for:
- [ ] All expression types
- [ ] Operator precedence
- [ ] Nested expressions
- [ ] Error cases: unterminated strings, invalid syntax
- [ ] Complex real-world expressions

### Task 5: Test Evaluator
**File**: `src/evaluator/mod.rs`

Add tests for:
- [ ] Context handling
- [ ] Variable scoping
- [ ] Error propagation
- [ ] Complex nested evaluations

## Acceptance Criteria
- [ ] >80% line coverage measured by cargo-tarpaulin
- [ ] All operators have comprehensive tests
- [ ] All format conversions tested
- [ ] Edge cases covered for all major functions
- [ ] No test failures
