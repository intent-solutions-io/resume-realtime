# Real-Time Collaborative Engine Blueprint

**Created:** 2025-12-07 14:45 CST (America/Chicago)
**Version:** 1.0
**Status:** Draft
**Phase Target:** Phase 2+

---

## Executive Summary

Blueprint for the real-time collaborative editing engine that will power multi-user resume editing with conflict resolution, presence awareness, and operational transformation.

## Architecture Overview

```
┌─────────────────────────────────────────────────────────────────┐
│                     Real-Time Resume                            │
├─────────────────────────────────────────────────────────────────┤
│  Browser Client (WASM)                                          │
│  ├── Leptos SSR + Hydration                                     │
│  ├── WebSocket Client                                           │
│  └── Local CRDT State                                           │
├─────────────────────────────────────────────────────────────────┤
│  Axum Server                                                     │
│  ├── HTTP Routes (SSR, API)                                     │
│  ├── WebSocket Handler                                          │
│  └── Session Manager                                            │
├─────────────────────────────────────────────────────────────────┤
│  Collaboration Layer                                            │
│  ├── Operational Transformation (OT) Engine                     │
│  ├── CRDT State Sync                                            │
│  └── Presence/Cursor Tracking                                   │
├─────────────────────────────────────────────────────────────────┤
│  Persistence Layer                                              │
│  ├── Redis (Session + Pub/Sub)                                  │
│  └── PostgreSQL (Document Storage)                              │
└─────────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. WebSocket Layer
- **Protocol:** Native WebSocket via `tokio-tungstenite`
- **Message Format:** Binary (MessagePack) for efficiency
- **Heartbeat:** 30s ping/pong
- **Reconnection:** Exponential backoff with state recovery

### 2. Operational Transformation (OT)
- **Algorithm:** Jupiter OT for text operations
- **Operations:** Insert, Delete, Retain
- **Conflict Resolution:** Server-authoritative with client prediction
- **History:** Rolling 100-operation buffer for undo

### 3. Presence System
- **Cursor Positions:** Real-time broadcast
- **User Colors:** Deterministic from user ID
- **Selection Ranges:** Visible to collaborators
- **Idle Detection:** 5min timeout

### 4. State Management
- **Client:** Local-first with optimistic updates
- **Server:** Single source of truth
- **Sync:** Delta-based with full-state fallback

## Data Structures

### Document State
```rust
struct ResumeDocument {
    id: Uuid,
    version: u64,
    content: ResumeContent,
    collaborators: Vec<Collaborator>,
    last_modified: DateTime<Utc>,
}

struct ResumeContent {
    sections: Vec<Section>,
    metadata: DocumentMetadata,
}

struct Section {
    id: Uuid,
    section_type: SectionType,
    content: String,  // Rich text as Markdown
    order: i32,
}
```

### Operation Types
```rust
enum Operation {
    Insert { pos: usize, text: String },
    Delete { pos: usize, len: usize },
    Retain { len: usize },
    SectionReorder { section_id: Uuid, new_order: i32 },
}
```

## Phase Roadmap

| Phase | Scope | Deliverables |
|-------|-------|--------------|
| 0 | Scaffold | Repo, docs, CI |
| 1 | MVP | SSR counter, hydration, Tailwind |
| 2 | WebSocket | WS connection, basic messaging |
| 3 | OT Engine | Operation types, transform logic |
| 4 | Presence | Cursors, selections, user awareness |
| 5 | Persistence | Redis sessions, PostgreSQL storage |
| 6 | Production | Auth, rate limiting, monitoring |

## Technical Decisions

### Why Rust + Leptos?
- Single language for client and server
- WASM performance for client-side OT
- Memory safety for concurrent operations
- SSR for SEO and initial load

### Why OT over CRDT?
- Simpler mental model for text editing
- Better control over operation ordering
- Proven in production (Google Docs, Etherpad)
- Can layer CRDT on top later if needed

### Why Redis for Sessions?
- Sub-millisecond latency
- Built-in pub/sub for broadcasting
- Atomic operations for consistency
- Horizontal scaling ready

## Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| OT complexity | High | Start with single-section MVP |
| WebSocket reliability | Medium | Reconnection + state recovery |
| Concurrent edits | High | Server-authoritative resolution |
| Latency | Medium | Optimistic updates + prediction |

## Success Metrics

- **Sync Latency:** < 100ms for operations
- **Concurrent Users:** 10+ per document
- **Conflict Rate:** < 1% visible conflicts
- **Reconnection:** < 2s with state recovery

## References

- [Operational Transformation (Wikipedia)](https://en.wikipedia.org/wiki/Operational_transformation)
- [Jupiter Algorithm Paper](https://dl.acm.org/doi/10.1145/215585.215706)
- [Leptos Documentation](https://leptos.dev/)
- [tokio-tungstenite](https://github.com/snapview/tokio-tungstenite)

---
intent solutions io
Contact: jeremy@intentsolutions.io
