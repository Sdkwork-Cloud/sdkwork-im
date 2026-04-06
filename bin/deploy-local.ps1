param(
    [switch]$SkipSmoke,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/deploy-local.ps1 [-SkipSmoke]"
    Write-Host "Delegate Docker deployment to deployments/scripts/bootstrap-local.ps1."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$bootstrapScript = Join-Path $root "deployments\scripts\bootstrap-local.ps1"

if (-not (Test-Path $bootstrapScript)) {
    throw "Missing bootstrap script: $bootstrapScript"
}

Write-Host "Delegating local-minimal Docker deployment to bootstrap-local.ps1 (docker compose profile)."
& $bootstrapScript -SkipSmoke:$SkipSmoke
