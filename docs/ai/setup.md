# AI Environment Setup

## Overview

This document describes the AI-augmented development environment configured for this repository. The setup integrates local code intelligence tools with OpenCode to minimize token usage while maximizing code understanding.

## Prerequisites

- **Windows PowerShell 5.1+** (the shell this environment targets)
- **Node.js 24+** (for OpenCode and npm-installed tools)
- **Rust toolchain 1.92+** (for cargo-installed tools)
- **Python 3.10+** (for code-review-graph dependencies via uv)

## Installed Tools

| Tool | Version | Installer | Purpose |
|------|---------|-----------|---------|
| OpenCode | 1.17.13 | npm | AI coding agent |
| code-review-graph | 2.3.6 | uv | Knowledge graph for code |
| ast-grep | 0.44.1 | npm | AST pattern search |
| ast-grep-mcp-server | 0.1.3 | cargo | MCP server for ast-grep |
| ripgrep | 15.1.0 | cargo | Fast file search |
| fd | 10.4.2 | cargo | Fast file find |
| Repomix | 1.16.0 | npm | Repository summarization |
| GitHub CLI | 2.87.3 | system | GitHub integration |
| uv / uvx | 0.11.26 | installer | Python package manager |
| pipx | 1.15.0 | uv | Python app isolation |
| rust-analyzer | 1.92.0 | rustup | Rust LSP server |
| typescript-language-server | 5.3.0 | npm | TypeScript LSP server |

## Quick Start

```powershell
# Build the knowledge graph
code-review-graph build

# Watch for changes (in background terminal)
code-review-graph watch

# Generate repo summary
repomix

# AST search
ast-grep run --pattern "unwrap()"

# Fast file search
rg "fn main" --type rust

# Fast file find
fd "mod.rs"
```

## Configuration Files

- `.opencode.json` — OpenCode MCP server configuration
- `AGENTS.md` — Agent instructions with graph-aware workflow
- `scripts/*.ps1` — Repository intelligence scripts
- `docs/ai/*.md` — This documentation suite
