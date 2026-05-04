$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$scriptPath = Join-Path $root 'scripts\run-local-minimal.mjs'
& node $scriptPath @args
exit $LASTEXITCODE
