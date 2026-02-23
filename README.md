# kael

[![CI](https://github.com/pabloism0x/kael/actions/workflows/ci.yml/badge.svg)](https://github.com/pabloism0x/kael/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE-MIT)

Claude Code configuration framework CLI.
Generate a complete `.claude/` directory from a single PRD.md file — one binary, fully offline, deterministic output.

## Install

### Homebrew

```bash
brew install pabloism0x/tap/kael
```

### Cargo

```bash
cargo install kael
```

### From source

```bash
git clone https://github.com/pabloism0x/kael.git
cd kael
cargo build --release
# binary at target/release/kael
```

## Quick Start

Create a `PRD.md` with YAML frontmatter describing your project:

```yaml
---
name: "my-api"
description: "REST API for user management"
stack:
  language: python
  framework: fastapi
  database: postgresql
  infra:
    - docker
    - github-actions
type: api
constraints:
  - always use async endpoints
  - validate all input with Pydantic
---
```

Run kael:

```bash
kael init --from PRD.md
```

Generated output:

```
CLAUDE.md                              # Project instructions for Claude Code
.claude/
  settings.json                        # Token optimization, MCP, hooks
  skills/
    _common/git-workflow/SKILL.md
    _common/ci-cd/SKILL.md
    python/fastapi/SKILL.md
    python/ml-ops/SKILL.md
  agents/
    _base/architect.md
    _base/reviewer.md
    _base/docs-writer.md
    _base/test-architect.md
    python/backend-expert.md
  commands/
    init.md
    review.md
    commit.md
    test.md
```

## How It Works

1. **Parse** — reads PRD.md YAML frontmatter (`name`, `stack`, `type`, etc.)
2. **Match** — selects skills, agents, and commands based on `stack.language` + `type`
3. **Generate** — renders CLAUDE.md and settings.json from Jinja2 templates
4. **Write** — creates `.claude/` directory with all matched components

Everything is bundled in the binary. No network calls, no external dependencies at runtime.

## PRD Schema

```yaml
---
name: "project-name"                # Required
description: "What this project does"

stack:
  language: rust | typescript | python | go    # Required
  framework: nextjs | fastapi | gin | custom
  database: postgresql | mysql | mongodb | redis
  infra:
    - docker
    - github-actions
    - kubernetes

type: library | cli | web | api | mobile       # Required

features:                           # Extra context for matching
  - async-runtime
  - zero-copy-patterns

constraints:                        # Included in generated CLAUDE.md
  - no-tokio-dependency

agents:                             # Override auto-matched agents
  - _base/architect
  - rust/perf-engineer

skills:                             # Override auto-matched skills
  - _common/git-workflow
  - rust/async-patterns

mcp:                                # MCP servers for settings.json
  - github

team:
  size: 3
  experience: junior | mid | senior
---
```

## Auto-matching

When `agents` and `skills` are omitted, kael selects components automatically:

| Language | Skills | Agents |
|----------|--------|--------|
| **rust** | async-patterns, error-handling, memory-optimization | perf-engineer, runtime-expert, unsafe-auditor |
| **typescript** | react-patterns, testing, nextjs* | node-expert, fullstack-expert*, react-expert* |
| **python** | fastapi, ml-ops | backend-expert, ml-engineer, data-engineer |
| **go** | api-patterns, concurrency, testing | systems-expert, api-expert |

\* Added when `stack.framework: nextjs`

**Always included:** `_common/git-workflow`, `_common/ci-cd`, `_base/architect`, `_base/reviewer`

**By project type:**

| Type | Extra agents | Extra commands |
|------|-------------|----------------|
| cli | debugger | test, release |
| library | docs-writer | test, release |
| api | docs-writer, test-architect | test |
| web | ui-developer | test |

## CLI Reference

```bash
kael init --from PRD.md              # Generate .claude/ configuration
kael init --from PRD.md --force      # Overwrite existing files
kael init --from PRD.md --minimal    # CLAUDE.md + commands only

kael generate --from PRD.md          # Regenerate CLAUDE.md
kael generate --from PRD.md --dry-run

kael add skill rust/ffi              # Add a component
kael add agent _base/security-auditor
kael add command debug

kael remove skill rust/ffi           # Remove a component

kael list skills                     # List available components
kael list agents --installed         # List installed only
kael list all --stack rust           # Filter by stack

kael doctor                          # Check configuration health
```

## Bundled Registry

| Category | Count | Examples |
|----------|-------|---------|
| Skills | 22 | git-workflow, async-patterns, nextjs, fastapi, docker |
| Agents | 19 | architect, reviewer, perf-engineer, fullstack-expert |
| Commands | 8 | init, commit, review, test, debug, release, security |

## Design Principles

- **Offline** — no network calls; everything bundled in the binary
- **Deterministic** — same input always produces same output
- **Non-destructive** — never overwrites without `--force`
- **Minimal scope** — only reads PRD.md, only writes to `.claude/`

## Contributing

1. Fork the repo
2. Create a feature branch
3. Make changes (run `cargo fmt && cargo clippy && cargo test`)
4. Open a PR

To add a new registry component, place it in the appropriate `registry/` subdirectory. It will be bundled at compile time automatically.

## License

[MIT](LICENSE-MIT)
