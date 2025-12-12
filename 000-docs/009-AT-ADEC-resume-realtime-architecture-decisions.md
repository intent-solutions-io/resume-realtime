# Architecture Decision Records: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Approved
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md

---

## ADR-001: Edge-First Architecture (Local Compute over Cloud)

### Status
**Accepted**

### Context
The recruitment technology industry operates on server-side processing with per-transaction pricing. Incumbents (DaXtra, RChilli, Affinda) charge $0.05-$0.20 per document parsed. This "Resume Tax" compounds at scale and creates vendor lock-in. Additionally, server-side processing creates GDPR/privacy liability.

### Decision
**All parsing, formatting, and enrichment will execute on the client device.** No candidate data will be transmitted to our servers. The extension will use:
- WebAssembly (WASM) for high-performance compute
- Local quantized LLMs for semantic extraction
- WebGPU acceleration for inference

### Consequences

**Positive:**
- Zero marginal cost per parse (no cloud compute bills)
- GDPR compliance by architecture (data never leaves device)
- Offline-capable functionality
- Eliminates vendor liability for data breaches

**Negative:**
- Initial model download (2-4GB, one-time)
- Cold start latency (~10-30s first inference)
- Dependent on client hardware capabilities
- Cannot aggregate data for analytics without differential privacy

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| Hybrid (server for heavy lifting) | Reintroduces per-transaction cost; privacy liability |
| Thin client (server-only) | Standard SaaS model; no competitive differentiation |
| On-premise enterprise servers | Requires enterprise sales; conflicts with PLG strategy |

---

## ADR-002: Rust + WASM for Compute Layer

### Status
**Accepted**

### Context
The compute layer handles PDF parsing, LLM inference, OCR, and OSINT operations. These are CPU/GPU-intensive tasks that must execute in the browser at acceptable speeds.

### Decision
**Use Rust compiled to WebAssembly for all performance-critical modules.** TypeScript handles UI; Rust handles compute.

### Rationale

| Language | Evaluation |
|----------|------------|
| **Rust** | Memory safety, no GC pauses, mature WASM ecosystem (wasm-pack, wasm-bindgen), WebGPU interop via web-sys |
| C++ | Maximum raw performance but prone to memory bugs; unsafe for concurrent operations |
| Go (TinyGo) | Smaller binaries but GC causes stutter during inference; less mature WASM tooling |
| AssemblyScript | TypeScript-like but limited library ecosystem for ML/parsing |

### Consequences

**Positive:**
- Near-native performance in browser
- Memory safety prevents crashes during concurrent OSINT queries
- Single codebase compiles to both native (testing) and WASM (production)
- Strong ecosystem for ML (candle, tract) and PDF (pdf-extract)

**Negative:**
- Steeper learning curve for frontend-focused teams
- Longer compile times than TypeScript
- Debugging WASM in browser is more complex

---

## ADR-003: Shadow DOM for UI Isolation

### Status
**Accepted**

### Context
The extension injects UI overlays into third-party websites (LinkedIn, Gmail, Bullhorn, Salesforce). These sites have complex CSS and JavaScript that can conflict with injected components.

### Decision
**Mount all extension UI inside Shadow DOM containers with closed mode.**

```javascript
const container = document.createElement('div');
const shadow = container.attachShadow({ mode: 'closed' });
// Mount React app into shadow
```

### Consequences

