#!/usr/bin/env pwsh
<#
.SYNOPSIS
    AST-aware code search using ast-grep.
.DESCRIPTION
    Search code by AST pattern rather than regex.
    Supports pattern matching, structural search, and rewriting.
.EXAMPLE
    ./scripts/astfind.ps1 -Pattern "SomeStruct::new"
    ./scripts/astfind.ps1 -Pattern "unwrap()" -Rewrite "context()?"
#>
param(
    [Parameter(Mandatory)]
    [string]$Pattern,
    [string]$Rewrite,
    [string]$Language,
    [switch]$Help
)
if ($Help) { Get-Help $PSCommandPath; exit 0 }
$args = @("run", "--pattern", $Pattern)
if ($Rewrite) { $args += @("--rewrite", $Rewrite) }
if ($Language) { $args += @("--lang", $Language) }
ast-grep @args
