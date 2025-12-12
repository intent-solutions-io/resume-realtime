# Implementation Tasks: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Active
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md

---

## Relevant Files

- `src/lib.rs` - Core WASM module entry point; Rust parsing logic
- `src/parser/mod.rs` - Resume parsing module (PDF extraction + LLM inference)
- `src/parser/extractor.rs` - Field extraction and JSON structuring
- `src/parser/extractor.test.rs` - Unit tests for field extraction
- `src/inference/mod.rs` - Local LLM inference wrapper (WebGPU + candle)
- `src/inference/mod.test.rs` - Unit tests for inference module
- `src/crypto/mod.rs` - Encryption utilities (AES-256, SHA-256 hashing)
- `src/crypto/mod.test.rs` - Unit tests for crypto module
- `extension/manifest.json` - Chrome extension manifest (V3)
- `extension/src/background.ts` - Service worker for extension lifecycle
- `extension/src/content/overlay.tsx` - Shadow DOM overlay component
- `extension/src/content/overlay.test.tsx` - Unit tests for overlay
- `extension/src/components/Parser.tsx` - HITL parsing interface
- `extension/src/components/Parser.test.tsx` - Unit tests for parser UI
- `extension/src/components/Badge.tsx` - "Candidate Exists" badge component
- `extension/src/hooks/useATS.ts` - Bullhorn/Salesforce API hooks
- `extension/src/hooks/useATS.test.ts` - Unit tests for ATS hooks
- `extension/src/storage/indexeddb.ts` - Encrypted IndexedDB wrapper
- `extension/src/storage/indexeddb.test.ts` - Unit tests for storage
- `extension/plasmo.config.ts` - Plasmo build configuration

### Notes

- Unit tests should be placed alongside the code files they test
- Run Rust tests: `cargo test --features ssr`
- Run TypeScript tests: `npm test` or `npx jest`
- Build WASM module: `wasm-pack build --target web`

---

## Instructions for Completing Tasks

As you complete each task, check it off by changing `- [ ]` to `- [x]`. Update the file after completing each sub-task, not just after completing an entire parent task.

Example:
- `- [ ] 1.1 Read file` → `- [x] 1.1 Read file` (after completing)

---

## Tasks

### Phase 1: MVP - Parser in a Box

- [ ] 0.0 Create feature branch
  - [ ] 0.1 Create and checkout a new branch for this feature: `git checkout -b feature/resume-realtime`

- [ ] 1.0 Set up extension scaffold with Plasmo
  - [ ] 1.1 Initialize Plasmo project: `npm create plasmo@latest resume-realtime`
  - [ ] 1.2 Configure manifest.json for Manifest V3 with required permissions (storage, activeTab, scripting)
  - [ ] 1.3 Set up TypeScript + React project structure
  - [ ] 1.4 Add Tailwind CSS for styling
  - [ ] 1.5 Create basic popup.tsx and options.tsx entry points
  - [ ] 1.6 Verify extension loads in Chrome with `plasmo dev`

- [ ] 2.0 Implement WASM parsing module (Rust)
  - [ ] 2.1 Create new Rust crate for WASM: `cargo new --lib resume-parser`
  - [ ] 2.2 Configure Cargo.toml for wasm32-unknown-unknown target with wasm-bindgen
  - [ ] 2.3 Add pdf-extract or pdf-rs dependency for PDF text extraction
  - [ ] 2.4 Implement `parse_pdf(bytes: &[u8]) -> Result<String, JsValue>` function
  - [ ] 2.5 Add wasm-pack build script to package.json
  - [ ] 2.6 Write unit tests for PDF text extraction
  - [ ] 2.7 Build and verify WASM module loads in browser

- [ ] 3.0 Integrate local LLM inference
  - [ ] 3.1 Evaluate transformers.js vs ONNX Runtime Web for WebGPU support
  - [ ] 3.2 Download and convert Phi-3-mini to ONNX format (q4 quantization)
  - [ ] 3.3 Create inference wrapper in TypeScript that loads model from cache
  - [ ] 3.4 Implement progressive model loading with download progress UI
  - [ ] 3.5 Create prompt template for resume field extraction
  - [ ] 3.6 Implement `extractFields(text: string) -> ResumeJSON` function
  - [ ] 3.7 Add WebGPU detection with CPU fallback
  - [ ] 3.8 Write integration tests for inference pipeline

- [ ] 4.0 Build HITL parsing interface
  - [ ] 4.1 Create split-pane layout component (PDF left, fields right)
  - [ ] 4.2 Integrate pdf.js viewer with page navigation
  - [ ] 4.3 Build editable field components for each resume section
  - [ ] 4.4 Implement click-to-highlight: clicking field scrolls PDF to source
  - [ ] 4.5 Implement highlight-to-correct: selecting PDF text updates field
  - [ ] 4.6 Add confidence score visual indicators per field
  - [ ] 4.7 Create drag-and-drop zone for PDF upload
  - [ ] 4.8 Write component tests for HITL interface

