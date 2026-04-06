param(
    [switch]$Release,
    [switch]$Foreground,
    [string]$BindAddress,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/start-local.ps1 [-Release] [-Foreground] [-BindAddress <host:port>]"
    Write-Host "Build and start local-minimal-node with config, pid/log management, and health wait."
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

function Stop-ManagedProcessAndRemovePidFile {
    param(
        [Parameter(Mandatory = $true)]
        [int]$ProcessId,
        [Parameter(Mandatory = $true)]
        [string]$PidFile,
        [string]$ExpectedProcessName = "local-minimal-node"
    )

    try {
        $process = Get-Process -Id $ProcessId -ErrorAction Stop
        if ($process.ProcessName -ieq $ExpectedProcessName) {
            Stop-Process -Id $ProcessId -Force -ErrorAction SilentlyContinue
            try {
                Wait-Process -Id $ProcessId -Timeout 5 -ErrorAction Stop
            }
            catch {
            }
        }
    }
    catch {
    }

    Remove-Item -Path $PidFile -Force -ErrorAction SilentlyContinue
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

function Get-HealthUrl {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBindAddress
    )

    $segments = $ResolvedBindAddress -split ':'
    if ($segments.Length -lt 2) {
        return "http://127.0.0.1:18090/healthz"
    }

    $port = $segments[-1]
    $bindHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($bindHost) -or $bindHost -eq "0.0.0.0" -or $bindHost -eq "::" -or $bindHost -eq "[::]") {
        $bindHost = "127.0.0.1"
    }

    return "http://$bindHost`:$port/healthz"
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$configFile = Join-Path $root ".runtime\local-minimal\config\local-minimal.env"
$installScript = Join-Path $PSScriptRoot "install-local.ps1"
$bindAddressProvided = $PSBoundParameters.ContainsKey('BindAddress')
$installArgs = @()
if ($Release) {
    $installArgs += "-Release"
}
if ($bindAddressProvided) {
    $installArgs += "-BindAddress"
    $installArgs += $BindAddress
}
& $installScript @installArgs

$profileDir = if ($Release) { "release" } else { "debug" }
$exeName = "local-minimal-node.exe"
$exePath = Join-Path $root "target\$profileDir\$exeName"

if (-not (Test-Path $exePath)) {
    throw "Binary not found: $exePath"
}

$runtimeDir = Join-Path $root ".runtime\local-minimal"
$logsDir = Join-Path $runtimeDir "logs"
$pidsDir = Join-Path $runtimeDir "pids"
$pidFile = Join-Path $pidsDir "local-minimal-node.pid"
$stdoutLog = Join-Path $logsDir "local-minimal-node.out.log"
$stderrLog = Join-Path $logsDir "local-minimal-node.err.log"

foreach ($logPath in @($stdoutLog, $stderrLog)) {
    if (-not (Test-Path $logPath)) {
        New-Item -ItemType File -Path $logPath | Out-Null
    }
}

$runningProcess = Get-RunningProcessFromPidFile -PidFile $pidFile
if ($null -ne $runningProcess) {
    throw "local-minimal-node is already running with PID $($runningProcess.Id). Stop it before starting a new instance."
}

$configBindAddress = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_BIND_ADDR"
$resolvedBindAddress = if ([string]::IsNullOrWhiteSpace($BindAddress)) { $configBindAddress } else { $BindAddress }
if ([string]::IsNullOrWhiteSpace($resolvedBindAddress)) {
    $resolvedBindAddress = "127.0.0.1:18090"
}
$configRuntimeDir = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_RUNTIME_DIR"
$resolvedRuntimeDir = if ([string]::IsNullOrWhiteSpace($configRuntimeDir)) {
    $runtimeDir
}
else {
    $configRuntimeDir
}
$configPublicBearerSecret = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"
$resolvedPublicBearerSecret = if ([string]::IsNullOrWhiteSpace($configPublicBearerSecret)) {
    $env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET
}
else {
    $configPublicBearerSecret
}
if ([string]::IsNullOrWhiteSpace($resolvedPublicBearerSecret)) {
    throw "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET must be configured before starting local-minimal-node."
}

$previousBindAddress = $env:CRAW_CHAT_BIND_ADDR
$hadPreviousBindAddress = $null -ne $previousBindAddress
$previousRuntimeDir = $env:CRAW_CHAT_RUNTIME_DIR
$hadPreviousRuntimeDir = $null -ne $previousRuntimeDir
$previousPublicBearerSecret = $env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET
$hadPreviousPublicBearerSecret = $null -ne $previousPublicBearerSecret
$env:CRAW_CHAT_BIND_ADDR = $resolvedBindAddress
$env:CRAW_CHAT_RUNTIME_DIR = $resolvedRuntimeDir
$env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET = $resolvedPublicBearerSecret

try {
    if ($Foreground) {
        Write-Host "Starting local-minimal-node in foreground on http://$resolvedBindAddress"
        & $exePath
        exit $LASTEXITCODE
    }

    Write-Host "Starting local-minimal-node in background on http://$resolvedBindAddress"
    $process = Start-Process `
        -FilePath $exePath `
        -WorkingDirectory $root `
        -PassThru `
        -RedirectStandardOutput $stdoutLog `
        -RedirectStandardError $stderrLog `
        -WindowStyle Hidden

    try {
        Set-Content -Path $pidFile -Value $process.Id

        $healthUrl = Get-HealthUrl -ResolvedBindAddress $resolvedBindAddress
        $ready = $false
        for ($attempt = 0; $attempt -lt 30; $attempt++) {
            Start-Sleep -Seconds 1

            $liveProcess = Get-RunningProcessFromPidFile -PidFile $pidFile
            if ($null -eq $liveProcess) {
                throw "local-minimal-node exited before becoming ready. Check logs: $stderrLog"
            }

            try {
                $response = Invoke-WebRequest -Uri $healthUrl -Method Get -TimeoutSec 2 -UseBasicParsing
                if ($response.StatusCode -eq 200) {
                    $ready = $true
                    break
                }
            }
            catch {
            }
        }
        if (-not $ready) {
            throw "local-minimal-node did not become healthy within 30 seconds: $healthUrl"
        }

        Write-Host "PID: $($process.Id)"
        Write-Host "stdout log: $stdoutLog"
        Write-Host "stderr log: $stderrLog"
        Write-Host "pid file: $pidFile"
        Write-Host "health: $healthUrl"
    }
    catch {
        Stop-ManagedProcessAndRemovePidFile -ProcessId $process.Id -PidFile $pidFile
        throw
    }
}
finally {
    if ($hadPreviousBindAddress) {
        $env:CRAW_CHAT_BIND_ADDR = $previousBindAddress
    }
    else {
        Remove-Item Env:CRAW_CHAT_BIND_ADDR -ErrorAction SilentlyContinue
    }

    if ($hadPreviousRuntimeDir) {
        $env:CRAW_CHAT_RUNTIME_DIR = $previousRuntimeDir
    }
    else {
        Remove-Item Env:CRAW_CHAT_RUNTIME_DIR -ErrorAction SilentlyContinue
    }

    if ($hadPreviousPublicBearerSecret) {
        $env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET = $previousPublicBearerSecret
    }
    else {
        Remove-Item Env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET -ErrorAction SilentlyContinue
    }
}
