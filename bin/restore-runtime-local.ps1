param(
    [string]$RuntimeDir,
    [string]$BackupDir,
    [string]$ExpectedPreviewFingerprint,
    [switch]$Json,
    [switch]$Release,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -BackupDir <path> [-RuntimeDir <path>] [-ExpectedPreviewFingerprint <value>] [-Json] [-Release]"
    Write-Host "Restore managed local-minimal runtime-dir state files from an explicit backup snapshot through the local-minimal-node restore entrypoint."
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

function Resolve-BinaryPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,
        [Parameter(Mandatory = $true)]
        [bool]$PreferRelease
    )

    $releasePath = Join-Path $Root "target\release\local-minimal-node.exe"
    $debugPath = Join-Path $Root "target\debug\local-minimal-node.exe"
    $candidates = if ($PreferRelease) {
        @($releasePath, $debugPath)
    }
    else {
        @($debugPath, $releasePath)
    }

    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) {
            return $candidate
        }
    }

    return $null
}

if ([string]::IsNullOrWhiteSpace($BackupDir)) {
    throw "BackupDir is required. Use -BackupDir <path>."
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$configFile = Join-Path $root ".runtime\local-minimal\config\local-minimal.env"
$resolvedRuntimeDir = if ($PSBoundParameters.ContainsKey('RuntimeDir')) {
    $RuntimeDir
}
else {
    $configRuntimeDir = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_RUNTIME_DIR"
    if ([string]::IsNullOrWhiteSpace($configRuntimeDir)) {
        Join-Path $root ".runtime\local-minimal"
    }
    else {
        $configRuntimeDir
    }
}

$binaryPath = Resolve-BinaryPath -Root $root -PreferRelease:$Release
$restoreArgs = @("restore-runtime-dir", "--runtime-dir", $resolvedRuntimeDir, "--backup-dir", $BackupDir)
if (-not [string]::IsNullOrWhiteSpace($ExpectedPreviewFingerprint)) {
    $restoreArgs += @("--expected-preview-fingerprint", $ExpectedPreviewFingerprint)
}
if ($Json) {
    $restoreArgs += "--json"
}

if ($null -ne $binaryPath) {
    & $binaryPath @restoreArgs
    exit $LASTEXITCODE
}

if ($null -ne (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $cargoArgs = @("run", "-p", "local-minimal-node", "--offline", "--")
    $cargoArgs += $restoreArgs
    cargo @cargoArgs
    exit $LASTEXITCODE
}

throw "local-minimal-node binary not found under target\debug or target\release, and cargo is unavailable."
