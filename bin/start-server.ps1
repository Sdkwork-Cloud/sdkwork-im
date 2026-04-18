param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "CrawChat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
    [string]$BinaryPath,
    [switch]$Release,
    [switch]$Foreground,
    [string]$HealthUrl,
    [switch]$SkipHealthCheck,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "config")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("LogDir")) {
    $LogDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "logs")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "run")
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/start-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-LogDir <path>] [-RunDir <path>] [-BinaryPath <path>] [-Release] [-Foreground] [-HealthUrl <url>] [-SkipHealthCheck]"
    Write-Host "Usage: cmd /c .\bin\start-server.cmd [--instance <name>] [--install-root <path>] [--config-dir <path>] [--log-dir <path>] [--run-dir <path>] [--binary-path <path>] [--release] [--foreground] [--health-url <url>] [--skip-health-check]"
    Write-Host "Start the craw-chat-server runtime service for an instance with config loading, binary resolution, log and run directory management, health checks, and status-friendly foreground or background execution."
    exit 0
}

function Read-ConfigValue {
    param([string]$ConfigFile, [string]$Key)
    if (-not (Test-Path $ConfigFile)) { return $null }
    foreach ($line in Get-Content -Path $ConfigFile) {
        if ($line -match "^\s*$Key:\s*(.+?)\s*$") {
            return $Matches[1].Trim().Trim('"')
        }
    }
    return $null
}

function Resolve-ServerBinaryPath {
    param([string]$Root, [string]$InstallRoot, [string]$ExplicitBinaryPath, [bool]$PreferRelease)
    if (-not [string]::IsNullOrWhiteSpace($ExplicitBinaryPath) -and (Test-Path $ExplicitBinaryPath)) {
        return $ExplicitBinaryPath
    }
    $envBinaryPath = [Environment]::GetEnvironmentVariable("CRAW_CHAT_SERVER_BINARY_PATH")
    if (-not [string]::IsNullOrWhiteSpace($envBinaryPath) -and (Test-Path $envBinaryPath)) {
        return $envBinaryPath
    }

    $installCandidates = @(
        (Join-Path $InstallRoot "bin\craw-chat-server.exe"),
        (Join-Path $InstallRoot "bin\web-gateway.exe")
    )
    foreach ($candidate in $installCandidates) {
        if (Test-Path $candidate) { return $candidate }
    }

    $releaseCandidate = Join-Path $Root "target\release\craw-chat-server.exe"
    $debugCandidate = Join-Path $Root "target\debug\craw-chat-server.exe"
    $legacyReleaseCandidate = Join-Path $Root "target\release\web-gateway.exe"
    $legacyDebugCandidate = Join-Path $Root "target\debug\web-gateway.exe"
    $candidates = if ($PreferRelease) {
        @($releaseCandidate, $debugCandidate, $legacyReleaseCandidate, $legacyDebugCandidate)
    }
    else {
        @($debugCandidate, $releaseCandidate, $legacyDebugCandidate, $legacyReleaseCandidate)
    }
    foreach ($candidate in $candidates) {
        if (Test-Path $candidate) { return $candidate }
    }

    $cargo = Get-Command cargo -ErrorAction SilentlyContinue
    if ($null -ne $cargo) {
        if ($PreferRelease) {
            cargo build --release -p web-gateway --offline | Out-Host
        }
        else {
            cargo build -p web-gateway --offline | Out-Host
        }
        foreach ($candidate in $candidates) {
            if (Test-Path $candidate) { return $candidate }
        }
    }

    return $null
}

function Resolve-HealthUrl {
    param([string]$ExplicitHealthUrl, [string]$ResolvedBindAddress)
    if (-not [string]::IsNullOrWhiteSpace($ExplicitHealthUrl)) { return $ExplicitHealthUrl }
    $segments = $ResolvedBindAddress -split ':'
    if ($segments.Length -lt 2) { return "http://127.0.0.1:18079/healthz" }
    $port = $segments[-1]
    $bindHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($bindHost) -or $bindHost -eq "0.0.0.0" -or $bindHost -eq "::" -or $bindHost -eq "[::]") {
        $bindHost = "127.0.0.1"
    }
    return "http://$bindHost`:$port/healthz"
}

