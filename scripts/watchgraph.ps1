#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Watch the repository and auto-update the code graph.
.DESCRIPTION
    Starts file watcher that incrementally updates the knowledge graph
    whenever a file changes. Run in a background terminal.
.EXAMPLE
    ./scripts/watchgraph.ps1
#>
param([switch]$Help)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
code-review-graph watch
