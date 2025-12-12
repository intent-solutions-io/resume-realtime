# PRD: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Draft
**Author:** intent solutions io

---

## Introduction/Overview

Resume Realtime is a browser-resident application that disrupts the recruitment technology oligopoly by shifting compute from cloud servers to the user's edge device. It eliminates the "Resume Tax" imposed by incumbents like DaXtra, RChilli, Bullhorn, and Allsorter by offering zero-cost parsing, formatting, and enrichment through local LLM inference and WebAssembly.

**Problem Statement:** Recruiters currently pay per-transaction fees for basic operations (parsing, formatting, enrichment) while suffering fragmented workflows across 5+ disconnected tools. This creates friction, breaks flow state, and compounds costs at scale.

**Solution:** A Chrome Extension that runs parsing/formatting locally via WASM + quantized LLMs, injects context-aware overlays into LinkedIn/Gmail/ATS interfaces, and unifies the entire recruitment workflow into a single "Super-Layer."

---

## Goals

1. **Zero-Cost Parsing:** Reduce marginal cost of resume parsing to $0.00 by executing inference on client hardware
2. **Workflow Unification:** Eliminate context switching by providing a persistent overlay across LinkedIn, Gmail, and major ATS platforms
3. **Privacy Supremacy:** Process all candidate data locally with zero server transmission (GDPR-compliant by architecture)
4. **PLG Distribution:** Achieve 1,000 active users via Chrome Web Store within 3 months through viral "free parser" hook
5. **ATS Integration:** Bi-directional sync with Bullhorn and Salesforce REST APIs by month 12
6. **Accuracy Parity:** Match or exceed DaXtra's claimed 95%+ parsing accuracy with Human-in-the-Loop corrections

---

## User Stories

### Primary Persona: Agency Recruiter (360-degree)

1. **As a recruiter**, I want to parse a resume instantly in my browser without uploading to a third-party server, so that I can capture candidate data without incurring per-document costs or privacy risks.

2. **As a recruiter**, I want to see a "Candidate Exists" badge when viewing a LinkedIn profile, so that I can avoid duplicate data entry and see existing notes from my ATS.

3. **As a recruiter**, I want to format/brand a resume with my agency's template in one click, so that I can send client-ready documents without leaving my browser or paying for Allsorter.

4. **As a recruiter**, I want to find a candidate's email address without switching to Lusha/Apollo, so that I can enrich profiles directly from LinkedIn using OSINT or my existing API keys (BYOK).

5. **As a recruiter**, I want to correct parsing errors by highlighting text on the original PDF, so that the parser learns my preferences over time.

6. **As a recruiter**, I want to schedule follow-up emails directly from Gmail without syncing to an external platform, so that sequences are reliable and context-aware.

### Secondary Persona: Agency Owner

7. **As an agency owner**, I want my team to share branded resume templates, so that output is consistent across recruiters.

8. **As an agency owner**, I want GDPR compliance alerts when candidate data is due for deletion, so that I can maintain regulatory compliance without manual tracking.

---

## Functional Requirements

### Phase 1: MVP - Parser in a Box

| ID | Requirement |
|----|-------------|
| FR-1.1 | Extension shall accept PDF files via drag-and-drop onto overlay |
| FR-1.2 | Extension shall extract text from PDF using pdf.js |
| FR-1.3 | Extension shall run quantized LLM (Phi-3/Llama-3 4-bit) via WASM + WebGPU for structured extraction |
| FR-1.4 | Extension shall output parsed resume as JSON with fields: name, email, phone, location, experience[], education[], skills[] |
| FR-1.5 | Extension shall display HITL interface with original PDF on left, extracted fields on right |
| FR-1.6 | Extension shall allow field correction by highlighting source text on PDF |
| FR-1.7 | Extension shall cache parsed data locally using IndexedDB (encrypted) |

### Phase 2: Overlay - Context Hijacker

