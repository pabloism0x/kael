# {{ name }}

{{ description }}

## Quick Facts

- **Stack**: {{ stack.language }}{% if stack.framework %} / {{ stack.framework }}{% endif %}
- **Type**: {{ type }}
{% if stack.database %}- **Database**: {{ stack.database }}{% endif %}

## Commands

{% if stack.language == "rust" -%}
- `cargo build` — Build the project
- `cargo test` — Run tests
- `cargo clippy` — Run linter
- `cargo fmt` — Format code
{% elif stack.language == "typescript" -%}
- `pnpm run dev` — Start development
- `pnpm test` — Run tests
- `pnpm run lint` — Run linter
- `pnpm run build` — Production build
{% elif stack.language == "python" -%}
- `pytest` — Run tests
- `ruff check .` — Run linter
- `ruff format .` — Format code
{% elif stack.language == "go" -%}
- `go build ./...` — Build
- `go test ./...` — Run tests
- `go vet ./...` — Run linter
{% endif %}

## Architecture

{% if features -%}
### Key Features
{% for feature in features -%}
- {{ feature }}
{% endfor %}
{% endif -%}

## Conventions

{% if stack.language == "rust" -%}
- Use `Result<T, E>` for fallible operations
- Prefer `&str` over `String` for function parameters
- Document public APIs with `///` doc comments
{% elif stack.language == "typescript" -%}
- Use TypeScript strict mode
- Prefer named exports
- Use `interface` for object types
{% elif stack.language == "python" -%}
- Use type hints for all functions
- Follow PEP 8 style guide
- Use `pathlib` for file paths
{% elif stack.language == "go" -%}
- Follow effective Go guidelines
- Use `context.Context` for cancellation
- Return errors, don't panic
{% endif %}

{% if constraints -%}
## Critical

{% for constraint in constraints -%}
- {{ constraint }}
{% endfor %}
{% endif -%}

## References

{% if agents -%}
### Agents
{% for agent in agents -%}
- @.claude/agents/{{ agent }}.md
{% endfor %}
{% endif -%}

{% if skills -%}
### Skills
{% for skill in skills -%}
- @.claude/skills/{{ skill }}/SKILL.md
{% endfor %}
{% endif -%}
