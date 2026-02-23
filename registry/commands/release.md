---
description: Prepare and create a release
allowed-tools: Read, Write, Bash(git:*, cargo:*, npm:*)
argument-hint: <version>
---

# Release

Prepare and create a new release.

## Arguments

- `$ARGUMENTS` — Version number (e.g., `1.0.0`, `patch`, `minor`, `major`)

## Process

### 1. Validate State

```bash
# Ensure clean working directory
git status --porcelain

# Ensure on main/master branch
git branch --show-current

# Ensure up to date
git fetch && git status -uno
```

### 2. Determine Version

If semantic keyword provided:
- `patch` → increment patch (1.0.0 → 1.0.1)
- `minor` → increment minor (1.0.0 → 1.1.0)
- `major` → increment major (1.0.0 → 2.0.0)

### 3. Update Version Files

| File | Action |
|------|--------|
| `Cargo.toml` | Update `version = "X.Y.Z"` |
| `package.json` | Update `"version": "X.Y.Z"` |
| `pyproject.toml` | Update `version = "X.Y.Z"` |
| `CHANGELOG.md` | Move Unreleased to new version |

### 4. Update CHANGELOG

```markdown
## [X.Y.Z] - YYYY-MM-DD

[Move items from Unreleased]

## [Unreleased]
```

### 5. Create Release Commit

```bash
git add -A
git commit -m "🔖 release: vX.Y.Z"
git tag -a vX.Y.Z -m "Release vX.Y.Z"
```

### 6. Output

```markdown
## Release Ready

**Version:** X.Y.Z
**Tag:** vX.Y.Z

### Changes
[Summary from CHANGELOG]

### Next Steps
1. `git push origin main --tags`
2. Create GitHub release
3. Publish to package registry
```

## Examples

```bash
# Specific version
/project:release 1.0.0

# Semantic bump
/project:release patch
/project:release minor
/project:release major
```

## Safety

- Confirm before making changes
- Show diff before committing
- Don't push automatically (user decision)