- [ ] 5.0 Implement encrypted local storage
  - [ ] 5.1 Create IndexedDB wrapper with typed schemas
  - [ ] 5.2 Implement AES-256-GCM encryption using SubtleCrypto
  - [ ] 5.3 Generate and securely store encryption key on first run
  - [ ] 5.4 Create CRUD operations for candidate records
  - [ ] 5.5 Implement Universal ID hashing (SHA-256 of name+linkedinUrl+email)
  - [ ] 5.6 Add data TTL tracking for GDPR compliance
  - [ ] 5.7 Write unit tests for storage layer

- [ ] 6.0 Phase 1 integration and testing
  - [ ] 6.1 Wire up drag-drop → parse → display flow end-to-end
  - [ ] 6.2 Test with 20 diverse resume formats (PDF, various layouts)
  - [ ] 6.3 Measure and optimize parse latency (target < 5s cached)
  - [ ] 6.4 Fix any memory leaks in WASM module
  - [ ] 6.5 Create Phase 1 AAR document

---

### Phase 2: Overlay - Context Hijacker

- [ ] 7.0 Implement Shadow DOM injection system
  - [ ] 7.1 Create content script that detects LinkedIn/Gmail URLs
  - [ ] 7.2 Implement Shadow DOM container creation with style isolation
  - [ ] 7.3 Mount React app inside Shadow Root
  - [ ] 7.4 Add CSS reset inside shadow container to prevent bleed
  - [ ] 7.5 Create collapsible sidebar component with drag handle
  - [ ] 7.6 Implement keyboard shortcut (Ctrl+Shift+S) to toggle sidebar
  - [ ] 7.7 Test isolation on LinkedIn, Gmail, Bullhorn, Salesforce

- [ ] 8.0 Build LinkedIn profile scraper
  - [ ] 8.1 Create DOM selectors for LinkedIn profile elements
  - [ ] 8.2 Extract: name, headline, current company, location, profile URL
  - [ ] 8.3 Handle LinkedIn's dynamic loading (MutationObserver)
  - [ ] 8.4 Compute Universal ID hash from extracted data
  - [ ] 8.5 Query IndexedDB for existing candidate match
  - [ ] 8.6 Display "Candidate Exists" badge with status/last contact
  - [ ] 8.7 Add error handling for selector changes (graceful degradation)

- [ ] 9.0 Implement OSINT enrichment
  - [ ] 9.1 Create GitHub email finder (query public commits by username)
  - [ ] 9.2 Create personal website email extractor (parse mailto links)
  - [ ] 9.3 Implement Clearbit-style email pattern guessing (first.last@company.com)
  - [ ] 9.4 Build BYOK settings UI for Lusha/Hunter/RocketReach API keys
  - [ ] 9.5 Create unified enrichment pipeline: OSINT first, then paid APIs
  - [ ] 9.6 Store enrichment results in IndexedDB
  - [ ] 9.7 Write tests for enrichment functions

- [ ] 10.0 Build Gmail integration
  - [ ] 10.1 Create content script for Gmail compose detection
  - [ ] 10.2 Inject "Send Later" button into compose toolbar
  - [ ] 10.3 Inject "Add to Sequence" button
  - [ ] 10.4 Implement Chrome alarms API scheduler for delayed send
  - [ ] 10.5 Create email sequence editor in sidebar
  - [ ] 10.6 Implement reply detection (stop sequence on reply)
  - [ ] 10.7 Test scheduling across browser restarts

- [ ] 11.0 Phase 2 integration and testing
  - [ ] 11.1 Test overlay on 10 LinkedIn profile variations
  - [ ] 11.2 Test Gmail injection on various inbox views
  - [ ] 11.3 Verify OSINT enrichment accuracy
  - [ ] 11.4 Create Phase 2 AAR document

---

### Phase 3: Integrator - ATS Killer

- [ ] 12.0 Implement Bullhorn OAuth integration
  - [ ] 12.1 Register extension as Bullhorn marketplace app
  - [ ] 12.2 Implement OAuth 2.0 authorization code flow
  - [ ] 12.3 Store and refresh access tokens securely
  - [ ] 12.4 Create Bullhorn REST API client wrapper
  - [ ] 12.5 Implement candidate search by Universal ID
  - [ ] 12.6 Implement candidate create/update operations
  - [ ] 12.7 Write integration tests with Bullhorn sandbox

