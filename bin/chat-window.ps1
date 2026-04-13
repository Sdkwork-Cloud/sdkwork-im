param(
    [Alias("base-url")]
    [string]$BaseUrl,
    [Alias("tenant-id")]
    [string]$TenantId = "t_demo",
    [Alias("conversation-id")]
    [string]$ConversationId,
    [Alias("user-id")]
    [string]$UserId,
    [Alias("session-id")]
    [string]$SessionId,
    [Alias("device-id")]
    [string]$DeviceId,
    [Alias("bearer-token")]
    [string]$BearerToken,
    [string]$Label,
    [Alias("message-prefix")]
    [string]$MessagePrefix,
    [switch]$Release,
    [switch]$Help
)

$ErrorActionPreference = 'Stop'

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

if ($Help -or [string]::IsNullOrWhiteSpace($ConversationId) -or [string]::IsNullOrWhiteSpace($UserId)) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/chat-window.ps1 -ConversationId <id> -UserId <id> [-BaseUrl <url>] [-TenantId <id>] [-SessionId <id>] [-DeviceId <id>] [-BearerToken <token>] [-Label <name>] [-MessagePrefix <prefix>] [-Release]"
    Write-Host "Usage: cmd /c .\bin\chat-window.cmd --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--bearer-token <token>] [--label <name>] [--message-prefix <prefix>] [--release]"
    Write-Host "Open one interactive chat terminal backed by bin/chat-cli.ps1 chat-session, optionally with a real bearer token."
    if ($Help) {
        exit 0
    }
    exit 1
}

$resolvedLabel = if ([string]::IsNullOrWhiteSpace($Label)) { $UserId } else { $Label }
$resolvedBaseUrl = Resolve-BaseUrl -RequestedBaseUrl $BaseUrl
$resolvedSessionId = if ([string]::IsNullOrWhiteSpace($SessionId)) { "s_$UserId" } else { $SessionId }
$resolvedDeviceId = if ([string]::IsNullOrWhiteSpace($DeviceId)) { "d_$UserId" } else { $DeviceId }
$resolvedMessagePrefix = if ($PSBoundParameters.ContainsKey('MessagePrefix')) { $MessagePrefix } else { "[$resolvedLabel] " }

try {
    $host.UI.RawUI.WindowTitle = "craw-chat [$resolvedLabel] [$ConversationId]"
}
catch {
}

$cliArgs = @()
if ($Release) {
    $cliArgs += "-Release"
}
$cliArgs += @(
    "--base-url", $resolvedBaseUrl,
    "--tenant-id", $TenantId,
    "--user-id", $UserId,
    "--session-id", $resolvedSessionId,
    "--device-id", $resolvedDeviceId
)

if (-not [string]::IsNullOrWhiteSpace($BearerToken)) {
    $cliArgs += @("--bearer-token", $BearerToken)
}

$cliArgs += @(
    "chat-session",
    "--conversation-id", $ConversationId,
    "--label", $resolvedLabel
)

if (-not [string]::IsNullOrWhiteSpace($resolvedMessagePrefix)) {
    $cliArgs += @("--message-prefix", $resolvedMessagePrefix)
}

Write-Host "Opening chat session: conversation=$ConversationId user=$UserId label=$resolvedLabel baseUrl=$resolvedBaseUrl"
Write-Host "Type /quit to exit."

& "$PSScriptRoot\chat-cli.ps1" @cliArgs
exit $LASTEXITCODE
