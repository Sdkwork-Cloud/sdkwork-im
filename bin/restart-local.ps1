param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [switch]$Release,
    [switch]$Foreground,
    [string]$BindAddress,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restart-local.ps1 [-ProfileName <local-minimal|local-default>] [-Release] [-Foreground] [-BindAddress <host:port>]"
    Write-Host "Usage: cmd /c .\bin\restart-local.cmd [--profile <local-minimal|local-default>] [--release] [--foreground] [--bind-addr <host:port>]"
    Write-Host "Restart local-minimal-node using the stop/start lifecycle scripts."
    exit 0
}

$stopScript = Join-Path $PSScriptRoot "stop-local.ps1"
$startScript = Join-Path $PSScriptRoot "start-local.ps1"

& $stopScript -ProfileName $ProfileName | Out-Host
$stopExitCode = if ($null -eq $LASTEXITCODE) { 0 } else { $LASTEXITCODE }
if ($stopExitCode -ne 0) {
    exit $stopExitCode
}

if ($PSBoundParameters.ContainsKey('BindAddress')) {
    & $startScript -ProfileName $ProfileName -Release:$Release -Foreground:$Foreground -BindAddress $BindAddress
}
else {
    & $startScript -ProfileName $ProfileName -Release:$Release -Foreground:$Foreground
}
