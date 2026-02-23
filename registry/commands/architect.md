---
description: Design system architecture or feature implementation plan
allowed-tools: Read, Glob, Grep, Bash(tree:*, ls:*, git:*)
argument-hint: <feature-or-system-description>
---

# Architecture Design

Design system architecture or create implementation plans for features.

## Arguments

- `$ARGUMENTS` — Feature description or system component to design

## Process

### 1. Understand Context

```bash
# Explore project structure
tree -L 3 -I 'node_modules|.git|dist|build|__pycache__|target'

# Identify key patterns
ls -la src/ || ls -la lib/ || ls -la internal/
```

Analyze:
- Existing architectural patterns
- Technology stack
- Module boundaries
- Data flow patterns

### 2. Load Architect Agent

Invoke `agents/_base/architect.md` with context:
- Project type (monolith, microservices, library)
- Current architecture constraints
- Team size and experience (from CLAUDE.md)

### 3. Design Components

For each component, define:

```
┌─────────────────────────────────────────┐
│ Component: [Name]                       │
├─────────────────────────────────────────┤
│ Responsibility:                         │
│   - [Single responsibility]             │
│                                         │
│ Interfaces:                             │
│   - Input: [Data/Events]                │
│   - Output: [Data/Events]               │
│                                         │
│ Dependencies:                           │
│   - [Internal modules]                  │
│   - [External services]                 │
│                                         │
│ Location: src/[path]/                   │
└─────────────────────────────────────────┘
```

### 4. Define Data Flow

```
[Source] → [Transform] → [Store] → [Present]
    │           │           │          │
    └───────────┴───────────┴──────────┘
                    │
              Error Handling
```

### 5. Output Architecture Document

Provide:
1. **Overview** — High-level system diagram (ASCII)
2. **Components** — Detailed component breakdown
3. **Interfaces** — API contracts and data types
4. **Dependencies** — External service requirements
5. **Implementation Plan** — Ordered task list

## Templates

### Feature Architecture

```markdown
## Feature: [Name]

### Overview
[1-2 sentence description]

### Components
1. [Component A] - [responsibility]
2. [Component B] - [responsibility]

### Data Flow
[Source] → [Process] → [Output]

### Files to Create/Modify
- `src/feature/index.ts` — Entry point
- `src/feature/types.ts` — Type definitions
- `src/feature/service.ts` — Business logic

### Dependencies
- [Existing module] for [purpose]
- [New package] for [purpose]
```

### System Architecture

```markdown
## System: [Name]

### Architecture Style
[Monolith | Microservices | Event-driven | Layered]

### Layer Breakdown
┌─────────────────────────────┐
│      Presentation Layer     │
├─────────────────────────────┤
│      Application Layer      │
├─────────────────────────────┤
│       Domain Layer          │
├─────────────────────────────┤
│    Infrastructure Layer     │
└─────────────────────────────┘

### Key Decisions
- [Decision 1] — [Rationale]
- [Decision 2] — [Rationale]
```

## Examples

```bash
# Design a new feature
/project:architect user authentication with OAuth2

# Design system component
/project:architect caching layer for API responses

# Design integration
/project:architect webhook processing system
```
