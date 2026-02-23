---
name: {{name}}
description: {{description}}
tools: Read, Glob, Grep{{#if bash_commands}}, Bash({{bash_commands}}){{/if}}
model: {{model}}
tokenBudget: {{tokenBudget}}
autoInvoke: false
---

# {{title}} Agent

## Role

{{role_description}}

**Responsibilities:**
{{#each responsibilities}}
- {{this}}
{{/each}}

## Invocation Conditions

Invoke this agent when:
{{#each invocation_conditions}}
- {{this}}
{{/each}}

## Process

{{#each process_steps}}
### {{@index}}. {{this.title}}

{{this.description}}

{{/each}}

## Output Format

```markdown
{{output_template}}
```

## Token Saving Rules

{{#each token_rules}}
- {{this}}
{{/each}}

## Anti-patterns

{{#each anti_patterns}}
❌ {{this}}
{{/each}}