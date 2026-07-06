# Warden

Peer-to-peer decentralized chat over SSH.

Messages travel directly between peers over SSH-secured channels with **no central server** storing messages or identities. Identity is your Ed25519 SSH keypair — no usernames, no passwords, no central registry.

## Architecture

```
Peer A (Ed25519 keypair)
  ├── DHT Node (libp2p Kademlia) ── discovers Peer B via DHT
  ├── SSH Server (russh, subsystem "chat") ── accepts connections
  └── SSH Client ── connects to Peer B's SSH server
          │
          ▼
Peer B (Ed25519 keypair)
  ├── DHT Node (libp2p Kademlia)
  ├── SSH Server (russh, subsystem "chat")
  └── SSH Client
```

### Layers

| Layer | Technology |
|-------|-----------|
| Identity | Ed25519 keypair, PeerID = bs58(sha256(pubkey)) |
| Discovery | Kademlia DHT via libp2p |
| NAT Traversal | DCUtR hole-punching + Circuit Relay fallback |
| Transport | SSH `chat` subsystem via russh |
| Protocol | Protobuf-framed ChatFrame messages |
| Storage | SQLite (rusqlite bundled) |

## Quick Start

### Prerequisites

- Rust 1.85+ (edition 2024)
- protoc (for prost-build, or use `cargo build` which downloads it)

### Build

```bash
cargo build --release
```

### Generate identity

```bash
warden identity init
```

Shows your PeerID. Saved to `warden_id` in the current directory.

### Listen for connections

```bash
warden listen --port 2222
```

### Connect to a peer

```bash
warden connect 192.168.1.5:2222
```

### Interactive chat (CLI)

```bash
warden daemon
```

Starts SSH server + DHT discovery and opens an interactive chat session.

### Terminal UI

```bash
warden tui --port 2222
```

## Commands

| Command | Description |
|---------|-------------|
| `identity init` | Generate and save a new Ed25519 keypair |
| `identity show` | Show your PeerID |
| `listen` | Start SSH server on a port |
| `connect <addr>` | Connect to a peer and open chat |
| `contacts list\|add\|remove` | Manage contacts |
| `history <peer_id>` | View message history with a peer |
| `outbox send <peer_id> <msg>` | Queue a message for offline delivery |
| `groups create\|list\|members\|send` | Group chat management |
| `daemon` | Full daemon: SSH + DHT + relay + persistence |
| `tui` | Launch terminal UI |

## Security

- **SSH public-key auth** — trust-list model via authorized_keys
- **Subsystem-only SSH** — `chat` subsystem only, no shell/exec/port forwarding
- **TOFU host keys** — server host key verified on reconnect (MITM detection)
- **Encrypted transport** — SSH provides encryption and authentication
- **Encrypted storage** — SQLite at rest (via SQLCipher-compatible schema)

## Project Structure

```
crates/
├── warden-core        # Shared types (PeerId, MessageId, protobuf types)
├── warden-identity    # Ed25519 key generation, signing, PeerID derivation
├── warden-transport   # Embedded SSH server/client, ChatSession, KnownHosts
├── warden-discovery   # libp2p Kademlia DHT, NAT traversal, relay
├── warden-protocol    # Protobuf frame encode/decode, outbox queue
├── warden-storage     # SQLite-backed persistence (contacts, messages, groups, outbox)
├── warden-tui         # Terminal UI (ratatui + crossterm)
└── warden-cli         # CLI binary with all subcommands
```

## License

MIT
