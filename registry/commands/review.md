---
description: Run code review on recent changes or specified files
allowed-tools: Read, Glob, Grep, Bash(git:*)
argument-hint: [file-or-commit-range]
---

# Code Review

Review code changes for quality, security, and best practices.

## Arguments

- `$ARGUMENTS` — File path, commit range, or empty for `HEAD~1..HEAD`

## Process

### 1. Determine Scope

```bash
# If no arguments, review last commit
git diff HEAD~1..HEAD

# If commit range
git diff $ARGUMENTS

# If file path
git diff HEAD -- $ARGUMENTS
```

### 2. Invoke Reviewer Agent

Load `agents/_base/reviewer.md` and apply review checklist:

- 🔴 Critical issues (security, data loss, crashes)
- 🟡 Warnings (missing error handling, performance)
- 🟢 Suggestions (naming, simplification)
- ✅ Good practices (acknowledge quality code)

### 3. Output Summary

Provide structured feedback with:
- Files reviewed count
- Issues by severity
- Verdict: Approve / Request Changes / Block

## Examples

```bash
# Review last commit
/project:review

# Review specific file
/project:review src/lib.rs

# Review commit range
/project:review abc123..def456

# Review staged changes
/project:review --staged
```