# Release Plan: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Draft
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md

---

## Release Overview

### Release Channels

| Channel | Purpose | Audience | Update Frequency |
|---------|---------|----------|------------------|
| **Alpha** | Internal testing | Dev team only | Daily |
| **Beta** | Early adopter feedback | Invited testers (100) | Weekly |
| **Production** | General availability | All users | Bi-weekly |

### Version Strategy

**Semantic Versioning:** `MAJOR.MINOR.PATCH`

| Component | Trigger |
|-----------|---------|
| MAJOR | Breaking changes, major feature |
| MINOR | New features, non-breaking |
| PATCH | Bug fixes, security patches |

**Phase Versions:**

| Phase | Target Version | Release Date |
|-------|---------------|--------------|
| Phase 1 MVP | v0.1.0 | Month 3 |
| Phase 2 Overlay | v0.2.0 | Month 6 |
| Phase 3 ATS Integration | v1.0.0 | Month 12 |
| Phase 4 Network | v2.0.0 | Year 2 |

---

## Phase 1 Release: MVP (v0.1.0)

### Release Scope

| Feature | Status | Priority |
|---------|--------|----------|
| Drag-drop PDF upload | Required | P0 |
| Local LLM parsing | Required | P0 |
| HITL field correction | Required | P0 |
| Encrypted storage | Required | P0 |
| Settings UI | Required | P1 |
| Chrome Web Store listing | Required | P0 |

### Release Timeline

| Milestone | Date | Owner | Exit Criteria |
|-----------|------|-------|---------------|
| Feature freeze | M3-W1 | Tech Lead | All P0 features complete |
| Alpha release | M3-W2 | QA Lead | Internal testing pass |
| Beta release | M3-W3 | Product | 100 beta testers onboarded |
| Release candidate | M3-W4 | Tech Lead | Zero P0/P1 bugs |
| Chrome store submission | M3-W4 | Product | Assets and listing complete |
| Production release | M3-W5 | Product | Store approval received |

### Quality Gates

| Gate | Criteria | Blocker |
|------|----------|---------|
| Code complete | All features merged to main | Yes |
| Unit tests | 100% pass, >80% coverage | Yes |
| Integration tests | 100% pass | Yes |
| E2E tests | 100% P0 flows pass | Yes |
| Security scan | 0 critical, 0 high | Yes |
| Performance | <5s parse latency (P95) | Yes |
| Accessibility | WCAG 2.1 AA compliance | Yes |
| Privacy review | Legal sign-off | Yes |

### Deployment Process

#### Pre-Release Checklist

- [ ] All PRs merged and CI green
- [ ] Version bumped in manifest.json and package.json
- [ ] Changelog updated
- [ ] Privacy policy published
- [ ] Terms of service published
- [ ] Extension assets created (icons, screenshots)
- [ ] Chrome Web Store listing drafted
- [ ] Rollback plan documented

#### Deployment Steps

```bash
# 1. Create release branch
git checkout -b release/v0.1.0

# 2. Bump version
npm version 0.1.0

# 3. Build production
npm run build:prod

# 4. Create Chrome Web Store package
npm run package:chrome

# 5. Upload to Chrome Web Store Developer Dashboard
# Manual step via https://chrome.google.com/webstore/devconsole

# 6. Submit for review
# Manual step - expect 1-3 business days

# 7. Tag release after approval
git tag v0.1.0
git push origin v0.1.0
```

### Rollback Plan

**Trigger Conditions:**
- Error rate > 5% in first 24 hours
- Critical security vulnerability discovered
- Crash rate > 1%

**Rollback Steps:**
1. Unpublish from Chrome Web Store (immediate)
2. Notify beta users via email
3. Investigate root cause
4. Hotfix or full rollback
5. Re-submit when resolved

---

## Phase 2 Release: Overlay (v0.2.0)

### Release Scope

| Feature | Status | Priority |
|---------|--------|----------|
| LinkedIn profile detection | Required | P0 |
| Shadow DOM overlay | Required | P0 |
| "Candidate Exists" badge | Required | P1 |
| One-click profile capture | Required | P1 |
| OSINT email enrichment | Required | P2 |
| Gmail "Send Later" | Required | P2 |
| BYOK settings | Required | P2 |

