# PRD: Resume Realtime MVP (Phase 1)

**Created:** 2025-12-08 CST (America/Chicago)
**Last Updated:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Draft
**Maintainer:** intent solutions io
**Related Documents:** 007-PP-PROD-resume-realtime-prd.md, 009-AT-ADEC-resume-realtime-architecture-decisions.md

---

## 1. Vision & Problem Statement

### Vision
**One-liner:** Eliminate the "Resume Tax" by enabling recruiters to parse resumes locally in their browser with zero cost, zero data transmission, and zero vendor lock-in.

### Problem Statement
Recruiters currently pay $0.05-$0.20 per document to parse resumes through cloud services (DaXtra, RChilli, Affinda). This creates:

| Pain Point | Impact | Validation |
|------------|--------|------------|
| Per-transaction costs | $800-$5,000/year for active recruiters | Affinda pricing: $0.05-$0.20/doc |
| Privacy liability | GDPR risk when data leaves device | Server-side = vendor liability |
| Vendor lock-in | Proprietary formats, no portability | API-only access |
| Context switching | 5+ tools, broken flow state | User interviews |

### MVP Scope
**Phase 1: Parser in a Box** - A standalone resume parsing tool that runs entirely in the browser. No overlays, no ATS integration, no team features. Just drag-drop-parse.

---

## 2. Objectives & Key Results (OKRs)

### Primary Objective
**Launch a functional local resume parser that proves the edge-compute thesis.**

| Key Result | Target | Measurement |
|------------|--------|-------------|
| KR1: Chrome Web Store approval | Approved | Binary |
| KR2: Parsing accuracy | ≥ 90% | 100-resume test set |
| KR3: Parse latency (cached) | < 5 seconds | P95 |
| KR4: Active weekly users | 100 | Extension analytics |
| KR5: Zero server transmission | 0 bytes | Network audit |

---

## 3. Users & Personas

### Primary Persona: Independent Recruiter

| Attribute | Description |
|-----------|-------------|
| **Role** | Agency recruiter or solo headhunter |
| **Tech comfort** | Uses Chrome daily, installs extensions |
| **Pain point** | Paying per-resume fees, privacy concerns |
| **Goal** | Parse resumes free, keep data local |
| **Success metric** | Parsed resume → usable JSON in < 10s |

### User Journey (MVP)

```
1. Install extension from Chrome Web Store
2. Click extension icon → Parser UI opens
3. Drag PDF onto drop zone
4. Wait for model download (first time only)
5. View extracted fields with confidence scores
6. Correct any errors via HITL interface
7. Copy JSON or save locally
```

---

## 4. Scope & Prioritization

### MVP (Phase 1) - IN SCOPE

| Feature | Priority | Complexity |
|---------|----------|------------|
| PDF drag-and-drop upload | P0 | Low |
| PDF text extraction (pdf.js) | P0 | Medium |
| Local LLM inference (Phi-3-mini q4) | P0 | High |
| Structured JSON output | P0 | Medium |
| HITL field correction UI | P1 | Medium |
| Encrypted local storage (IndexedDB) | P1 | Medium |
| WebGPU acceleration | P1 | High |
| WASM SIMD fallback | P1 | Medium |
| Model download with progress UI | P0 | Low |
| Settings page (model selection) | P2 | Low |

### OUT OF SCOPE (Future Phases)

- LinkedIn/Gmail overlay injection
- ATS integration (Bullhorn, Salesforce)
- Team features / P2P sync
- Resume formatting / branding
- OSINT enrichment
- Email sequencing
- Mobile support

---

## 5. Functional Requirements

### FR-MVP-01: PDF Upload

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-01.1 | Accept PDF via drag-and-drop | Drop zone highlights on dragover; accepts .pdf MIME type |
| FR-MVP-01.2 | Accept PDF via file picker | Click to browse; filters to .pdf |
| FR-MVP-01.3 | Validate file size | Reject files > 10MB with error message |
| FR-MVP-01.4 | Display upload progress | Progress bar during file read |

### FR-MVP-02: Text Extraction

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-02.1 | Extract text from PDF | Use pdf.js; return raw text string |
| FR-MVP-02.2 | Handle multi-page PDFs | Concatenate all pages |
| FR-MVP-02.3 | Preserve text order | Maintain reading order (top-to-bottom, left-to-right) |
| FR-MVP-02.4 | Handle extraction errors | Display user-friendly error; log details |

