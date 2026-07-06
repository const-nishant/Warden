# Warden — Peer-to-Peer Decentralized Chat over SSH

## Project Overview

A decentralized chat application where messages travel directly between peers over SSH-secured channels, with no central server storing messages or identities. Peer discovery and NAT traversal are handled by a DHT-based overlay network (Kademlia via libp2p); SSH is used purely as the authenticated transport layer once two peers have found each other.

**Identity = Ed25519 SSH keypair.** No usernames, no passwords, no central registry.

## Architecture

```
+-------------------+     +--------------------+     +-------------------+
|   Peer A          |     |  Discovery Overlay  |     |   Peer B          |
|  SSH Keypair      |<--->|  Kademlia DHT       |<--->|  SSH Keypair      |
|  DHT Node         |     |  Rendezvous Nodes   |     |  DHT Node         |
|  SSH Client/Svr   |     |  Relay Nodes        |     |  SSH Client/Svr   |
|  Encrypted Store  |     +--------------------+     |  Encrypted Store  |
+-------------------+                                  +-------------------+
        |                      SSH tunnel                      |
        +------------------------------------------------------+
                         (pubkey auth, subsystem "chat")
```

### Layer Stack

1. **Identity** — Ed25519 SSH keypair, PeerID = base58(SHA-256(pubkey))
2. **Discovery** — Kademlia DHT (libp2p) maps PeerID → multiaddr
3. **NAT Traversal** — Hole-punching + relay fallback (libp2p circuit relay)
4. **Transport** — SSH subsystem `chat` (embedded SSH server per peer)
5. **Application** — Protobuf-framed ChatFrame messages over SSH channel
6. **Storage** — Encrypted local SQLite database

## Tech Stack

| Layer | Choice | Skill |
|-------|--------|-------|
| Language | Rust | `rust-async-patterns`, `rust-best-practices` |
| Discovery/NAT | libp2p (`rust-libp2p`) | — |
| SSH Transport | `thrussh` / `russh` | — |
| Message Framing | Protocol Buffers | `protobuf` |
| Local Storage | SQLCipher or age-encrypted SQLite | — |
| Client | CLI first, desktop later | — |

## Core Components

- **Identity & Key Manager** — Generate/store Ed25519 keypair, derive PeerID
- **Discovery Module** — libp2p Kademlia DHT client
- **NAT Traversal** — Hole punching + circuit relay fallback
- **SSH Transport** — Embedded SSH server (subsystem-only, hardened)
- **Chat Protocol Handler** — Framed ChatFrame (protobuf) over SSH channel
- **Local Storage** — Encrypted message/contact store
- **Outbox Queue** — Store-and-forward for offline delivery

## Security Model

- SSH public key auth for peer authentication (trust-list / `authorized_keys` model)
- Sign-then-encrypt message frames (portable verification beyond SSH channel)
- Embedded SSH server restricted to `chat` subsystem only — no shell, exec, or port forwarding
- Encrypted local storage at rest
- Allowlist-only spam protection in v1

## Milestones

| Phase | Scope |
|-------|-------|
| M0 | SSH subsystem chat between 2 hardcoded IPs |
| M1 | Kademlia DHT peer discovery |
| M2 | NAT traversal (hole punch + relay) |
| M3 | MVP: contacts, encrypted history, basic UI |
| M4 | Offline store-and-forward delivery |
| M5 | Group chat (multi-peer fanout) |

## Key Design Decisions

- **SSH Subsystem pattern (Option A):** Each peer runs an embedded SSH server; peers connect and request the `chat` subsystem. Reuses well-audited SSH transport/auth.
- **Direct connection first:** Try hole punch, fall back to relay. Relay sees only encrypted SSH bytes.
- **No offline delivery in v1:** Sender queues locally, retries when DHT shows peer online.
- **DHT is public:** PeerID ↔ address mappings are discoverable (same tradeoff as BitTorrent/IPFS).

## Installed Skills

- `rust-async-patterns` — Rust async/await, Tokio, async patterns
- `rust-best-practices` — Rust conventions, idioms, and code quality
- `protobuf` — Protocol Buffer design, buf CLI, schema evolution

## Agent Guidelines

### Tool Hierarchy (use in this order)
1. **code-review-graph MCP tools** — structural queries, callers/callees, impact radius
2. **ast-grep MCP** — AST-aware structural search (before regex)
3. **LSP references** — go-to-definition, find-all-references via rust-analyzer
4. **ripgrep** — only when graph and AST don't cover the need
5. **Read** — read only specific functions/modules, never entire files

### Code Change Rules
- **Preserve project architecture** — match existing patterns for modules, error handling, imports
- **Minimal edits** — change only what's necessary; no incidental reformatting
- **No `any` in Rust** — keep type-safe; avoid `unsafe` unless absolutely required
- **Preserve formatting** — do not reformat existing code; match surrounding style
- **Read before write** — understand the file's imports and conventions before editing
- **Avoid unnecessary edits** — if a change isn't requested, don't make it

### Context Minimization
- Never read an entire file when a function/block suffices
- Use `get_minimal_context` (~100 tokens) as your first entry point
- Use `query_graph` to discover relationships before reading files
- Batch reads in parallel when you need multiple files

<!-- code-review-graph MCP tools -->
## MCP Tools: code-review-graph

**IMPORTANT: This project has a knowledge graph. ALWAYS use the
code-review-graph MCP tools BEFORE using Grep/Glob/Read to explore
the codebase.** The graph is faster, cheaper (fewer tokens), and gives
you structural context (callers, dependents, test coverage) that file
scanning cannot.

### When to use graph tools FIRST

- **Exploring code**: `semantic_search_nodes` or `query_graph` instead of Grep
- **Understanding impact**: `get_impact_radius` instead of manually tracing imports
- **Code review**: `detect_changes` + `get_review_context` instead of reading entire files
- **Finding relationships**: `query_graph` with callers_of/callees_of/imports_of/tests_for
- **Architecture questions**: `get_architecture_overview` + `list_communities`

Fall back to Grep/Glob/Read **only** when the graph doesn't cover what you need.

### Key Tools

| Tool | Use when |
| ------ | ---------- |
| `detect_changes` | Reviewing code changes — gives risk-scored analysis |
| `get_review_context` | Need source snippets for review — token-efficient |
| `get_impact_radius` | Understanding blast radius of a change |
| `get_affected_flows` | Finding which execution paths are impacted |
| `query_graph` | Tracing callers, callees, imports, tests, dependencies |
| `semantic_search_nodes` | Finding functions/classes by name or keyword |
| `get_architecture_overview` | Understanding high-level codebase structure |
| `refactor_tool` | Planning renames, finding dead code |

### Workflow

1. The graph auto-updates on file changes (via hooks).
2. Use `detect_changes` for code review.
3. Use `get_affected_flows` to understand impact.
4. Use `query_graph` pattern="tests_for" to check coverage.
