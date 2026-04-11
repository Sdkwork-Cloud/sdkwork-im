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

    # local-default still reuses the current local-minimal runtime contract until it owns a dedicated topology.
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
