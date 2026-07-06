# Warden Website — Design Document

## Goal

A single-page landing page for **Warden**, a peer-to-peer decentralized chat over SSH tool. The site should be minimal, technical, and inspire trust — similar to tokio.rs, brew.sh, and rust-lang.org.

## Tech Stack Recommendation

| Layer | Choice | Notes |
|-------|--------|-------|
| Framework | **Astro** (static) | Zero JS by default, markdown-friendly, ideal for landing pages |
| Styling | **Tailwind CSS v4** | Utility-first, small bundle, dark mode built-in |
| Deployment | **GitHub Pages** | Free, auto-deploys from `gh-pages` branch via CI |
| Fonts | **Inter** (headings) + **JetBrains Mono** (code) | System fonts, fast load |
| Icons | **Phosphor Icons** or **Lucide** | Lightweight SVG icon sets |

**Alternative stack:** Vite + vanilla HTML/CSS (if Astro is overkill for a single page). Choose Astro only if you plan to add blog/docs pages later.

## Page Structure

```
┌──────────────────────────────────────────────────────────┐
│  Nav: [logo] Warden  ·  GitHub  ·  Docs  ·  Install      │
├──────────────────────────────────────────────────────────┤
│  HERO                                                    │
│  "Peer-to-peer decentralized chat over SSH"              │
│  Subtitle + install command + CTA buttons                │
├──────────────────────────────────────────────────────────┤
│  FEATURES (3-column grid)                                │
│  ● No central server  ● SSH-secured  ● Identity = Keypair│
├──────────────────────────────────────────────────────────┤
│  HOW IT WORKS (architecture diagram)                     │
│  [ASCII or SVG diagram: Peer A ↔ DHT ↔ Peer B]          │
├──────────────────────────────────────────────────────────┤
│  QUICK START (code block terminal-style)                 │
│  install → identity → listen → connect                   │
├──────────────────────────────────────────────────────────┤
│  COMMANDS (compact table)                                │
│  identity, listen, connect, daemon, tui, groups...       │
├──────────────────────────────────────────────────────────┤
│  ARCHITECTURE DEEP DIVE (layers stack)                   │
│  Identity → Discovery → Transport → Protocol → Storage   │
├──────────────────────────────────────────────────────────┤
│  SECURITY                                                 │
│  TOFU, subsystem-only, encrypted storage                 │
├──────────────────────────────────────────────────────────┤
│  FOOTER                                                  │
│  GitHub · Docs · License (MIT)                           │
└──────────────────────────────────────────────────────────┘
```

## Content Sections

### 1. Navigation
- **Logo**: Terminal-style glyph `>_` or a shield icon
- **Links**: GitHub repo, Docs (link to `docs/` or README), Install (`#install` anchor)

### 2. Hero
```
Warden
Peer-to-peer decentralized chat over SSH

Messages travel directly between peers — no servers, no central registry.
Your identity is your Ed25519 SSH keypair.

[Install] [GitHub] [Read the docs]
```

Install command:
```bash
curl -fsSL https://github.com/const-nishant/Warden/releases/latest/download/install.sh | bash
# or
cargo install warden --git https://github.com/const-nishant/Warden.git
```

### 3. Features (three pillars)

| Icon | Title | Description |
|------|-------|-------------|
| 🛡️ | No Central Server | Messages go directly between peers over SSH. DHT-based discovery means no central registry. |
| 🔑 | Identity = Keypair | Your Ed25519 SSH keypair is your identity. No usernames, no passwords, no accounts. |
| 🔒 | SSH-Secured Transport | Reuses well-audited SSH for encrypted transport and public-key authentication. |

### 4. How It Works (architecture diagram)

Simple diagram showing:
```
Peer A (Ed25519 keypair) ──→ Kademlia DHT ←── Peer B (Ed25519 keypair)
                                      │
                                      ▼
                              SSH tunnel (subsystem "chat")
                              Protobuf-framed messages
```

### 5. Quick Start

```
# 1. Install
cargo install warden --git https://github.com/const-nishant/Warden

# 2. Generate identity
warden identity init
# → PeerID: 12D3KooW...

# 3. Start listening
warden listen --port 2222

# 4. Connect to a peer
warden connect 192.168.1.5:2222

# 5. Launch TUI
warden tui --port 2222
```

### 6. Commands Reference

