# Phase 1.1: Fix Existing Tests and Coverage Infrastructure

## Objective
Fix all currently failing tests and set up coverage measurement infrastructure.

## Current State
- 29 tests passing
- 9 tests failing
- No coverage measurement

## Tasks

### Task 1: Fix Parser Tests
**Files**: `src/parser/expression.rs`

1. **test_parse_select** - Fails with "Unexpected character in expression"
   - Issue: Parser doesn't recognize `select()` function call syntax
   - Fix: Update function call parsing to handle parenthesized expressions

2. **test_parse_group** - Assertion mismatch
   - Issue: Group parsing returns different structure than expected
   - Fix: Adjust test expectation or parser behavior

3. **test_parse_array** - Nested array issue
   - Issue: `[1, 2, 3]` is parsed as nested array
   - Fix: Fix comma operator parsing

4. **test_parse_object** - Fails with "Expected , or } in object constructor"
   - Issue: Object key parsing doesn't handle string literals
   - Fix: Update object constructor to parse string keys

5. **test_parse_keys_function** - Returns FieldAccess instead of Keys
   - Issue: Bare identifier `keys` parsed as field access
   - Fix: Add special handling for built-in functions without parentheses

6. **test_parse_length_function** - Same as keys

### Task 2: Fix Evaluator Tests
**Files**: `src/evaluator/mod.rs`

1. **test_eval_select** - Parser error propagates
   - Depends on: Task 1.1 (fix parser first)

2. **test_eval_keys** - "Field 'keys' not found"
   - Issue: `keys` identifier not recognized as function
   - Fix: Same as parser fix

3. **test_eval_length** - "Cannot access field 'length' on non-object"
   - Issue: `length` identifier not recognized as function
   - Fix: Same as parser fix

### Task 3: Add Coverage Infrastructure
**Files**: `Cargo.toml`, `.github/workflows/`

1. Add cargo-tarpaulin to dev dependencies
2. Create coverage script
3. Set up coverage reporting

## Test Fixes Required

### Parser Expression Test Fixes

```rust
// test_parse_select - needs to parse select(.active == true)
// Currently fails because select is not recognized as function

// test_parse_keys_function - bare "keys" should be Keys function
// Currently parsed as FieldAccess { target: Identity, field: "keys" }

// test_parse_length_function - same issue as keys
```

### Solution Approach

For bare identifiers that match built-in function names, we need to:
1. Check if identifier matches a built-in function
2. If yes, create the appropriate Expression variant
3. If no, treat as FieldAccess

This should happen in `parse_identifier_or_function`.

## Acceptance Criteria
- [ ] All 38 unit tests passing
- [ ] Coverage measurement working with cargo-tarpaulin
- [ ] No regressions in existing functionality
