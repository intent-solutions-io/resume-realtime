# Test Plan: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Draft
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md

---

## Test Strategy Overview

### Test Pyramid Distribution

```
        /\
       /  \     E2E Tests (10%)
      /----\    - Full user flows
     /      \   - Cross-browser validation
    /--------\  Integration Tests (20%)
   /          \ - Component interaction
  /            \- API contracts
 /--------------\ Unit Tests (70%)
                  - Functions, modules
                  - WASM bindings
```

| Level | Coverage Target | Framework |
|-------|-----------------|-----------|
| Unit | 80% line, 75% branch | Jest (TS), cargo test (Rust) |
| Integration | 60% critical paths | Jest + Testing Library |
| E2E | 100% P0 user stories | Playwright |

### Quality Gates

| Gate | Threshold | Blocks |
|------|-----------|--------|
| Unit test pass | 100% | PR merge |
| Unit coverage | > 80% | PR merge |
| Integration pass | 100% | Staging deploy |
| E2E pass | 100% | Production deploy |
| Security scan | 0 critical | Any deploy |
| Performance | < 5s parse latency | Production deploy |

---

## Test Environments

### Environment Matrix

| Environment | Purpose | Data | Refresh |
|-------------|---------|------|---------|
| Local | Developer testing | Mock/synthetic | On demand |
| CI | Automated testing | Synthetic fixtures | Per commit |
| Staging | Pre-release validation | Anonymized production-like | Weekly |
| Production | Live monitoring | Real user data | N/A |

### Browser Matrix (E2E)

| Browser | Version | Priority | WebGPU |
|---------|---------|----------|--------|
| Chrome | Latest, Latest-1 | P0 | Yes |
| Edge | Latest | P1 | Yes |
| Firefox | Latest | P2 | Experimental |
| Safari | Latest | P3 | No (fallback) |

---

## Unit Testing

### TypeScript/React Components

**Framework:** Jest + React Testing Library

**Coverage Requirements:**
- `extension/src/components/*` - 90%
- `extension/src/hooks/*` - 85%
- `extension/src/storage/*` - 80%
- `extension/src/utils/*` - 95%

**Example Test Structure:**

```typescript
// extension/src/components/Parser.test.tsx
describe('Parser Component', () => {
  describe('Drag and Drop', () => {
    it('should highlight drop zone on drag over', () => {});
    it('should accept PDF files', () => {});
    it('should reject non-PDF files with error', () => {});
    it('should show progress indicator on drop', () => {});
  });

  describe('Field Display', () => {
    it('should render all extracted fields', () => {});
    it('should show confidence scores', () => {});
    it('should highlight low-confidence fields', () => {});
  });

  describe('HITL Correction', () => {
    it('should scroll PDF on field click', () => {});
    it('should update field on text selection', () => {});
    it('should persist corrections to storage', () => {});
  });
});
```

### Rust/WASM Modules

**Framework:** cargo test + wasm-bindgen-test

**Coverage Requirements:**
- `resume-parser/src/extractor.rs` - 90%
- `resume-parser/src/inference.rs` - 80%
- `resume-parser/src/crypto.rs` - 95%

**Example Test Structure:**

```rust
// resume-parser/src/extractor.rs
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_text_from_pdf() {
        let pdf_bytes = include_bytes!("../fixtures/sample_resume.pdf");
        let text = extract_text(pdf_bytes).unwrap();
        assert!(text.contains("John Smith"));
    }

    #[test]
    fn test_parse_experience_section() {
        let text = "Software Engineer at Google\n2020-2023\nBuilt scalable systems";
        let exp = parse_experience(text);
        assert_eq!(exp[0].company, "Google");
        assert_eq!(exp[0].title, "Software Engineer");
    }

    #[test]
    fn test_handle_malformed_pdf() {
        let bad_bytes = b"not a pdf";
        let result = extract_text(bad_bytes);
        assert!(result.is_err());
    }
}
```

### Encryption Module Tests

