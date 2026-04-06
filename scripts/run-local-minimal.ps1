$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

Write-Host "Building local-minimal-node..."
cargo build -p local-minimal-node --offline

Write-Host "Starting local-minimal-node on http://127.0.0.1:18090"
cargo run -p local-minimal-node --offline

