param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [switch]$Release,
    [string]$BindAddress = "127.0.0.1:18090",
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/install-local.ps1 [-ProfileName <local-minimal|local-default>] [-Release] [-BindAddress <host:port>]"
    Write-Host "Usage: cmd /c .\bin\install-local.cmd [--profile <local-minimal|local-default>] [--release] [--bind-addr <host:port>]"
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

$root = Split-Path -Parent $PSScriptRoot
$runtimeProfileHelper = Join-Path $PSScriptRoot "_runtime-profile-common.ps1"
if (Test-Path $runtimeProfileHelper) {
    . $runtimeProfileHelper
}
else {
    function Resolve-RuntimeProfileConfigFiles {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root,
            [Parameter(Mandatory = $true)]
            [ValidateSet("local-minimal", "local-default")]
            [string]$ProfileName
        )

        switch ($ProfileName) {
            "local-default" {
                return @(
                    (Join-Path $Root ".runtime\local-default\config\local-default.env"),
                    (Join-Path $Root ".runtime\local-minimal\config\local-minimal.env")
                )
            }
            default {
                return @(
                    (Join-Path $Root ".runtime\local-minimal\config\local-minimal.env")
                )
            }
        }
    }

    function Resolve-RuntimeDirFromProfile {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root,
            [Parameter(Mandatory = $true)]
            [ValidateSet("local-minimal", "local-default")]
            [string]$ProfileName
        )

        foreach ($configFile in Resolve-RuntimeProfileConfigFiles -Root $Root -ProfileName $ProfileName) {
            $configRuntimeDir = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_RUNTIME_DIR"
            if (-not [string]::IsNullOrWhiteSpace($configRuntimeDir)) {
                return $configRuntimeDir
            }
        }

        return Join-Path $Root ".runtime\local-minimal"
    }
}

Set-Location $root

if (-not (Test-Command -Name "cargo")) {
    throw "cargo is unavailable. Install the Rust toolchain and ensure cargo is on PATH."
}

$initConfigScript = Join-Path $PSScriptRoot "init-config-local.ps1"
$bindAddressProvided = $PSBoundParameters.ContainsKey('BindAddress')
& $initConfigScript -ProfileName $ProfileName -BindAddress $BindAddress -Force:$bindAddressProvided | Out-Host

$serviceRoot = Resolve-RuntimeDirFromProfile -Root $root -ProfileName $ProfileName
$logsDir = Join-Path $serviceRoot "logs"
$pidsDir = Join-Path $serviceRoot "pids"

foreach ($path in @($serviceRoot, $logsDir, $pidsDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

if ($Release) {
    Write-Host "Building local-minimal-node in release mode..."
    cargo build --release -p local-minimal-node --offline | Out-Host
}
else {
    Write-Host "Building local-minimal-node in debug mode..."
    cargo build -p local-minimal-node --offline | Out-Host
}

Write-Host "Runtime directories prepared under $serviceRoot"
