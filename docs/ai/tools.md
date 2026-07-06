# Tools Reference

## code-review-graph

**Purpose:** Builds a persistent, incrementally-updated structural knowledge graph of the codebase using Tree-sitter AST parsing.

**Installation:** `uv tool install code-review-graph`

**Key Commands:**

| Command | Description |
|---------|-------------|
| `code-review-graph build` | Full graph build from scratch |
| `code-review-graph update` | Incremental update (git-based) |
| `code-review-graph watch` | File watcher for auto-updates |
| `code-review-graph status` | Graph health and statistics |
| `code-review-graph serve` | Start MCP server for AI tools |
| `code-review-graph detect-changes` | Risk-scored change analysis |
| `code-review-graph visualize` | Generate HTML graph visualization |
| `code-review-graph install` | Auto-configure for supported AI platforms |

**MCP Tools (available via OpenCode):**
- `build_or_update_graph_tool` — Build or incrementally update
- `get_minimal_context_tool` — Ultra-compact context (~100 tokens)
- `get_impact_radius_tool` — Blast radius of changed files
- `get_review_context_tool` — Token-optimized review context
- `query_graph_tool` — Callers, callees, tests, imports, inheritance
- `semantic_search_nodes_tool` — Search by name or meaning
- `detect_changes_tool` — Risk-scored change impact
- `refactor_tool` — Rename preview, dead code detection

---

## ast-grep

**Purpose:** Structural code search using AST patterns rather than regex.

**Installation:** `npm install -g @ast-grep/cli`

**Key Commands:**

| Command | Description |
|---------|-------------|
| `ast-grep run --pattern "..."` | Search by AST pattern |
| `ast-grep run --pattern "..." --rewrite "..."` | Search and rewrite |
| `ast-grep run --pattern "..." --lang rust` | Language-specific search |
| `ast-grep lsp` | Start LSP server |
| `ast-grep scan` | Scan with rule configuration |
| `ast-grep test` | Test ast-grep rules |

**Examples:**
```bash
# Find all unwrap() calls in Rust
ast-grep run --pattern "unwrap()" --lang rust

# Find all async function definitions
ast-grep run --pattern "async fn $$($$$)" --lang rust

# Find uses of a specific struct
ast-grep run --pattern "SomeStruct" --lang rust
```

**MCP Server:** `ast-grep-mcp-server` (installed via cargo) provides AST search capabilities through MCP.

---

## ripgrep (rg)

**Purpose:** Ultra-fast recursive file content search with regex support.

**Installation:** `cargo install ripgrep`

**Key Usage:**
```bash
rg "pattern"                      # Basic search
rg "fn main" --type rust          # Search only Rust files
rg "impl" -g "*.rs"               # Glob filter
rg "TODO|FIXME" --type rust       # Multi-pattern
rg "pattern" --context 3          # Show surrounding context
rg "pattern" -l                   # List files only
```

---

## fd

**Purpose:** Fast, user-friendly file finder.

**Installation:** `cargo install fd-find`

**Key Usage:**
```bash
fd "main.rs"                      # Find file
fd "mod" --type d                 # Find directories
fd "*.rs" -x rg "fn main" {}     # Find and exec
fd --ext rs "server"              # By extension
```

---

## Repomix

**Purpose:** Pack entire repository into AI-friendly context files.

**Installation:** `npm install -g repomix`

**Key Usage:**
```bash
repomix                           # Generate repo summary (stdout)
repomix --output repo-context.txt # Write to file
repomix --style markdown          # Markdown output
```

---

## GitHub CLI (gh)

**Purpose:** GitHub operations from the command line.

**Version:** 2.87.3

**Key Usage:**
```bash
gh pr status                      # Check PR status
gh pr create --title "..."        # Create PR
gh pr review 123                  # Review PR
gh issue list                     # List issues
```

---

## uv / uvx

**Purpose:** Fast Python package installer and resolver.

**Installation:** Installed via `irm https://astral.sh/uv/install.ps1 | iex`

**Key Usage:**
```bash
uv tool install <package>         # Install Python tools (like pipx)
uvx <package>                     # Run Python tools without installing
uv venv                           # Create virtual environment
uv pip install <package>          # Install from requirements
```

---

## rust-analyzer

**Purpose:** Rust LSP server providing code completion, navigation, and refactoring.

**Installation:** `rustup component add rust-analyzer`

**Version:** 1.92.0

Provides IDE features including:
- Go to definition / implementation
- Find all references
- Type hints and hover information
- Code actions and refactoring
- Workspace symbol search
