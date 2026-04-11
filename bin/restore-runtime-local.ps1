param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [string]$RuntimeDir,
    [string]$BackupDir,
    [string]$ExpectedPreviewFingerprint,
    [switch]$Json,
    [switch]$Release,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/restore-runtime-local.ps1 -BackupDir <path> [-ProfileName <local-minimal|local-default>] [-RuntimeDir <path>] [-ExpectedPreviewFingerprint <value>] [-Json] [-Release]"
    Write-Host "Restore managed local runtime-dir state files for the selected local-minimal/local-default profile from an explicit backup snapshot through the local-minimal-node restore entrypoint."
    exit 0
}

if ([string]::IsNullOrWhiteSpace($BackupDir)) {
    throw "BackupDir is required. Use -BackupDir <path>."
}

$root = Split-Path -Parent $PSScriptRoot
$runtimeProfileHelper = Join-Path $PSScriptRoot "_runtime-profile-common.ps1"
if (Test-Path $runtimeProfileHelper) {
    . $runtimeProfileHelper
}
else {
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

    function Resolve-RuntimeDirFromProfile {
        param(
            [Parameter(Mandatory = $true)]
            [string]$Root,
            [Parameter(Mandatory = $true)]
            [ValidateSet("local-minimal", "local-default")]
            [string]$ProfileName
        )

        $configFiles = if ($ProfileName -eq "local-default") {
            @(
                (Join-Path $Root ".runtime\local-default\config\local-default.env"),
                (Join-Path $Root ".runtime\local-minimal\config\local-minimal.env")
            )
        }
        else {
            @((Join-Path $Root ".runtime\local-minimal\config\local-minimal.env"))
        }

        foreach ($configFile in $configFiles) {
            $configRuntimeDir = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_RUNTIME_DIR"
            if (-not [string]::IsNullOrWhiteSpace($configRuntimeDir)) {
                return $configRuntimeDir
            }
        }

        return Join-Path $Root ".runtime\local-minimal"
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
}
Set-Location $root

$resolvedRuntimeDir = if ($PSBoundParameters.ContainsKey('RuntimeDir')) {
    $RuntimeDir
}
else {
    Resolve-RuntimeDirFromProfile -Root $root -ProfileName $ProfileName
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
