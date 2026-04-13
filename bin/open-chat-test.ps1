param(
    [Alias("base-url")]
    [string]$BaseUrl,
    [Alias("tenant-id")]
    [string]$TenantId = "t_demo",
    [Alias("conversation-id")]
    [string]$ConversationId,
    [Alias("owner-user-id")]
    [string]$OwnerUserId = "u_owner",
    [Alias("owner-login")]
    [string]$OwnerLogin,
    [Alias("owner-password")]
    [string]$OwnerPassword,
    [Alias("guest-user-id")]
    [string]$GuestUserId = "u_guest",
    [Alias("guest-login")]
    [string]$GuestLogin,
    [Alias("guest-password")]
    [string]$GuestPassword,
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

function Resolve-BindAddressFromBaseUrl {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl
    )

    try {
        $uri = [System.Uri]$ResolvedBaseUrl
    }
    catch {
        throw "base url must be a valid absolute http(s) url: $ResolvedBaseUrl"
    }

    if (-not $uri.IsAbsoluteUri -or ($uri.Scheme -ne "http" -and $uri.Scheme -ne "https")) {
        throw "base url must be a valid absolute http(s) url: $ResolvedBaseUrl"
    }

    $resolvedHostName = if ([string]::IsNullOrWhiteSpace($uri.Host)) { "127.0.0.1" } else { $uri.Host }
    $port = if ($uri.IsDefaultPort) {
        if ($uri.Scheme -eq "https") { 443 } else { 80 }
    }
    else {
        $uri.Port
    }

    return "$resolvedHostName`:$port"
}

function Resolve-SeededImPassword {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Login
    )

    switch ($Login) {
        "u_owner" { return "Owner#2026" }
        "u_guest" { return "Guest#2026" }
        "u_demo" { return "Demo#2026" }
        default { return $null }
    }
}

function Resolve-ImLogin {
    param(
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [string]$RequestedLogin
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedLogin)) {
        return $RequestedLogin
    }

    return $RequestedUserId
}

function Resolve-ImPassword {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Login,
        [string]$RequestedPassword
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedPassword)) {
        return $RequestedPassword
    }

    $seededPassword = Resolve-SeededImPassword -Login $Login
    if (-not [string]::IsNullOrWhiteSpace($seededPassword)) {
        return $seededPassword
    }

    throw "No password was provided for login '$Login'. Supply -OwnerPassword/-GuestPassword for non-seeded accounts."
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

function Invoke-ImUserLogin {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [Parameter(Mandatory = $true)]
        [string]$Login,
        [Parameter(Mandatory = $true)]
        [string]$Password,
        [Parameter(Mandatory = $true)]
        [string]$SessionId,
        [Parameter(Mandatory = $true)]
        [string]$DeviceId
    )

    $loginResponse = Invoke-ChatCliJson -Arguments @(
        "--base-url", $ResolvedBaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", $RequestedUserId,
        "--session-id", $SessionId,
        "--device-id", $DeviceId,
        "login",
        "--login", $Login,
        "--password", $Password,
        "--client-kind", "im_user"
    )

    $accessToken = [string]$loginResponse.accessToken
    if ([string]::IsNullOrWhiteSpace($accessToken)) {
        throw "login response did not include accessToken for '$Login'"
    }

    $resolvedUserId = if ($null -ne $loginResponse.user -and -not [string]::IsNullOrWhiteSpace([string]$loginResponse.user.id)) {
        [string]$loginResponse.user.id
    }
    else {
        $RequestedUserId
    }

    return [pscustomobject]@{
        UserId = $resolvedUserId
        Login = $Login
        BearerToken = $accessToken
        RefreshToken = [string]$loginResponse.refreshToken
        SessionId = $SessionId
        DeviceId = $DeviceId
    }
}