```typescript
// extension/src/storage/crypto.test.ts
describe('Encryption Module', () => {
  it('should generate 256-bit key on init', async () => {});
  it('should encrypt data with AES-256-GCM', async () => {});
  it('should decrypt data correctly', async () => {});
  it('should fail decryption with wrong key', async () => {});
  it('should use unique IV per encryption', async () => {});
  it('should hash Universal ID with SHA-256', async () => {});
});
```

---

## Integration Testing

### Component Integration Tests

**Scope:** Verify components work together correctly

```typescript
// extension/src/integration/parse-flow.test.ts
describe('Parse Flow Integration', () => {
  it('should complete full parse pipeline', async () => {
    // 1. Drop PDF
    // 2. Extract text (WASM)
    // 3. Run LLM inference
    // 4. Display results
    // 5. Store encrypted
  });

  it('should handle HITL correction flow', async () => {
    // 1. Parse resume
    // 2. Click field
    // 3. PDF scrolls
    // 4. Select text
    // 5. Field updates
    // 6. Storage updated
  });
});
```

### API Contract Tests

**Scope:** Verify extension correctly calls ATS APIs

```typescript
// extension/src/integration/bullhorn-api.test.ts
describe('Bullhorn API Integration', () => {
  it('should authenticate with OAuth flow', async () => {});
  it('should refresh expired tokens', async () => {});
  it('should create candidate via REST API', async () => {});
  it('should handle API rate limiting', async () => {});
  it('should map fields correctly to Bullhorn schema', async () => {});
});
```

### WASM-JS Bridge Tests

```typescript
// extension/src/integration/wasm-bridge.test.ts
describe('WASM Bridge', () => {
  it('should load WASM module successfully', async () => {});
  it('should pass PDF bytes to Rust extractor', async () => {});
  it('should receive JSON response from WASM', async () => {});
  it('should handle WASM memory limits gracefully', async () => {});
  it('should report errors from Rust to JS', async () => {});
});
```

---

## End-to-End Testing

### Framework: Playwright

**Configuration:**

```typescript
// playwright.config.ts
export default defineConfig({
  projects: [
    { name: 'chrome', use: { ...devices['Desktop Chrome'] } },
    { name: 'edge', use: { ...devices['Desktop Edge'] } },
    { name: 'firefox', use: { ...devices['Desktop Firefox'] } },
  ],
  webServer: {
    command: 'npm run dev',
    port: 3000,
  },
});
```

### Critical User Flows (P0)

#### E2E-001: First-Time Parse Flow

```typescript
test('new user can parse a resume', async ({ page }) => {
  // Load extension
  await page.goto('chrome-extension://[id]/popup.html');

  // Accept privacy consent
  await page.click('[data-testid="accept-privacy"]');

  // Upload resume
  const dropZone = page.locator('[data-testid="drop-zone"]');
  await dropZone.setInputFiles('fixtures/sample_resume.pdf');

  // Wait for model download (first time)
  await expect(page.locator('[data-testid="download-progress"]')).toBeVisible();
  await expect(page.locator('[data-testid="download-progress"]')).toBeHidden({ timeout: 60000 });

  // Verify parse results
  await expect(page.locator('[data-testid="field-name"]')).toHaveText('John Smith');
  await expect(page.locator('[data-testid="field-email"]')).toContainText('@');

  // Verify local storage
  const stored = await page.evaluate(() => localStorage.getItem('hasSeenWelcome'));
  expect(stored).toBe('true');
});
```

#### E2E-002: LinkedIn Overlay Flow

```typescript
test('overlay detects candidate on LinkedIn', async ({ page, context }) => {
  // Install extension
  await context.addInitScript({ path: 'extension/dist/content.js' });

  // Navigate to LinkedIn profile
  await page.goto('https://www.linkedin.com/in/sample-profile');

  // Wait for overlay injection
  await expect(page.locator('[data-testid="resume-realtime-overlay"]')).toBeVisible();

  // Verify profile scrape
  await page.click('[data-testid="save-profile"]');
  await expect(page.locator('[data-testid="save-success"]')).toBeVisible();
});
```

#### E2E-003: Bullhorn Sync Flow

