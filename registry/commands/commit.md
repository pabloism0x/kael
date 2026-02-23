---
description: Create a well-structured git commit with conventional commit format
allowed-tools: Read, Bash(git:*)
argument-hint: [commit-type]
---

# Git Commit

Create a git commit with conventional commit format and meaningful message.

## Arguments

- `$ARGUMENTS` — Optional commit type override (feat, fix, refactor, docs, test, chore)

## Process

### 1. Analyze Changes

```bash
# Check git status
git status --porcelain

# View staged changes
git diff --cached --stat

# If nothing staged, show unstaged changes
git diff --stat
```

### 2. Determine Commit Type

Based on changed files and content, select appropriate type:

| Type | Description | Examples |
|------|-------------|----------|
| `feat` | New feature | New function, endpoint, component |
| `fix` | Bug fix | Error correction, crash fix |
| `refactor` | Code restructure | No behavior change |
| `docs` | Documentation | README, comments, JSDoc |
| `test` | Tests | Add/modify tests |
| `chore` | Maintenance | Dependencies, config |
| `perf` | Performance | Optimization |
| `style` | Formatting | Whitespace, linting |

### 3. Identify Scope

Determine the module or area affected:
- Component name (e.g., `auth`, `api`, `ui`)
- File type (e.g., `config`, `types`)
- Feature area (e.g., `login`, `checkout`)

### 4. Write Commit Message

Format: `type(scope): description`

Rules:
- Use imperative mood ("add" not "added")
- Don't capitalize first letter
- No period at end
- Max 72 characters for subject
- Add body for complex changes

### 5. Execute Commit

```bash
# Stage all changes if needed
git add -A

# Commit with message
git commit -m "type(scope): description"
```

## Examples

```bash
# Auto-detect and commit
/project:commit

# Force commit type
/project:commit feat

# Will produce commits like:
# feat(auth): add OAuth2 support for GitHub
# fix(api): handle null response in user endpoint
# refactor(utils): extract date formatting logic
# docs(readme): update installation instructions
```

## Output

After commit, display:
- Commit hash (short)
- Commit message
- Files changed count
- Insertions/deletions summary
