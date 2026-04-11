param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [switch]$SkipSmoke,
    [string]$SmokeBaseUrl = ""
)

$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $root

$composeFile = "deployments/docker-compose/$ProfileName.yml"
$smokeScript = Join-Path $root "tools\smoke\local_stack_smoke.ps1"
$composeVersionCommand = "docker compose version"
$composePsCommand = "docker compose -f $composeFile ps"
$composeLogsCommand = "docker compose -f $composeFile logs --tail 200"

function Show-ComposeDiagnostics {
    Write-Warning "Collecting docker compose diagnostics for $ProfileName profile."

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

if (-not (Test-Path (Join-Path $root $composeFile))) {
    throw "Missing compose profile: $composeFile"
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

Write-Host "Building and starting $ProfileName deployment profile..."
& docker compose -f $composeFile up -d --build
if ($LASTEXITCODE -ne 0) {
    Show-ComposeDiagnostics
    throw "Docker compose failed for $ProfileName profile."
}

if (-not $SkipSmoke) {
    try {
        if ([string]::IsNullOrWhiteSpace($SmokeBaseUrl)) {
            & $smokeScript
        }
        else {
            & $smokeScript -BaseUrl $SmokeBaseUrl
        }
    }
    catch {
        Show-ComposeDiagnostics
        throw "Smoke verification failed for $ProfileName profile. $($_.Exception.Message)"
    }

    Write-Host "$ProfileName profile is ready."
    exit 0
}

Write-Host "$ProfileName profile started without smoke verification."
