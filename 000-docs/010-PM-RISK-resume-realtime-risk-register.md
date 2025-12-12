# Risk Register: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Active
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md
**Review Cadence:** Weekly (High), Bi-weekly (Medium), Monthly (Low)

---

## Risk Management Framework

### Probability Scale

| Level | Probability | Description |
|-------|-------------|-------------|
| 1 | < 10% | Rare - Unlikely to occur |
| 2 | 10-25% | Unlikely - Could occur in exceptional circumstances |
| 3 | 25-50% | Possible - Might occur at some point |
| 4 | 50-75% | Likely - Will probably occur |
| 5 | > 75% | Almost Certain - Expected to occur |

### Impact Scale

| Level | Impact | Description |
|-------|--------|-------------|
| 1 | Negligible | < $5K cost, no schedule impact |
| 2 | Minor | $5-25K cost, < 1 week delay |
| 3 | Moderate | $25-100K cost, 1-4 week delay |
| 4 | Major | $100-500K cost, 1-3 month delay |
| 5 | Critical | > $500K cost, > 3 month delay, project failure |

### Risk Score Matrix

```
Risk Score = Probability × Impact

Score 1-4:   LOW (Green)    - Monitor
Score 5-9:   MEDIUM (Yellow) - Mitigate
Score 10-14: HIGH (Orange)   - Active Management
Score 15-25: CRITICAL (Red)  - Immediate Action
```

---

## High-Priority Risks (Score 15-25)

### RISK-001: WebGPU Browser Compatibility Failure

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-001 |
| **Category** | Technical |
| **Probability** | 4 (Likely) |
| **Impact** | 4 (Major) |
| **Score** | 16 (CRITICAL) |
| **Owner** | Tech Lead |
| **Status** | Open |

**Description:**
WebGPU is required for performant local LLM inference. Safari lacks full WebGPU support, and Firefox support is experimental. If WebGPU adoption stalls, inference latency on non-Chrome browsers becomes unacceptable (>60s).

**Triggers:**
- Safari WebGPU support not shipped by Q2 2025
- Firefox WebGPU remains behind flag
- >30% of target users on non-Chrome browsers

**Mitigation Strategy:**
1. Implement WASM SIMD CPU fallback (slower but functional)
2. Use smaller models (Phi-3-mini 1.3B) for CPU inference
3. Display browser upgrade prompt for optimal experience
4. Track browser distribution in analytics to inform priority

**Contingency Plan:**
- Hybrid mode: Offer optional cloud inference for non-WebGPU browsers
- Partner with WebLLM project for polyfill improvements

**Review Date:** Weekly

---

### RISK-002: LinkedIn DOM Structure Changes

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-002 |
| **Category** | Technical |
| **Probability** | 5 (Almost Certain) |
| **Impact** | 3 (Moderate) |
| **Score** | 15 (CRITICAL) |
| **Owner** | Frontend Lead |
| **Status** | Open |

**Description:**
The overlay relies on DOM scraping to extract LinkedIn profile data. LinkedIn regularly updates their frontend, breaking selectors. Each breakage causes loss of "Candidate Exists" functionality until patched.

**Triggers:**
- LinkedIn deploys new profile page layout
- React component class names change
- LinkedIn implements anti-scraping measures

**Mitigation Strategy:**
1. Use resilient selectors (data attributes, ARIA labels) over class names
2. Implement multiple fallback selector strategies
3. Add automated DOM structure monitoring (daily)
4. Build 24-hour hotfix deployment pipeline
5. Cache last-known-good selectors

**Contingency Plan:**
- Graceful degradation: Show "Profile sync unavailable" without crashing
- Community-sourced selector updates via GitHub issues

**Review Date:** Weekly

---

### RISK-003: ATS API Access Revocation

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-003 |
| **Category** | Business |
| **Probability** | 3 (Possible) |
| **Impact** | 5 (Critical) |
| **Score** | 15 (CRITICAL) |
| **Owner** | Product Owner |
| **Status** | Open |

**Description:**
Bullhorn or Salesforce could revoke API access or reject marketplace listing, eliminating core Phase 3 functionality (ATS sync). This would remove primary monetization driver.

**Triggers:**
- Marketplace application rejected
- Terms of service violation claim
- Competitor pressure on ATS vendors

**Mitigation Strategy:**
1. Apply for official marketplace partnership early (Phase 2)
2. Ensure strict compliance with API terms of service
3. Avoid features that compete directly with ATS core functionality
4. Build relationships with ATS developer relations teams
5. Document all API usage for audit

**Contingency Plan:**
- CSV export/import as manual sync fallback
- Support alternative ATS platforms (Greenhouse, Lever)
- Position as "productivity layer" not "ATS replacement"

**Review Date:** Bi-weekly

---

## Medium-Priority Risks (Score 10-14)

### RISK-004: Quantized Model Accuracy Insufficient

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-004 |
| **Category** | Technical |
| **Probability** | 3 (Possible) |
| **Impact** | 4 (Major) |
| **Score** | 12 (HIGH) |
| **Owner** | ML Engineer |
| **Status** | Open |

**Description:**
4-bit quantized models (Phi-3, Llama-3.2) may not achieve 95% parsing accuracy target on complex resume layouts (multi-column, tables, graphics).

**Mitigation Strategy:**
1. Benchmark 10+ model variants on 500-resume test set
2. Implement HITL correction to compensate for errors
3. Use ensemble approach (multiple model passes)
4. Fine-tune with LoRA on recruiter-corrected data

**Contingency Plan:**
- Offer "High Accuracy Mode" with larger model (8B, slower)
- Hybrid: Use cloud API for flagged low-confidence parses

