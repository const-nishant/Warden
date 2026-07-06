# AI-Assisted Workflow

## Core Principle

**Use the knowledge graph first, then AST search, then LSP, then grep.**

This hierarchy minimizes tokens and latency while maximizing context quality.

## Standard Workflow

### Exploring Code

1. **Query the graph** — `query_graph` for callers, callees, imports
2. **Semantic search** — `semantic_search_nodes` to find by name/purpose
3. **AST search** — `ast-grep run --pattern "..."` for structural matches
4. **Grep** — `rg "keyword"` only when graph and AST don't cover it

### Making Changes

1. **Check impact** — `get_impact_radius` before modifying any function
2. **Get context** — `get_review_context` for the files you need to change
3. **Get affected flows** — `get_affected_flows` for execution path impact
4. **Read minimum** — Read only specific functions, not entire files
5. **Check coverage** — `query_graph pattern="tests_for"` for test files

### Code Review

1. **Detect changes** — `detect_changes` for risk-scored analysis
2. **Review context** — `get_review_context` for token-optimized snippets
3. **Check architecture** — `get_architecture_overview` for structural impact

### Refactoring

1. **Plan renames** — `refactor_tool` for rename preview and dead code
2. **Check blast radius** — `get_impact_radius` for all affected locations
3. **Verify flows** — `get_affected_flows` to ensure no breakage

## Commands Reference

```powershell
# Graph operations
graph              # code-review-graph build
watchgraph         # code-review-graph watch
repoctx            # repomix generate summary

# Search operations
astfind -p "..."   # ast-grep AST search
rg "pattern"       # ripgrep regex search
fd "filename"      # fd file search

# Graph queries
code-review-graph status                          # graph health
code-review-graph detect-changes                  # change analysis
code-review-graph get-minimal-context             # compact context
code-review-graph visualize                       # HTML graph viz
```

## LSP Integration

Before manual searching, prefer LSP-based navigation:

- **Go to definition** — Use LSP references instead of grep
- **Find all references** — Use LSP symbol lookup
- **Type inspection** — Use `rust-analyzer` / `typescript-language-server`

rust-analyzer is configured via rustup component. typescript-language-server is installed globally via npm.
