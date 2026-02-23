---
description: Security audit and vulnerability assessment
allowed-tools: Read, Glob, Grep, Bash(grep:*, find:*, git:*, npm audit:*, cargo audit:*, pip-audit:*, go list:*)
argument-hint: [scope-or-specific-concern]
---

# Security Audit

Perform security audit, identify vulnerabilities, and suggest remediations.

## Arguments

- `$ARGUMENTS` — Scope (full, auth, api, deps) or specific security concern

## Process

### 1. Determine Audit Scope

| Scope | Coverage |
|-------|----------|
| `full` | Complete security review |
| `auth` | Authentication & authorization |
| `api` | API security (input validation, injection) |
| `deps` | Dependency vulnerabilities |
| `secrets` | Hardcoded secrets detection |
| `config` | Security configuration review |

### 2. Load Security Auditor Agent

Invoke `agents/_base/security-auditor.md` with:
- Audit scope
- Project technology stack
- Known sensitive areas

### 3. Run Automated Checks

#### Dependency Audit
```bash
# Node.js
npm audit --json || yarn audit --json

# Python
pip-audit || safety check

# Rust
cargo audit

# Go
go list -m -json all | nancy sleuth
```

#### Secret Detection
```bash
# Search for potential secrets
grep -rn "password\s*=\|api_key\|secret\|token\s*=" --include="*.{js,ts,py,go,rs}" .
grep -rn "BEGIN.*PRIVATE KEY" .

# Check for .env files in git
git ls-files | grep -E "\.env$|\.env\."
```

### 4. Manual Review Checklist

#### OWASP Top 10 Review

| # | Vulnerability | Check | Status |
|---|---------------|-------|--------|
| 1 | Injection | SQL/NoSQL/Command injection points | |
| 2 | Broken Auth | Session management, password policies | |
| 3 | Sensitive Data | Encryption, data exposure | |
| 4 | XXE | XML parser configuration | |
| 5 | Broken Access Control | Authorization checks | |
| 6 | Security Misconfig | Default settings, headers | |
| 7 | XSS | Output encoding, CSP | |
| 8 | Insecure Deserialization | Untrusted data handling | |
| 9 | Known Vulnerabilities | Dependency versions | |
| 10 | Insufficient Logging | Audit trails, monitoring | |

#### Code Patterns to Flag

```javascript
// SQL Injection risk
query(`SELECT * FROM users WHERE id = ${userId}`)  // BAD
query('SELECT * FROM users WHERE id = ?', [userId])  // GOOD

// XSS risk
innerHTML = userInput  // BAD
textContent = userInput  // GOOD

// Command Injection risk
exec(`ls ${userPath}`)  // BAD
execFile('ls', [userPath])  // GOOD
```

### 5. Classification

Classify findings by severity:

| Severity | Description | Response Time |
|----------|-------------|---------------|
| 🔴 Critical | Actively exploitable, data breach risk | Immediate |
| 🟠 High | Significant risk, requires action | 24-48 hours |
| 🟡 Medium | Moderate risk, should address | 1-2 weeks |
| 🟢 Low | Minor risk, best practice | Next sprint |
| ℹ️ Info | Recommendations, hardening | Backlog |

### 6. Generate Report

```markdown
## Security Audit Report

**Date:** [Date]
**Scope:** [Audit scope]
**Auditor:** Gromit Security Auditor

### Executive Summary
- Critical: [count]
- High: [count]
- Medium: [count]
- Low: [count]

### Findings

#### [SEV-001] [Severity] - [Title]
**Location:** `src/path/file.ts:123`
**Description:** [What was found]
**Impact:** [Potential consequences]
**Remediation:** [How to fix]
**References:** [CWE/CVE links]

### Dependency Vulnerabilities
[List from automated tools]

### Recommendations
1. [Priority action 1]
2. [Priority action 2]
```

## Examples

```bash
# Full security audit
/project:security full

# Audit authentication only
/project:security auth

# Check dependencies
/project:security deps

# Investigate specific concern
/project:security "check for SQL injection in user queries"
```

## Output

Provide:
1. **Summary** — Finding counts by severity
2. **Critical/High Findings** — Detailed with remediation
3. **Dependency Report** — Vulnerable packages
4. **Action Items** — Prioritized fix list
5. **References** — OWASP, CWE links
