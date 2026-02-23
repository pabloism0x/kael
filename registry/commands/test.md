---
description: Run tests and analyze results
allowed-tools: Read, Bash
argument-hint: [test-pattern]
---

# Run Tests

Execute tests and provide analysis of results.

## Arguments

- `$ARGUMENTS` — Test pattern or path (optional)

## Process

### 1. Detect Test Framework

Check project files to determine test command:

| File | Framework | Command |
|------|-----------|---------|
| `Cargo.toml` | Rust | `cargo test` |
| `package.json` | Node | `npm test` |
| `pyproject.toml` | Python | `pytest` |
| `go.mod` | Go | `go test ./...` |

### 2. Run Tests

```bash
# Run all tests
[detected-command]

# Run specific tests
[detected-command] $ARGUMENTS
```

### 3. Analyze Results

If tests fail:
- Identify failing tests
- Show relevant error messages
- Suggest potential fixes

If tests pass:
- Show summary
- Report coverage if available

## Examples

```bash
# Run all tests
/project:test

# Run specific test file
/project:test auth

# Run with pattern
/project:test "user*"
```

## Output

```markdown
## Test Results

**Status:** ✅ Passed | ❌ Failed
**Total:** X tests
**Passed:** Y
**Failed:** Z
**Duration:** N seconds

### Failed Tests
[If any, show details and suggestions]

### Coverage
[If available]
```