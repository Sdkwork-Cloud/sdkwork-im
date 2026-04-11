param(
    [Alias("base-url")]
    [string]$BaseUrl,
    [Alias("tenant-id")]
    [string]$TenantId = "t_demo",
    [Alias("conversation-id")]
    [string]$ConversationId,
    [Alias("owner-user-id")]
    [string]$OwnerUserId = "u_owner",
    [Alias("guest-user-id")]
    [string]$GuestUserId = "u_guest",
    [Alias("owner-label")]
    [string]$OwnerLabel = "owner",
    [Alias("guest-label")]
    [string]$GuestLabel = "guest",
    [switch]$Release,
    [Alias("skip-start")]
    [switch]$SkipStart,
    [Alias("use-console-windows")]
    [switch]$UseConsoleWindows,
    [Alias("scripted-validation")]
    [switch]$ScriptedValidation,
    [Alias("validation-message")]
    [string]$ValidationMessage,
    [switch]$Json,
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

function Invoke-ChatCliCaptured {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [switch]$AllowEmpty
    )

    $root = Split-Path -Parent $PSScriptRoot
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = "powershell.exe"
    $startInfo.WorkingDirectory = $root
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.CreateNoWindow = $true
    $startInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8
    $startInfo.StandardErrorEncoding = [System.Text.Encoding]::UTF8

    $allArgs = @(
        "-NoProfile",
        "-ExecutionPolicy", "Bypass",
        "-File", (Join-Path $PSScriptRoot "chat-cli.ps1")
    )
    if ($Release) {
        $allArgs += "-Release"
    }
    $allArgs += $Arguments
    $startInfo.Arguments = (($allArgs | ForEach-Object { Quote-ProcessArgument $_ }) -join ' ')

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo

    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    if ($process.ExitCode -ne 0) {
        throw "chat-cli invocation failed with exit code $($process.ExitCode): $($Arguments -join ' ')`n$stderr`n$stdout"
    }

    if ([string]::IsNullOrWhiteSpace($stdout) -and -not $AllowEmpty) {
        throw "chat-cli invocation returned empty output: $($Arguments -join ' ')"
    }

    return [pscustomobject]@{
        Stdout = $stdout
        Stderr = $stderr
    }
}

function Invoke-ChatCliJson {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [switch]$AllowEmpty
    )

    $result = Invoke-ChatCliCaptured -Arguments $Arguments -AllowEmpty:$AllowEmpty
    if ([string]::IsNullOrWhiteSpace($result.Stdout)) {
        return $null
    }

    return $result.Stdout | ConvertFrom-Json
}

function Parse-JsonLines {
    param(
        [AllowNull()]
        [string]$Text
    )

    $items = @()
    if ([string]::IsNullOrWhiteSpace($Text)) {
        return $items
    }

    foreach ($line in ($Text -split "\r?\n")) {
        $trimmed = $line.Trim()
        if ([string]::IsNullOrWhiteSpace($trimmed)) {
            continue
        }

        $items += ($trimmed | ConvertFrom-Json)
    }

    return $items
}

