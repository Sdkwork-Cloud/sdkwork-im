param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [string]$RuntimeDir,
    [switch]$Json,
    [switch]$Release,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/repair-runtime-local.ps1 [-ProfileName <local-minimal|local-default>] [-RuntimeDir <path>] [-Json] [-Release]"
    Write-Host "Usage: cmd /c .\bin\repair-runtime-local.cmd [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]"
    Write-Host "Repair missing managed local runtime-dir state files for the selected local-minimal/local-default profile, then replay social journal truth through governance-service when state/social-commit-journal.json is present."
    exit 0
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
            $configRuntimeDir = Read-ConfigValue -ConfigFile $configFile -Key "SDKWORK_IM_RUNTIME_DIR"
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

function Resolve-ControlPlaneBinaryPath {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,
        [Parameter(Mandatory = $true)]
        [bool]$PreferRelease
    )

    $releasePath = Join-Path $Root "target\release\governance-service.exe"
    $debugPath = Join-Path $Root "target\debug\governance-service.exe"
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

$resolvedRuntimeDir = if ($PSBoundParameters.ContainsKey('RuntimeDir')) {
    $RuntimeDir
}
else {
    Resolve-RuntimeDirFromProfile -Root $root -ProfileName $ProfileName
}

$binaryPath = Resolve-BinaryPath -Root $root -PreferRelease:$Release
$repairArgs = @("repair-runtime-dir", "--runtime-dir", $resolvedRuntimeDir)
if ($Json) {
    $repairArgs += "--json"
}

if ($null -ne $binaryPath) {
    & $binaryPath @repairArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
elseif ($null -ne (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $cargoArgs = @("run", "-p", "local-minimal-node", "--offline", "--")
    $cargoArgs += $repairArgs
    cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}
else {
    throw "local-minimal-node binary not found under target\debug or target\release, and cargo is unavailable."
}

$socialJournalPath = Join-Path $resolvedRuntimeDir "state\social-commit-journal.json"
if (-not (Test-Path $socialJournalPath)) {
    exit 0
}

$socialRepairArgs = @("repair-social-runtime-dir", "--runtime-dir", $resolvedRuntimeDir)
if ($Json) {
    $socialRepairArgs += "--json"
}

$controlPlaneBinaryPath = Resolve-ControlPlaneBinaryPath -Root $root -PreferRelease:$Release
if ($null -ne $controlPlaneBinaryPath) {
    if ($Json) {
        $null = & $controlPlaneBinaryPath @socialRepairArgs
    }
    else {
        & $controlPlaneBinaryPath @socialRepairArgs
    }
    exit $LASTEXITCODE
}

if ($null -ne (Get-Command cargo -ErrorAction SilentlyContinue)) {
    $socialCargoArgs = @("run", "-p", "governance-service", "--offline", "--")
    $socialCargoArgs += $socialRepairArgs
    if ($Json) {
        $null = cargo @socialCargoArgs
    }
    else {
        cargo @socialCargoArgs
    }
    exit $LASTEXITCODE
}

throw "social commit journal exists at $socialJournalPath, but governance-service binary was not found under target\debug or target\release and cargo is unavailable."