### Progressive Rollout

| Stage | % Users | Duration | Criteria to Advance |
|-------|---------|----------|---------------------|
| Canary | 1% | 24 hours | No critical errors |
| Limited | 10% | 3 days | Error rate <1% |
| Expanded | 50% | 7 days | NPS > 30 |
| Full | 100% | - | Stable metrics |

### Feature Flags

```json
{
  "linkedin_overlay": {
    "enabled": true,
    "rollout_percentage": 100
  },
  "osint_enrichment": {
    "enabled": true,
    "rollout_percentage": 50
  },
  "gmail_send_later": {
    "enabled": true,
    "rollout_percentage": 25
  }
}
```

---

## Phase 3 Release: ATS Integration (v1.0.0)

### Release Scope

| Feature | Status | Priority |
|---------|--------|----------|
| Bullhorn OAuth | Required | P0 |
| Salesforce OAuth | Required | P0 |
| Save to ATS | Required | P0 |
| PDF formatting | Required | P1 |
| Resume anonymization | Required | P1 |
| GDPR expiry alerts | Required | P2 |
| Pro tier paywall | Required | P0 |
| Stripe integration | Required | P0 |

### Major Version Considerations

**v1.0.0 Significance:**
- First "production-ready" release
- Pro tier monetization begins
- ATS marketplace listings (Bullhorn/Salesforce)
- Marketing campaign launch

### Enterprise Readiness Checklist

- [ ] SOC 2 Type I assessment started
- [ ] Enterprise support documentation
- [ ] Team admin features
- [ ] Audit logging
- [ ] Data export functionality
- [ ] SLA commitments published

---

## Phase 4 Release: Network (v2.0.0)

### Release Scope

| Feature | Status | Priority |
|---------|--------|----------|
| P2P team sync | Required | P0 |
| Shared templates | Required | P1 |
| Differential privacy aggregation | Required | P2 |
| Market insights dashboard | Required | P2 |
| Give-to-Get enrichment | Stretch | P3 |

### Breaking Changes

| Change | Migration Path |
|--------|---------------|
| Storage schema v2 | Auto-migration on update |
| API version upgrade | Deprecate v1 after 6 months |

---

## Release Communication

### Stakeholder Matrix

| Stakeholder | Communication | Timing |
|-------------|---------------|--------|
| Dev team | Slack + standup | Continuous |
| Beta testers | Email + in-app | T-7 days |
| All users | In-app notification | Release day |
| Press/influencers | Email embargo | T-1 day |
| Support team | Training session | T-3 days |

### Release Notes Template

```markdown
# Resume Realtime v0.X.0

## What's New
- Feature 1: Description
- Feature 2: Description

## Improvements
- Improvement 1
- Improvement 2

## Bug Fixes
- Fix 1
- Fix 2

## Known Issues
- Issue 1: Workaround

## Upgrade Notes
- Any manual steps required
```

### Communication Timeline

| Time | Action |
|------|--------|
| T-14 days | Internal announcement |
| T-7 days | Beta tester preview |
| T-3 days | Support team training |
| T-1 day | Press embargo lift |
| Release day | Public announcement |
| T+1 day | Monitor and respond |
| T+7 days | Post-release review |

---

## Monitoring & Observability

### Release Health Metrics

| Metric | Source | Alert Threshold |
|--------|--------|-----------------|
| Error rate | Sentry | > 1% |
| Crash-free rate | Chrome dashboard | < 99% |
| Parse latency P95 | Custom telemetry | > 10s |
| Daily active users | Analytics | < previous day |
| Install/uninstall ratio | Chrome dashboard | < 5:1 |
| NPS score | In-app survey | < 30 |

### Alerting

