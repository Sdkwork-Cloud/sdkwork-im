param(
    [string]$BaseUrl,
    [string]$TenantId = "t_demo",
    [string]$ConversationId,
    [string]$OwnerUserId = "u_owner",
    [string]$GuestUserId = "u_guest",
    [string]$OwnerLabel = "owner",
    [string]$GuestLabel = "guest",
    [switch]$Release,
    [switch]$SkipStart,
    [switch]$UseConsoleWindows,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

function Test-ChatHealth {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Url
    )

    try {
        $response = Invoke-WebRequest -Uri "$Url/healthz" -Method Get -TimeoutSec 2 -UseBasicParsing
        return $response.StatusCode -eq 200
    }
    catch {
        return $false
    }
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

function Resolve-BaseUrl {
    param(
        [string]$RequestedBaseUrl
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedBaseUrl)) {
        return $RequestedBaseUrl
    }

    $configFile = Join-Path (Split-Path -Parent $PSScriptRoot) ".runtime\local-minimal\config\local-minimal.env"
    $bindAddress = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_BIND_ADDR"
    if ([string]::IsNullOrWhiteSpace($bindAddress)) {
        return "http://127.0.0.1:18090"
    }

    $segments = $bindAddress -split ':'
    $port = $segments[-1]
    $resolvedHost = ($segments[0..($segments.Length - 2)] -join ':').Trim()
    if ([string]::IsNullOrWhiteSpace($resolvedHost) -or $resolvedHost -eq "0.0.0.0" -or $resolvedHost -eq "::" -or $resolvedHost -eq "[::]") {
        $resolvedHost = "127.0.0.1"
    }
    return "http://$resolvedHost`:$port"
}

function Quote-ProcessArgument {
    param(
        [AllowNull()]
        [string]$Value
    )

    if ($null -eq $Value) {
        return '""'
    }

    if ($Value -notmatch '[\s"]') {
        return $Value
    }

    return '"' + ($Value -replace '"', '\"') + '"'
}

function Invoke-ChatCli {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments
    )

    $allArgs = @()
    if ($Release) {
        $allArgs += "-Release"
    }
    $allArgs += $Arguments

    & "$PSScriptRoot\chat-cli.ps1" @allArgs
    if ($LASTEXITCODE -ne 0) {
        throw "chat-cli invocation failed with exit code ${LASTEXITCODE}: $($Arguments -join ' ')"
    }
}

