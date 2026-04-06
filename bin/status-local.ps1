param(
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/status-local.ps1"
    Write-Host "Show local-minimal-node pid, config, stdout/stderr logs, health status, and the next runtime-dir inspection/repair/list/preview/restore steps."
    exit 0
}

function Get-RunningProcessFromPidFile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PidFile,
        [string]$ExpectedProcessName = "local-minimal-node"
    )

    if (-not (Test-Path $PidFile)) {
        return $null
    }

    $raw = (Get-Content -Path $PidFile -ErrorAction SilentlyContinue | Select-Object -First 1)
    if ([string]::IsNullOrWhiteSpace($raw)) {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    try {
        $process = Get-Process -Id ([int]$raw.Trim()) -ErrorAction Stop
    }
    catch {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    if (-not ($process.ProcessName -ieq $ExpectedProcessName)) {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    return $process
}

function Read-ConfigValue {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ConfigFile,
        [Parameter(Mandatory = $true)]
        [string]$Key
    )

    if (-not (Test-Path $ConfigFile)) {
        return $null
    }

    foreach ($line in Get-Content -Path $ConfigFile) {
        $trimmed = $line.Trim()
        if ($trimmed.Length -eq 0 -or $trimmed.StartsWith('#')) {
            continue
        }

        $parts = $trimmed -split '=', 2
        if ($parts.Count -eq 2 -and $parts[0].Trim() -eq $Key) {
            return $parts[1].Trim()
        }
    }

    return $null
}

function Get-HealthUrl {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBindAddress
    )

    $segments = $ResolvedBindAddress -split ':'
    if ($segments.Length -lt 2) {
        return "http://127.0.0.1:18090/healthz"
    }

    $port = $segments[-1]
    $bindHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($bindHost) -or $bindHost -eq "0.0.0.0" -or $bindHost -eq "::" -or $bindHost -eq "[::]") {
        $bindHost = "127.0.0.1"
    }

    return "http://$bindHost`:$port/healthz"
}

function Get-HealthStatus {
    param(
        [Parameter(Mandatory = $true)]
        [string]$HealthUrl
    )

    try {
        $response = Invoke-WebRequest -Uri $HealthUrl -Method Get -TimeoutSec 2 -UseBasicParsing
        if ($response.StatusCode -eq 200) {
            return "ok"
        }
    }
    catch {
    }

    return "unreachable"
}

$root = Split-Path -Parent $PSScriptRoot
$configFile = Join-Path $root ".runtime\local-minimal\config\local-minimal.env"
$pidFile = Join-Path $root ".runtime\local-minimal\pids\local-minimal-node.pid"
$stdoutLog = Join-Path $root ".runtime\local-minimal\logs\local-minimal-node.out.log"
$stderrLog = Join-Path $root ".runtime\local-minimal\logs\local-minimal-node.err.log"
$bindAddress = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_BIND_ADDR"
if ([string]::IsNullOrWhiteSpace($bindAddress)) {
    $bindAddress = "127.0.0.1:18090"
}
$healthUrl = Get-HealthUrl -ResolvedBindAddress $bindAddress

$process = Get-RunningProcessFromPidFile -PidFile $pidFile

Write-Host "config: $configFile"
Write-Host "bind: $bindAddress"
Write-Host "health: $healthUrl"
Write-Host "stdout log: $stdoutLog"
Write-Host "stderr log: $stderrLog"
Write-Host "runtime inspection: powershell -ExecutionPolicy Bypass -File bin/inspect-runtime-local.ps1"
Write-Host "runtime repair: powershell -ExecutionPolicy Bypass -File bin/repair-runtime-local.ps1"
Write-Host "runtime backups: powershell -ExecutionPolicy Bypass -File bin/list-runtime-backups-local.ps1"
Write-Host "runtime restore preview: powershell -ExecutionPolicy Bypass -File bin/preview-runtime-restore-local.ps1 -BackupDir <path>"
Write-Host "runtime restore: powershell -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -BackupDir <path> -ExpectedPreviewFingerprint <previewFingerprint>"

if ($null -eq $process) {
    Write-Host "status: stopped"
    Write-Host "health status: stopped"
    exit 0
}

Write-Host "status: running"
Write-Host "pid: $($process.Id)"
Write-Host "health status: $(Get-HealthStatus -HealthUrl $healthUrl)"
