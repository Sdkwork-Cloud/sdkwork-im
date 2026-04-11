param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [switch]$SkipSmoke,
    [string]$SmokeBaseUrl = "",
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/deploy-local.ps1 [-ProfileName <local-minimal|local-default>] [-SkipSmoke] [-SmokeBaseUrl <url>]"
    Write-Host "Usage: cmd /c .\bin\deploy-local.cmd [--profile <local-minimal|local-default>] [--skip-smoke] [--smoke-base-url <url>]"
    Write-Host "Delegate Docker deployment to deployments/scripts/bootstrap-local.ps1."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$bootstrapScript = Join-Path $root "deployments\scripts\bootstrap-local.ps1"

if (-not (Test-Path $bootstrapScript)) {
    throw "Missing bootstrap script: $bootstrapScript"
}

Write-Host "Delegating $ProfileName Docker deployment to bootstrap-local.ps1 (docker compose profile)."
& $bootstrapScript -ProfileName $ProfileName -SkipSmoke:$SkipSmoke -SmokeBaseUrl $SmokeBaseUrl
