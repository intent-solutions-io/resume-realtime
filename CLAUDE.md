# CLAUDE.md

This file provides guidance to Claude Code when working with this repository.

## Project Overview

**real-time-resume** is a real-time collaborative resume builder built with Rust, Leptos SSR + hydration, Axum, and Tailwind CSS.

## Documentation Standards (MANDATORY)

### Flat 000-docs Rule

- **ALL documentation lives in `000-docs/`**
- **NO SUBDIRECTORIES EVER** — the folder must remain strictly flat
- If you need to organize, use the filename prefix system, not folders

### Filename Convention

Format: `NNN-CC-ABCD-short-description.md`

| Component | Meaning |
|-----------|---------|
| `NNN` | 3-digit sequence number (001, 002, ...) |
| `CC` | Category code (2 letters) |
| `ABCD` | Document type (4 letters) |
| `short-description` | Kebab-case description |

**Category Codes (CC):**
- `DR` = Documentation Reference (standards, templates)
- `AA` = After Action (AARs, retrospectives)
- `PL` = Planning (blueprints, roadmaps)
- `TK` = Technical (architecture, API docs)
- `OP` = Operations (runbooks, procedures)

**Document Types (ABCD):**
- `STND` = Standard
- `TMPL` = Template
- `AACR` = After Action Completion Report
- `BPRT` = Blueprint
- `ARCH` = Architecture
- `RNBK` = Runbook

### New AAR Per Phase Rule

- **Every phase completion MUST produce a NEW AAR file**
- **DO NOT copy prior AARs** — write fresh each time
- **DO NOT "update" old AARs** unless explicitly instructed
- Each AAR is a point-in-time snapshot of that phase's completion

### AAR Required Sections

Every AAR must include:

1. **Metadata** (date/time CST, phase, status)
2. **Executive Summary** (1-2 sentences)
3. **What Changed** (bullet list)
4. **Why** (rationale)
5. **Evidence & Traceability** (repo URL, branch, commit SHA)
6. **Verification** (commands run, output summary)
7. **Risks/Gotchas** (known issues)
8. **Rollback** (how to undo if needed)
9. **Next Steps** (what comes next)
10. **Footer** (exactly as specified below)

### AAR Timestamp Format

All AARs must include CST timestamp:
```
YYYY-MM-DD HH:MM CST (America/Chicago)
```

### AAR Footer (EXACT)

Every AAR must end with exactly:
```
---
intent solutions io — confidential IP
Contact: jeremy@intentsolutions.io
```

## Commit Discipline

Use conventional commits:

- `chore(scaffold):` — project setup, config
- `feat(component):` — new features
- `fix(component):` — bug fixes
- `docs(standard):` — documentation standards
- `docs(template):` — templates
- `docs(aar):` — After Action Reports
- `ci(workflow):` — CI/CD changes
- `refactor(component):` — code restructuring

**AAR commits specifically:**
```
docs(aar): phase N description
```

## Development Commands

```bash
# Development
cargo leptos watch

# Production build
cargo leptos build --release

# Run tests
cargo test

# Format check
cargo fmt --check

# Lint
cargo clippy -- -D warnings
```

## Quick Reference

| Rule | Enforcement |
|------|-------------|
| Flat 000-docs | NO subdirectories, ever |
| New AAR per phase | Fresh file each time |
| CST timestamps | America/Chicago timezone |
| Footer format | Exact text required |
| No secrets | Never commit tokens/keys |

## Security

- **NEVER commit secrets, tokens, or signed URLs**
- Use environment variables for sensitive config
- Review `.gitignore` before committing

---
intent solutions io — confidential IP
Contact: jeremy@intentsolutions.io
