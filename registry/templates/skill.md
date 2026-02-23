---
name: {{name}}
description: {{description}}
---

# {{title}}

## Quick Reference

| Task | Command |
|------|---------|
{{#each quick_reference}}
| {{this.task}} | `{{this.command}}` |
{{/each}}

## Patterns

{{#each patterns}}
### {{this.name}}

{{this.description}}

```{{../language}}
{{this.example}}
```

{{/each}}

## Anti-patterns

{{#each anti_patterns}}
### ❌ {{this.name}}

{{this.description}}

```{{../language}}
// Bad
{{this.bad_example}}

// Good
{{this.good_example}}
```

{{/each}}

## Examples

{{#each examples}}
### {{this.title}}

{{this.description}}

```{{../language}}
{{this.code}}
```

{{/each}}

## References

{{#each references}}
- [{{this.title}}]({{this.url}})
{{/each}}