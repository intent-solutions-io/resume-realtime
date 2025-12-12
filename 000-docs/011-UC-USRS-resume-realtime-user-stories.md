# User Stories: Resume Realtime

**Created:** 2025-12-08 CST (America/Chicago)
**Version:** 1.0
**Status:** Active
**PRD Reference:** 007-PP-PROD-resume-realtime-prd.md

---

## Story Framework

### INVEST Principles
All stories follow INVEST criteria:
- **I**ndependent: Can be developed in isolation
- **N**egotiable: Details refined through discussion
- **V**aluable: Delivers user/business value
- **E**stimable: Can be sized by team
- **S**mall: Fits in single sprint
- **T**estable: Clear acceptance criteria

### Story Point Scale (Fibonacci)

| Points | Complexity | Example |
|--------|------------|---------|
| 1 | Trivial | Config change, copy update |
| 2 | Simple | Single component, no deps |
| 3 | Standard | Multiple components, known pattern |
| 5 | Complex | New pattern, integration required |
| 8 | Very Complex | Multiple integrations, unknowns |
| 13 | Epic-level | Should be broken down |

### Priority Levels

| Priority | Description |
|----------|-------------|
| P0 | Critical - Blocks release |
| P1 | High - Core functionality |
| P2 | Medium - Important feature |
| P3 | Low - Nice to have |

---

## Epic 1: Resume Parsing (Phase 1)

### US-101: Drag-and-Drop Resume Upload

| Field | Value |
|-------|-------|
| **ID** | US-101 |
| **Epic** | Resume Parsing |
| **Priority** | P0 |
| **Points** | 3 |
| **Sprint** | 1 |

**Story:**
As a **recruiter**, I want to **drag and drop a PDF resume onto the extension overlay**, so that I can **quickly parse a candidate's information without navigating file dialogs**.

**Acceptance Criteria:**

```gherkin
Given I have the extension overlay open
When I drag a PDF file over the drop zone
Then the drop zone should highlight indicating it accepts the file

Given I drop a valid PDF file
When the file is received
Then a parsing progress indicator should display
And the PDF should begin processing within 1 second

Given I drop an invalid file type (e.g., .exe, .zip)
When the file is rejected
Then an error message should display: "Please upload a PDF file"
```

**Technical Notes:**
- Use HTML5 Drag and Drop API
- Validate MIME type: application/pdf
- Max file size: 10MB

---

### US-102: Local LLM Resume Extraction

| Field | Value |
|-------|-------|
| **ID** | US-102 |
| **Epic** | Resume Parsing |
| **Priority** | P0 |
| **Points** | 8 |
| **Sprint** | 2-3 |

**Story:**
As a **recruiter**, I want the **extension to extract structured data from resumes using local AI**, so that I can **capture candidate information without sending data to external servers**.

**Acceptance Criteria:**

```gherkin
Given I have uploaded a PDF resume
When the parsing completes
Then the following fields should be extracted:
  | Field | Example |
  | Name | John Smith |
  | Email | john@example.com |
  | Phone | (555) 123-4567 |
  | Location | Austin, TX |
  | Experience | Array of job objects |
  | Education | Array of education objects |
  | Skills | Array of skill strings |

Given the model is not yet cached
When I first use the parser
Then a download progress bar should display
And the model should cache for future use

Given parsing completes
When I view the results
Then each field should display a confidence score (0-100%)
```

**Technical Notes:**
- Use Phi-3-mini (1.3B) quantized to q4
- WebGPU for inference, WASM SIMD fallback
- Target latency: <5s cached, <30s cold start

---

### US-103: HITL Field Correction

| Field | Value |
|-------|-------|
| **ID** | US-103 |
| **Epic** | Resume Parsing |
| **Priority** | P1 |
| **Points** | 5 |
| **Sprint** | 3 |

**Story:**
As a **recruiter**, I want to **correct parsing errors by highlighting text on the original PDF**, so that I can **fix mistakes quickly and train the parser for better future accuracy**.

