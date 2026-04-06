param(
    [switch]$Release,
    [string]$BindAddress = "127.0.0.1:18090",
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/install-local.ps1 [-Release] [-BindAddress <host:port>]"
    Write-Host "Build local-minimal-node offline, initialize config, and prepare .runtime directories."
    exit 0
}

function Test-Command {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Name
    )

    return $null -ne (Get-Command $Name -ErrorAction SilentlyContinue)
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

if (-not (Test-Command -Name "cargo")) {
    throw "cargo is unavailable. Install the Rust toolchain and ensure cargo is on PATH."
}

$runtimeRoot = Join-Path $root ".runtime"
$serviceRoot = Join-Path $runtimeRoot "local-minimal"
$configDir = Join-Path $serviceRoot "config"
$logsDir = Join-Path $serviceRoot "logs"
$pidsDir = Join-Path $serviceRoot "pids"

foreach ($path in @($runtimeRoot, $serviceRoot, $configDir, $logsDir, $pidsDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path | Out-Null
    }
}

$initConfigScript = Join-Path $PSScriptRoot "init-config-local.ps1"
$bindAddressProvided = $PSBoundParameters.ContainsKey('BindAddress')
& $initConfigScript -BindAddress $BindAddress -Force:$bindAddressProvided | Out-Host

if ($Release) {
    Write-Host "Building local-minimal-node in release mode..."
    cargo build --release -p local-minimal-node --offline | Out-Host
}
else {
    Write-Host "Building local-minimal-node in debug mode..."
    cargo build -p local-minimal-node --offline | Out-Host
}

Write-Host "Runtime directories prepared under $serviceRoot"
