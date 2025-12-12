# Architecture Decision Records: Resume Realtime MVP

**Created:** 2025-12-08 CST (America/Chicago)
**Last Updated:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** 🟢 Accepted
**Maintainer:** intent solutions io
**PRD Reference:** 014-PP-PROD-resume-realtime-mvp-prd.md

---

## Decision Summary

This document captures the architectural decisions for the Resume Realtime MVP (Phase 1: Parser in a Box). These decisions establish the technical foundation for all future phases.

---

## ADR-MVP-001: Leptos SSR + Hydration for Extension UI

### Status
🟢 **Accepted**

### Decision Statement
We will **use Leptos with SSR + hydration for the extension popup UI** in order to **maintain a unified Rust codebase and leverage WASM performance** accepting that **initial bundle size will be larger than pure React**.

### Context
The MVP requires a popup UI for drag-drop parsing and field editing. Options include:
- React + TypeScript (industry standard for extensions)
- Leptos (Rust-native, compiles to WASM)
- Vanilla JS (minimal dependencies)

The existing codebase already uses Leptos with Axum for SSR.

### Decision Drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| Unified Rust codebase | High | Single language for UI + inference |
| WASM performance | High | Shared memory with inference engine |
| Developer expertise | Medium | Team knows Rust |
| Bundle size | Low | Acceptable for extension |
| Ecosystem maturity | Medium | Leptos 0.6 is stable |

### Alternatives Considered

| Option | Pros | Cons | Score |
|--------|------|------|-------|
| **Leptos** | Unified codebase, WASM perf | Larger bundle, less ecosystem | **8/10** |
| React + TS | Industry standard, small bundle | Two languages, JS/WASM bridge | 7/10 |
| Vanilla JS | Smallest bundle | No component model, maintenance | 4/10 |

### Consequences

**Positive:**
- Single Rust codebase (UI + inference + storage)
- Direct WASM memory access (no serialization overhead)
- Type safety across entire stack
- Reactive signals for real-time UI updates

**Negative:**
- Larger initial bundle (~500KB vs ~100KB React)
- Steeper learning curve for new contributors
- Less community examples for extension development

### Implementation Notes
- Use `cargo-leptos` for build orchestration
- Popup renders via Leptos hydration
- Share types between UI and inference layers

---

## ADR-MVP-002: transformers.js for Local LLM Inference

### Status
🟢 **Accepted**

### Decision Statement
We will **use transformers.js with Phi-3-mini q4** in order to **run local LLM inference with WebGPU acceleration** accepting that **initial model download is ~1.5GB**.

### Context
Resume parsing requires semantic understanding beyond regex. Options:
- transformers.js (Hugging Face, browser-native)
- llama.cpp WASM (raw performance)
- ONNX Runtime Web (Microsoft)
- candle (Rust-native)

### Decision Drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| WebGPU acceleration | High | 5-10x speedup |
| Model ecosystem | High | Hugging Face hub access |
| Browser compatibility | High | Chrome, Edge, Firefox |
| API simplicity | Medium | Pipeline abstraction |
| Bundle size | Low | Model dominates |

### Alternatives Considered

| Option | Pros | Cons | Score |
|--------|------|------|-------|
| **transformers.js** | WebGPU, easy API, Phi-3 support | JS library (not Rust) | **9/10** |
| candle (Rust) | Rust-native, WASM | Less mature WebGPU | 7/10 |
| llama.cpp WASM | Raw performance | Complex setup | 6/10 |
| ONNX Runtime Web | Microsoft backing | Heavier bundle | 6/10 |

### Model Selection

| Model | Size (q4) | Accuracy | Latency | Decision |
|-------|-----------|----------|---------|----------|
| **Phi-3-mini (1.3B)** | 1.5GB | Good | Fast | **Primary** |
| Llama-3.2-1B | 1GB | Acceptable | Very Fast | Fallback |
| Llama-3.2-3B | 2.5GB | Better | Moderate | Future |

### Consequences

**Positive:**
- WebGPU provides near-native inference speed
- Phi-3 optimized for instruction following
- Quantization keeps model browser-cacheable
- Fallback to WASM SIMD on non-WebGPU browsers

