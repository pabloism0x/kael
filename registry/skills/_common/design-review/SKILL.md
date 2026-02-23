---
name: design-review
description: Design system validation, UI/UX review, and Figma-to-code consistency. Use when reviewing implemented components against design specs.
---

# Design Review Patterns

## Quick Reference

| Review Type | Focus | Tools |
|-------------|-------|-------|
| Visual Audit | Pixel accuracy | Figma overlay, screenshot comparison |
| Token Audit | Design tokens | CSS inspection, token validation |
| Accessibility | WCAG compliance | axe, Lighthouse, manual testing |
| Responsive | Breakpoints | DevTools, real devices |
| Interaction | States, animations | User flow testing |

## Visual Accuracy Checklist

### Spacing & Layout

- [ ] Margins match design spec
- [ ] Padding is consistent
- [ ] Grid alignment verified
- [ ] White space matches design
- [ ] Component spacing follows design system

```css
/* Token validation example */
/* Bad: Hardcoded values */
padding: 16px;
margin: 24px;

/* Good: Using design tokens */
padding: var(--spacing-4);
margin: var(--spacing-6);
```

### Typography

- [ ] Font family matches spec
- [ ] Font sizes follow scale
- [ ] Line heights are correct
- [ ] Letter spacing applied
- [ ] Font weights match

```css
/* Typography token audit */
/* Bad */
font-size: 14px;
line-height: 1.5;
font-weight: 500;

/* Good */
font-size: var(--text-sm);
line-height: var(--leading-normal);
font-weight: var(--font-medium);
```

### Colors

- [ ] Primary colors match
- [ ] Secondary colors match
- [ ] Text colors correct
- [ ] Background colors correct
- [ ] Border colors correct
- [ ] Shadow colors correct

```css
/* Color token audit */
/* Bad */
color: #6366f1;
background: #f3f4f6;

/* Good */
color: var(--color-primary-500);
background: var(--color-gray-100);
```

## Design Token Validation

### Token Categories

```css
:root {
  /* Colors */
  --color-primary-50: #eef2ff;
  --color-primary-500: #6366f1;
  --color-primary-900: #312e81;

  /* Typography */
  --font-sans: 'Inter', system-ui, sans-serif;
  --text-xs: 0.75rem;    /* 12px */
  --text-sm: 0.875rem;   /* 14px */
  --text-base: 1rem;     /* 16px */
  --text-lg: 1.125rem;   /* 18px */
  --text-xl: 1.25rem;    /* 20px */

  /* Spacing */
  --spacing-1: 0.25rem;  /* 4px */
  --spacing-2: 0.5rem;   /* 8px */
  --spacing-4: 1rem;     /* 16px */
  --spacing-6: 1.5rem;   /* 24px */
  --spacing-8: 2rem;     /* 32px */

  /* Border Radius */
  --radius-sm: 0.25rem;
  --radius-md: 0.375rem;
  --radius-lg: 0.5rem;
  --radius-full: 9999px;

  /* Shadows */
  --shadow-sm: 0 1px 2px rgba(0, 0, 0, 0.05);
  --shadow-md: 0 4px 6px rgba(0, 0, 0, 0.1);
  --shadow-lg: 0 10px 15px rgba(0, 0, 0, 0.1);
}
```

### Token Mismatch Report

```markdown
## Token Audit Report

### ❌ Mismatches Found

| Location | Expected | Actual | Severity |
|----------|----------|--------|----------|
| Button padding | var(--spacing-4) | 15px | Medium |
| Card shadow | var(--shadow-md) | custom shadow | Low |
| Header color | var(--color-primary-500) | #6366f0 | High |

### ✅ Correctly Applied

- Typography scale
- Border radius
- Grid spacing
```

## Component States Review

### Interactive States

```
┌─────────────────────────────────────────────┐
│ State         │ Visual Change               │
├─────────────────────────────────────────────┤
│ Default       │ Base appearance             │
│ Hover         │ Slight color change         │
│ Focus         │ Focus ring visible          │
│ Active        │ Pressed/darker state        │
│ Disabled      │ Reduced opacity, no cursor  │
│ Loading       │ Spinner, disabled actions   │
│ Error         │ Red border, error message   │
│ Success       │ Green indicator             │
└─────────────────────────────────────────────┘
```

### State Checklist

- [ ] Hover state implemented
- [ ] Focus state visible (accessibility)
- [ ] Active/pressed state
- [ ] Disabled state (visual + functional)
- [ ] Loading state (if applicable)
- [ ] Error state (form inputs)
- [ ] Empty state (lists, tables)
- [ ] Skeleton loading state

## Accessibility Review

### WCAG Checklist

#### Perceivable

- [ ] Color contrast ratio ≥ 4.5:1 (normal text)
- [ ] Color contrast ratio ≥ 3:1 (large text)
- [ ] Information not conveyed by color alone
- [ ] Images have alt text
- [ ] Videos have captions

#### Operable