- [ ] 13.0 Implement Salesforce OAuth integration
  - [ ] 13.1 Register extension as Salesforce connected app
  - [ ] 13.2 Implement OAuth 2.0 with PKCE flow
  - [ ] 13.3 Handle Lightning Locker restrictions (visual read + API write)
  - [ ] 13.4 Create Salesforce REST API client wrapper
  - [ ] 13.5 Map resume fields to Salesforce Contact/Candidate object
  - [ ] 13.6 Implement candidate sync operations
  - [ ] 13.7 Write integration tests with Salesforce sandbox

- [ ] 14.0 Build "Save to ATS" workflow
  - [ ] 14.1 Add "Save to ATS" button to LinkedIn overlay
  - [ ] 14.2 Create ATS selection modal (Bullhorn vs Salesforce)
  - [ ] 14.3 Implement field mapping UI for custom ATS fields
  - [ ] 14.4 Show sync status indicator on LinkedIn profiles
  - [ ] 14.5 Handle sync conflicts (local vs remote precedence)
  - [ ] 14.6 Add bulk save option for search result pages

- [ ] 15.0 Build PDF formatting engine
  - [ ] 15.1 Integrate pdf-lib for client-side PDF generation
  - [ ] 15.2 Create template data structure (header, fonts, colors, logo position)
  - [ ] 15.3 Build template editor in settings UI
  - [ ] 15.4 Implement resume-to-PDF rendering from JSON
  - [ ] 15.5 Add logo injection with configurable placement
  - [ ] 15.6 Implement one-click anonymization (redact name, email, phone)
  - [ ] 15.7 Create download and preview functionality
  - [ ] 15.8 Write tests for PDF generation

- [ ] 16.0 Implement GDPR compliance features
  - [ ] 16.1 Add TTL configuration in settings (default 6 months)
  - [ ] 16.2 Create background job to check TTL expiry daily
  - [ ] 16.3 Generate "GDPR Expiry Warning" notifications
  - [ ] 16.4 Implement local data deletion with confirmation
  - [ ] 16.5 Implement ATS delete API calls on scrub action
  - [ ] 16.6 Add audit log for data deletion events
  - [ ] 16.7 Create compliance dashboard in settings

- [ ] 17.0 Implement Pro tier features and paywall
  - [ ] 17.1 Integrate Stripe for payment processing
  - [ ] 17.2 Create Pro tier feature gates (ATS sync, team templates)
  - [ ] 17.3 Build upgrade modal with pricing
  - [ ] 17.4 Implement license validation on extension load
  - [ ] 17.5 Add trial period logic (14 days free Pro)
  - [ ] 17.6 Create customer portal link for subscription management

- [ ] 18.0 Phase 3 integration and testing
  - [ ] 18.1 End-to-end test: LinkedIn → parse → format → save to Bullhorn
  - [ ] 18.2 End-to-end test: same flow with Salesforce
  - [ ] 18.3 Test GDPR scrub flow
  - [ ] 18.4 Test Pro tier payment and access control
  - [ ] 18.5 Performance audit (memory, latency)
  - [ ] 18.6 Create Phase 3 AAR document

---

### Phase 4: Network - The Moat

- [ ] 19.0 Implement P2P team sync
  - [ ] 19.1 Integrate peerjs or raw WebRTC for peer discovery
  - [ ] 19.2 Design sync protocol for candidate pool sharing
  - [ ] 19.3 Implement conflict resolution for concurrent edits
  - [ ] 19.4 Add team invitation flow (share link or QR code)
  - [ ] 19.5 Create team management UI in settings
  - [ ] 19.6 Implement shared template sync across team
  - [ ] 19.7 Add offline-first sync queue with retry logic

- [ ] 20.0 Implement differential privacy aggregation
  - [ ] 20.1 Research differential privacy libraries for browser
  - [ ] 20.2 Design aggregation schema for salary data
  - [ ] 20.3 Implement local noise injection for privacy
  - [ ] 20.4 Build anonymized insights dashboard
  - [ ] 20.5 Create opt-in consent flow for data contribution
  - [ ] 20.6 Implement "Give-to-Get" credit system

- [ ] 21.0 Phase 4 integration and testing
  - [ ] 21.1 Test P2P sync with 5+ simultaneous users
  - [ ] 21.2 Verify privacy guarantees with audit
  - [ ] 21.3 Load test aggregation with simulated data
  - [ ] 21.4 Create Phase 4 AAR document

---

## Verification Checklist

- [ ] All Phase 1 unit tests passing
- [ ] All Phase 2 unit tests passing
- [ ] All Phase 3 integration tests passing
- [ ] Chrome Web Store listing approved
- [ ] Firefox Add-ons listing approved (stretch)
- [ ] Privacy policy and terms of service published
- [ ] Documentation site live
- [ ] 1,000 active users achieved

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