**Acceptance Criteria:**

```gherkin
Given parsed results are displayed
When I click on a field (e.g., "Email")
Then the PDF viewer should scroll to the source text
And the source text should be highlighted

Given I select text on the PDF
When I choose "Set as [Field Name]"
Then the field value should update to my selection
And the correction should be saved locally

Given I have made corrections
When I view the same candidate later
Then my corrections should persist
```

---

### US-104: Encrypted Local Storage

| Field | Value |
|-------|-------|
| **ID** | US-104 |
| **Epic** | Resume Parsing |
| **Priority** | P1 |
| **Points** | 5 |
| **Sprint** | 2 |

**Story:**
As a **recruiter**, I want my **parsed candidate data encrypted on my device**, so that I can **protect sensitive information from unauthorized access**.

**Acceptance Criteria:**

```gherkin
Given I parse a new resume
When the data is stored
Then it should be encrypted with AES-256-GCM

Given I close and reopen the browser
When I access stored candidates
Then data should decrypt seamlessly with my session

Given another application tries to read IndexedDB
When they access the raw data
Then they should see encrypted blobs, not plaintext
```

---

## Epic 2: Browser Overlay (Phase 2)

### US-201: LinkedIn Profile Detection

| Field | Value |
|-------|-------|
| **ID** | US-201 |
| **Epic** | Browser Overlay |
| **Priority** | P0 |
| **Points** | 3 |
| **Sprint** | 4 |

**Story:**
As a **recruiter**, I want the **extension to automatically detect when I'm viewing a LinkedIn profile**, so that I can **access extension features in context without manual activation**.

**Acceptance Criteria:**

```gherkin
Given I navigate to linkedin.com/in/*
When the page loads
Then the extension overlay should become available
And a small icon should appear near the profile

Given I navigate away from a profile page
When I reach a non-profile LinkedIn page
Then the overlay should minimize/hide
```

---

### US-202: Candidate Exists Badge

| Field | Value |
|-------|-------|
| **ID** | US-202 |
| **Epic** | Browser Overlay |
| **Priority** | P1 |
| **Points** | 5 |
| **Sprint** | 4 |

**Story:**
As a **recruiter**, I want to **see a badge if a LinkedIn profile matches someone in my database**, so that I can **avoid duplicate data entry and see their current status**.

**Acceptance Criteria:**

```gherkin
Given I view a LinkedIn profile
When the Universal ID matches a stored candidate
Then a "Candidate Exists" badge should display
And the badge should show:
  | Info | Example |
  | Status | Active Candidate |
  | Last Contact | 3 days ago |
  | Notes Preview | "Great .NET dev, prefers remote" |

Given the profile is new (no match)
When the check completes
Then a "New Profile" indicator should display
And a "Save to Database" button should be prominent
```

---

### US-203: One-Click Profile Capture

| Field | Value |
|-------|-------|
| **ID** | US-203 |
| **Epic** | Browser Overlay |
| **Priority** | P1 |
| **Points** | 5 |
| **Sprint** | 5 |

**Story:**
As a **recruiter**, I want to **capture a LinkedIn profile to my local database with one click**, so that I can **build my talent pool without manual data entry**.

**Acceptance Criteria:**

```gherkin
Given I view a LinkedIn profile without a match
When I click "Save Profile"
Then the extension should scrape:
  | Field | Source |
  | Name | Profile header |
  | Headline | Below name |
  | Location | Location field |
  | Current Company | Experience section |
  | LinkedIn URL | Browser URL |

And the profile should be stored locally with encrypted data
And a success toast should confirm the save
```

---

### US-204: OSINT Email Enrichment

| Field | Value |
|-------|-------|
| **ID** | US-204 |
| **Epic** | Browser Overlay |
| **Priority** | P2 |
| **Points** | 8 |
| **Sprint** | 5-6 |