- [ ] Keyboard navigation works
- [ ] Focus order is logical
- [ ] Focus indicator visible
- [ ] No keyboard traps
- [ ] Touch targets ≥ 44x44px

#### Understandable

- [ ] Labels are clear
- [ ] Error messages are helpful
- [ ] Instructions are provided
- [ ] Language is specified

#### Robust

- [ ] Valid HTML
- [ ] ARIA attributes correct
- [ ] Works with screen readers

### Accessibility Testing Commands

```bash
# Run Lighthouse accessibility audit
npx lighthouse https://example.com --only-categories=accessibility

# Run axe-core
npx @axe-core/cli https://example.com

# Check color contrast
npx wcag-contrast-check "#6366f1" "#ffffff"
```

## Responsive Design Review

### Breakpoint Checklist

| Breakpoint | Width | Check |
|------------|-------|-------|
| Mobile | 320px - 480px | [ ] |
| Mobile Large | 481px - 768px | [ ] |
| Tablet | 769px - 1024px | [ ] |
| Desktop | 1025px - 1440px | [ ] |
| Large Desktop | 1441px+ | [ ] |

### Responsive Issues to Check

- [ ] Text doesn't overflow
- [ ] Images scale properly
- [ ] Touch targets adequate on mobile
- [ ] Navigation works on all sizes
- [ ] Tables scroll horizontally if needed
- [ ] Modals fit on mobile
- [ ] Forms are usable on mobile

## Figma-to-Code Comparison

### Comparison Workflow

```
1. Open Figma design in Dev Mode
2. Open implemented component in browser
3. Use overlay tool to compare
4. Document discrepancies
5. Create fix tickets
```

### Comparison Tools

```bash
# Screenshot comparison
npx backstopjs test

# Visual regression testing
npx percy snapshot ./test/snapshots

# Manual: Figma overlay browser extension
```

### Discrepancy Report Template

```markdown
## Component: Button

### Visual Comparison

| Aspect | Figma Spec | Implementation | Status |
|--------|------------|----------------|--------|
| Height | 40px | 40px | ✅ |
| Padding | 16px 24px | 14px 22px | ❌ |
| Border Radius | 8px | 8px | ✅ |
| Font Size | 14px | 14px | ✅ |
| Font Weight | 500 | 400 | ❌ |

### Screenshots

[Figma] vs [Implementation]

### Priority: Medium

### Suggested Fix
Update padding to `var(--spacing-4) var(--spacing-6)`
Update font-weight to `var(--font-medium)`
```

## Common Issues

### Token Mismatch

```css
/* Issue: Hardcoded value instead of token */
❌ color: #6366f1;
✅ color: var(--color-primary-500);

❌ padding: 16px;
✅ padding: var(--spacing-4);

❌ border-radius: 8px;
✅ border-radius: var(--radius-lg);
```

### Missing Variants

```tsx
// Issue: Not all Figma variants implemented
❌ <Button>Click me</Button>

✅ <Button variant="primary">Click me</Button>
✅ <Button variant="secondary">Click me</Button>
✅ <Button variant="ghost">Click me</Button>
✅ <Button size="sm">Click me</Button>
✅ <Button size="lg">Click me</Button>
✅ <Button disabled>Click me</Button>
✅ <Button loading>Click me</Button>
```

### Inconsistent Spacing

```css
/* Issue: Mixed spacing values */
❌
.card { padding: 20px; }
.modal { padding: 24px; }
.dialog { padding: 18px; }

✅
.card { padding: var(--spacing-5); }
.modal { padding: var(--spacing-6); }
.dialog { padding: var(--spacing-5); }
```

## Review Output Format

### Summary Report

```markdown
# Design Review: [Component/Page Name]

## Overview
- **Reviewer**: [Name]
- **Date**: [Date]
- **Figma Link**: [URL]
- **Implementation**: [URL/PR]

## Score: 85/100

### ✅ Passed (12)
- Typography scale
- Color tokens
- Grid alignment
- Responsive mobile
- Keyboard navigation
...

### ❌ Issues (3)

#### 🔴 Critical
1. Focus states missing on interactive elements

#### 🟡 Warning
1. Button padding 2px off spec
2. Card shadow slightly different

#### 🟢 Minor
1. Hover transition slightly faster than spec

## Action Items
- [ ] Add focus states - Priority: High
- [ ] Fix button padding - Priority: Medium
- [ ] Adjust shadow - Priority: Low

## Notes
Component is mostly aligned with design. Focus states are the main blocker for accessibility compliance.
```

## Automation

### CI Integration

```yaml
# .github/workflows/design-review.yml
name: Design Review

on:
  pull_request:
    paths:
      - 'src/components/**'

jobs:
  visual-test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Visual regression test
        run: npx backstopjs test

      - name: Upload report
        uses: actions/upload-artifact@v4
        with:
          name: backstop-report
          path: backstop_data/html_report
```

### Pre-commit Hooks

```json
{
  "hooks": {
    "pre-commit": [
      "npx stylelint --fix",
      "npx eslint --fix",
      "node scripts/check-design-tokens.js"
    ]
  }
}
```
