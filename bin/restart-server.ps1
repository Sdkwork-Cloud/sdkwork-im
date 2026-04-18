param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "CrawChat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
    [string]$BinaryPath,
    [switch]$Release,
    [switch]$Foreground,
    [string]$HealthUrl,
    [switch]$SkipHealthCheck,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restart-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-RunDir <path>] [-BinaryPath <path>] [-Release] [-Foreground] [-HealthUrl <url>] [-SkipHealthCheck]"
    Write-Host "Restart craw-chat-server using the stop/start runtime service scripts and preserve instance/config/status semantics."
    exit 0
}

$stopScript = Join-Path $PSScriptRoot "stop-server.ps1"
$startScript = Join-Path $PSScriptRoot "start-server.ps1"
& $stopScript -InstanceName $InstanceName -ConfigDir $ConfigDir -RunDir $RunDir | Out-Host
if ($null -ne $LASTEXITCODE -and $LASTEXITCODE -ne 0) {
    exit $LASTEXITCODE
}
& $startScript -InstanceName $InstanceName -InstallRoot $InstallRoot -ConfigDir $ConfigDir -LogDir $LogDir -RunDir $RunDir -BinaryPath $BinaryPath -Release:$Release -Foreground:$Foreground -HealthUrl $HealthUrl -SkipHealthCheck:$SkipHealthCheck
