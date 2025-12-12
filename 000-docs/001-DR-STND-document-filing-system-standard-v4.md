# DOCUMENT FILING SYSTEM STANDARD v4.2 (LLM/AI-ASSISTANT FRIENDLY)
**Purpose:** Universal, deterministic naming + filing standard for project docs with canonical cross-repo "6767" standards series
**Status:** Production Standard (v3-compatible, v4.0-compatible)
**Last Updated:** 2025-12-07
**Changelog:** v4.2 enforces strict flat 000-docs (no subdirectories)
**Applies To:** All projects in intent-solutions-io organization

---

## 0) ONE-SCREEN RULES (AI SHOULD MEMORIZE THESE)
1) **Two filename families only:**
   - **Project docs:** `NNN-CC-ABCD-short-description.ext`
   - **Canonical standards:** `6767-[TOPIC-]CC-ABCD-short-description.ext`
2) **NNN is chronological** (001–999). **6767 never uses extra numeric IDs in filenames.**
3) **All codes are mandatory:** `CC` (category) + `ABCD` (type).
4) **Description is short:** 1–4 words (project), 1–5 words (6767), **kebab-case**, lowercase.
5) **Subdocs:** either `005a` letter suffix or `006-1` numeric suffix.
6) **6767 doc IDs like "6767-120" may appear in headers/content for cross-ref, but NOT in the filename.**

---

## 1) FILENAME SPEC (DETERMINISTIC)
### 1.1 Project Docs (NNN series)
**Pattern**

NNN-CC-ABCD-short-description.ext

**Fields**
- `NNN`: 001–999 (zero padded, chronological)
- `CC`: 2-letter category code (table below)
- `ABCD`: 4-letter doc type abbreviation (tables below)
- `short-description`: 1–4 words, kebab-case
- `ext`: `.md` preferred; others allowed (`.pdf`, `.txt`, `.xlsx`, etc.)

**Examples**

001-AT-ADEC-initial-architecture.md
005-PM-TASK-api-endpoints.md
009-AA-AACR-sprint-1-review.md

### 1.2 Sub-Docs (same parent number)
**Option A — letter suffix**

005-PM-TASK-api-endpoints.md
005a-PM-TASK-auth-endpoints.md
005b-PM-TASK-payment-endpoints.md

**Option B — numeric suffix**

006-PM-RISK-security-audit.md
006-1-PM-RISK-encryption-review.md
006-2-PM-RISK-access-controls.md

### 1.3 Canonical Standards (6767 series)
**Purpose:** Cross-repo reusable SOPs, standards, patterns, architectures.

**Pattern**

6767-{a|b|c|...}-[TOPIC-]CC-ABCD-short-description.ext

**Fields**
- `6767`: fixed canonical prefix (used ONCE)
- `{a|b|c|...}`: **mandatory letter suffix** for chronological ordering (a, b, c, d, etc.)
- `[TOPIC-]`: optional uppercase grouping prefix (examples: `INLINE`, `LAZY`, `SLKDEV`)
- `CC`: same category codes as NNN series
- `ABCD`: 4-letter type code (same master tables)
- `short-description`: 1–5 words, kebab-case

---

## 2) FAST DECISION: WHICH SERIES DO I USE?

| If the doc is… | Use… |
|---|---|
| reusable standard/process/pattern across multiple repos | **6767** |
| specific to one repo/app/phase/sprint/implementation | **NNN** |

---

## 3) 000-docs Flatness Rule (Strict)

**Purpose:** Keep all documentation in a single flat directory for simplicity and discoverability.

**Rules:**
- `000-docs/` contains all docs (NNN and 6767) at one level.
- **No subdirectories allowed under `000-docs/`.**
- If assets are needed, store them adjacent to the doc file (same folder) and keep naming clear.

**Folder Structure:**
```
000-docs/
├── 001-DR-STND-document-filing-system-v4.md    # NNN project docs
├── 002-DR-TMPL-aar-template.md
├── 003-AA-AACR-phase-0-scaffold.md
├── 6767-a-DR-STND-canonical-standard.md        # 6767 canonical docs
└── 6767-b-DR-INDEX-standards-catalog.md
```

---

## 4) CATEGORIES (CC) — 2 LETTERS
| Code | Category |
|---|---|
| PP | Product & Planning |
| AT | Architecture & Technical |
| DC | Development & Code |
| TQ | Testing & Quality |
| OD | Operations & Deployment |
| LS | Logs & Status |
| RA | Reports & Analysis |
| MC | Meetings & Communication |
| PM | Project Management |
| DR | Documentation & Reference |
| UC | User & Customer |
| BL | Business & Legal |
| RL | Research & Learning |
| AA | After Action & Review |
| WA | Workflows & Automation |
| DD | Data & Datasets |
| MS | Miscellaneous |

---

## 5) DOCUMENT TYPES (ABCD) — 4 LETTERS (KEY TYPES)

### DR — Documentation & Reference
REFF, GUID, MANL, FAQS, GLOS, SOPS, TMPL, CHKL, STND, INDEX

### AA — After Action & Review
AACR, LESN, PMRT

### AT — Architecture & Technical
ADEC, ARCH, DSGN, APIS, SDKS, INTG, DIAG

### PM — Project Management
TASK, BKLG, SPRT, RETR, STND, RISK, ISSU

### PP — Product & Planning
PROD, PLAN, RMAP, BREQ, FREQ, SOWK, KPIS, OKRS

---

## 6) NAMING CONSTRAINTS (HARD RULES)
**DO**
- lowercase kebab-case descriptions
- keep descriptions short (avoid sentence titles)
- use `.md` for most docs
- keep `NNN` chronological
- keep `6767` for cross-repo standards only

**DON'T**
- no underscores / camelCase in descriptions
- no special chars except hyphens
- no missing category or type codes
- no numeric IDs after `6767-` in filenames (v3+ rule)
- no subdirectories under 000-docs/

---

## 7) EXAMPLES (COPY/PASTE)
### Project docs

000-docs/
001-DR-STND-document-filing-system-v4.md
002-DR-TMPL-aar-template.md
003-AA-AACR-phase-0-scaffold.md
004-AA-AACR-phase-1-rust-mvp.md
005-AT-ARCH-system-design.md

### Canonical standards

000-docs/
6767-a-DR-STND-document-filing-standard.md
6767-b-DR-INDEX-standards-catalog.md

---

## 8) AI ASSISTANT OPERATING INSTRUCTIONS (STRICT)
When creating or renaming a document:
1) Decide series: **6767** if cross-repo standard; else **NNN**.
2) Pick `CC` from Category table.
3) Pick `ABCD` from Type tables (do not invent).
4) Create filename using the exact pattern rules.
5) Keep description short and kebab-case.
6) If a 6767 doc needs an internal ID for cross-ref, place it in the header only.
7) **Place ALL docs (both NNN and 6767) directly in `000-docs/`** — no subdirectories.
8) **After every phase, create an AAR:** `NNN-AA-AACR-phase-<n>-short-description.md`

---

**DOCUMENT FILING SYSTEM STANDARD v4.2**
*Fully compatible with v3.0 and v4.0; optimized for AI assistants and deterministic naming.*
*v4.2 enforces strict flat 000-docs (no subdirectories allowed).*

---
intent solutions io
Contact: jeremy@intentsolutions.io
