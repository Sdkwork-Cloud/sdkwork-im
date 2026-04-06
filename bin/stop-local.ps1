param(
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/stop-local.ps1"
    Write-Host "Stop the local-minimal-node background process and remove the pid file."
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

$root = Split-Path -Parent $PSScriptRoot
$pidFile = Join-Path $root ".runtime\local-minimal\pids\local-minimal-node.pid"
$process = Get-RunningProcessFromPidFile -PidFile $pidFile

if ($null -eq $process) {
    Write-Host "local-minimal-node is not running."
    exit 0
}

Write-Host "Stopping local-minimal-node PID $($process.Id)"
Stop-Process -Id $process.Id -ErrorAction Stop

try {
    Wait-Process -Id $process.Id -Timeout 30 -ErrorAction Stop
}
catch {
    if ($null -ne (Get-Process -Id $process.Id -ErrorAction SilentlyContinue)) {
        throw "local-minimal-node PID $($process.Id) did not exit within 30 seconds."
    }
}

Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
Write-Host "local-minimal-node stopped."