function Get-ManagedProcess {
    param([string]$PidFile, [string]$ExpectedProcessName)
    if (-not (Test-Path $PidFile)) { return $null }
    $raw = Get-Content -Path $PidFile -ErrorAction SilentlyContinue | Select-Object -First 1
    if ([string]::IsNullOrWhiteSpace($raw)) { return $null }
    try {
        $process = Get-Process -Id ([int]$raw.Trim()) -ErrorAction Stop
        if (-not [string]::IsNullOrWhiteSpace($ExpectedProcessName) -and $process.ProcessName -ine $ExpectedProcessName) {
            return $null
        }
        return $process
    }
    catch { return $null }
}

$root = Split-Path -Parent $PSScriptRoot
$serverYamlPath = Join-Path $ConfigDir "server.yaml"
if (-not (Test-Path $serverYamlPath)) {
    throw "Missing server config. Run init-config-server first: $serverYamlPath"
}

$resolvedBindAddress = Read-ConfigValue -ConfigFile $serverYamlPath -Key "bindAddress"
if ([string]::IsNullOrWhiteSpace($resolvedBindAddress)) {
    $resolvedBindAddress = "127.0.0.1:18079"
}
$resolvedBinaryPath = Resolve-ServerBinaryPath -Root $root -InstallRoot $InstallRoot -ExplicitBinaryPath $BinaryPath -PreferRelease:$Release
if ([string]::IsNullOrWhiteSpace($resolvedBinaryPath)) {
    throw "Unable to resolve craw-chat-server binary. Set -BinaryPath, install a packaged binary under $InstallRoot, or build web-gateway."
}

$resolvedHealthUrl = Resolve-HealthUrl -ExplicitHealthUrl $HealthUrl -ResolvedBindAddress $resolvedBindAddress
$stdoutLog = Join-Path $LogDir "craw-chat-server.out.log"
$stderrLog = Join-Path $LogDir "craw-chat-server.err.log"
$pidFile = Join-Path $RunDir "craw-chat-server.pid"
$processInfoPath = Join-Path $RunDir "craw-chat-server.process.json"
foreach ($path in @($LogDir, $RunDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$expectedProcessName = [System.IO.Path]::GetFileNameWithoutExtension($resolvedBinaryPath)
$existing = Get-ManagedProcess -PidFile $pidFile -ExpectedProcessName $expectedProcessName
if ($null -ne $existing) {
    throw "craw-chat-server is already running with PID $($existing.Id)."
}

$env:CRAW_CHAT_WEB_GATEWAY_BIND = $resolvedBindAddress
$serverArguments = @("--config", $serverYamlPath)

if ($Foreground) {
    & $resolvedBinaryPath @serverArguments
    exit $LASTEXITCODE
}

$process = Start-Process -FilePath $resolvedBinaryPath -ArgumentList $serverArguments -WorkingDirectory $root -PassThru -RedirectStandardOutput $stdoutLog -RedirectStandardError $stderrLog
$process.Id | Set-Content -Path $pidFile -Encoding utf8
([ordered]@{
    binaryPath = $resolvedBinaryPath
    processName = $expectedProcessName
    bindAddress = $resolvedBindAddress
    healthUrl = $resolvedHealthUrl
} | ConvertTo-Json -Depth 4) | Set-Content -Path $processInfoPath -Encoding utf8

if (-not $SkipHealthCheck) {
    for ($i = 0; $i -lt 30; $i++) {
        Start-Sleep -Seconds 1
        if ($null -eq (Get-Process -Id $process.Id -ErrorAction SilentlyContinue)) {
            Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
            throw "craw-chat-server exited before becoming healthy. Check logs: $stderrLog"
        }
        try {
            $response = Invoke-WebRequest -Uri $resolvedHealthUrl -UseBasicParsing -TimeoutSec 2
            if ($response.StatusCode -eq 200) {
                Write-Host "Started craw-chat-server in background on $resolvedHealthUrl"
                exit 0
            }
        }
        catch { }
    }
    Stop-Process -Id $process.Id -Force -ErrorAction SilentlyContinue
    Remove-Item -Path $pidFile -Force -ErrorAction SilentlyContinue
    throw "craw-chat-server did not become healthy within 30 seconds: $resolvedHealthUrl"
}

Write-Host "Started craw-chat-server in background without health wait."