function Invoke-ScriptedValidation {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedValidationMessage,
        [Parameter(Mandatory = $true)]
        [string]$OwnerSessionId,
        [Parameter(Mandatory = $true)]
        [string]$OwnerDeviceId,
        [Parameter(Mandatory = $true)]
        [string]$GuestSessionId,
        [Parameter(Mandatory = $true)]
        [string]$GuestDeviceId
    )

    $watchOutputFile = Join-Path ([System.IO.Path]::GetTempPath()) ("craw-chat-watch-{0}.stdout" -f ([guid]::NewGuid().ToString("N")))
    $watchErrorFile = Join-Path ([System.IO.Path]::GetTempPath()) ("craw-chat-watch-{0}.stderr" -f ([guid]::NewGuid().ToString("N")))

    try {
        $watchArgs = @(
            "-NoProfile",
            "-ExecutionPolicy", "Bypass",
            "-File", (Join-Path $PSScriptRoot "chat-cli.ps1")
        )
        if ($Release) {
            $watchArgs += "-Release"
        }
        $watchArgs += @(
            "--base-url", $ResolvedBaseUrl,
            "--tenant-id", $TenantId,
            "--user-id", $GuestUserId,
            "--session-id", $GuestSessionId,
            "--device-id", $GuestDeviceId,
            "watch",
            "--conversation-id", $ConversationId,
            "--event-type", "message.posted",
            "--exit-after-events", "1",
            "--idle-timeout-seconds", "5"
        )

        $watchProcess = Start-Process -FilePath "powershell.exe" `
            -ArgumentList $watchArgs `
            -PassThru `
            -WindowStyle Hidden `
            -RedirectStandardOutput $watchOutputFile `
            -RedirectStandardError $watchErrorFile

        Start-Sleep -Milliseconds 500

        $clientMessageId = "open_chat_test_scripted_{0}" -f (Get-Date -Format "yyyyMMddHHmmssfff")
        $null = Invoke-ChatCliJson -Arguments @(
            "--base-url", $ResolvedBaseUrl,
            "--tenant-id", $TenantId,
            "--user-id", $OwnerUserId,
            "--session-id", $OwnerSessionId,
            "--device-id", $OwnerDeviceId,
            "send-message",
            "--conversation-id", $ConversationId,
            "--summary", $ResolvedValidationMessage,
            "--text", $ResolvedValidationMessage,
            "--client-msg-id", $clientMessageId
        )

        if (-not $watchProcess.WaitForExit(15000)) {
            try {
                $watchProcess.Kill()
            }
            catch {
            }
            throw "scripted validation watch did not complete before timeout"
        }

        $watchStdout = if (Test-Path $watchOutputFile) {
            Get-Content -Raw -Path $watchOutputFile
        }
        else {
            ""
        }
        $watchStderr = if (Test-Path $watchErrorFile) {
            Get-Content -Raw -Path $watchErrorFile
        }
        else {
            ""
        }

        $watchFrames = @(Parse-JsonLines -Text $watchStdout)
        if ($watchFrames.Count -eq 0) {
            throw "scripted validation watch did not produce any frames`n$watchStderr`n$watchStdout"
        }
        $eventWindow = $watchFrames |
            Where-Object { $_.type -eq "event.window" -and $_.reason -eq "push" } |
            Select-Object -First 1

        $deliveredSummary = $null
        if ($null -ne $eventWindow -and $null -ne $eventWindow.window -and $null -ne $eventWindow.window.items -and $eventWindow.window.items.Count -gt 0) {
            $payloadText = $eventWindow.window.items[0].payload
            if (-not [string]::IsNullOrWhiteSpace($payloadText)) {
                $payload = $payloadText | ConvertFrom-Json
                $deliveredSummary = $payload.summary
            }
        }

        $timeline = Invoke-ChatCliJson -Arguments @(
            "--base-url", $ResolvedBaseUrl,
            "--tenant-id", $TenantId,
            "--user-id", $GuestUserId,
            "--session-id", $GuestSessionId,
            "--device-id", $GuestDeviceId,
            "timeline",
            "--conversation-id", $ConversationId
        )
        $timelineSummaries = @()
        if ($null -ne $timeline -and $null -ne $timeline.items) {
            $timelineSummaries = @($timeline.items | ForEach-Object { $_.summary })
        }

        return [ordered]@{
            mode = "scripted"
            conversationId = $ConversationId
            ownerUserId = $OwnerUserId
            guestUserId = $GuestUserId
            validationMessage = $ResolvedValidationMessage
            watchFrameTypes = @($watchFrames | ForEach-Object { $_.type })
            watchDelivered = ($deliveredSummary -eq $ResolvedValidationMessage)
            timelineContainsValidationMessage = ($timelineSummaries -contains $ResolvedValidationMessage)
        }
    }
    finally {
        Remove-Item -LiteralPath $watchOutputFile -ErrorAction SilentlyContinue
        Remove-Item -LiteralPath $watchErrorFile -ErrorAction SilentlyContinue
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
    $workingDirectory = Split-Path -Parent $PSScriptRoot

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

    try {
        $process = Start-Process -FilePath "powershell.exe" `
            -ArgumentList $ArgumentList `
            -WorkingDirectory $workingDirectory `
            -PassThru `
            -WindowStyle Normal
        if ($null -ne $process) {
            return $process.Id
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 [-ConversationId <id>] [-BaseUrl <url>] [-TenantId <id>] [-OwnerUserId <id>] [-GuestUserId <id>] [-OwnerLabel <label>] [-GuestLabel <label>] [-Release] [-SkipStart] [-UseConsoleWindows] [-ScriptedValidation] [-ValidationMessage <text>] [-Json]"
    Write-Host "Usage: cmd /c .\bin\open-chat-test.cmd [--conversation-id <id>] [--base-url <url>] [--tenant-id <id>] [--owner-user-id <id>] [--guest-user-id <id>] [--owner-label <label>] [--guest-label <label>] [--release] [--skip-start] [--use-console-windows] [--scripted-validation] [--validation-message <text>] [--json]"
    Write-Host "Create a local test conversation and either open two visible chat windows or run scripted watch/timeline validation."
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
$resolvedValidationMessage = if ([string]::IsNullOrWhiteSpace($ValidationMessage)) {
    "step12 scripted validation $ConversationId"
}
else {
    $ValidationMessage
}

if (-not $SkipStart -and -not (Test-ChatHealth -Url $BaseUrl)) {
    Write-Host "Local service is not healthy. Starting local-minimal-node..."
    Start-HostedLocalService
}

if (-not (Test-ChatHealth -Url $BaseUrl)) {
    throw "Chat service is not healthy at $BaseUrl"
}

if ($ScriptedValidation) {
    $null = Invoke-ChatCliJson -Arguments @(
        "--base-url", $BaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", $OwnerUserId,
        "--session-id", $ownerSessionId,
        "--device-id", $ownerDeviceId,
        "create-conversation",
        "--conversation-id", $ConversationId,
        "--conversation-type", "group"
    )

    $null = Invoke-ChatCliJson -Arguments @(
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

    $summary = Invoke-ScriptedValidation `
        -ResolvedBaseUrl $BaseUrl `
        -ResolvedValidationMessage $resolvedValidationMessage `
        -OwnerSessionId $ownerSessionId `
        -OwnerDeviceId $ownerDeviceId `
        -GuestSessionId $guestSessionId `
        -GuestDeviceId $guestDeviceId

    if ($Json) {
        $summary | ConvertTo-Json -Depth 8
    }
    else {
        Write-Host "Scripted validation completed."
        Write-Host "conversationId: $($summary.conversationId)"
        Write-Host "validationMessage: $($summary.validationMessage)"
        Write-Host "watchDelivered: $($summary.watchDelivered)"
        Write-Host "timelineContainsValidationMessage: $($summary.timelineContainsValidationMessage)"
    }
    exit 0
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
