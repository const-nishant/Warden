#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Generate a repository summary / context.
.DESCRIPTION
    Uses repomix to produce a consolidated context of the repository.
    Pipe output to a file or clipboard for sharing with AI tools.
.EXAMPLE
    ./scripts/repoctx.ps1
    ./scripts/repoctx.ps1 | Set-Clipboard
#>
param(
    [string]$Output,
    [switch]$Help
)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
if ($Output) {
    repomix --output $Output
} else {
    repomix
}