**Negative:**
- 1.5GB initial download (one-time)
- transformers.js is JavaScript (bridge to Rust)
- WebGPU support varies by browser

### Fallback Strategy
```
1. Check navigator.gpu availability
2. If WebGPU → transformers.js with device: 'webgpu'
3. If no WebGPU → transformers.js with device: 'wasm'
4. If WASM fails → Display browser upgrade prompt
```

---

## ADR-MVP-003: pdf.js for Client-Side PDF Extraction

### Status
🟢 **Accepted**

### Decision Statement
We will **use Mozilla pdf.js for PDF text extraction** in order to **reliably extract text client-side with broad format support** accepting that **complex layouts may require post-processing**.

### Context
PDFs must be parsed entirely in-browser. No server round-trips.

### Decision Drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| Client-side only | Critical | Architectural constraint |
| Format compatibility | High | Must handle diverse resumes |
| Maintenance | High | Active Mozilla project |
| Text ordering | Medium | Reading order matters |

### Alternatives Considered

| Option | Pros | Cons | Score |
|--------|------|------|-------|
| **pdf.js** | Mozilla-maintained, broad support | Large bundle | **9/10** |
| pdf-lib | Smaller, good for generation | Weaker extraction | 6/10 |
| pdfjs-dist | Lighter wrapper | Same core as pdf.js | 8/10 |

### Consequences

**Positive:**
- Handles 99%+ of PDF formats
- Preserves text positioning for source highlighting
- Active maintenance and security updates
- Works offline

**Negative:**
- ~500KB bundle contribution
- Complex multi-column layouts may mis-order text
- Scanned PDFs need OCR (out of MVP scope)

### Implementation Notes
- Use `getTextContent()` for extraction
- Store text positions for HITL source highlighting
- Handle password-protected PDFs gracefully (error message)

---

## ADR-MVP-004: IndexedDB + SubtleCrypto for Encrypted Storage

### Status
🟢 **Accepted**

### Decision Statement
We will **use IndexedDB with AES-256-GCM encryption via SubtleCrypto** in order to **persist candidate data securely across sessions** accepting that **key management adds UX complexity**.

### Context
Parsed resumes must persist locally with encryption at rest.

### Decision Drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| Storage capacity | High | Resumes + model cache |
| Encryption strength | High | GDPR compliance |
| Async API | Medium | Non-blocking UI |
| Browser support | High | All modern browsers |

### Alternatives Considered

| Option | Pros | Cons | Score |
|--------|------|------|-------|
| **IndexedDB + SubtleCrypto** | Native, unlimited, async | Complex API | **9/10** |
| localStorage | Simple API | 5MB limit, sync, no encryption | 3/10 |
| chrome.storage.local | Extension-native | 10MB limit | 5/10 |
| SQLite WASM | SQL queries | Heavy bundle, overkill | 4/10 |

### Schema Design

```javascript
// IndexedDB stores
{
  candidates: {
    keyPath: 'universalId',
    indexes: ['name', 'email', 'parsedAt'],
    // All values encrypted
  },
  settings: {
    keyPath: 'key',
    // Unencrypted
  }
}
```

### Encryption Flow

```
1. First run: Generate 256-bit key via crypto.subtle.generateKey()
2. Store key in chrome.storage.session (cleared on browser close)
3. Encrypt: crypto.subtle.encrypt('AES-GCM', key, data)
4. Store encrypted blob in IndexedDB
5. Decrypt on read: crypto.subtle.decrypt('AES-GCM', key, blob)
```

### Consequences

**Positive:**
- Unlimited storage capacity
- Strong encryption (AES-256-GCM)
- No third-party dependencies
- Async = non-blocking

**Negative:**
- Key lost if browser data cleared (acceptable for MVP)
- ~5-10ms encryption overhead per record
- More complex than localStorage

---

## ADR-MVP-005: Manifest V3 Chrome Extension Architecture

### Status
🟢 **Accepted**