function Start-DetachedPowerShellWindow {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$ArgumentList
    )

    $commandLine = "powershell.exe " + (($ArgumentList | ForEach-Object {
                Quote-ProcessArgument $_
            }) -join ' ')

    try {
        $createResult = Invoke-CimMethod -ClassName Win32_Process -MethodName Create -Arguments @{
            CommandLine = $commandLine
        }
        if ($createResult.ReturnValue -eq 0) {
            return $createResult.ProcessId
        }
    }
    catch {
    }

    $tempScript = Join-Path ([System.IO.Path]::GetTempPath()) ("craw-chat-detached-launch-{0}.vbs" -f ([guid]::NewGuid().ToString("N")))
    $escapedCommandLine = $commandLine.Replace("""", """""")
    @"
Set shell = CreateObject("WScript.Shell")
shell.Run "$escapedCommandLine", 1, False
"@ | Set-Content -Path $tempScript -Encoding ASCII

    Start-Process -FilePath "wscript.exe" -ArgumentList @($tempScript) | Out-Null
    return $null
}

function Start-HostedLocalService {
    $root = Split-Path -Parent $PSScriptRoot
    $profileDir = if ($Release) { "release" } else { "debug" }
    $exePath = Join-Path $root "target\$profileDir\local-minimal-node.exe"

    if (-not (Test-Path $exePath)) {
        $installArgs = @()
        if ($Release) {
            $installArgs += "-Release"
        }
        & "$PSScriptRoot\install-local.ps1" @installArgs
        if ($LASTEXITCODE -ne 0) {
            throw "Failed to build local-minimal-node."
        }
    }

    $configFile = Join-Path $root ".runtime\local-minimal\config\local-minimal.env"
    $bindAddress = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_BIND_ADDR"
    $runtimeDir = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_RUNTIME_DIR"
    $publicBearerSecret = Read-ConfigValue -ConfigFile $configFile -Key "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"

    $hostScriptPath = Join-Path ([System.IO.Path]::GetTempPath()) "craw-chat-local-hosted-$profileDir.ps1"
    @"
`$env:CRAW_CHAT_BIND_ADDR = "$bindAddress"
`$env:CRAW_CHAT_RUNTIME_DIR = "$runtimeDir"
`$env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET = "$publicBearerSecret"
Set-Location "$root"
& "$exePath"
"@ | Set-Content -Path $hostScriptPath -Encoding UTF8

    Start-Process -FilePath "powershell.exe" -WindowStyle Hidden -ArgumentList @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", $hostScriptPath
    ) | Out-Null

    for ($attempt = 0; $attempt -lt 30; $attempt++) {
        Start-Sleep -Seconds 1
        if (Test-ChatHealth -Url $BaseUrl) {
            return
        }
    }

    throw "Hosted local-minimal-node did not become healthy at $BaseUrl"
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 [-ConversationId <id>] [-BaseUrl <url>] [-TenantId <id>] [-OwnerUserId <id>] [-GuestUserId <id>] [-OwnerLabel <label>] [-GuestLabel <label>] [-Release] [-SkipStart] [-UseConsoleWindows]"
    Write-Host "Create a local test conversation and open two visible chat windows for manual validation."
    exit 0
}

$BaseUrl = Resolve-BaseUrl -RequestedBaseUrl $BaseUrl

if ([string]::IsNullOrWhiteSpace($ConversationId)) {
    $ConversationId = "c_demo_{0}" -f (Get-Date -Format "yyyyMMddHHmmss")
}

$ownerSessionId = "s_$OwnerUserId"
$ownerDeviceId = "d_$OwnerUserId"
$guestSessionId = "s_$GuestUserId"
$guestDeviceId = "d_$GuestUserId"

if (-not $SkipStart -and -not (Test-ChatHealth -Url $BaseUrl)) {
    Write-Host "Local service is not healthy. Starting local-minimal-node..."
    Start-HostedLocalService
}

if (-not (Test-ChatHealth -Url $BaseUrl)) {
    throw "Chat service is not healthy at $BaseUrl"
}

Invoke-ChatCli -Arguments @(
    "--base-url", $BaseUrl,
    "--tenant-id", $TenantId,
    "--user-id", $OwnerUserId,
    "--session-id", $ownerSessionId,
    "--device-id", $ownerDeviceId,
    "create-conversation",
    "--conversation-id", $ConversationId,
    "--conversation-type", "group"
)

Invoke-ChatCli -Arguments @(
    "--base-url", $BaseUrl,
    "--tenant-id", $TenantId,
    "--user-id", $OwnerUserId,
    "--session-id", $ownerSessionId,
    "--device-id", $ownerDeviceId,
    "add-member",
    "--conversation-id", $ConversationId,
    "--principal-id", $GuestUserId,
    "--principal-kind", "user",
    "--role", "member"
)

$windowScript = if ($UseConsoleWindows) {
    Join-Path $PSScriptRoot "chat-window.ps1"
}
else {
    Join-Path $PSScriptRoot "chat-window-gui.ps1"
}
$ownerArgs = @(
    "-NoProfile",
    "-ExecutionPolicy", "Bypass",
    "-File", $windowScript,
    "-BaseUrl", $BaseUrl,
    "-TenantId", $TenantId,
    "-ConversationId", $ConversationId,
    "-UserId", $OwnerUserId,
    "-SessionId", $ownerSessionId,
    "-DeviceId", $ownerDeviceId,
    "-Label", $OwnerLabel,
    "-MessagePrefix", "[$OwnerLabel] "
)
if ($Release) {
    $ownerArgs += "-Release"
}
if ($UseConsoleWindows) {
    $ownerArgs = @("-NoExit") + $ownerArgs
}

$guestArgs = @(
    "-NoProfile",
    "-ExecutionPolicy", "Bypass",
    "-File", $windowScript,
    "-BaseUrl", $BaseUrl,
    "-TenantId", $TenantId,
    "-ConversationId", $ConversationId,
    "-UserId", $GuestUserId,
    "-SessionId", $guestSessionId,
    "-DeviceId", $guestDeviceId,
    "-Label", $GuestLabel,
    "-MessagePrefix", "[$GuestLabel] "
)
if ($Release) {
    $guestArgs += "-Release"
}
if ($UseConsoleWindows) {
    $guestArgs = @("-NoExit") + $guestArgs
}

$ownerProcessId = Start-DetachedPowerShellWindow -ArgumentList $ownerArgs
$guestProcessId = Start-DetachedPowerShellWindow -ArgumentList $guestArgs

Write-Host "Opened two chat windows."
Write-Host "conversationId: $ConversationId"
Write-Host "owner: $OwnerUserId"
Write-Host "guest: $GuestUserId"
if ($null -ne $ownerProcessId) {
    Write-Host "ownerWindowPid: $ownerProcessId"
}
if ($null -ne $guestProcessId) {
    Write-Host "guestWindowPid: $guestProcessId"
}