| ID | Requirement |
|----|-------------|
| FR-2.1 | Extension shall inject Shadow DOM overlay into LinkedIn profile pages |
| FR-2.2 | Extension shall inject Shadow DOM overlay into Gmail compose/inbox views |
| FR-2.3 | Extension shall scrape LinkedIn profile data (name, headline, experience) via DOM reading |
| FR-2.4 | Extension shall compute Universal ID hash from (Name + LinkedIn URL + Email) |
| FR-2.5 | Extension shall display "Candidate Exists" badge if Universal ID matches local database |
| FR-2.6 | Extension shall run OSINT enrichment (GitHub commits, personal websites) before querying paid APIs |
| FR-2.7 | Extension shall support BYOK (Bring Your Own Key) for Lusha, Hunter, RocketReach APIs |
| FR-2.8 | Extension shall inject "Send Later" and "Add to Sequence" buttons into Gmail compose |
| FR-2.9 | Extension shall use Chrome alarms API for local email scheduling |

### Phase 3: Integrator - ATS Killer

| ID | Requirement |
|----|-------------|
| FR-3.1 | Extension shall authenticate with Bullhorn via OAuth 2.0 |
| FR-3.2 | Extension shall authenticate with Salesforce via OAuth 2.0 |
| FR-3.3 | Extension shall push parsed candidate data to ATS via REST API |
| FR-3.4 | Extension shall inject "Save to ATS" button on LinkedIn profiles |
| FR-3.5 | Extension shall read ATS candidate status via API and display in overlay |
| FR-3.6 | Extension shall generate branded PDF using pdf-lib with agency templates |
| FR-3.7 | Extension shall offer one-click anonymization (redact contact info) |
| FR-3.8 | Extension shall track candidate data TTL and alert on GDPR expiry |
| FR-3.9 | Extension shall issue delete commands to ATS API on GDPR scrub action |

### Phase 4: Network - The Moat

| ID | Requirement |
|----|-------------|
| FR-4.1 | Extension shall sync candidate pools P2P using WebRTC |
| FR-4.2 | Extension shall aggregate anonymized salary data with differential privacy |
| FR-4.3 | Extension shall support team-shared templates via peer sync |
| FR-4.4 | Extension shall implement "Give-to-Get" crowdsourced enrichment pool |

---

## Non-Goals (Out of Scope)

1. **Native mobile app** - Browser extension only; mobile is future phase
2. **Full ATS replacement** - We augment, not replace; ATS remains system of record
3. **Server-side processing** - No cloud compute for parsing/formatting (architectural constraint)
4. **Job board scraping** - Focus on LinkedIn; Indeed/Monster are out of scope for MVP
5. **Outbound calling/VoIP** - Email sequencing only; phone is separate product
6. **Custom LLM training** - Use pre-trained quantized models; fine-tuning is future enhancement
7. **Enterprise SSO/SAML** - Individual recruiter focus first; enterprise auth is Phase 4+

---

## Design Considerations

### UI/UX Requirements

1. **Overlay Injection:**
   - Shadow DOM for CSS/JS isolation (no bleed from host page)
   - React-based sidebar (collapsible, draggable)
   - Dark/light theme matching host site

2. **HITL Parsing Interface:**
   - Split-pane: PDF viewer (left) + extracted fields (right)
   - Click field to highlight source in PDF
   - Highlight-to-correct interaction pattern
   - Confidence scores per field (visual indicator)

3. **Context Badges:**
   - "Candidate Exists" pill on LinkedIn profiles
   - Last contact date + status from ATS
   - Color-coded (green = active, yellow = stale, red = DNR)

4. **Formatting Preview:**
   - Real-time template preview before generation
   - Logo injection with position controls
   - Anonymization toggle (redact name/contact)

### Visual Design Principles

- Minimal chrome (the extension, not the browser)
- Non-intrusive: collapsed by default on page load
- Consistent iconography with recruitment mental models
- Keyboard shortcuts for power users (Ctrl+Shift+P to parse)

---

## Technical Considerations

### Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    BROWSER EXTENSION                        │
├─────────────────────────────────────────────────────────────┤
│  UI Layer: TypeScript + React + Plasmo                      │
│  - Shadow DOM overlays (LinkedIn/Gmail/ATS)                │
│  - HITL validation interface                               │
│  - Context-aware badges and alerts                         │
├─────────────────────────────────────────────────────────────┤
│  Compute Layer: Rust → WASM (wasm-bindgen + web-sys)       │
│  - Resume parsing (pdf.js + custom extractors)             │
│  - Local LLM inference (candle/tract + WebGPU)             │
│  - OSINT enrichment (concurrent queries)                   │
│  - OCR (tesseract-wasm)                                    │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer: IndexedDB + localStorage                   │
│  - Universal ID system (hashed identities)                 │
│  - Encrypted candidate data (SubtleCrypto)                 │
│  - Cached LLM models (2-4GB quantized)                     │
├─────────────────────────────────────────────────────────────┤
│  Network Layer: WebRTC (Phase 4)                           │
│  - P2P team sync                                           │
│  - Differential privacy aggregation                        │
└─────────────────────────────────────────────────────────────┘
```

### Technical Constraints

1. **Model Size:** Quantized LLMs (4-bit) at 2-4GB must be browser-cachable
2. **Initial Load:** First inference ~10-30s acceptable; subsequent < 3s
3. **WebGPU Dependency:** Chrome primary; Firefox/Safari require polyfill or fallback
4. **Manifest V3:** Service workers instead of persistent background pages
5. **CSP Bypass:** Salesforce Lightning Locker requires visual DOM read + API write pattern
6. **Memory Limit:** Stay under 4GB browser heap for stability

### Dependencies

| Component | Library/Tool | Purpose |
|-----------|--------------|---------|
| Extension Framework | Plasmo | Manifest V3, hot-reload, cross-browser |
| UI Framework | React 18 | Shadow DOM mounting, rich interactions |
| PDF Extraction | pdf.js | Client-side text extraction |
| PDF Generation | pdf-lib | Client-side branded PDF creation |
| LLM Runtime | transformers.js | q4/q8 quantized models + WebGPU |
| WASM Compiler | wasm-pack | Rust → WASM build toolchain |
| Crypto | SubtleCrypto (Web API) | Hashing, encryption |
| P2P | peerjs / WebRTC | Team sync (Phase 4) |

### Security Requirements

1. All candidate data encrypted at rest (IndexedDB + AES-256)
2. Universal ID hashed with SHA-256 (not reversible)
3. API keys (BYOK) stored in encrypted localStorage
4. No external network calls for parsing/formatting (audit-verifiable)
5. Content Security Policy compliant injection

---

## Success Metrics

| Metric | Target | Measurement |
|--------|--------|-------------|
| Active Users (Phase 1) | 1,000 | Chrome Web Store installs with >1 parse/week |
| Parsing Accuracy | > 95% | Sampled audit of 100 diverse resumes vs. ground truth |
| Parse Latency (cached model) | < 5s | P95 from drop to JSON output |
| Parse Latency (cold start) | < 30s | P95 including model load |
| DAU/MAU Ratio | > 40% | Indicates sticky daily usage |
| HITL Correction Rate | < 10% | Fields requiring manual correction |
| Pro Conversion (Phase 3) | > 5% | Free users upgrading to $29/mo tier |
| NPS Score | > 50 | Post-use survey |
| Churn Rate | < 5%/mo | Monthly user retention |

---

## Open Questions

1. **Model Selection:** Phi-3-mini (1.3B) vs. Llama-3.2-1B for best accuracy/size tradeoff?
2. **Safari Support:** How to handle WebGPU absence on Safari (fallback to CPU inference)?
3. **ATS Partnership:** Pursue official Bullhorn marketplace listing or stay guerrilla?
4. **Pricing Tier Structure:** $29/mo Pro tier sufficient? Need Team tier at $XX/seat?
5. **LoRA Fine-tuning:** Should user corrections train personalized adapter, or just correct output?
6. **OSINT Legal:** Confirm GitHub email scraping and public Whois queries are terms-compliant

---

## Appendix: Competitive Positioning

| Feature | DaXtra | RChilli | Affinda | Resume Realtime |
|---------|--------|---------|---------|----------------|
| Pricing | Opaque/Enterprise | Credit-based | $0.05-$0.20/doc | **Free (unlimited)** |
| Data Location | Server | Server | Server | **Client (local)** |
| GDPR Burden | Vendor liability | Vendor liability | Vendor liability | **User-controlled** |
| Accuracy | ~95% (claimed) | ~90% | ~93% | ~95% + HITL |
| Integration | API-only | API-only | API-only | **Browser-native** |
| Formatting | No | No | No | **Yes (pdf-lib)** |
| Enrichment | No | No | No | **Yes (BYOK/OSINT)** |

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