```yaml
# alerts.yaml
- name: high_error_rate
  condition: error_rate > 0.01
  severity: critical
  channels: [slack, pagerduty]

- name: parse_latency_regression
  condition: p95_latency > 10000
  severity: warning
  channels: [slack]

- name: uninstall_spike
  condition: uninstalls > 2x_daily_avg
  severity: warning
  channels: [slack, email]
```

### Post-Release Validation

**24-Hour Checklist:**
- [ ] Error rate within SLA
- [ ] No critical user reports
- [ ] Performance metrics stable
- [ ] Revenue/conversion tracking (if applicable)

**7-Day Review:**
- [ ] User feedback analyzed
- [ ] Adoption metrics reviewed
- [ ] Known issues prioritized
- [ ] Retrospective scheduled

---

## Rollback Procedures

### Automatic Rollback Triggers

| Condition | Action |
|-----------|--------|
| Error rate > 5% for 15 min | Pause rollout |
| Error rate > 10% for 5 min | Auto-rollback |
| Crash rate > 2% | Auto-rollback |
| Revenue drop > 50% | Manual review |

### Manual Rollback Steps

1. **Chrome Web Store:**
   - Log into Developer Dashboard
   - Unpublish current version
   - Republish previous version
   - Timeline: ~1 hour for propagation

2. **User Communication:**
   - In-app banner: "We're fixing an issue"
   - Email to affected users
   - Status page update

3. **Post-Mortem:**
   - Incident timeline
   - Root cause analysis
   - Prevention measures
   - Publish within 48 hours

---

## Governance & Compliance

### Change Advisory Board

| Role | Responsibility |
|------|---------------|
| Product Owner | Feature prioritization |
| Tech Lead | Technical approval |
| QA Lead | Quality sign-off |
| Security | Security review |
| Legal | Privacy/compliance review |

### Approval Matrix

| Change Type | Approvers Required |
|-------------|-------------------|
| Major release | All CAB members |
| Minor release | Product + Tech Lead |
| Patch/hotfix | Tech Lead |
| Emergency fix | Tech Lead (retroactive CAB) |

### Compliance Checklist

| Requirement | Phase 1 | Phase 3 |
|-------------|---------|---------|
| Privacy policy | Required | Required |
| Terms of service | Required | Required |
| GDPR compliance | Required | Required |
| Data Processing Agreement | - | Required |
| SOC 2 Type I | - | In progress |
| Chrome policy compliance | Required | Required |

---

## Release Calendar

### 2025 Release Schedule

| Version | Type | Target Date | Status |
|---------|------|-------------|--------|
| v0.1.0-alpha | Alpha | Month 2 | Planned |
| v0.1.0-beta | Beta | Month 3 W1 | Planned |
| v0.1.0 | GA | Month 3 W4 | Planned |
| v0.1.1 | Patch | Month 4 | If needed |
| v0.2.0-beta | Beta | Month 5 | Planned |
| v0.2.0 | GA | Month 6 | Planned |
| v0.3.0 | Minor | Month 9 | Planned |
| v1.0.0-rc | RC | Month 11 | Planned |
| v1.0.0 | Major GA | Month 12 | Planned |

### Freeze Periods

| Period | Duration | Reason |
|--------|----------|--------|
| Dec 20 - Jan 5 | 2 weeks | Holiday freeze |
| Major release +3 days | 3 days | Stabilization |

---

## Appendix: Chrome Web Store Assets

### Required Assets

| Asset | Dimensions | Format |
|-------|------------|--------|
| Icon small | 16x16, 32x32, 48x48 | PNG |
| Icon large | 128x128 | PNG |
| Screenshot 1-5 | 1280x800 or 640x400 | PNG/JPG |
| Promotional tile small | 440x280 | PNG |
| Promotional tile large | 920x680 | PNG |
| Marquee | 1400x560 | PNG |

### Store Listing Content

**Title:** Resume Realtime - Free Resume Parser

**Summary (132 chars):**
Parse resumes instantly in your browser. Zero cost, zero data sharing. Works on LinkedIn, Gmail, and major ATS platforms.

**Description (16,000 chars max):**
[Full description with features, benefits, privacy emphasis]

**Category:** Productivity

**Language:** English (US)

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