**Positive:**
- Complete CSS isolation (host styles don't bleed into extension)
- Extension styles don't affect host page
- Works across all sites regardless of CSS framework
- Consistent UI appearance on LinkedIn, Gmail, Salesforce, etc.

**Negative:**
- Some browser dev tools have limited Shadow DOM inspection
- Cannot easily share styles with host page (rarely needed)
- Additional complexity for accessibility (focus management across shadow boundary)

### Alternatives Considered

| Alternative | Rejected Because |
|-------------|------------------|
| iframe | Cross-origin restrictions limit communication; feels "foreign" |
| CSS namespacing (BEM/scoped) | Insufficient isolation; host !important rules still bleed |
| Direct DOM injection | CSS/JS conflicts cause layout bugs |

---

## ADR-004: Quantized LLMs for Local Inference

### Status
**Accepted**

### Context
Resume parsing requires semantic understanding (not just regex). Cloud LLM APIs (GPT-4o: $5/1M tokens) are expensive at scale. On-device inference is possible with small language models (SLMs).

### Decision
**Use 4-bit quantized models (Phi-3-mini or Llama-3.2-1B) via transformers.js with WebGPU acceleration.**

### Model Evaluation

| Model | Size (q4) | Accuracy | Latency |
|-------|-----------|----------|---------|
| Phi-3-mini (1.3B) | ~1.5GB | Good | Fast |
| Llama-3.2-1B | ~1GB | Acceptable | Very Fast |
| Llama-3.2-3B | ~2.5GB | Better | Moderate |
| Llama-3-8B | ~4GB | Best | Slow |

### Consequences

**Positive:**
- Zero inference cost after model download
- Privacy (no data sent to cloud)
- Competitive accuracy (~95%) with HITL fallback
- WebGPU provides 5-10x speedup vs CPU

**Negative:**
- Large initial download (1-4GB)
- WebGPU support varies (Chrome best, Firefox/Safari lagging)
- Smaller models have accuracy limits for complex layouts
- GPU memory contention with other browser tabs

### Fallback Strategy
1. Try WebGPU inference
2. Fall back to WASM SIMD (CPU)
3. Degrade to regex extraction if both fail

---

## ADR-005: Visual DOM Read + API Write for ATS Bypass

### Status
**Accepted**

### Context
Salesforce Lightning Locker and Bullhorn's CSP restrict third-party script execution and DOM manipulation. Traditional extension techniques (hooking into React components, simulating clicks) are blocked.

### Decision
**Bypass ATS security by reading data visually (DOM scraping/OCR) and writing via public REST APIs.**

### Implementation

```
READ: Extension → DOM Scrape → Visual Text Extraction
WRITE: Extension → OAuth Token → REST API → ATS Database
```

### Consequences

**Positive:**
- Completely bypasses Lightning Locker restrictions
- Works with any ATS that has REST API (not just Bullhorn/Salesforce)
- More reliable than click simulation
- Official API access improves compliance posture

**Negative:**
- Requires users to authenticate with ATS (OAuth flow)
- DOM selectors may break on ATS updates (graceful degradation needed)
- API rate limits apply
- Some ATS features may not be exposed via API

---

## ADR-006: Universal ID System

### Status
**Accepted**

### Context
Recruiters encounter the same candidate across multiple platforms (LinkedIn, Indeed, email). Duplicate detection requires a stable identifier that works without server-side deduplication.

### Decision
**Hash candidate identity from deterministic fields: SHA-256(lowercase(name) + linkedinUrl + primaryEmail)**

### Consequences

**Positive:**
- Works offline (no server lookup)
- Privacy-preserving (hash not reversible)
- Consistent across devices if fields match
- Fast lookup in local IndexedDB

**Negative:**
- Collisions if name/email differs slightly (fuzzy matching needed)
- LinkedIn URL changes break linkage (rare)
- No central deduplication across agencies

### Collision Mitigation
- Normalize names (trim, lowercase, remove accents)
- Use canonical LinkedIn URL format
- Store alternate hashes for fuzzy matching

---

## ADR-007: Product-Led Growth Distribution

### Status
**Accepted**

### Context
Incumbents use Sales-Led Growth (SLG) with enterprise contracts, demos, and long sales cycles. This model requires significant upfront capital and sales headcount.

### Decision
**Distribute via Chrome Web Store with freemium model.** Free tier = unlimited parsing. Pro tier ($29/mo) = ATS sync, team templates.

### Rationale
- Chrome Web Store bypasses IT procurement
- Individual recruiter can install without approval
- Viral spread within agencies ("tool that makes you 10x faster")
- Pro conversion happens organically when team needs compliance/sync

### Consequences

**Positive:**
- Near-zero customer acquisition cost
- Rapid iteration based on user feedback
- Network effects as teams adopt
- Bottom-up enterprise penetration

**Negative:**
- No guaranteed revenue from free users
- Support burden without revenue (limit scope of free tier)
- Slower enterprise adoption vs. direct sales
- Vulnerable to copycat extensions

---

## ADR-008: IndexedDB with Client-Side Encryption

### Status
**Accepted**

### Context
Candidate data must persist locally between sessions. Browser storage options include localStorage (5MB limit, synchronous), IndexedDB (unlimited, async), and Cache API (for static assets).

### Decision
**Use IndexedDB with AES-256-GCM encryption via Web Crypto API (SubtleCrypto).**

### Schema

```javascript
const stores = {
  candidates: { keyPath: 'universalId', indexes: ['name', 'email', 'ttlExpiry'] },
  templates: { keyPath: 'id' },
  sequences: { keyPath: 'id' },
  settings: { keyPath: 'key' }
};
```

### Encryption Flow
1. Generate 256-bit key on first run
2. Store key in chrome.storage.session (cleared on browser close) or derive from user passphrase
3. Encrypt all candidate records before write
4. Decrypt on read

### Consequences

**Positive:**
- Unlimited storage capacity
- Async API doesn't block UI
- Encryption protects data at rest
- GDPR-compliant with TTL-based deletion

**Negative:**
- More complex than localStorage
- Key management UX (what if user clears browser data?)
- Encryption adds latency (~5-10ms per record)

---

## ADR-009: WebRTC for P2P Team Sync (Phase 4)

### Status
**Proposed** (Phase 4)

### Context
Team collaboration requires sharing candidate pools without a central server. Traditional sync requires running backend infrastructure with ongoing costs.

### Decision
**Use WebRTC DataChannel for peer-to-peer sync between team members.**

### Consequences

**Positive:**
- No server infrastructure cost
- Real-time sync with low latency
- Works across firewalls (STUN/TURN)
- Decentralized = no single point of failure

**Negative:**
- Complex connection establishment
- Requires signaling server for initial peer discovery
- NAT traversal may fail in some corporate networks
- Conflict resolution is complex without central authority

### Alternative for Consideration
- Cloudflare Durable Objects (low-cost serverless state)
- CRDTs for conflict-free sync

---

## Decision Log

| ADR | Title | Status | Date |
|-----|-------|--------|------|
| 001 | Edge-First Architecture | Accepted | 2025-12-08 |
| 002 | Rust + WASM Compute | Accepted | 2025-12-08 |
| 003 | Shadow DOM Isolation | Accepted | 2025-12-08 |
| 004 | Quantized Local LLMs | Accepted | 2025-12-08 |
| 005 | Visual Read + API Write | Accepted | 2025-12-08 |
| 006 | Universal ID System | Accepted | 2025-12-08 |
| 007 | PLG Distribution | Accepted | 2025-12-08 |
| 008 | IndexedDB + Encryption | Accepted | 2025-12-08 |
| 009 | WebRTC P2P Sync | Proposed | 2025-12-08 |

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