### FR-MVP-03: LLM Inference

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-03.1 | Download quantized model | Phi-3-mini q4 (~1.5GB); show progress |
| FR-MVP-03.2 | Cache model locally | Store in Cache API; persist across sessions |
| FR-MVP-03.3 | Run inference via WebGPU | Use transformers.js with WebGPU backend |
| FR-MVP-03.4 | Fallback to WASM SIMD | Detect WebGPU absence; use CPU inference |
| FR-MVP-03.5 | Extract structured fields | Output: name, email, phone, location, experience[], education[], skills[] |
| FR-MVP-03.6 | Include confidence scores | 0-100% per field |

### FR-MVP-04: Results Display

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-04.1 | Show extracted fields | Editable form with all fields |
| FR-MVP-04.2 | Display confidence indicators | Color-coded: green (>80%), yellow (50-80%), red (<50%) |
| FR-MVP-04.3 | Show original PDF | Split-pane view: PDF left, fields right |
| FR-MVP-04.4 | Link fields to source | Click field → highlight source in PDF |

### FR-MVP-05: HITL Correction

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-05.1 | Edit field values | Inline editing with save |
| FR-MVP-05.2 | Select text from PDF | Highlight text → assign to field |
| FR-MVP-05.3 | Add/remove array items | Add/delete experience, education, skills entries |
| FR-MVP-05.4 | Persist corrections | Save to IndexedDB |

### FR-MVP-06: Local Storage

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-06.1 | Encrypt candidate data | AES-256-GCM via SubtleCrypto |
| FR-MVP-06.2 | Generate encryption key | Create on first run; store in chrome.storage.session |
| FR-MVP-06.3 | Store parsed resumes | IndexedDB with universalId key |
| FR-MVP-06.4 | Export as JSON | Download button → JSON file |

### FR-MVP-07: Extension Shell

| ID | Requirement | Acceptance Criteria |
|----|-------------|---------------------|
| FR-MVP-07.1 | Manifest V3 compliant | Pass Chrome Web Store validation |
| FR-MVP-07.2 | Popup UI | Click icon → parser interface |
| FR-MVP-07.3 | Settings page | Model selection, storage management |
| FR-MVP-07.4 | Keyboard shortcut | Ctrl+Shift+R opens parser |

---

## 6. Non-Functional Requirements

### Performance

| Metric | Target | Measurement |
|--------|--------|-------------|
| Parse latency (cached model) | < 5s | P95 |
| Parse latency (cold start) | < 30s | P95 |
| Model download time | < 60s | On 50Mbps connection |
| UI responsiveness | < 100ms | Input to feedback |
| Memory usage | < 4GB | Peak during inference |

### Security

| Requirement | Implementation |
|-------------|----------------|
| Data at rest encryption | AES-256-GCM |
| No external network calls | Audit: zero requests during parse |
| Key management | Session-based; cleared on browser close |
| CSP compliance | No inline scripts; no eval |

### Usability

| Requirement | Target |
|-------------|--------|
| Time to first parse | < 2 minutes from install |
| Error recovery | Clear messages; retry options |
| Accessibility | WCAG 2.1 AA keyboard navigation |

### Reliability

| Requirement | Target |
|-------------|--------|
| Crash-free sessions | > 99% |
| Graceful degradation | CPU fallback if WebGPU fails |
| Data persistence | Survive browser restart |

---

## 7. Technical Architecture (MVP)

```
┌─────────────────────────────────────────────────────────────┐
│                 CHROME EXTENSION (Manifest V3)              │
├─────────────────────────────────────────────────────────────┤
│  Popup UI: React + TypeScript                               │
│  ├── Drop zone component                                    │
│  ├── PDF viewer (pdf.js)                                    │
│  ├── Field editor (HITL)                                    │
│  └── Settings page                                          │
├─────────────────────────────────────────────────────────────┤
│  Inference Engine: transformers.js                          │
│  ├── Model: Phi-3-mini q4 (~1.5GB)                         │
│  ├── Backend: WebGPU (primary) / WASM SIMD (fallback)      │
│  └── Cache: Cache API (persistent)                          │
├─────────────────────────────────────────────────────────────┤
│  Storage Layer: IndexedDB + SubtleCrypto                    │
│  ├── Candidates store (encrypted)                           │
│  ├── Settings store                                         │
│  └── Key: chrome.storage.session                            │
└─────────────────────────────────────────────────────────────┘
```