| Command | Description |
|---------|-------------|
| `identity init` | Generate and save a new Ed25519 keypair |
| `identity show` | Display your PeerID |
| `listen --port <n>` | Start SSH chat server |
| `connect <addr>` | Connect to a remote peer |
| `daemon` | Full node: SSH + DHT + relay + persistence |
| `tui --port <n>` | Launch terminal UI |
| `contacts list\|add\|remove` | Manage contacts |
| `history <peer_id>` | View message history |
| `outbox send <peer_id> <msg>` | Queue offline message |
| `groups create\|list\|members\|send` | Group chat management |

### 7. Architecture Deep Dive

Layer stack as cards or list:

- **Identity** — Ed25519 SSH keypair, PeerID = base58(SHA-256(pubkey))
- **Discovery** — Kademlia DHT (libp2p) maps PeerID → multiaddr
- **NAT Traversal** — Hole-punching (DCUtR) + relay fallback
- **Transport** — SSH subsystem `chat` via russh (embedded server per peer)
- **Protocol** — Protobuf-framed ChatFrame messages
- **Storage** — Encrypted local SQLite database

Tech badges: Rust · libp2p · russh · Protocol Buffers · ratatui · SQLite

### 8. Security

- **No shell access** — SSH server restricted to `chat` subsystem only
- **TOFU host keys** — MITM detection via known_hosts file
- **Allowlist contacts** — Only known peers can connect
- **Encrypted at rest** — Local SQLite storage

### 9. Footer

```
Built with Rust · MIT License
GitHub · Issues · Discussions
```

## Color Palette (Dark Theme)

- **Background**: `#0d1117` (GitHub dark)
- **Surface**: `#161b22`
- **Border**: `#30363d`
- **Accent (primary)**: `#58a6ff` (blue)
- **Accent (success)**: `#3fb950` (green)
- **Text**: `#c9d1d9`
- **Text muted**: `#8b949e`
- **Code bg**: `#1c2128`

These match GitHub's dark theme and give a developer-friendly feel.

## Content to Create

### Copy (text content)
- [ ] Hero tagline + subtitle
- [ ] Feature descriptions (3 pillars)
- [ ] Quick start walkthrough
- [ ] Commands table
- [ ] Architecture layer descriptions

### Visual assets
- [ ] **Logo**: A simple SVG icon (shield or terminal glyph)
- [ ] **Architecture diagram**: SVG or inline ASCII in a `<pre>` block
- [ ] **Hero illustration**: Optional — could be a terminal window mockup showing the TUI
- [ ] **Screenshot**: Capture of `warden tui` in action (split-pane chat)

### Technical setup
- [ ] `CNAME` or DNS setup for custom domain (e.g., `warden.chat`)
- [ ] GitHub Actions workflow to build & deploy to `gh-pages`
- [ ] Ensure `.nojekyll` file is present (for GitHub Pages with `_` prefix dirs)

## Deployment

Recommended: **GitHub Pages** via GitHub Actions.

```yaml
# .github/workflows/website.yml (simplified)
name: Deploy website
on:
  push:
    branches: [main]
    paths: [website/**]
jobs:
  deploy:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm ci && npm run build
        working-directory: website/
      - uses: peaceiris/actions-gh-pages@v3
        with:
          github_token: ${{ secrets.GITHUB_TOKEN }}
          publish_dir: website/dist
```

## Responsive Breakpoints

| Breakpoint | Width | Layout |
|-----------|-------|--------|
| Mobile | < 640px | Single column, hamburger nav |
| Tablet | 640–1024px | 2-column features |
| Desktop | > 1024px | Full 3-column layout |

## Accessibility

- All interactive elements focusable and keyboard-navigable
- Color contrast ratios ≥ 4.5:1
- Alt text on all images/diagrams
- Semantic HTML (`<nav>`, `<main>`, `<section>`, `<footer>`)

## Pre-built Implementation Options

You can build this from scratch using the design above, or use one of these:

1. **Vanilla HTML + Tailwind CDN** — Single `index.html` with Tailwind via CDN (no build step)
2. **Astro + Tailwind** — Static site generator, good if you plan to expand
3. **Next.js** — Overkill for a landing page, only if you need SSR/blog

**Recommended:** Start with option 1 (vanilla HTML + Tailwind CDN) for simplicity, then migrate to Astro if you add docs/blog pages.

## What You Need to Build

1. **Project folder** — e.g., `website/` at repo root
2. **`index.html`** — Single page with all sections (or Astro pages)
3. **CSS** — Tailwind (CDN or npm)
4. **Assets** — Logo SVG, architecture diagram, TUI screenshot
5. **Build/deploy config** — CI workflow for GitHub Pages
6. **Copy (text)** — Write the actual content for each section (use the README as source)
