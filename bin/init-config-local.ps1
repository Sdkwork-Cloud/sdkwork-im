param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [string]$BindAddress = "127.0.0.1:18090",
    [switch]$Force,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-config-local.ps1 [-ProfileName <local-minimal|local-default>] [-BindAddress <host:port>] [-Force]"
    Write-Host "Usage: cmd /c .\bin\init-config-local.cmd [--profile <local-minimal|local-default>] [--bind-addr <host:port>] [--force]"
    Write-Host "Create or update the selected local runtime config file."
    exit 0
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

function New-PublicBearerSecret {
    $bytes = New-Object byte[] 32
    $rng = [System.Security.Cryptography.RandomNumberGenerator]::Create()
    try {
        $rng.GetBytes($bytes)
    }
    finally {
        $rng.Dispose()
    }

    return ([Convert]::ToBase64String($bytes)).TrimEnd('=').Replace('+', '-').Replace('/', '_')
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

function Resolve-PrimaryConfigFileFromProfile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,
        [Parameter(Mandatory = $true)]
        [ValidateSet("local-minimal", "local-default")]
        [string]$ProfileName
    )

    return @(Resolve-RuntimeProfileConfigFiles -Root $Root -ProfileName $ProfileName)[0]
}

$configFile = Resolve-PrimaryConfigFileFromProfile -Root $root -ProfileName $ProfileName
$configDir = Split-Path -Parent $configFile
$runtimeDir = Resolve-RuntimeDirFromProfile -Root $root -ProfileName $ProfileName
$logsDir = Join-Path $runtimeDir "logs"
$pidsDir = Join-Path $runtimeDir "pids"
$stateDir = Join-Path $runtimeDir "state"

foreach ($path in @($configDir, $runtimeDir, $logsDir, $pidsDir, $stateDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

if ((Test-Path $configFile) -and -not $Force) {
    Write-Host "Config already exists: $configFile"
    exit 0
}

$publicBearerSecret = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"
if ([string]::IsNullOrWhiteSpace($publicBearerSecret)) {
    $publicBearerSecret = New-PublicBearerSecret
}

$content = @(
    "# $ProfileName runtime config"
    "CRAW_CHAT_BIND_ADDR=$BindAddress"
    "CRAW_CHAT_RUNTIME_DIR=$runtimeDir"
    "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=$publicBearerSecret"
)

Set-Content -Path $configFile -Value $content
Write-Host "Config written: $configFile"
