param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "config")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "run")
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/stop-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-RunDir <path>]"
    Write-Host "Stop the craw-chat-server runtime service for an instance by using the pid file under the run directory, honoring config ownership, and reporting status."
    exit 0
}

$pidFile = Join-Path $RunDir "craw-chat-server.pid"
$processInfoPath = Join-Path $RunDir "craw-chat-server.process.json"
if (-not (Test-Path $pidFile)) {
    Write-Host "craw-chat-server is not running."
    exit 0
}

$rawPid = Get-Content -Path $pidFile -ErrorAction SilentlyContinue | Select-Object -First 1
if ([string]::IsNullOrWhiteSpace($rawPid)) {
    Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
    Write-Host "craw-chat-server pid file was empty and has been cleared."
    exit 0
}

try {
    $pid = [int]$rawPid.Trim()
    $process = Get-Process -Id $pid -ErrorAction Stop
    Stop-Process -Id $pid -ErrorAction Stop
    try { Wait-Process -Id $pid -Timeout 30 -ErrorAction Stop } catch { }
    Write-Host "Stopped craw-chat-server PID $pid"
}
catch {
    Write-Host "craw-chat-server process from pid file is not running."
}

Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
Remove-Item -Path $processInfoPath -Force -ErrorAction SilentlyContinue
