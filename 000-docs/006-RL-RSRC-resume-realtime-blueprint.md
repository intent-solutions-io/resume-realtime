# Resume Realtime - Full Blueprint Analysis (Late 2025)

**Document Type:** Technical Architecture / Strategy Blueprint
**Created:** 2025-12-08
**Project:** Browser Generator - Edge Computing Resume Platform

---

## Updated Recommendations for Building the Resume Realtime Platform

Based on the complete blueprint in "Edge Computing Resumes .txt" (which details a disruptive browser extension for recruitment using edge compute, local LLMs, WASM, and a PLG model), here's an enhanced, practical guide to implementing it. This builds on our prior discussion, incorporating the full details from Parts III-VI: feature commoditization (e.g., Allsorter/Lusha killers), economic siege (reverse-SaaS pricing), regulatory moats (Zero-Trust), and the phased roadmap.

The core idea—shifting compute to the edge for zero-marginal-cost parsing/formatting—is highly feasible in 2025, with maturing tech like WebGPU-accelerated LLMs in browsers making local inference viable for quantized models like Llama 3 (e.g., 1-3B params at 2-4GB, running at usable speeds on consumer GPUs).

Incumbent pricing remains a vulnerability: Affinda starts at ~$800/year, RChilli is custom/quote-based, and DaXtra is opaque/enterprise-focused, aligning with your "Resume Tax" critique.

The stack prioritizes your emphasis on WASM for high-perf modules (parsing, OCR, OSINT), TypeScript for the extension core (Shadow DOM overlays, React UI), and local-first privacy. Rust remains the top choice for WASM due to safety and ecosystem maturity—avoiding C++ bugs or Go's GC pauses—while enabling "fearless" features like concurrent OSINT queries.

---

## Core Stack Recommendation

### Extension Framework/UI: TypeScript + React (with Plasmo or CRXJS)

**Why:** Handles the "Super-Layer" overlay (Shadow DOM injection for isolation on LinkedIn/Gmail/ATS), HITL UI (side-by-side PDF validation with highlights), and context-aware features (e.g., "Candidate Exists" badges via DOM scraping). React enables rich interactions like drag-and-drop editing, real-time previews, and animations. Plasmo streamlines Manifest V3 compliance, hot-reload, and cross-browser support (Chrome/Edge/Firefox/Safari).

**Alignment with Blueprint:** Directly supports:
- 2.2 (Browser Overlay)
- 3.4 (HITL with visual grounding)
- 3.3 (Inbox injection for sequencing)
- 5.2 (GDPR alerts)

Use IndexedDB or localStorage for the "Universal ID" system (hashed identities) and encrypted local DB.

**Ecosystem:**
- `pdf.js` for PDF viewing/extraction
- `pdf-lib` for in-browser formatting/anonymization (Allsorter killer, 3.1)
- `docx` for Word handling
- Chrome's alarms API for local scheduling

**Dev Tips:** Start with Shadow DOM for CSS/JS isolation:
```javascript
element.attachShadow({mode: 'open'})
```
Mount React apps without host conflicts. Handle CSP/Locker bypass via visual DOM reads (no direct hooks) and background API writes (OAuth for Bullhorn/Salesforce REST).

---

### Performance-Critical Modules (Parsing, Inference, OSINT): Rust + WASM

**Why:** Best for the "Edge-Compute Revolution" (2.1)—compiling to WASM for near-native speeds in browser, with memory safety for error-free parsing/enrichment. Supports WebGPU acceleration for local SLMs (e.g., quantized Llama 3 or Phi-3 via ONNX), enabling semantic extraction at zero cost. Rust's crates excel at concurrent tasks (e.g., multi-threaded OSINT from GitHub/Whois) and fine-tuning (LoRA adapters for personalized accuracy, 3.4).

**Vs. Alternatives (as in 2.1):**

| Language | Pros | Cons |
|----------|------|------|
| **Rust** | Memory safety, WebGPU interop, mature WASM ecosystem | Steeper learning curve |
| **C++** | Max raw perf (e.g., porting Tesseract OCR) | Prone to leaks/crashes—use only for legacy libs |
| **Go** | Simpler for quick modules (TinyGo for small binaries) | GC can stutter during inference; less mature WASM ecosystem |

**Alignment with Blueprint:** Powers:
- 2.1.1 (local inference for "Unlimited Parsing")
- 3.1 (JSON structuring post-parse)
- 3.2 (OSINT scripts before paid APIs)
- 3.4 (HITL corrections/fine-tuning)
- 4.4 (P2P sync via WebRTC wrappers in Phase 4)

For LLMs, use crates like `candle` (ONNX runtime) or `tract`—quantize to 4-bit for 2-4GB caches, runnable via transformers.js interop.

**Ecosystem:**
- `wasm-pack` for builds
- Integrate with JS via bindings (e.g., expose a `parse_resume(pdf_bytes: &[u8]) -> String` fn)
- For WebGPU, set `device: 'webgpu'` in pipelines

