---
description: Systematic debugging of errors, failures, or unexpected behavior
allowed-tools: Read, Glob, Grep, Bash(*, !rm, !git push)
argument-hint: <error-message-or-symptom>
---

# Debug

Systematically investigate and resolve bugs, errors, or unexpected behavior.

## Arguments

- `$ARGUMENTS` — Error message, stack trace, or description of unexpected behavior

## Process

### 1. Gather Information

```bash
# Check recent changes
git log --oneline -10
git diff HEAD~3..HEAD --stat

# Find related error logs
grep -r "error\|Error\|ERROR" logs/ | tail -20
```

Questions to answer:
- When did it start happening?
- Is it reproducible?
- What changed recently?

### 2. Load Debugger Agent

Invoke `agents/_base/debugger.md` with:
- Error message or symptom
- Relevant file paths
- Recent changes context

### 3. Reproduce the Issue

```bash
# Run tests to confirm
npm test -- --grep "related-test"
# or
pytest -k "related_test" -v
# or
go test -v -run "RelatedTest"
```

### 4. Apply Debugging Strategy

#### Error Message Analysis
```
Error: [Error Type]
    at [function] ([file]:[line])
    at [caller] ([file]:[line])
```

Extract:
- Error type/class
- Location (file:line)
- Call stack
- Input values

#### Binary Search (for regressions)
```bash
git bisect start
git bisect bad HEAD
git bisect good <last-known-good-commit>
# Test and mark each commit
```

#### Isolation Strategy
1. Identify smallest reproducible case
2. Remove unrelated code paths
3. Add logging at boundaries
4. Test each component in isolation

### 5. Hypothesis Testing

For each potential cause:

| # | Hypothesis | Test | Result | Action |
|---|------------|------|--------|--------|
| 1 | [Theory A] | [How to verify] | Pass/Fail | [Next step] |
| 2 | [Theory B] | [How to verify] | Pass/Fail | [Next step] |

### 6. Implement Fix

Once root cause identified:
1. Write failing test that reproduces the bug
2. Implement minimal fix
3. Verify test passes
4. Check for similar issues elsewhere

### 7. Document Findings

```markdown
## Bug Report

### Symptom
[What was observed]

### Root Cause
[Why it happened]

### Fix
[What was changed]

### Prevention
[How to avoid in future]
```

## Common Patterns

### Null/Undefined Errors
- Check data source
- Verify API responses
- Add defensive checks

### Type Errors
- Check type definitions
- Verify serialization/deserialization
- Check generic constraints

### Race Conditions
- Add logging with timestamps
- Check async/await usage
- Review lock/mutex usage

### Memory Issues
- Profile memory usage
- Check for leaks (unclosed resources)
- Review caching strategy

## Examples

```bash
# Debug specific error
/project:debug "TypeError: Cannot read property 'id' of undefined"

# Debug test failure
/project:debug "test user.create fails intermittently"

# Debug performance issue
/project:debug "API endpoint /users takes 5s to respond"

# Debug with stack trace
/project:debug "Error at src/services/auth.ts:142"
```

## Output

Provide:
1. **Root Cause** — Clear explanation
2. **Evidence** — How it was identified
3. **Fix** — Code changes (if made)
4. **Verification** — How to confirm fix
5. **Prevention** — Future safeguards