```typescript
test('user can save candidate to Bullhorn', async ({ page }) => {
  // Setup: Pre-authenticate with Bullhorn sandbox
  await setupBullhornAuth(page);

  // Parse a resume
  await parseResume(page, 'fixtures/sample_resume.pdf');

  // Click save to ATS
  await page.click('[data-testid="save-to-ats"]');

  // Select Bullhorn
  await page.click('[data-testid="ats-bullhorn"]');

  // Verify field mapping
  await expect(page.locator('[data-testid="field-map-name"]')).toBeVisible();

  // Confirm save
  await page.click('[data-testid="confirm-save"]');

  // Verify success
  await expect(page.locator('[data-testid="bullhorn-id"]')).toContainText('BH-');
});
```

#### E2E-004: PDF Formatting Flow

```typescript
test('user can generate branded PDF', async ({ page }) => {
  // Parse resume first
  await parseResume(page, 'fixtures/sample_resume.pdf');

  // Click format
  await page.click('[data-testid="format-resume"]');

  // Select template
  await page.click('[data-testid="template-professional"]');

  // Verify preview
  await expect(page.locator('[data-testid="pdf-preview"]')).toBeVisible();

  // Toggle anonymize
  await page.click('[data-testid="anonymize-toggle"]');
  await expect(page.locator('[data-testid="pdf-preview"]')).not.toContainText('john@example.com');

  // Download
  const [download] = await Promise.all([
    page.waitForEvent('download'),
    page.click('[data-testid="download-pdf"]'),
  ]);
  expect(download.suggestedFilename()).toMatch(/resume.*\.pdf/);
});
```

---

## Performance Testing

### Benchmarks

| Metric | Target | Measurement |
|--------|--------|-------------|
| Parse latency (cached) | < 5s | P95 of 100 parses |
| Parse latency (cold) | < 30s | P95 including model load |
| Memory usage | < 4GB | Peak during inference |
| Extension load time | < 500ms | Time to interactive |
| Overlay injection | < 200ms | Time to render on LinkedIn |

### Performance Test Suite

```typescript
// performance/parse-benchmark.test.ts
describe('Parse Performance', () => {
  const resumes = glob.sync('fixtures/resumes/*.pdf');

  it('should parse within SLA (cached model)', async () => {
    // Warm up model cache
    await parseResume(resumes[0]);

    const times = [];
    for (const resume of resumes.slice(0, 100)) {
      const start = performance.now();
      await parseResume(resume);
      times.push(performance.now() - start);
    }

    const p95 = percentile(times, 95);
    expect(p95).toBeLessThan(5000); // 5 seconds
  });

  it('should not exceed memory limit', async () => {
    const initialMemory = await getHeapSize();

    for (const resume of resumes.slice(0, 50)) {
      await parseResume(resume);
    }

    const peakMemory = await getHeapSize();
    expect(peakMemory - initialMemory).toBeLessThan(4 * 1024 * 1024 * 1024); // 4GB
  });
});
```

---

## Security Testing

### Automated Security Scans

| Tool | Target | Frequency |
|------|--------|-----------|
| npm audit | Dependencies | Every PR |
| Snyk | Dependencies + code | Daily |
| ESLint security plugin | Source code | Every PR |
| cargo audit | Rust dependencies | Every PR |

### Security Test Cases

```typescript
// security/encryption.test.ts
describe('Security Tests', () => {
  describe('Data at Rest', () => {
    it('should never store plaintext PII in IndexedDB', async () => {});
    it('should use unique IV for each encryption', async () => {});
    it('should clear sensitive data on logout', async () => {});
  });

  describe('Data in Transit', () => {
    it('should only make HTTPS requests', async () => {});
    it('should validate SSL certificates', async () => {});
    it('should not leak data to analytics', async () => {});
  });

  describe('Input Validation', () => {
    it('should sanitize file names before display', async () => {});
    it('should reject files over 10MB', async () => {});
    it('should escape HTML in parsed fields', async () => {});
  });

  describe('Extension Permissions', () => {
    it('should request minimum necessary permissions', async () => {});
    it('should not access tabs without user action', async () => {});
  });
});
```

### Penetration Testing Checklist