Chrome leads WebGPU support, with Firefox/Safari improving in 2025. Benchmarks show 5-10x speedup for LLM inference on mid-range GPUs. Examples: Hugging Face spaces demo Llama-3.2-1B in-browser for text tasks.

**Dev Tips - Basic Inference Example:**

In Rust, implement model loading; in JS:
```javascript
const pipe = await pipeline('text-generation', 'path/to/quantized-llama.onnx', {
  dtype: 'q4',
  device: 'webgpu'
});
const output = await pipe('Extract skills from resume: [text]');
```

---

### Auxiliary Tools/Libs

| Category | Recommendation | Purpose |
|----------|---------------|---------|
| **LLM Runtime** | transformers.js | Browser-friendly, supports q4/q8/fp16 dtypes for Llama 3; experimental WebGPU. ONNX Runtime Web for heavier models. |
| **Enrichment/Automation** | fetch + reqwest/cheerio | BYOK aggregator (3.2), API calls (Lusha/Hunter keys stored locally), OSINT web scraping |
| **Privacy/Compliance** | SubtleCrypto | Hashing Universal IDs, encrypting local data, TTL alerts with alarms API (5.2) |
| **P2P (Phase 4)** | WebRTC / peerjs / webrtc crate | Team sync; Rust's webrtc crate compiled to WASM |

---

## Phased Implementation Roadmap

Follow your sequence for disciplined rollout—focus on PLG hooks like free parsing to drive virality.

### Phase 1: MVP - Parser in a Box (Months 1-3)

**Scope:**
- TypeScript extension shell + Rust WASM for core parsing
- Drag-drop PDF → local LLM extraction → JSON output
- Test accuracy (aim >95% vs. DaXtra) on diverse resumes

**Metric:** Reddit buzz—search shows ongoing frustrations with legacy parsers' costs/speed.

**Launch:** Chrome Web Store with "Free Resume Parser" keywords.

---

### Phase 2: Overlay - Context Hijacker (Months 4-6)

**Scope:**
- Add React Shadow DOM for injections on LinkedIn/Gmail
- Rust for OSINT enrichment (e.g., GitHub email hunts)
- Implement HITL: pdf.js viewer with highlights, instant corrections

**Metric:** DAU tied to site usage; add "One-Click Anonymization" for quick wins.

---

### Phase 3: Integrator - ATS Killer (Months 7-12)

**Scope:**
- TypeScript for bi-directional ATS sync (REST wrappers for Bullhorn/Salesforce OAuth)
- Bypass moats with Rust OCR (e.g., tesseract-wasm) for reads
- Pro tier ($29/mo): Team templates, API writes

**Metric:** Freemium conversions; enforce GDPR scrubs locally.

---

### Phase 4: Network - The Moat (Year 2+)

**Scope:**
- Rust WASM for P2P (WebRTC) and differential privacy aggregation
- Anonymized salary insights
- Add "Give-to-Get" crowdsourcing

**Metric:** Enterprise upsells; network effects from shared pools.

---

## Potential Challenges & Mitigations (2025 Reality)

### Model Size/Performance
- **Challenge:** 2-4GB quantized models are browser-cachable, but initial load ~10-30s
- **Mitigation:** Progressive loading or smaller SLMs (Phi-3-mini). WebGPU boosts inference to 10-20 tokens/sec on laptops.

### Browser Limits
- **Challenge:** Manifest V3 caps background compute
- **Mitigation:** Use service workers. WebGPU is Chrome-strong, but polyfill for Firefox/Safari.

### Monetization/Marketing
- **Strategy:** "Tax-Free" messaging on Reddit/LinkedIn/X—target "DaXtra alternative" searches
- **Viral Hook:** Free tier hooks; Pro for compliance/sync

### Legal/Ethics
- **Strength:** Zero-Trust is a GDPR win
- **Caution:** Ensure OSINT complies with terms (e.g., no aggressive scraping). Start small to avoid incumbent lawsuits.

---

## Technology Stack Summary

```
┌─────────────────────────────────────────────────────────────┐
│                    BROWSER EXTENSION                        │
├─────────────────────────────────────────────────────────────┤
│  UI Layer: TypeScript + React + Plasmo/CRXJS               │
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
│  - Encrypted candidate data                                │
│  - Cached LLM models (2-4GB quantized)                     │
├─────────────────────────────────────────────────────────────┤
│  Network Layer: WebRTC (Phase 4)                           │
│  - P2P team sync                                           │
│  - Differential privacy aggregation                        │
└─────────────────────────────────────────────────────────────┘
```

---

## Conclusion

This stack (Rust WASM + TypeScript/React) matches your vision perfectly—empowering recruiters with local, edge-based tools to dismantle the oligopoly. It's disruptive, scalable, and leverages 2025 tech trends like browser AI runtimes.

---

**Generated:** 2025-12-08
**Source:** Edge Computing Resumes Blueprint Analysis