function Get-ChatCliAuthArguments {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [psobject]$AuthContext
    )

    $arguments = @(
        "--base-url", $ResolvedBaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", $AuthContext.UserId,
        "--session-id", $AuthContext.SessionId,
        "--device-id", $AuthContext.DeviceId
    )

    if (-not [string]::IsNullOrWhiteSpace([string]$AuthContext.BearerToken)) {
        $arguments += @("--bearer-token", [string]$AuthContext.BearerToken)
    }

    return $arguments
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
        [psobject]$OwnerAuth,
        [Parameter(Mandatory = $true)]
        [psobject]$GuestAuth
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
        $watchArgs += (Get-ChatCliAuthArguments -ResolvedBaseUrl $ResolvedBaseUrl -AuthContext $GuestAuth)
        $watchArgs += @(
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
        $null = Invoke-ChatCliJson -Arguments ((Get-ChatCliAuthArguments -ResolvedBaseUrl $ResolvedBaseUrl -AuthContext $OwnerAuth) + @(
                "send-message",
                "--conversation-id", $ConversationId,
                "--summary", $ResolvedValidationMessage,
                "--text", $ResolvedValidationMessage,
                "--client-msg-id", $clientMessageId
            ))

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

        $timeline = Invoke-ChatCliJson -Arguments ((Get-ChatCliAuthArguments -ResolvedBaseUrl $ResolvedBaseUrl -AuthContext $GuestAuth) + @(
                "timeline",
                "--conversation-id", $ConversationId
            ))
        $timelineSummaries = @()
        if ($null -ne $timeline -and $null -ne $timeline.items) {
            $timelineSummaries = @($timeline.items | ForEach-Object { $_.summary })
        }

        return [ordered]@{
            mode = "scripted"
            conversationId = $ConversationId
            ownerUserId = $OwnerAuth.UserId
            guestUserId = $GuestAuth.UserId
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
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl
    )

    $resolvedBindAddress = Resolve-BindAddressFromBaseUrl -ResolvedBaseUrl $ResolvedBaseUrl
    if ($Release) {
        & "$PSScriptRoot\start-local.ps1" -ProfileName "local-minimal" -Release -BindAddress $resolvedBindAddress
    }
    else {
        & "$PSScriptRoot\start-local.ps1" -ProfileName "local-minimal" -BindAddress $resolvedBindAddress
    }
    if ($LASTEXITCODE -ne 0) {
        throw "Failed to start local-minimal-node via start-local.ps1."
    }

    if (-not (Test-ChatHealth -Url $ResolvedBaseUrl)) {
        throw "Chat service is not healthy at $ResolvedBaseUrl after start-local.ps1 completed"
    }
}

function Stop-LocalMinimalNodeListener {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl
    )

    $bindAddress = Resolve-BindAddressFromBaseUrl -ResolvedBaseUrl $ResolvedBaseUrl
    $port = [int](($bindAddress -split ':')[-1])
    $listeners = @(Get-NetTCPConnection -LocalPort $port -State Listen -ErrorAction SilentlyContinue)
    if ($listeners.Count -eq 0) {
        return $false
    }

    foreach ($listener in $listeners) {
        $process = Get-Process -Id $listener.OwningProcess -ErrorAction SilentlyContinue
        if ($null -eq $process) {
            continue
        }

        if ($process.ProcessName -ne "local-minimal-node") {
            throw "Port $port is occupied by non-local-minimal-node process '$($process.ProcessName)' (PID $($process.Id))."
        }

        Stop-Process -Id $process.Id -Force -ErrorAction Stop
        try {
            Wait-Process -Id $process.Id -Timeout 15 -ErrorAction Stop
        }
        catch {
            if ($null -ne (Get-Process -Id $process.Id -ErrorAction SilentlyContinue)) {
                throw "local-minimal-node PID $($process.Id) did not exit after forced stop."
            }
        }
    }

    return $true
}

function Invoke-ImUserLoginWithRecovery {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [Parameter(Mandatory = $true)]
        [string]$Login,
        [Parameter(Mandatory = $true)]
        [string]$Password,
        [Parameter(Mandatory = $true)]
        [string]$SessionId,
        [Parameter(Mandatory = $true)]
        [string]$DeviceId
    )

    try {
        return Invoke-ImUserLogin `
            -ResolvedBaseUrl $ResolvedBaseUrl `
            -RequestedUserId $RequestedUserId `
            -Login $Login `
            -Password $Password `
            -SessionId $SessionId `
            -DeviceId $DeviceId
    }
    catch {
        $message = $_.Exception.Message
        $signingSecretMissing =
            $message -match "auth_signing_secret_missing" -or
            $message -match "public bearer signing secret is missing"

        if ($SkipStart -or -not $signingSecretMissing) {
            throw
        }

        Write-Host "Local service auth preflight failed. Recycling local-minimal-node with standard start-local.ps1..."
        $stopped = Stop-LocalMinimalNodeListener -ResolvedBaseUrl $ResolvedBaseUrl
        if (-not $stopped) {
            throw
        }

        Start-HostedLocalService -ResolvedBaseUrl $ResolvedBaseUrl
        return Invoke-ImUserLogin `
            -ResolvedBaseUrl $ResolvedBaseUrl `
            -RequestedUserId $RequestedUserId `
            -Login $Login `
            -Password $Password `
            -SessionId $SessionId `
            -DeviceId $DeviceId
    }
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/open-chat-test.ps1 [-ConversationId <id>] [-BaseUrl <url>] [-TenantId <id>] [-OwnerUserId <id>] [-OwnerLogin <id>] [-OwnerPassword <secret>] [-GuestUserId <id>] [-GuestLogin <id>] [-GuestPassword <secret>] [-OwnerLabel <label>] [-GuestLabel <label>] [-Release] [-SkipStart] [-UseConsoleWindows] [-ScriptedValidation] [-ValidationMessage <text>] [-Json]"
    Write-Host "Usage: cmd /c .\bin\open-chat-test.cmd [--conversation-id <id>] [--base-url <url>] [--tenant-id <id>] [--owner-user-id <id>] [--owner-login <id>] [--owner-password <secret>] [--guest-user-id <id>] [--guest-login <id>] [--guest-password <secret>] [--owner-label <label>] [--guest-label <label>] [--release] [--skip-start] [--use-console-windows] [--scripted-validation] [--validation-message <text>] [--json]"
    Write-Host "Create a local test conversation, authenticate owner and guest through real login, then either open two visible chat windows or run scripted watch/timeline validation."
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
    Start-HostedLocalService -ResolvedBaseUrl $BaseUrl
}

