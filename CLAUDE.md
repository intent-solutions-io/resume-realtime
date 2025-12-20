## Task Tracking (Beads / bd)
- Use `bd` for ALL tasks/issues (no markdown TODO lists).
- Start of session: `bd ready`
- Create work: `bd create "Title" -p 1 --description "Context + acceptance criteria"`
- Update status: `bd update <id> --status in_progress`
- Finish: `bd close <id> --reason "Done"`
- End of session: `bd sync` (flush/import/export + git sync)
- Manual testing safety:
  - Prefer `BEADS_DIR` to isolate a workspace if needed. (`BEADS_DB` exists but is deprecated.)


# CLAUDE.md


### Beads upgrades
- After upgrading `bd`, run: `bd info --whats-new`
- If `bd info` warns about hooks, run: `bd hooks install`
This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**real-time-resume** is a real-time collaborative resume builder built with Rust, Leptos SSR + hydration, Axum, and Tailwind CSS.

## Prerequisites

```bash
cargo install cargo-leptos trunk
rustup target add wasm32-unknown-unknown
```

Project uses pinned toolchain: `nightly-2024-10-01` (see `rust-toolchain.toml`).

## Build & Development Commands

```bash
# Development with hot reload (SSR mode)
cargo leptos watch

# Production build
cargo leptos build --release

# Run tests (requires ssr feature)
cargo test --features ssr

# Run a single test
cargo test --features ssr test_name

# Format and lint
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings

# Build WASM client only
cargo build --target wasm32-unknown-unknown --features hydrate

# Build server only
cargo build --features ssr
```

Server runs at `http://127.0.0.1:3000` (reload port: 3001).

## Architecture

### Dual-Target Compilation

This project compiles to **two different targets** from the same codebase:

1. **Server binary** (`--features ssr`): Axum server with Leptos SSR, renders initial HTML
2. **WASM client** (`--features hydrate`): Hydrates server-rendered HTML, handles interactivity

The `cargo-leptos` tool orchestrates both builds automatically.

### Feature Flags

| Feature | Target | Purpose |
|---------|--------|---------|
| `ssr` | Native | Server-side rendering, Axum routes, tokio runtime |
| `hydrate` | WASM | Client-side hydration, browser APIs |

These features are **mutually exclusive** - never enable both simultaneously.

### Key Files

| File | Purpose |
|------|---------|
| `src/lib.rs` | Shared components + `hydrate()` entry for WASM |
| `src/main.rs` | Dual main: Axum server (ssr) or hydrate call (wasm) |
| `Cargo.toml [package.metadata.leptos]` | Leptos build configuration |

### Request Flow

```
Browser Request → Axum (SSR) → HTML + WASM bundle
                     ↓
                 Leptos hydrates → Interactive SPA
```

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
- `PP` = Product Planning (PRDs, roadmaps)
- `PM` = Project Management (tasks, risks)
- `TK` = Technical (architecture, API docs)
- `AT` = Architecture (decisions, patterns)
- `UC` = Use Cases (user stories, scenarios)
- `TQ` = Test/Quality (test plans, QA)
- `OD` = Operations/Delivery (releases, deployments)
- `RL` = Research/Learning (blueprints, research)
- `OP` = Operations (runbooks, procedures)

**Document Types (ABCD):**
- `STND` = Standard
- `TMPL` = Template
- `AACR` = After Action Completion Report
- `BPRT` = Blueprint
- `ARCH` = Architecture
- `RNBK` = Runbook
- `PROD` = Product Requirements Document
- `TASK` = Task List
- `ADEC` = Architecture Decision
- `RISK` = Risk Register
- `USRS` = User Stories
- `TEST` = Test Plan
- `RLSE` = Release Plan
- `RSRC` = Research/Resource

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
intent solutions io
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
intent solutions io
Contact: jeremy@intentsolutions.io
