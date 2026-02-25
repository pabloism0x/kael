---
name: model-separation
description: Route Claude model selection by task type to optimize quota usage on Max plans.
---

# Model Separation

## Purpose

Route Claude model selection by task type to optimize quota usage on Max plans.
Opus and Sonnet have independent weekly sub-limits — separating them prevents one from starving the other.

## Policy

| Task Category | Model | Scope |
|---|---|---|
| Architecture, schema design, dependency review | `models.review` | Opus-class |
| Code review, security audit, design validation | `models.review` | Opus-class |
| Implementation, entity generation, boilerplate | `models.default` | Sonnet-class |
| Test writing, fixture generation | `models.default` | Sonnet-class |
| Refactoring (mechanical transformations) | `models.default` | Sonnet-class |
| API documentation, ADRs | `models.review` | Opus-class |

## Rules

- Never use the review model for mechanical code generation
- Never use the default model for architecture decisions that affect module boundaries
- Proto schema changes always go through the review model
- If `models.review` is not configured, all tasks use `models.default`

## Configuration

```yaml
# PRD.md
models:
  default: sonnet-4-6
  review: opus-4-6
```

## Quota Awareness

- Monitor weekly usage through Settings > Usage in claude.ai
- If review quota is low, batch architecture questions into fewer, larger prompts
- Implementation tasks on Sonnet have significantly more headroom — prefer small, focused prompts