### Technology Stack

| Layer | Technology | Rationale |
|-------|------------|-----------|
| Extension Framework | Plasmo | Manifest V3, hot-reload, TypeScript |
| UI | React 18 | Component model, ecosystem |
| PDF | pdf.js | Mozilla-maintained, client-side |
| LLM | transformers.js | Browser-native, WebGPU support |
| Storage | IndexedDB | Unlimited capacity, async |
| Crypto | SubtleCrypto | Native Web API, no dependencies |

---

## 8. Risk Assessment

| Risk | Probability | Impact | Mitigation |
|------|-------------|--------|------------|
| WebGPU not available | Medium | High | WASM SIMD fallback; browser upgrade prompt |
| Model accuracy < 90% | Medium | High | HITL corrections; model tuning; user feedback |
| Chrome store rejection | Low | Critical | Pre-submission policy review; minimal permissions |
| Model too large for cache | Low | Medium | Chunked download; Lite mode with smaller model |
| Parse latency > 30s | Medium | Medium | Smaller model option; progress UI |

---

## 9. Success Metrics

### Launch Criteria (MVP Complete)

- [ ] Chrome Web Store listing approved
- [ ] 100-resume test set: ≥ 90% accuracy
- [ ] Parse latency P95 < 5s (cached)
- [ ] Zero external network calls during parse
- [ ] WCAG 2.1 AA keyboard navigation

### Post-Launch KPIs

| Metric | Target | Timeframe |
|--------|--------|-----------|
| Weekly active users | 100 | Month 1 |
| Resumes parsed | 1,000 | Month 1 |
| HITL correction rate | < 20% | Month 1 |
| Crash-free rate | > 99% | Ongoing |
| 5-star rating | > 4.0 | Month 2 |

---

## 10. Release Strategy

### Phase 1 Milestones

| Milestone | Target | Exit Criteria |
|-----------|--------|---------------|
| M1: Scaffold | Week 1 | Extension loads; popup renders |
| M2: PDF Extract | Week 2 | Text extracted from 10 test PDFs |
| M3: LLM Inference | Week 4 | Fields extracted with confidence |
| M4: HITL UI | Week 5 | Corrections persist |
| M5: Storage | Week 6 | Encrypted save/load works |
| M6: Polish | Week 7 | Performance targets met |
| M7: Launch | Week 8 | Chrome store approved |

### Rollout Plan

| Stage | Audience | Duration | Success Criteria |
|-------|----------|----------|------------------|
| Alpha | Internal (3 users) | 1 week | No critical bugs |
| Beta | Invited testers (20) | 2 weeks | > 85% accuracy, < 10s parse |
| GA | Public | Ongoing | Store approval |

---

## 11. Open Questions

| Question | Owner | Due Date | Resolution |
|----------|-------|----------|------------|
| Phi-3-mini vs Llama-3.2-1B? | Tech Lead | Week 2 | Benchmark both |
| Safari fallback strategy? | Tech Lead | Week 3 | CPU-only for Safari |
| Privacy policy hosting? | Product | Week 6 | GitHub Pages |

---

## 12. Appendix

### Resume Field Schema

```typescript
interface ParsedResume {
  universalId: string;          // SHA-256 hash
  name: string;
  email: string | null;
  phone: string | null;
  location: string | null;
  experience: {
    company: string;
    title: string;
    startDate: string;
    endDate: string | null;
    description: string;
    confidence: number;
  }[];
  education: {
    institution: string;
    degree: string;
    field: string;
    graduationDate: string | null;
    confidence: number;
  }[];
  skills: string[];
  rawText: string;
  parsedAt: string;             // ISO timestamp
  source: 'upload' | 'linkedin';
}
```

### Competitive Reference

| Feature | DaXtra | Affinda | Resume Realtime MVP |
|---------|--------|---------|---------------------|
| Price | Enterprise | $0.05-$0.20/doc | **Free** |
| Data location | Server | Server | **Local** |
| Offline | No | No | **Yes** |
| HITL correction | No | No | **Yes** |

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
