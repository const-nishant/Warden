#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build or update the code-review-graph knowledge graph.
.DESCRIPTION
    Builds the structural code graph for the current repository.
    Run this when you clone a new repo or need a full rebuild.
.EXAMPLE
    ./scripts/graph.ps1
    ./scripts/graph.ps1 --skip-flows
#>
param(
    [switch]$SkipFlows,
    [switch]$Help
)
if ($Help) {
    Get-Help $PSCommandPath
    exit 0
}
$args = @()
if ($SkipFlows) { $args += "--skip-flows" }
code-review-graph build @args
