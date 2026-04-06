param(
    [string]$BindAddress = "127.0.0.1:18090",
    [switch]$Force,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-config-local.ps1 [-BindAddress <host:port>] [-Force]"
    Write-Host "Create or update the local-minimal runtime config file."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$runtimeDir = Join-Path $root ".runtime\local-minimal"
$configDir = Join-Path $runtimeDir "config"
$logsDir = Join-Path $runtimeDir "logs"
$pidsDir = Join-Path $runtimeDir "pids"
$stateDir = Join-Path $runtimeDir "state"
$configFile = Join-Path $configDir "local-minimal.env"

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

foreach ($path in @($runtimeDir, $configDir, $logsDir, $pidsDir, $stateDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path | Out-Null
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
    "# local-minimal runtime config"
    "CRAW_CHAT_BIND_ADDR=$BindAddress"
    "CRAW_CHAT_RUNTIME_DIR=$runtimeDir"
    "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET=$publicBearerSecret"
)

Set-Content -Path $configFile -Value $content
Write-Host "Config written: $configFile"