- [ ] XSS via parsed resume content
- [ ] Injection via file name
- [ ] OAuth token theft
- [ ] IndexedDB data extraction
- [ ] Extension privilege escalation
- [ ] CSP bypass attempts

---

## Accessibility Testing

### WCAG 2.1 AA Compliance

| Criterion | Test Method |
|-----------|-------------|
| Keyboard navigation | Manual + Playwright |
| Screen reader support | NVDA/VoiceOver manual |
| Color contrast | axe-core automated |
| Focus indicators | Playwright visual |
| ARIA labels | axe-core + manual |

### Automated Accessibility Tests

```typescript
// accessibility/a11y.test.ts
import { test, expect } from '@playwright/test';
import AxeBuilder from '@axe-core/playwright';

test('parser component is accessible', async ({ page }) => {
  await page.goto('/parser');

  const results = await new AxeBuilder({ page })
    .withTags(['wcag2a', 'wcag2aa'])
    .analyze();

  expect(results.violations).toEqual([]);
});
```

---

## Test Data Management

### Synthetic Resume Fixtures

| Category | Count | Purpose |
|----------|-------|---------|
| Simple (1 page, text-only) | 50 | Baseline accuracy |
| Complex (multi-column) | 30 | Layout parsing |
| Long (5+ pages) | 20 | Performance testing |
| Non-English | 20 | i18n validation |
| Edge cases | 30 | Error handling |

### Data Generation

```typescript
// fixtures/generate-resumes.ts
// Uses faker.js to generate synthetic resumes
// No real PII - all names/emails are fake
```

### Production Data (Staging Only)

- Anonymized production resumes for accuracy validation
- Differential privacy applied to all real data
- Requires security approval to access

---

## Continuous Integration

### CI Pipeline (GitHub Actions)

```yaml
# .github/workflows/test.yml
name: Test Suite
on: [push, pull_request]

jobs:
  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Install dependencies
        run: npm ci
      - name: Run TypeScript tests
        run: npm test -- --coverage
      - name: Run Rust tests
        run: cargo test --all
      - name: Upload coverage
        uses: codecov/codecov-action@v3

  integration-tests:
    runs-on: ubuntu-latest
    needs: unit-tests
    steps:
      - name: Run integration tests
        run: npm run test:integration

  e2e-tests:
    runs-on: ubuntu-latest
    needs: integration-tests
    steps:
      - name: Install Playwright
        run: npx playwright install --with-deps
      - name: Run E2E tests
        run: npm run test:e2e
      - name: Upload test artifacts
        uses: actions/upload-artifact@v3
        with:
          name: playwright-report
          path: playwright-report/

  security-scan:
    runs-on: ubuntu-latest
    steps:
      - name: npm audit
        run: npm audit --audit-level=high
      - name: cargo audit
        run: cargo audit
```

---

## Test Reporting

### Metrics Dashboard

| Metric | Target | Current |
|--------|--------|---------|
| Unit test coverage | > 80% | TBD |
| E2E pass rate | 100% | TBD |
| Flaky test rate | < 2% | TBD |
| Avg test duration | < 10 min | TBD |
| Bugs escaped to prod | < 1/sprint | TBD |

### Failure Response

| Severity | Response Time | Action |
|----------|---------------|--------|
| E2E failure | < 4 hours | Block deploy, fix immediately |
| Unit test failure | < 24 hours | PR blocked, fix before merge |
| Flaky test | < 1 week | Quarantine and fix |

---

## Appendix: Test Fixtures

### Sample Resume Locations

```
fixtures/
├── resumes/
│   ├── simple/
│   │   ├── 001_entry_level.pdf
│   │   ├── 002_mid_career.pdf
│   │   └── ...
│   ├── complex/
│   │   ├── 001_two_column.pdf
│   │   ├── 002_graphic_heavy.pdf
│   │   └── ...
│   └── edge_cases/
│       ├── 001_no_email.pdf
│       ├── 002_foreign_characters.pdf
│       └── ...
├── mock_responses/
│   ├── bullhorn/
│   └── salesforce/
└── expected_outputs/
    └── parsed_json/
```

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