**Story:**
As a **recruiter**, I want the **extension to find candidate emails using public sources before using paid credits**, so that I can **reduce enrichment costs**.

**Acceptance Criteria:**

```gherkin
Given I click "Find Email" on a profile
When OSINT search runs
Then the extension should check (in order):
  1. GitHub public commits (if username found)
  2. Personal website mailto links
  3. Company email pattern guessing

Given OSINT finds an email
When displayed
Then it should show source: "Found via GitHub"
And no paid credits should be consumed

Given OSINT fails
When I have BYOK configured
Then the extension should query paid API (Lusha/Hunter)
And display credit cost before confirming
```

---

### US-205: Gmail Send Later

| Field | Value |
|-------|-------|
| **ID** | US-205 |
| **Epic** | Browser Overlay |
| **Priority** | P2 |
| **Points** | 5 |
| **Sprint** | 6 |

**Story:**
As a **recruiter**, I want to **schedule emails to send later directly from Gmail**, so that I can **time outreach for optimal response rates without leaving my inbox**.

**Acceptance Criteria:**

```gherkin
Given I am composing an email in Gmail
When the extension injects UI
Then a "Send Later" button should appear in the toolbar

Given I click "Send Later"
When the scheduler opens
Then I should see time slot options:
  | Option | Description |
  | Tomorrow 9am | Next business morning |
  | Custom | Date/time picker |

Given I schedule an email
When the scheduled time arrives
Then the email should send automatically (if browser open)
Or queue for next browser open
```

---

## Epic 3: ATS Integration (Phase 3)

### US-301: Bullhorn OAuth Login

| Field | Value |
|-------|-------|
| **ID** | US-301 |
| **Epic** | ATS Integration |
| **Priority** | P0 |
| **Points** | 5 |
| **Sprint** | 7 |

**Story:**
As a **recruiter using Bullhorn**, I want to **connect my Bullhorn account to the extension**, so that I can **sync candidates between my browser and ATS**.

**Acceptance Criteria:**

```gherkin
Given I click "Connect Bullhorn" in settings
When the OAuth flow starts
Then I should be redirected to Bullhorn login
And after login, redirected back to extension

Given OAuth completes successfully
When I return to extension
Then my Bullhorn connection status should show "Connected"
And my Bullhorn username should display

Given my token expires
When I perform an ATS action
Then the extension should refresh the token automatically
Or prompt re-authentication if refresh fails
```

---

### US-302: Save to Bullhorn

| Field | Value |
|-------|-------|
| **ID** | US-302 |
| **Epic** | ATS Integration |
| **Priority** | P0 |
| **Points** | 8 |
| **Sprint** | 8 |

**Story:**
As a **recruiter**, I want to **save a parsed candidate directly to Bullhorn**, so that I can **add new candidates to my ATS without leaving LinkedIn**.

**Acceptance Criteria:**

```gherkin
Given I have a candidate in the extension
And I am connected to Bullhorn
When I click "Save to Bullhorn"
Then a field mapping modal should display
And I should see which fields will sync

Given I confirm the save
When the API call succeeds
Then a success message should display with Bullhorn ID
And the local record should update with synced status

Given the candidate already exists in Bullhorn
When I try to save
Then I should see "Candidate exists" with option to update
```

---

### US-303: Resume Formatting with Branding

| Field | Value |
|-------|-------|
| **ID** | US-303 |
| **Epic** | ATS Integration |
| **Priority** | P1 |
| **Points** | 8 |
| **Sprint** | 9 |

**Story:**
As a **recruiter**, I want to **generate a branded PDF resume from parsed data**, so that I can **send client-ready documents without using Allsorter**.

**Acceptance Criteria:**

