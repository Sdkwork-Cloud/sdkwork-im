param(
    [switch]$Release,
    [switch]$Foreground,
    [string]$BindAddress,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restart-local.ps1 [-Release] [-Foreground] [-BindAddress <host:port>]"
    Write-Host "Restart local-minimal-node using the stop/start lifecycle scripts."
    exit 0
}

$stopScript = Join-Path $PSScriptRoot "stop-local.ps1"
$startScript = Join-Path $PSScriptRoot "start-local.ps1"

& $stopScript | Out-Host
$stopExitCode = if ($null -eq $LASTEXITCODE) { 0 } else { $LASTEXITCODE }
if ($stopExitCode -ne 0) {
    exit $stopExitCode
}

$startArgs = @()
if ($Release) {
    $startArgs += "-Release"
}
if ($Foreground) {
    $startArgs += "-Foreground"
}
if ($PSBoundParameters.ContainsKey('BindAddress')) {
    $startArgs += "-BindAddress"
    $startArgs += $BindAddress
}
& $startScript @startArgs
