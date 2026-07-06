# Troubleshooting

## Tools

### code-review-graph fails with "uvx not found"

```powershell
# Add uv to PATH
$env:Path = "$env:USERPROFILE\.local\bin;$env:Path"
# Or add permanently to PowerShell profile
```

### code-review-graph build fails with no nodes

This is expected for empty or new repositories. The graph will populate once source files exist. Supported languages include Rust, TypeScript, Go, Python, and 15+ others via Tree-sitter.

### ast-grep not found

```powershell
npm install -g @ast-grep/cli
```

### ripgrep (rg) not found

```powershell
cargo install ripgrep
```

### fd not found

```powershell
cargo install fd-find
```

### ast-grep-mcp-server not found

```powershell
cargo install ast-grep-mcp
```

### rust-analyzer not found

```powershell
rustup component add rust-analyzer
```

## OpenCode

### OpenCode cannot connect to MCP servers

1. Verify `.opencode.json` exists and has correct paths
2. Check that `uvx` / `ast-grep-mcp-server` are in PATH
3. Restart OpenCode after modifying `.opencode.json`

### OpenCode plugin not loaded

The code-review-graph plugin is at:
```
$env:USERPROFILE\.config\opencode\plugins\crg-plugin.ts
```

If it's missing, re-run:
```powershell
code-review-graph install --platform opencode -y
```

## Graph Issues

### Graph is stale

```powershell
code-review-graph update
```

### Graph needs full rebuild

```powershell
code-review-graph build
```

### Graph has wrong working directory

Ensure `.opencode.json` has the correct `cwd` field for the code-review-graph MCP server.

## MCP Configuration

The `ast-grep` MCP server uses the Rust binary (`ast-grep-mcp-server`). If it fails to start, check:

1. Binary is in PATH: `which ast-grep-mcp-server`
2. Config file exists if `--config` is specified in args
3. No port conflicts (uses stdio, not network)
