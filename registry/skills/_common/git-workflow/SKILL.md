---
name: git-workflow
description: Git branching strategies, commit conventions, and workflow patterns. Use when working with version control.
---

# Git Workflow

## Quick Reference

| Task | Command |
|------|---------|
| New feature branch | `git checkout -b feat/description` |
| New fix branch | `git checkout -b fix/description` |
| Stage all | `git add -A` |
| Commit | `git commit -m "type: description"` |
| Push branch | `git push -u origin branch-name` |
| Rebase on main | `git rebase main` |
| Squash commits | `git rebase -i HEAD~n` |

## Branch Naming

```
feat/   - New features
fix/    - Bug fixes
docs/   - Documentation only
refactor/ - Code refactoring
test/   - Adding tests
chore/  - Maintenance tasks
```

**Examples:**
- `feat/user-authentication`
- `fix/memory-leak-in-reactor`
- `docs/api-reference`

## Commit Convention

### Format

```
<emoji> <type>: <description>

[optional body]

[optional footer]
```

### Types with Emoji

| Emoji | Type | Description |
|-------|------|-------------|
| 🎉 | init | Initial commit |
| ✨ | feat | New feature |
| 🐛 | fix | Bug fix |
| 📝 | docs | Documentation |
| ♻️ | refactor | Code refactoring |
| 🧪 | test | Adding tests |
| 🔧 | chore | Maintenance |
| 🚀 | perf | Performance |
| 🔖 | release | Version release |
| ⬆️ | deps | Dependency update |

### Examples

```
✨ feat: add user authentication endpoint
🐛 fix: resolve memory leak in connection pool
📝 docs: update API documentation for v2
♻️ refactor: simplify error handling logic
🔖 release: v1.0.0
```

## Workflow Patterns

### Feature Development

```bash
# 1. Start from main
git checkout main
git pull origin main

# 2. Create feature branch
git checkout -b feat/my-feature

# 3. Work and commit
git add -A
git commit -m "✨ feat: add new feature"

# 4. Keep up to date
git fetch origin
git rebase origin/main

# 5. Push and create PR
git push -u origin feat/my-feature
```

### Hotfix

```bash
# 1. Branch from main
git checkout main
git pull origin main
git checkout -b fix/critical-bug

# 2. Fix and commit
git add -A
git commit -m "🐛 fix: resolve critical bug"

# 3. Push immediately
git push -u origin fix/critical-bug
# Create PR with priority label
```

### Release

```bash
# 1. Ensure main is ready
git checkout main
git pull origin main

# 2. Tag release
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

## Anti-patterns

❌ Committing directly to main
❌ Vague commit messages ("fix stuff", "update")
❌ Giant commits with multiple unrelated changes
❌ Force pushing to shared branches
❌ Not pulling before pushing