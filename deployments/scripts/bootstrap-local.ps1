param(
    [switch]$SkipSmoke
)

$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $root

$composeFile = "deployments/docker-compose/local-minimal.yml"
$smokeScript = Join-Path $root "tools\smoke\local_stack_smoke.ps1"
$composeVersionCommand = "docker compose version"
$composePsCommand = "docker compose -f $composeFile ps"
$composeLogsCommand = "docker compose -f $composeFile logs --tail 200"

function Show-ComposeDiagnostics {
    Write-Warning "Collecting docker compose diagnostics for local-minimal profile."

    Write-Host "Running $composePsCommand"
    & docker compose -f $composeFile ps
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "docker compose ps did not complete successfully."
    }

    Write-Host "Running $composeLogsCommand"
    & docker compose -f $composeFile logs --tail 200
    if ($LASTEXITCODE -ne 0) {
        Write-Warning "docker compose logs did not complete successfully."
    }
}

Write-Host "Checking Docker availability..."
cmd /c "docker --version"
if ($LASTEXITCODE -ne 0) {
    throw "Docker CLI is unavailable. Install Docker and ensure the docker command is on PATH."
}

Write-Host "Checking Docker daemon..."
cmd /c "docker info >nul 2>nul"
if ($LASTEXITCODE -ne 0) {
    throw "Docker daemon is unavailable. Start Docker Desktop or Docker Engine, then rerun bootstrap-local.ps1."
}

Write-Host "Checking Docker compose plugin..."
cmd /c "$composeVersionCommand >nul 2>nul"
if ($LASTEXITCODE -ne 0) {
    throw "Docker compose plugin is unavailable. Install the Docker Compose plugin and retry."
}

Write-Host "Building and starting local-minimal deployment profile..."
& docker compose -f $composeFile up -d --build
if ($LASTEXITCODE -ne 0) {
    Show-ComposeDiagnostics
    throw "Docker compose failed for local-minimal profile."
}

if (-not $SkipSmoke) {
    try {
        & $smokeScript
    }
    catch {
        Show-ComposeDiagnostics
        throw "Smoke verification failed for local-minimal profile. $($_.Exception.Message)"
    }

    Write-Host "local-minimal profile is ready."
    exit 0
}

Write-Host "local-minimal profile started without smoke verification."
