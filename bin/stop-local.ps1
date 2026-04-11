param(
    [ValidateSet("local-minimal", "local-default")]
    [string]$ProfileName = "local-minimal",
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/stop-local.ps1 [-ProfileName <local-minimal|local-default>]"
    Write-Host "Usage: cmd /c .\bin\stop-local.cmd [--profile <local-minimal|local-default>]"
    Write-Host "Stop the local-minimal-node background process and remove the pid file."
    exit 0
}

function Get-RunningProcessFromPidFile {
    param(
        [Parameter(Mandatory = $true)]
        [string]$PidFile,
        [string]$ExpectedProcessName = "local-minimal-node"
    )

    if (-not (Test-Path $PidFile)) {
        return $null
    }

    $raw = (Get-Content -Path $PidFile -ErrorAction SilentlyContinue | Select-Object -First 1)
    if ([string]::IsNullOrWhiteSpace($raw)) {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    try {
        $process = Get-Process -Id ([int]$raw.Trim()) -ErrorAction Stop
    }
    catch {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    if (-not ($process.ProcessName -ieq $ExpectedProcessName)) {
        Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
        return $null
    }

    return $process
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

$runtimeDir = Resolve-RuntimeDirFromProfile -Root $root -ProfileName $ProfileName
$pidFile = Join-Path $runtimeDir "pids\local-minimal-node.pid"
$process = Get-RunningProcessFromPidFile -PidFile $pidFile

if ($null -eq $process) {
    Write-Host "local-minimal-node is not running."
    exit 0
}

Write-Host "Stopping local-minimal-node PID $($process.Id)"
Stop-Process -Id $process.Id -ErrorAction Stop

try {
    Wait-Process -Id $process.Id -Timeout 30 -ErrorAction Stop
}
catch {
    if ($null -ne (Get-Process -Id $process.Id -ErrorAction SilentlyContinue)) {
        throw "local-minimal-node PID $($process.Id) did not exit within 30 seconds."
    }
}

Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
Write-Host "local-minimal-node stopped."