if (-not (Test-ChatHealth -Url $BaseUrl)) {
    throw "Chat service is not healthy at $BaseUrl"
}

$resolvedOwnerLogin = Resolve-ImLogin -RequestedUserId $OwnerUserId -RequestedLogin $OwnerLogin
$resolvedOwnerPassword = Resolve-ImPassword -Login $resolvedOwnerLogin -RequestedPassword $OwnerPassword
$resolvedGuestLogin = Resolve-ImLogin -RequestedUserId $GuestUserId -RequestedLogin $GuestLogin
$resolvedGuestPassword = Resolve-ImPassword -Login $resolvedGuestLogin -RequestedPassword $GuestPassword

$ownerAuth = Invoke-ImUserLoginWithRecovery `
    -ResolvedBaseUrl $BaseUrl `
    -RequestedUserId $OwnerUserId `
    -Login $resolvedOwnerLogin `
    -Password $resolvedOwnerPassword `
    -SessionId $ownerSessionId `
    -DeviceId $ownerDeviceId
$guestAuth = Invoke-ImUserLoginWithRecovery `
    -ResolvedBaseUrl $BaseUrl `
    -RequestedUserId $GuestUserId `
    -Login $resolvedGuestLogin `
    -Password $resolvedGuestPassword `
    -SessionId $guestSessionId `
    -DeviceId $guestDeviceId

$ownerCliAuthArgs = @(Get-ChatCliAuthArguments -ResolvedBaseUrl $BaseUrl -AuthContext $ownerAuth)
$guestCliAuthArgs = @(Get-ChatCliAuthArguments -ResolvedBaseUrl $BaseUrl -AuthContext $guestAuth)

if ($ScriptedValidation) {
    $null = Invoke-ChatCliJson -Arguments ($ownerCliAuthArgs + @(
            "create-conversation",
            "--conversation-id", $ConversationId,
            "--conversation-type", "group"
        ))

    $null = Invoke-ChatCliJson -Arguments ($ownerCliAuthArgs + @(
            "add-member",
            "--conversation-id", $ConversationId,
            "--principal-id", $guestAuth.UserId,
            "--principal-kind", "user",
            "--role", "member"
        ))

    $summary = Invoke-ScriptedValidation `
        -ResolvedBaseUrl $BaseUrl `
        -ResolvedValidationMessage $resolvedValidationMessage `
        -OwnerAuth $ownerAuth `
        -GuestAuth $guestAuth

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

Invoke-ChatCli -Arguments ($ownerCliAuthArgs + @(
        "create-conversation",
        "--conversation-id", $ConversationId,
        "--conversation-type", "group"
    ))

Invoke-ChatCli -Arguments ($ownerCliAuthArgs + @(
        "add-member",
        "--conversation-id", $ConversationId,
        "--principal-id", $guestAuth.UserId,
        "--principal-kind", "user",
        "--role", "member"
    ))

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
    "-UserId", $ownerAuth.UserId,
    "-SessionId", $ownerAuth.SessionId,
    "-DeviceId", $ownerAuth.DeviceId,
    "-BearerToken", $ownerAuth.BearerToken,
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
    "-UserId", $guestAuth.UserId,
    "-SessionId", $guestAuth.SessionId,
    "-DeviceId", $guestAuth.DeviceId,
    "-BearerToken", $guestAuth.BearerToken,
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
Write-Host "owner: $($ownerAuth.UserId)"
Write-Host "guest: $($guestAuth.UserId)"
if ($null -ne $ownerProcessId) {
    Write-Host "ownerWindowPid: $ownerProcessId"
}
if ($null -ne $guestProcessId) {
    Write-Host "guestWindowPid: $guestProcessId"
}