### Decision Statement
We will **build as a Manifest V3 Chrome extension** in order to **distribute via Chrome Web Store and comply with modern extension standards** accepting that **service worker limitations require architectural adjustments**.

### Context
Chrome Web Store requires Manifest V3 for new extensions as of 2024.

### Decision Drivers

| Driver | Weight | Notes |
|--------|--------|-------|
| Store distribution | Critical | PLG strategy |
| Future-proofing | High | MV2 deprecated |
| API access | High | Need storage, scripting |
| Service worker model | Medium | Replaces background pages |

### Manifest Structure

```json
{
  "manifest_version": 3,
  "name": "Resume Realtime",
  "version": "0.1.0",
  "permissions": [
    "storage",
    "activeTab"
  ],
  "action": {
    "default_popup": "popup.html"
  },
  "icons": { ... }
}
```

### Consequences

**Positive:**
- Chrome Web Store distribution
- Modern security model
- Cross-browser potential (Edge, Firefox)
- Service worker = efficient resource usage

**Negative:**
- No persistent background page (service worker sleeps)
- Some APIs differ from MV2
- Must handle service worker wake-up latency

### Implementation Notes
- Use Plasmo framework for MV3 scaffolding
- Store large data in IndexedDB (not chrome.storage)
- Model cache uses Cache API (persists across service worker restarts)

---

## ADR-MVP-006: WebGPU Primary with WASM SIMD Fallback

### Status
🟢 **Accepted**

### Decision Statement
We will **prioritize WebGPU for inference with WASM SIMD as fallback** in order to **maximize performance on capable hardware while maintaining broad compatibility** accepting that **fallback is 5-10x slower**.

### Context
LLM inference is compute-intensive. WebGPU provides GPU acceleration but isn't universally available.

### Browser Support Matrix (Dec 2024)

| Browser | WebGPU | WASM SIMD |
|---------|--------|-----------|
| Chrome 113+ | ✅ | ✅ |
| Edge 113+ | ✅ | ✅ |
| Firefox 118+ | 🟡 Flag | ✅ |
| Safari 17+ | ❌ | ✅ |

### Detection Logic

```typescript
async function selectBackend(): Promise<'webgpu' | 'wasm'> {
  if (navigator.gpu) {
    try {
      const adapter = await navigator.gpu.requestAdapter();
      if (adapter) return 'webgpu';
    } catch {}
  }
  return 'wasm';
}
```

### Consequences

**Positive:**
- 5-10x faster inference on WebGPU
- Broad fallback coverage via WASM
- Graceful degradation (no crashes)

**Negative:**
- Safari users get slow experience
- Firefox requires manual flag
- Must test both paths

### Performance Targets

| Backend | Parse Latency | Acceptable |
|---------|---------------|------------|
| WebGPU | < 5s | ✅ |
| WASM SIMD | < 30s | ✅ (with progress UI) |

---

## Decision Log

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| MVP-001 | Leptos SSR + Hydration | 🟢 Accepted | 2025-12-08 |
| MVP-002 | transformers.js + Phi-3 | 🟢 Accepted | 2025-12-08 |
| MVP-003 | pdf.js Extraction | 🟢 Accepted | 2025-12-08 |
| MVP-004 | IndexedDB + Encryption | 🟢 Accepted | 2025-12-08 |
| MVP-005 | Manifest V3 Extension | 🟢 Accepted | 2025-12-08 |
| MVP-006 | WebGPU + WASM Fallback | 🟢 Accepted | 2025-12-08 |

---

## Review Schedule

| Review Type | Frequency | Next Review |
|-------------|-----------|-------------|
| Technical validation | After each milestone | M2 complete |
| Performance audit | Weekly during dev | Week 3 |
| Security review | Pre-launch | Week 7 |
| Full ADR review | Post-MVP | Month 4 |

---

## Related Documents

| Document | Purpose |
|----------|---------|
| 014-PP-PROD-resume-realtime-mvp-prd.md | MVP requirements |
| 009-AT-ADEC-resume-realtime-architecture-decisions.md | Full-scope ADRs |
| 016-PM-TASK-resume-realtime-mvp-tasks.md | Implementation tasks |

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
