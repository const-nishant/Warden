# Warden Website — Design Document

## Goal

A single-page landing page for Warden. Minimalist, technical, creative — inspired by brutalist/neo-brutalist web design trends (2025-2026).

## Design Direction

**Neo-brutalist / ultra-minimal:** Thick raw borders, no rounded corners, maximum whitespace, bold typography as primary visual element, monochrome + single blue accent, grain texture overlay, terminal/ASCII aesthetic.

## Tech Stack

| Layer | Choice |
|-------|--------|
| Framework | Vanilla HTML (single page) |
| Styling | CSS custom properties + Tailwind CDN (for layout utilities only) |
| Fonts | Inter (body), JetBrains Mono (code) |
| Deployment | GitHub Pages via `website/` folder |

## Page Structure

```
nav: warden · install · commands · github

hero:
  "peer-to-peer decentralized chat over SSH"
  (banner-size typography, hover animation on each char)
  subtitle + CTA buttons

terminal preview:
  [border-framed terminal window mockup]
  $ warden identity init
  $ warden tui --port 2222
  $ /connect 192.168.1.5:2222

features (3-column grid):
  01 no server · 02 your key, your id · 03 ssh-secured

stack (visual progress bars):
  identity · discovery · nat · transport · protocol · storage

commands (table):
  identity init · listen · connect · daemon · tui · /connect · /group

install (code block):
  cargo install + quick start walkthrough

security (4-column grid):
  subsystem-only · tofu · allowlist · encrypted at rest

footer:
  built with rust · mit license · github · issues · v0.2.0
```

## Color Palette

| Role | Light | Dark |
|------|-------|------|
| Background | `#fafafa` | `#0a0a0a` |
| Text | `#0a0a0a` | `#fafafa` |
| Muted | `#a0a0a0` | `#666` |
| Line / Border | `#e0e0e0` | `#222` |
| Accent (blue) | `#2563eb` | `#3b82f6` |

## Key Design Elements

- **Hero char hover:** Each letter in the hero bounces up + changes color on hover
- **Grain texture:** Fixed SVG noise overlay at 2.5% opacity
- **Brutal borders:** 2px solid borders + offset box-shadow (4px/2px)
- **Stack bars:** Horizontal bars with different widths to show layer abstraction
- **Terminal preview:** Framed window with dot controls + animated cursor blink
- **Fade-up scroll:** Sections animate in as user scrolls via IntersectionObserver

## Interactive Elements

- Hero character hover animation
- Brutal card hover (shifts -2px, deeper shadow)
- Stack bar hover (reduced opacity)
- Cursor blink animation in terminal preview

## Responsive

- Single column on mobile, grid columns auto-fit (250px/200px min)
- Hero font scales with `clamp()`
- Stack wraps on small screens

## Assets

- `assets/logo.svg` — Minimalist 32×32 icon (rounded rect with 3 horizontal lines)
- `assets/arch.svg` — Brutalist architecture diagram (Peer A ↔ Kademlia DHT ↔ Peer B)

## Deployment

GitHub Pages from `website/` folder at repo root. No build step. `.github/workflows/website.yml` deploys on push to main.

## Content Sources

README.md, project docs, crate-level doc comments.