---

### RISK-005: Chrome Extension Policy Rejection

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-005 |
| **Category** | Business |
| **Probability** | 2 (Unlikely) |
| **Impact** | 5 (Critical) |
| **Score** | 10 (HIGH) |
| **Owner** | Product Owner |
| **Status** | Open |

**Description:**
Chrome Web Store could reject or remove extension for policy violations (e.g., data collection, DOM manipulation, permissions scope).

**Mitigation Strategy:**
1. Minimize requested permissions to absolute necessity
2. Clear privacy policy explaining local-only processing
3. No remote code execution
4. Pre-submission review against Chrome policies

**Contingency Plan:**
- Self-hosted distribution via .crx file
- Firefox Add-ons as primary distribution
- Progressive Web App fallback (limited functionality)

---

### RISK-006: Model Size Exceeds Browser Cache Limits

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-006 |
| **Category** | Technical |
| **Probability** | 3 (Possible) |
| **Impact** | 3 (Moderate) |
| **Score** | 9 (MEDIUM) |
| **Owner** | Tech Lead |
| **Status** | Open |

**Description:**
Quantized LLMs (2-4GB) may exceed browser storage quotas or cause eviction, requiring re-download on each session.

**Mitigation Strategy:**
1. Use Cache API with persistent storage request
2. Implement chunked download with resume capability
3. Offer "Lite Mode" with smaller model (<1GB)
4. Store in IndexedDB as blob for persistence

**Contingency Plan:**
- Native companion app for model storage
- Cloud-hosted model as fallback (breaks privacy guarantee)

---

### RISK-007: GDPR/Privacy Regulatory Challenge

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-007 |
| **Category** | Legal |
| **Probability** | 2 (Unlikely) |
| **Impact** | 4 (Major) |
| **Score** | 8 (MEDIUM) |
| **Owner** | Legal/Compliance |
| **Status** | Open |

**Description:**
Despite local-only architecture, regulators may challenge GDPR compliance (e.g., arguing extension code is "processing" even without data transmission).

**Mitigation Strategy:**
1. Legal review of architecture vs GDPR definitions
2. Explicit user consent flow on first run
3. Data Processing Agreement template for enterprise
4. Third-party privacy audit and certification

**Contingency Plan:**
- Engage GDPR specialist counsel
- Modify architecture if required by ruling

---

## Lower-Priority Risks (Score 5-9)

### RISK-008: Key Personnel Departure

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-008 |
| **Category** | Organizational |
| **Probability** | 2 (Unlikely) |
| **Impact** | 3 (Moderate) |
| **Score** | 6 (MEDIUM) |
| **Owner** | Project Manager |
| **Status** | Open |

**Description:**
Loss of Rust/WASM specialist or ML engineer could delay development significantly given specialized skill requirements.

**Mitigation Strategy:**
1. Comprehensive documentation of all systems
2. Pair programming to spread knowledge
3. Identify backup contractors with Rust/WASM experience
4. Competitive retention packages for key roles

---

### RISK-009: Competitor Fast-Follow

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-009 |
| **Category** | Business |
| **Probability** | 4 (Likely) |
| **Impact** | 2 (Minor) |
| **Score** | 8 (MEDIUM) |
| **Owner** | Product Owner |
| **Status** | Monitoring |

**Description:**
Incumbents (DaXtra, RChilli) or well-funded startups could copy edge-compute approach once proven viable.

**Mitigation Strategy:**
1. Move fast - first-mover advantage in PLG
2. Build network effects in Phase 4 (P2P sync, crowdsourced data)
3. Continuous feature velocity
4. Community/brand building around "local-first" message

---

### RISK-010: OSINT Legal Liability

| Attribute | Value |
|-----------|-------|
| **ID** | RISK-010 |
| **Category** | Legal |
| **Probability** | 2 (Unlikely) |
| **Impact** | 3 (Moderate) |
| **Score** | 6 (MEDIUM) |
| **Owner** | Legal/Compliance |
| **Status** | Open |

**Description:**
GitHub email scraping or public data aggregation could violate terms of service or data protection laws.

**Mitigation Strategy:**
1. Legal review of all OSINT sources
2. Only use publicly available data
3. Respect robots.txt and rate limits
4. Implement opt-out mechanism

---

## Risk Monitoring Dashboard

### Current Risk Summary

| Priority | Count | Action Required |
|----------|-------|-----------------|
| Critical (15-25) | 3 | Immediate escalation |
| High (10-14) | 3 | Active mitigation |
| Medium (5-9) | 4 | Monitor and mitigate |
| Low (1-4) | 0 | Accept |

### Key Risk Indicators (KRIs)

| Indicator | Threshold | Current | Status |
|-----------|-----------|---------|--------|
| WebGPU browser coverage | > 70% | ~65% | Yellow |
| LinkedIn selector stability | < 1 break/month | N/A | Green |
| Model accuracy (test set) | > 95% | TBD | Gray |
| Chrome store approval | Approved | Pending | Yellow |
| Legal review complete | Done | In Progress | Yellow |

---

## Escalation Matrix

| Risk Score | Escalation Level | Response Time |
|------------|------------------|---------------|
| 15-25 | Executive/Founder | < 4 hours |
| 10-14 | Project Manager | < 24 hours |
| 5-9 | Team Lead | < 1 week |
| 1-4 | Team Member | Next sprint |

---

## Review History

| Date | Reviewer | Changes |
|------|----------|---------|
| 2025-12-08 | Initial | Document created with 10 risks identified |

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