```gherkin
Given I have parsed candidate data
When I click "Format Resume"
Then a template selection modal should display
And I should see a live preview of the formatted PDF

Given I select a template with my agency logo
When the PDF generates
Then the output should include:
  | Element | Requirement |
  | Header | Agency logo + candidate name |
  | Contact | Formatted phone/email/location |
  | Experience | Chronological, consistent styling |
  | Skills | Bulleted or tagged layout |

Given I want to anonymize the resume
When I toggle "Anonymize"
Then name, email, phone should be redacted
And the preview should update in real-time
```

---

### US-304: GDPR Expiry Alerts

| Field | Value |
|-------|-------|
| **ID** | US-304 |
| **Epic** | ATS Integration |
| **Priority** | P2 |
| **Points** | 5 |
| **Sprint** | 10 |

**Story:**
As an **agency owner**, I want to **receive alerts when candidate data is due for GDPR deletion**, so that I can **maintain compliance without manual tracking**.

**Acceptance Criteria:**

```gherkin
Given candidate data has a 6-month TTL (configurable)
When a candidate reaches 30 days before expiry
Then a notification should appear in extension
And an email alert should send (if configured)

Given I receive a GDPR alert
When I click "Review"
Then I should see list of expiring candidates
And options: "Extend 6 months" or "Delete Now"

Given I click "Delete Now"
When confirmed
Then local data should be purged
And if synced, ATS delete API should be called
And audit log should record the deletion
```

---

## Epic 4: Team Features (Phase 4)

### US-401: Team Template Sharing

| Field | Value |
|-------|-------|
| **ID** | US-401 |
| **Epic** | Team Features |
| **Priority** | P2 |
| **Points** | 5 |
| **Sprint** | 12 |

**Story:**
As an **agency owner**, I want my **recruiters to share resume templates**, so that **all client submissions have consistent branding**.

**Acceptance Criteria:**

```gherkin
Given I create a new template as team admin
When I mark it as "Team Template"
Then all team members should see it in their template list
And it should be marked with a "Team" badge

Given a team member tries to edit a team template
When they make changes
Then they should have option to "Save as Personal Copy"
And original team template should remain unchanged
```

---

### US-402: P2P Candidate Pool Sync

| Field | Value |
|-------|-------|
| **ID** | US-402 |
| **Epic** | Team Features |
| **Priority** | P3 |
| **Points** | 13 |
| **Sprint** | 13-14 |

**Story:**
As a **team of recruiters**, I want to **share candidate pools without a central server**, so that we can **collaborate while maintaining privacy**.

**Acceptance Criteria:**

```gherkin
Given I am on a team
When a teammate adds a candidate
Then the candidate should sync to my local database
And Universal ID deduplication should prevent duplicates

Given I am offline
When a teammate syncs
Then changes should queue
And sync when I come back online

Given a sync conflict occurs
When the same candidate is edited by two people
Then most recent edit should win
And conflict should be logged for review
```

---

## Story Summary by Sprint

| Sprint | Stories | Total Points |
|--------|---------|--------------|
| 1 | US-101 | 3 |
| 2 | US-102 (partial), US-104 | 8 |
| 3 | US-102 (complete), US-103 | 10 |
| 4 | US-201, US-202 | 8 |
| 5 | US-203, US-204 (partial) | 10 |
| 6 | US-204 (complete), US-205 | 10 |
| 7 | US-301 | 5 |
| 8 | US-302 | 8 |
| 9 | US-303 | 8 |
| 10 | US-304 | 5 |
| 12 | US-401 | 5 |
| 13-14 | US-402 | 13 |

**Total: 93 story points across 14 sprints**

---

## Definition of Done

A story is complete when:
- [ ] All acceptance criteria pass
- [ ] Code reviewed and approved
- [ ] Unit test coverage > 80%
- [ ] Integration tests pass
- [ ] Accessibility validated (keyboard nav, screen reader)
- [ ] Performance benchmarks met
- [ ] Documentation updated
- [ ] Deployed to staging and verified

---

---
intent solutions io
Contact: jeremy@intentsolutions.io

---
**Created:** 2025-12-08 CST (America/Chicago)
