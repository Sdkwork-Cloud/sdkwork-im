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
    [string]$Login,
    [string]$Password,
    [string]$Label,
    [Alias("message-prefix")]
    [string]$MessagePrefix,
    [Alias("diagnostics-file")]
    [string]$DiagnosticsFile,
    [Alias("skip-connect")]
    [switch]$SkipConnect,
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

$resolvedBaseUrl = Resolve-BaseUrl -RequestedBaseUrl $BaseUrl
$resolvedLabel = if ([string]::IsNullOrWhiteSpace($Label)) { $UserId } else { $Label }
$resolvedSessionId = if ([string]::IsNullOrWhiteSpace($SessionId)) { "s_$UserId" } else { $SessionId }
$resolvedDeviceId = if ([string]::IsNullOrWhiteSpace($DeviceId)) { "d_$UserId" } else { $DeviceId }
$resolvedMessagePrefix = if ($PSBoundParameters.ContainsKey('MessagePrefix')) { $MessagePrefix } else { "[$resolvedLabel] " }
$resolvedAutomationAction = [Environment]::GetEnvironmentVariable("CRAW_CHAT_GUI_AUTOMATION_ACTION")
$resolvedAutomationDelayMs = 250
$automationDelayMsText = [Environment]::GetEnvironmentVariable("CRAW_CHAT_GUI_AUTOMATION_DELAY_MS")
if (-not [string]::IsNullOrWhiteSpace($automationDelayMsText)) {
    $parsedAutomationDelayMs = 0
    if ([int]::TryParse($automationDelayMsText, [ref]$parsedAutomationDelayMs) -and $parsedAutomationDelayMs -ge 0) {
        $resolvedAutomationDelayMs = $parsedAutomationDelayMs
    }
}
$resolvedDiagnosticsFile = if ([string]::IsNullOrWhiteSpace($DiagnosticsFile)) {
    $runtimeLogDir = Join-Path (Split-Path -Parent $PSScriptRoot) ".runtime\local-minimal\logs\chat-window-gui"
    Join-Path $runtimeLogDir ("{0}-{1}-{2}.log" -f `
            (Get-Date -Format "yyyyMMddHHmmss"), `
            (($resolvedLabel -replace '[^a-zA-Z0-9_-]', '_')), `
            (($ConversationId -replace '[^a-zA-Z0-9_-]', '_')))
}
else {
    $DiagnosticsFile
}

$diagnosticsParent = Split-Path -Parent $resolvedDiagnosticsFile
if (-not [string]::IsNullOrWhiteSpace($diagnosticsParent)) {
    New-Item -ItemType Directory -Force -Path $diagnosticsParent | Out-Null
}

function Write-Diagnostic {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Message
    )

    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss.fff"
    Add-Content -Path $resolvedDiagnosticsFile -Value ("[{0}] {1}" -f $timestamp, $Message)
}

function Resolve-SeededImPassword {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedLogin
    )

    switch ($ResolvedLogin) {
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
        [string]$ResolvedLogin,
        [string]$RequestedPassword
    )

    if (-not [string]::IsNullOrWhiteSpace($RequestedPassword)) {
        return $RequestedPassword
    }

    $seededPassword = Resolve-SeededImPassword -ResolvedLogin $ResolvedLogin
    if (-not [string]::IsNullOrWhiteSpace($seededPassword)) {
        return $seededPassword
    }

    throw "No password was provided for login '$ResolvedLogin'. Supply -Login/-Password for non-seeded accounts."
}

function Get-HttpErrorDetail {
    param(
        [Parameter(Mandatory = $true)]
        [System.Exception]$Exception
    )

    $message = $Exception.Message
    try {
        $response = $Exception.Response
        if ($null -ne $response) {
            $stream = $response.GetResponseStream()
            if ($null -ne $stream) {
                $reader = New-Object System.IO.StreamReader($stream)
                $body = $reader.ReadToEnd()
                if (-not [string]::IsNullOrWhiteSpace($body)) {
                    $message = "$message :: $body"
                }
            }
        }
    }
    catch {
    }

    return $message
}

function Invoke-AuthenticatedJsonRequest {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$BearerToken,
        [Parameter(Mandatory = $true)]
        [string]$Method,
        [Parameter(Mandatory = $true)]
        [string]$Path,
        $Body,
        [switch]$AllowEmpty
    )

    $uri = ($ResolvedBaseUrl.TrimEnd('/')) + $Path
    $headers = @{
        Authorization = "Bearer $BearerToken"
    }

    try {
        if ($null -eq $Body) {
            $response = Invoke-WebRequest -Uri $uri -Method $Method -Headers $headers -UseBasicParsing
        }
        else {
            $jsonBody = $Body | ConvertTo-Json -Depth 12 -Compress
            $response = Invoke-WebRequest -Uri $uri -Method $Method -Headers $headers -ContentType "application/json" -Body $jsonBody -UseBasicParsing
        }
    }
    catch {
        throw "HTTP $Method $Path failed: $(Get-HttpErrorDetail -Exception $_.Exception)"
    }

    if ([string]::IsNullOrWhiteSpace($response.Content)) {
        if ($AllowEmpty) {
            return $null
        }
        throw "HTTP $Method $Path returned empty output"
    }

    return $response.Content | ConvertFrom-Json
}

function Ensure-RtcSessionExistsForInvite {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$BearerToken,
        [Parameter(Mandatory = $true)]
        [string]$RtcSessionId,
        [Parameter(Mandatory = $true)]
        [string]$ConversationId,
        [Parameter(Mandatory = $true)]
        [string]$RtcMode
    )

    $null = Invoke-AuthenticatedJsonRequest `
        -ResolvedBaseUrl $ResolvedBaseUrl `
        -BearerToken $BearerToken `
        -Method "POST" `
        -Path "/im/v3/api/calls/sessions" `
        -Body ([ordered]@{
                rtcSessionId = $RtcSessionId
                conversationId = $ConversationId
                rtcMode = $RtcMode
            })
}

function Get-RtcActionFailureMessage {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Action,
        [Parameter(Mandatory = $true)]
        [string]$FailureMessage
    )

    if ($FailureMessage -notmatch 'rtc_session_not_found') {
        return $FailureMessage
    }

    switch ($Action) {
        "accept" {
            return "$FailureMessage :: wait for the owner window to create and invite the rtc session before accepting"
        }
        "reject" {
            return "$FailureMessage :: wait for the owner window to create and invite the rtc session before rejecting"
        }
        "signal" {
            return "$FailureMessage :: create and invite the rtc session before sending signaling payloads"
        }
        "end" {
            return "$FailureMessage :: create and invite the rtc session before ending it"
        }
        "credentials" {
            return "$FailureMessage :: create and invite the rtc session before requesting participant credentials"
        }
        "recording" {
            return "$FailureMessage :: create and invite the rtc session before requesting a recording artifact"
        }
        default {
            return $FailureMessage
        }
    }
}

function Resolve-ChatCliExecutablePath {
    $root = Split-Path -Parent $PSScriptRoot
    $profileDir = if ($Release) { "release" } else { "debug" }
    $exePath = Join-Path $root "target\$profileDir\craw-chat-cli.exe"

    if (Test-ChatCliExecutableNeedsBuild -Root $root -ExePath $exePath) {
        $cargoArgs = @("build", "-p", "craw-chat-cli")
        if ($Release) {
            $cargoArgs += "--release"
        }

        Write-Diagnostic ("chat-cli build start: cargo " + ($cargoArgs -join " "))
        $buildOutput = & cargo @cargoArgs 2>&1
        if ($LASTEXITCODE -ne 0) {
            $buildText = ($buildOutput | ForEach-Object { $_.ToString() }) -join [Environment]::NewLine
            throw "failed to build craw-chat-cli`n$buildText"
        }
        Write-Diagnostic "chat-cli build completed"
    }

    if (-not (Test-Path $exePath)) {
        throw "craw-chat-cli binary was not found after build: $exePath"
    }

    return $exePath
}

function Get-ChatCliBuildInputs {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root
    )

    @(
        (Join-Path $Root "Cargo.lock"),
        (Join-Path $Root "tools\chat-cli\Cargo.toml"),
        (Join-Path $Root "tools\chat-cli\src")
    )
}

function Test-ChatCliExecutableNeedsBuild {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Root,
        [Parameter(Mandatory = $true)]
        [string]$ExePath
    )

    if (-not (Test-Path $ExePath)) {
        return $true
    }

    $exeTimestamp = (Get-Item -LiteralPath $ExePath).LastWriteTimeUtc
    foreach ($inputPath in Get-ChatCliBuildInputs -Root $Root) {
        if (-not (Test-Path -LiteralPath $inputPath)) {
            continue
        }

        $item = Get-Item -LiteralPath $inputPath
        if ($item.PSIsContainer) {
            $newerSource = Get-ChildItem -LiteralPath $inputPath -File -Recurse |
                Where-Object { $_.LastWriteTimeUtc -gt $exeTimestamp } |
                Select-Object -First 1
            if ($null -ne $newerSource) {
                return $true
            }
            continue
        }

        if ($item.LastWriteTimeUtc -gt $exeTimestamp) {
            return $true
        }
    }

    return $false
}

function Invoke-ChatCliJson {
    param(
        [Parameter(Mandatory = $true)]
        [string[]]$Arguments,
        [switch]$AllowEmpty
    )

    $exePath = Resolve-ChatCliExecutablePath
    $root = Split-Path -Parent $PSScriptRoot
    $startInfo = New-Object System.Diagnostics.ProcessStartInfo
    $startInfo.FileName = $exePath
    $startInfo.WorkingDirectory = $root
    $startInfo.UseShellExecute = $false
    $startInfo.RedirectStandardOutput = $true
    $startInfo.RedirectStandardError = $true
    $startInfo.CreateNoWindow = $true
    $startInfo.StandardOutputEncoding = [System.Text.Encoding]::UTF8
    $startInfo.StandardErrorEncoding = [System.Text.Encoding]::UTF8
    $startInfo.Arguments = (($Arguments | ForEach-Object { Quote-ProcessArgument $_ }) -join ' ')

    $process = New-Object System.Diagnostics.Process
    $process.StartInfo = $startInfo

    [void]$process.Start()
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    if ($process.ExitCode -ne 0) {
        throw "chat-cli invocation failed with exit code $($process.ExitCode): $($Arguments -join ' ')`n$stderr`n$stdout"
    }

    if (-not [string]::IsNullOrWhiteSpace($stderr)) {
        Write-Diagnostic ("chat-cli stderr: " + $stderr.Trim())
    }

    $text = $stdout
    if ([string]::IsNullOrWhiteSpace($text)) {
        if ($AllowEmpty) {
            return $null
        }
        throw "chat-cli invocation returned empty output: $($Arguments -join ' ')"
    }

    return $text | ConvertFrom-Json
}

function Invoke-ImUserLogin {
    param(
        [Parameter(Mandatory = $true)]
        [string]$ResolvedBaseUrl,
        [Parameter(Mandatory = $true)]
        [string]$RequestedUserId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedLogin,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedPassword,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedSessionId,
        [Parameter(Mandatory = $true)]
        [string]$ResolvedDeviceId
    )

    $loginResponse = Invoke-ChatCliJson -Arguments @(
        "--base-url", $ResolvedBaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", $RequestedUserId,
        "--session-id", $ResolvedSessionId,
        "--device-id", $ResolvedDeviceId,
        "login",
        "--login", $ResolvedLogin,
        "--password", $ResolvedPassword,
        "--client-kind", "im_user"
    )

    $accessToken = [string]$loginResponse.accessToken
    if ([string]::IsNullOrWhiteSpace($accessToken)) {
        throw "login response did not include accessToken for '$ResolvedLogin'"
    }

    $resolvedAuthUserId = if ($null -ne $loginResponse.user -and -not [string]::IsNullOrWhiteSpace([string]$loginResponse.user.id)) {
        [string]$loginResponse.user.id
    }
    else {
        $RequestedUserId
    }

    return [pscustomobject]@{
        UserId = $resolvedAuthUserId
        SessionId = $ResolvedSessionId
        DeviceId = $ResolvedDeviceId
        BearerToken = $accessToken
        AuthMode = "real-login"
    }
}

$script:resolvedAuthContext = $null

function Resolve-ChatAuthContext {
    if ($null -ne $script:resolvedAuthContext) {
        return $script:resolvedAuthContext
    }

    if (-not [string]::IsNullOrWhiteSpace($BearerToken)) {
        $script:resolvedAuthContext = [pscustomobject]@{
            UserId = $UserId
            SessionId = $resolvedSessionId
            DeviceId = $resolvedDeviceId
            BearerToken = $BearerToken
            AuthMode = "provided-bearer"
        }
        Write-Diagnostic "auth mode=provided-bearer"
        return $script:resolvedAuthContext
    }

    $resolvedLogin = Resolve-ImLogin -RequestedUserId $UserId -RequestedLogin $Login
    $shouldUseRealLogin = -not [string]::IsNullOrWhiteSpace($Login) `
        -and -not [string]::IsNullOrWhiteSpace($Password)

    if ($shouldUseRealLogin) {
        $script:resolvedAuthContext = Invoke-ImUserLogin `
            -ResolvedBaseUrl $resolvedBaseUrl `
            -RequestedUserId $UserId `
            -ResolvedLogin $resolvedLogin `
            -ResolvedPassword $Password `
            -ResolvedSessionId $resolvedSessionId `
            -ResolvedDeviceId $resolvedDeviceId
        Write-Diagnostic ("auth mode=real-login user=" + [string]$script:resolvedAuthContext.UserId + " login=" + $resolvedLogin)
        return $script:resolvedAuthContext
    }

    $script:resolvedAuthContext = [pscustomobject]@{
        UserId = $UserId
        SessionId = $resolvedSessionId
        DeviceId = $resolvedDeviceId
        BearerToken = $null
        AuthMode = "manual-login-pending"
    }
    Write-Diagnostic ("auth mode=" + [string]$script:resolvedAuthContext.AuthMode)
    return $script:resolvedAuthContext
}

function New-ClientMessageId {
    return "gui_{0}_{1}_{2}" -f `
        ($resolvedLabel -replace '[^a-zA-Z0-9_-]', '_'), `
        (Get-Date -Format "yyyyMMddHHmmssfff"), `
        ([guid]::NewGuid().ToString("N").Substring(0, 8))
}

function Get-ChatCliAuthArguments {
    $authContext = Resolve-ChatAuthContext
    if ([string]::IsNullOrWhiteSpace([string]$authContext.BearerToken)) {
        throw "manual login is required before sending chat or RTC requests"
    }

    $arguments = @(
        "--base-url", $resolvedBaseUrl,
        "--tenant-id", $TenantId,
        "--user-id", [string]$authContext.UserId,
        "--session-id", [string]$authContext.SessionId,
        "--device-id", [string]$authContext.DeviceId
    )

    if (-not [string]::IsNullOrWhiteSpace([string]$authContext.BearerToken)) {
        $arguments += @("--bearer-token", [string]$authContext.BearerToken)
    }

    return $arguments
}

if ($Help -or [string]::IsNullOrWhiteSpace($ConversationId) -or [string]::IsNullOrWhiteSpace($UserId)) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/chat-window-gui.ps1 -ConversationId <id> -UserId <id> [-BaseUrl <url>] [-TenantId <id>] [-SessionId <id>] [-DeviceId <id>] [-BearerToken <token>] [-Login <id>] [-Password <secret>] [-Label <name>] [-MessagePrefix <prefix>] [-DiagnosticsFile <path>] [-SkipConnect] [-Release]"
    Write-Host "Usage: cmd /c .\bin\chat-window-gui.cmd --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--bearer-token <token>] [--login <id>] [--password <secret>] [--label <name>] [--message-prefix <prefix>] [--diagnostics-file <path>] [--skip-connect] [--release]"
    Write-Host "Open one visible GUI chat window with visible account/password login, polling chat timeline/send-message commands, and RTC signaling controls for local video or voice call testing."
    if ($Help) {
        exit 0
    }
    exit 1
}

Write-Diagnostic "script start label=$resolvedLabel conversation=$ConversationId baseUrl=$resolvedBaseUrl"

Add-Type -AssemblyName System.Windows.Forms
Add-Type -AssemblyName System.Drawing
[System.Windows.Forms.Application]::SetUnhandledExceptionMode([System.Windows.Forms.UnhandledExceptionMode]::CatchException)

[System.Windows.Forms.Application]::add_ThreadException({
    param($sender, $eventArgs)
    Write-Diagnostic ("thread exception: " + $eventArgs.Exception.ToString())
})

[System.AppDomain]::CurrentDomain.add_UnhandledException({
    param($sender, $eventArgs)
    $exceptionText = if ($null -ne $eventArgs.ExceptionObject) {
        $eventArgs.ExceptionObject.ToString()
    }
    else {
        "null"
    }
    Write-Diagnostic ("unhandled exception terminating=$($eventArgs.IsTerminating): " + $exceptionText)
})

$form = New-Object System.Windows.Forms.Form
$form.Text = "craw-chat [$resolvedLabel] [$ConversationId]"
$form.Width = 1240
$form.Height = 780
$form.StartPosition = 'CenterScreen'

$statusLabel = New-Object System.Windows.Forms.Label
$statusLabel.Location = New-Object System.Drawing.Point(12, 12)
$statusLabel.Size = New-Object System.Drawing.Size(1190, 20)
$statusLabel.Text = "connecting: $resolvedLabel @ $resolvedBaseUrl"
$form.Controls.Add($statusLabel)

$authGroup = New-Object System.Windows.Forms.GroupBox
$authGroup.Location = New-Object System.Drawing.Point(12, 40)
$authGroup.Size = New-Object System.Drawing.Size(660, 96)
$authGroup.Text = "Login"
$form.Controls.Add($authGroup)

$userLabel = New-Object System.Windows.Forms.Label
$userLabel.Location = New-Object System.Drawing.Point(12, 24)
$userLabel.Size = New-Object System.Drawing.Size(80, 18)
$userLabel.Text = "User ID"
$authGroup.Controls.Add($userLabel)

$userIdDisplayBox = New-Object System.Windows.Forms.TextBox
$userIdDisplayBox.Location = New-Object System.Drawing.Point(12, 46)
$userIdDisplayBox.Size = New-Object System.Drawing.Size(120, 24)
$userIdDisplayBox.Text = $UserId
$authGroup.Controls.Add($userIdDisplayBox)

$conversationLabel = New-Object System.Windows.Forms.Label
$conversationLabel.Location = New-Object System.Drawing.Point(146, 24)
$conversationLabel.Size = New-Object System.Drawing.Size(90, 18)
$conversationLabel.Text = "Conversation"
$authGroup.Controls.Add($conversationLabel)

$conversationDisplayBox = New-Object System.Windows.Forms.TextBox
$conversationDisplayBox.Location = New-Object System.Drawing.Point(146, 46)
$conversationDisplayBox.Size = New-Object System.Drawing.Size(200, 24)
$conversationDisplayBox.Text = $ConversationId
$authGroup.Controls.Add($conversationDisplayBox)

$loginLabel = New-Object System.Windows.Forms.Label
$loginLabel.Location = New-Object System.Drawing.Point(360, 24)
$loginLabel.Size = New-Object System.Drawing.Size(80, 18)
$loginLabel.Text = "Login"
$authGroup.Controls.Add($loginLabel)

$loginTextBox = New-Object System.Windows.Forms.TextBox
$loginTextBox.Location = New-Object System.Drawing.Point(360, 46)
$loginTextBox.Size = New-Object System.Drawing.Size(120, 24)
$initialLoginValue = $(if ([string]::IsNullOrWhiteSpace($Login)) { $UserId } else { $Login })
$loginTextBox.Text = $initialLoginValue
$authGroup.Controls.Add($loginTextBox)

$passwordLabel = New-Object System.Windows.Forms.Label
$passwordLabel.Location = New-Object System.Drawing.Point(494, 24)
$passwordLabel.Size = New-Object System.Drawing.Size(80, 18)
$passwordLabel.Text = "Password"
$authGroup.Controls.Add($passwordLabel)

$passwordTextBox = New-Object System.Windows.Forms.TextBox
$passwordTextBox.Location = New-Object System.Drawing.Point(494, 46)
$passwordTextBox.Size = New-Object System.Drawing.Size(120, 24)
$passwordTextBox.UseSystemPasswordChar = $true
$initialPasswordValue = $Password
$initialResolvedLogin = $initialLoginValue.Trim()
if ([string]::IsNullOrWhiteSpace($initialPasswordValue) -and -not [string]::IsNullOrWhiteSpace($initialResolvedLogin)) {
    $seededPassword = Resolve-SeededImPassword -ResolvedLogin $initialResolvedLogin
    if (-not [string]::IsNullOrWhiteSpace($seededPassword)) {
        $initialPasswordValue = $seededPassword
        Write-Diagnostic ("seeded password prefilled for login=" + $initialResolvedLogin)
    }
}
$passwordTextBox.Text = $initialPasswordValue
$authGroup.Controls.Add($passwordTextBox)

$loginButton = New-Object System.Windows.Forms.Button
$loginButton.Location = New-Object System.Drawing.Point(360, 72)
$loginButton.Size = New-Object System.Drawing.Size(78, 20)
$loginButton.Text = "Login"
$authGroup.Controls.Add($loginButton)

$logoutButton = New-Object System.Windows.Forms.Button
$logoutButton.Location = New-Object System.Drawing.Point(446, 72)
$logoutButton.Size = New-Object System.Drawing.Size(78, 20)
$logoutButton.Text = "Logout"
$authGroup.Controls.Add($logoutButton)

$manualRefreshButton = New-Object System.Windows.Forms.Button
$manualRefreshButton.Location = New-Object System.Drawing.Point(532, 72)
$manualRefreshButton.Size = New-Object System.Drawing.Size(82, 20)
$manualRefreshButton.Text = "Refresh"
$authGroup.Controls.Add($manualRefreshButton)

$transcriptBox = New-Object System.Windows.Forms.TextBox
$transcriptBox.Location = New-Object System.Drawing.Point(12, 148)
$transcriptBox.Size = New-Object System.Drawing.Size(660, 470)
$transcriptBox.Multiline = $true
$transcriptBox.ReadOnly = $true
$transcriptBox.ScrollBars = 'Vertical'
$transcriptBox.Font = New-Object System.Drawing.Font("Consolas", 10)
$form.Controls.Add($transcriptBox)

$rtcGroup = New-Object System.Windows.Forms.GroupBox
$rtcGroup.Location = New-Object System.Drawing.Point(684, 40)
$rtcGroup.Size = New-Object System.Drawing.Size(536, 578)
$rtcGroup.Text = "RTC Signaling / Video Call Test"
$form.Controls.Add($rtcGroup)

$rtcSessionLabel = New-Object System.Windows.Forms.Label
$rtcSessionLabel.Location = New-Object System.Drawing.Point(12, 24)
$rtcSessionLabel.Size = New-Object System.Drawing.Size(90, 18)
$rtcSessionLabel.Text = "RTC Session"
$rtcGroup.Controls.Add($rtcSessionLabel)

$rtcSessionTextBox = New-Object System.Windows.Forms.TextBox
$rtcSessionTextBox.Location = New-Object System.Drawing.Point(12, 46)
$rtcSessionTextBox.Size = New-Object System.Drawing.Size(220, 24)
$rtcSessionTextBox.Text = "rtc_$ConversationId"
$rtcGroup.Controls.Add($rtcSessionTextBox)

$rtcModeLabel = New-Object System.Windows.Forms.Label
$rtcModeLabel.Location = New-Object System.Drawing.Point(246, 24)
$rtcModeLabel.Size = New-Object System.Drawing.Size(80, 18)
$rtcModeLabel.Text = "Mode"
$rtcGroup.Controls.Add($rtcModeLabel)

$rtcModeCombo = New-Object System.Windows.Forms.ComboBox
$rtcModeCombo.Location = New-Object System.Drawing.Point(246, 46)
$rtcModeCombo.Size = New-Object System.Drawing.Size(100, 24)
$rtcModeCombo.DropDownStyle = [System.Windows.Forms.ComboBoxStyle]::DropDownList
[void]$rtcModeCombo.Items.Add("video")
[void]$rtcModeCombo.Items.Add("voice")
$rtcModeCombo.SelectedIndex = 0
$rtcGroup.Controls.Add($rtcModeCombo)

$signalingStreamLabel = New-Object System.Windows.Forms.Label
$signalingStreamLabel.Location = New-Object System.Drawing.Point(12, 78)
$signalingStreamLabel.Size = New-Object System.Drawing.Size(130, 18)
$signalingStreamLabel.Text = "Signaling Stream"
$rtcGroup.Controls.Add($signalingStreamLabel)

$signalingStreamTextBox = New-Object System.Windows.Forms.TextBox
$signalingStreamTextBox.Location = New-Object System.Drawing.Point(12, 100)
$signalingStreamTextBox.Size = New-Object System.Drawing.Size(220, 24)
$signalingStreamTextBox.Text = "st_$ConversationId"
$rtcGroup.Controls.Add($signalingStreamTextBox)

$artifactMessageLabel = New-Object System.Windows.Forms.Label
$artifactMessageLabel.Location = New-Object System.Drawing.Point(246, 78)
$artifactMessageLabel.Size = New-Object System.Drawing.Size(120, 18)
$artifactMessageLabel.Text = "Artifact Message"
$rtcGroup.Controls.Add($artifactMessageLabel)

$artifactMessageTextBox = New-Object System.Windows.Forms.TextBox
$artifactMessageTextBox.Location = New-Object System.Drawing.Point(246, 100)
$artifactMessageTextBox.Size = New-Object System.Drawing.Size(220, 24)
$rtcGroup.Controls.Add($artifactMessageTextBox)

$signalTypeLabel = New-Object System.Windows.Forms.Label
$signalTypeLabel.Location = New-Object System.Drawing.Point(12, 132)
$signalTypeLabel.Size = New-Object System.Drawing.Size(90, 18)
$signalTypeLabel.Text = "Signal Type"
$rtcGroup.Controls.Add($signalTypeLabel)

$signalTypeTextBox = New-Object System.Windows.Forms.TextBox
$signalTypeTextBox.Location = New-Object System.Drawing.Point(12, 154)
$signalTypeTextBox.Size = New-Object System.Drawing.Size(160, 24)
$signalTypeTextBox.Text = "rtc.offer"
$rtcGroup.Controls.Add($signalTypeTextBox)

$schemaRefLabel = New-Object System.Windows.Forms.Label
$schemaRefLabel.Location = New-Object System.Drawing.Point(186, 132)
$schemaRefLabel.Size = New-Object System.Drawing.Size(90, 18)
$schemaRefLabel.Text = "Schema Ref"
$rtcGroup.Controls.Add($schemaRefLabel)

$schemaRefTextBox = New-Object System.Windows.Forms.TextBox
$schemaRefTextBox.Location = New-Object System.Drawing.Point(186, 154)
$schemaRefTextBox.Size = New-Object System.Drawing.Size(170, 24)
$schemaRefTextBox.Text = "webrtc.offer.v1"
$rtcGroup.Controls.Add($schemaRefTextBox)

$participantLabel = New-Object System.Windows.Forms.Label
$participantLabel.Location = New-Object System.Drawing.Point(370, 132)
$participantLabel.Size = New-Object System.Drawing.Size(120, 18)
$participantLabel.Text = "Credential User"
$rtcGroup.Controls.Add($participantLabel)

$participantTextBox = New-Object System.Windows.Forms.TextBox
$participantTextBox.Location = New-Object System.Drawing.Point(370, 154)
$participantTextBox.Size = New-Object System.Drawing.Size(140, 24)
$participantTextBox.Text = "u_guest"
$rtcGroup.Controls.Add($participantTextBox)

$signalPayloadLabel = New-Object System.Windows.Forms.Label
$signalPayloadLabel.Location = New-Object System.Drawing.Point(12, 186)
$signalPayloadLabel.Size = New-Object System.Drawing.Size(120, 18)
$signalPayloadLabel.Text = "Signal Payload"
$rtcGroup.Controls.Add($signalPayloadLabel)

$signalPayloadTextBox = New-Object System.Windows.Forms.TextBox
$signalPayloadTextBox.Location = New-Object System.Drawing.Point(12, 208)
$signalPayloadTextBox.Size = New-Object System.Drawing.Size(498, 160)
$signalPayloadTextBox.Multiline = $true
$signalPayloadTextBox.ScrollBars = 'Vertical'
$signalPayloadTextBox.Font = New-Object System.Drawing.Font("Consolas", 9)
$signalPayloadTextBox.Text = '{"sdp":"demo"}'
$rtcGroup.Controls.Add($signalPayloadTextBox)

$rtcCreateButton = New-Object System.Windows.Forms.Button
$rtcCreateButton.Location = New-Object System.Drawing.Point(12, 380)
$rtcCreateButton.Size = New-Object System.Drawing.Size(78, 28)
$rtcCreateButton.Text = "Create"
$rtcGroup.Controls.Add($rtcCreateButton)

$rtcInviteButton = New-Object System.Windows.Forms.Button
$rtcInviteButton.Location = New-Object System.Drawing.Point(98, 380)
$rtcInviteButton.Size = New-Object System.Drawing.Size(78, 28)
$rtcInviteButton.Text = "Invite"
$rtcGroup.Controls.Add($rtcInviteButton)

$rtcAcceptButton = New-Object System.Windows.Forms.Button
$rtcAcceptButton.Location = New-Object System.Drawing.Point(184, 380)
$rtcAcceptButton.Size = New-Object System.Drawing.Size(78, 28)
$rtcAcceptButton.Text = "Accept"
$rtcGroup.Controls.Add($rtcAcceptButton)

$rtcRejectButton = New-Object System.Windows.Forms.Button
$rtcRejectButton.Location = New-Object System.Drawing.Point(270, 380)
$rtcRejectButton.Size = New-Object System.Drawing.Size(78, 28)
$rtcRejectButton.Text = "Reject"
$rtcGroup.Controls.Add($rtcRejectButton)

$rtcEndButton = New-Object System.Windows.Forms.Button
$rtcEndButton.Location = New-Object System.Drawing.Point(356, 380)
$rtcEndButton.Size = New-Object System.Drawing.Size(78, 28)
$rtcEndButton.Text = "End"
$rtcGroup.Controls.Add($rtcEndButton)

$rtcSignalButton = New-Object System.Windows.Forms.Button
$rtcSignalButton.Location = New-Object System.Drawing.Point(12, 416)
$rtcSignalButton.Size = New-Object System.Drawing.Size(110, 28)
$rtcSignalButton.Text = "Send Signal"
$rtcGroup.Controls.Add($rtcSignalButton)

$rtcCredentialButton = New-Object System.Windows.Forms.Button
$rtcCredentialButton.Location = New-Object System.Drawing.Point(130, 416)
$rtcCredentialButton.Size = New-Object System.Drawing.Size(110, 28)
$rtcCredentialButton.Text = "Credentials"
$rtcGroup.Controls.Add($rtcCredentialButton)

$rtcRecordingButton = New-Object System.Windows.Forms.Button
$rtcRecordingButton.Location = New-Object System.Drawing.Point(248, 416)
$rtcRecordingButton.Size = New-Object System.Drawing.Size(100, 28)
$rtcRecordingButton.Text = "Recording"
$rtcGroup.Controls.Add($rtcRecordingButton)

$diagnosticsBox = New-Object System.Windows.Forms.TextBox
$diagnosticsBox.Location = New-Object System.Drawing.Point(12, 456)
$diagnosticsBox.Size = New-Object System.Drawing.Size(498, 98)
$diagnosticsBox.Multiline = $true
$diagnosticsBox.ReadOnly = $true
$diagnosticsBox.ScrollBars = 'Vertical'
$diagnosticsBox.Font = New-Object System.Drawing.Font("Consolas", 9)
$rtcGroup.Controls.Add($diagnosticsBox)

$inputBox = New-Object System.Windows.Forms.TextBox
$inputBox.Location = New-Object System.Drawing.Point(12, 630)
$inputBox.Size = New-Object System.Drawing.Size(1090, 28)
$inputBox.Font = New-Object System.Drawing.Font("Consolas", 11)
$form.Controls.Add($inputBox)

$sendButton = New-Object System.Windows.Forms.Button
$sendButton.Location = New-Object System.Drawing.Point(1110, 626)
$sendButton.Size = New-Object System.Drawing.Size(110, 34)
$sendButton.Text = "Send"
$form.Controls.Add($sendButton)

$footerLabel = New-Object System.Windows.Forms.Label
$footerLabel.Location = New-Object System.Drawing.Point(12, 670)
$footerLabel.Size = New-Object System.Drawing.Size(1190, 20)
$footerLabel.Text = "Use Login to acquire a bearer token. RTC buttons validate signaling only. Enter sends, /refresh refreshes, /quit closes."
$form.Controls.Add($footerLabel)

$refreshTimer = New-Object System.Windows.Forms.Timer
$refreshTimer.Interval = 1000
$refreshInProgress = $false
$lastTranscript = ""

$appendLine = {
    param([string]$line)

    if ([string]::IsNullOrWhiteSpace($line)) {
        return
    }

    $transcriptBox.AppendText($line + [Environment]::NewLine)
    $transcriptBox.SelectionStart = $transcriptBox.TextLength
    $transcriptBox.ScrollToCaret()
}

$appendDiagnostic = {
    param([string]$line)

    if ([string]::IsNullOrWhiteSpace($line)) {
        return
    }

    Write-Diagnostic $line
    $diagnosticsBox.AppendText($line + [Environment]::NewLine)
    $diagnosticsBox.SelectionStart = $diagnosticsBox.TextLength
    $diagnosticsBox.ScrollToCaret()
}

$setInteractiveState = {
    $hasBearer = $null -ne $script:resolvedAuthContext -and -not [string]::IsNullOrWhiteSpace([string]$script:resolvedAuthContext.BearerToken)
    $sendButton.Enabled = $hasBearer
    $logoutButton.Enabled = $hasBearer
    $manualRefreshButton.Enabled = $true
    $rtcCreateButton.Enabled = $hasBearer
    $rtcInviteButton.Enabled = $hasBearer
    $rtcAcceptButton.Enabled = $hasBearer
    $rtcRejectButton.Enabled = $hasBearer
    $rtcEndButton.Enabled = $hasBearer
    $rtcSignalButton.Enabled = $hasBearer
    $rtcCredentialButton.Enabled = $hasBearer
    $rtcRecordingButton.Enabled = $hasBearer
}

$setManualPendingState = {
    $script:resolvedAuthContext = $null
    $statusLabel.Text = "manual login required: $resolvedLabel @ $resolvedBaseUrl"
    & $appendDiagnostic "auth mode=manual-login-pending"
    & $setInteractiveState
}

$setAuthenticatedState = {
    param($authContext)

    $script:resolvedAuthContext = $authContext
    if (-not [string]::IsNullOrWhiteSpace([string]$authContext.UserId)) {
        $userIdDisplayBox.Text = [string]$authContext.UserId
    }
    $statusLabel.Text = "authenticated: $([string]$authContext.UserId) @ $resolvedBaseUrl"
    & $appendDiagnostic ("auth mode={0} user={1}" -f [string]$authContext.AuthMode, [string]$authContext.UserId)
    & $setInteractiveState
}

$performLogin = {
    $resolvedLoginInput = $loginTextBox.Text.Trim()
    $resolvedPasswordInput = $passwordTextBox.Text
    if ([string]::IsNullOrWhiteSpace($resolvedLoginInput)) {
        $statusLabel.Text = "login required: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic "login failed: login is required"
        return
    }
    if ([string]::IsNullOrWhiteSpace($resolvedPasswordInput)) {
        $statusLabel.Text = "password required: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic "login failed: password is required"
        return
    }

    $requestedUserId = $userIdDisplayBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($requestedUserId)) {
        $requestedUserId = $resolvedLoginInput
    }

    try {
        $authContext = Invoke-ImUserLogin `
            -ResolvedBaseUrl $resolvedBaseUrl `
            -RequestedUserId $requestedUserId `
            -ResolvedLogin $resolvedLoginInput `
            -ResolvedPassword $resolvedPasswordInput `
            -ResolvedSessionId $resolvedSessionId `
            -ResolvedDeviceId $resolvedDeviceId
        & $setAuthenticatedState $authContext
        if (-not $SkipConnect) {
            & $refreshTimeline
            $refreshTimer.Start()
        }
    }
    catch {
        $statusLabel.Text = "login failed: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic ("login failed: " + $_.Exception.Message)
    }
}

$invokeRtcAction = {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Action
    )

    if ($null -eq $script:resolvedAuthContext -or [string]::IsNullOrWhiteSpace([string]$script:resolvedAuthContext.BearerToken)) {
        $statusLabel.Text = "manual login required: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic ("rtc $Action failed: manual login is required before sending chat or RTC requests")
        return
    }

    $rtcSessionId = $rtcSessionTextBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($rtcSessionId)) {
        $statusLabel.Text = "rtc $Action failed: missing rtc session id"
        & $appendDiagnostic ("rtc $Action failed: rtc session id is required")
        return
    }

    $conversationIdValue = $conversationDisplayBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($conversationIdValue)) {
        $statusLabel.Text = "rtc $Action failed: missing conversation id"
        & $appendDiagnostic ("rtc $Action failed: conversation id is required")
        return
    }

    if ([string]::IsNullOrWhiteSpace($artifactMessageTextBox.Text) -and @("accept", "reject", "end") -contains $Action) {
        $artifactMessageTextBox.Text = New-ArtifactMessageId -Action $Action
    }

    try {
        switch ($Action) {
            "create" {
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions" `
                    -Body ([ordered]@{
                            rtcSessionId = $rtcSessionId
                            conversationId = $conversationIdValue
                            rtcMode = [string]$rtcModeCombo.SelectedItem
                        })
            }
            "invite" {
                Ensure-RtcSessionExistsForInvite `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -RtcSessionId $rtcSessionId `
                    -ConversationId $conversationIdValue `
                    -RtcMode ([string]$rtcModeCombo.SelectedItem)
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/invite" `
                    -Body ([ordered]@{
                            signalingStreamId = $signalingStreamTextBox.Text.Trim()
                        })
            }
            "accept" {
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/accept" `
                    -Body ([ordered]@{
                            artifactMessageId = $artifactMessageTextBox.Text.Trim()
                        })
            }
            "reject" {
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/reject" `
                    -Body ([ordered]@{
                            artifactMessageId = $artifactMessageTextBox.Text.Trim()
                        })
            }
            "end" {
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/end" `
                    -Body ([ordered]@{
                            artifactMessageId = $artifactMessageTextBox.Text.Trim()
                        })
            }
            "signal" {
                $signalPayloadValue = $signalPayloadTextBox.Text.Trim()
                if ([string]::IsNullOrWhiteSpace($signalPayloadValue)) {
                    throw "signal payload is required"
                }

                $payloadObject = $signalPayloadValue | ConvertFrom-Json
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/signals" `
                    -Body ([ordered]@{
                            signalType = $signalTypeTextBox.Text.Trim()
                            schemaRef = $schemaRefTextBox.Text.Trim()
                            payload = ($payloadObject | ConvertTo-Json -Depth 12 -Compress)
                        })
            }
            "credentials" {
                $participantId = $participantTextBox.Text.Trim()
                if ([string]::IsNullOrWhiteSpace($participantId)) {
                    throw "credential participant is required"
                }
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "POST" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/credentials" `
                    -Body ([ordered]@{
                            participantId = $participantId
                        })
            }
            "recording" {
                $response = Invoke-AuthenticatedJsonRequest `
                    -ResolvedBaseUrl $resolvedBaseUrl `
                    -BearerToken ([string]$script:resolvedAuthContext.BearerToken) `
                    -Method "GET" `
                    -Path "/im/v3/api/calls/sessions/$rtcSessionId/artifacts/recording" `
                    -Body $null
            }
            default {
                throw "unsupported rtc action '$Action'"
            }
        }

        & $appendDiagnostic ("rtc {0} ok: {1}" -f $Action, ($response | ConvertTo-Json -Depth 12 -Compress))
        if ($Action -ne "credentials" -and $Action -ne "recording") {
            & $refreshTimeline
        }
    }
    catch {
        $statusLabel.Text = "rtc $Action failed: $resolvedLabel @ $resolvedBaseUrl"
        $failureMessage = Get-RtcActionFailureMessage `
            -Action $Action `
            -FailureMessage ([string]$_.Exception.Message)
        & $appendDiagnostic ("rtc $Action failed: " + $failureMessage)
    }
}

$renderTimeline = {
    param($timeline)

    $items = @()
    if ($null -ne $timeline -and $null -ne $timeline.items) {
        $items = @($timeline.items | Sort-Object messageSeq)
    }

    $lines = foreach ($item in $items) {
        if (-not [string]::IsNullOrWhiteSpace($item.summary)) {
            $item.summary
        }
        else {
            "[message:$($item.messageId)]"
        }
    }

    $nextTranscript = ($lines -join [Environment]::NewLine)
    if ($lastTranscript -ne $nextTranscript) {
        $transcriptBox.Text = $nextTranscript
        $transcriptBox.SelectionStart = $transcriptBox.TextLength
        $transcriptBox.ScrollToCaret()
        $script:lastTranscript = $nextTranscript
    }
}

$refreshTimeline = {
    if ($refreshInProgress) {
        return
    }

    if ($null -eq $script:resolvedAuthContext -or [string]::IsNullOrWhiteSpace([string]$script:resolvedAuthContext.BearerToken)) {
        $statusLabel.Text = "manual login required: $resolvedLabel @ $resolvedBaseUrl"
        return
    }

    $conversationIdValue = $conversationDisplayBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($conversationIdValue)) {
        $statusLabel.Text = "conversation required: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic "timeline refresh skipped: conversation id is required"
        return
    }

    $script:refreshInProgress = $true
    try {
        $timeline = Invoke-ChatCliJson -Arguments ((Get-ChatCliAuthArguments) + @(
                "timeline",
                "--conversation-id", $conversationIdValue
            ))
        & $renderTimeline $timeline
        $statusLabel.Text = "connected: $resolvedLabel @ $resolvedBaseUrl (last sync $(Get-Date -Format 'HH:mm:ss'))"
    }
    catch {
        $statusLabel.Text = "error: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic ("timeline refresh failed: " + $_.Exception.Message)
    }
    finally {
        $script:refreshInProgress = $false
    }
}

$sendCurrent = {
    $text = $inputBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($text)) {
        return
    }

    if ($text -ieq "/quit" -or $text -ieq "/exit") {
        $form.Close()
        return
    }

    if ($text -ieq "/refresh") {
        & $refreshTimeline
        $inputBox.Clear()
        $inputBox.Focus()
        return
    }

    $conversationIdValue = $conversationDisplayBox.Text.Trim()
    if ([string]::IsNullOrWhiteSpace($conversationIdValue)) {
        $statusLabel.Text = "conversation required: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic "send failed: conversation id is required"
        $inputBox.Focus()
        return
    }

    $summary = if ([string]::IsNullOrWhiteSpace($resolvedMessagePrefix)) {
        $text
    }
    else {
        "$resolvedMessagePrefix$text"
    }

    try {
        $null = Invoke-ChatCliJson -Arguments ((Get-ChatCliAuthArguments) + @(
                "send-message",
                "--conversation-id", $conversationIdValue,
                "--summary", $summary,
                "--text", $summary,
                "--client-msg-id", (New-ClientMessageId)
            ))
        & $appendDiagnostic ("message sent: " + $summary)
        $inputBox.Clear()
        & $refreshTimeline
    }
    catch {
        $statusLabel.Text = "error: $resolvedLabel @ $resolvedBaseUrl"
        & $appendDiagnostic ("send failed: " + $_.Exception.Message)
    }

    $inputBox.Focus()
}

[void]$refreshTimer.Add_Tick({
    & $refreshTimeline
})

[void]$loginButton.Add_Click($performLogin)
[void]$logoutButton.Add_Click({
    $refreshTimer.Stop()
    & $setManualPendingState
})
[void]$manualRefreshButton.Add_Click($refreshTimeline)
[void]$sendButton.Add_Click($sendCurrent)
[void]$rtcCreateButton.Add_Click({ & $invokeRtcAction "create" })
[void]$rtcInviteButton.Add_Click({ & $invokeRtcAction "invite" })
[void]$rtcAcceptButton.Add_Click({ & $invokeRtcAction "accept" })
[void]$rtcRejectButton.Add_Click({ & $invokeRtcAction "reject" })
[void]$rtcEndButton.Add_Click({ & $invokeRtcAction "end" })
[void]$rtcSignalButton.Add_Click({ & $invokeRtcAction "signal" })
[void]$rtcCredentialButton.Add_Click({ & $invokeRtcAction "credentials" })
[void]$rtcRecordingButton.Add_Click({ & $invokeRtcAction "recording" })
[void]$inputBox.Add_KeyDown({
    param($sender, $eventArgs)
    if ($eventArgs.KeyCode -eq [System.Windows.Forms.Keys]::Enter -and -not $eventArgs.Shift) {
        $eventArgs.SuppressKeyPress = $true
        & $sendCurrent
    }
})

[void]$form.Add_Shown({
    Write-Diagnostic "form shown"
    $authContext = Resolve-ChatAuthContext
    if ([string]::IsNullOrWhiteSpace([string]$authContext.BearerToken)) {
        & $setManualPendingState
    }
    else {
        & $setAuthenticatedState $authContext
    }

    if ($SkipConnect) {
        $statusLabel.Text = "offline launch: $resolvedLabel @ $resolvedBaseUrl"
        Write-Diagnostic "skip-connect launch requested"
    }
    elseif (-not [string]::IsNullOrWhiteSpace([string]$authContext.BearerToken)) {
        & $refreshTimeline
        $refreshTimer.Start()
    }

    if (-not [string]::IsNullOrWhiteSpace($resolvedAutomationAction)) {
        if ([string]$authContext.AuthMode -eq "manual-login-pending") {
            Write-Diagnostic ("automation action={0} delayMs={1}" -f $resolvedAutomationAction, $resolvedAutomationDelayMs)
            $script:automationTimer = New-Object System.Windows.Forms.Timer
            $script:automationTimer.Interval = [Math]::Max(1, $resolvedAutomationDelayMs)
            [void]$script:automationTimer.Add_Tick({
                $script:automationTimer.Stop()
                $script:automationTimer.Dispose()
                $script:automationTimer = $null

                switch ($resolvedAutomationAction) {
                    "click-login" {
                        Write-Diagnostic "automation execute=click-login"
                        & $performLogin
                    }
                    default {
                        Write-Diagnostic ("automation ignored unsupported action=" + $resolvedAutomationAction)
                    }
                }
            })
            $script:automationTimer.Start()
        }
        else {
            Write-Diagnostic ("automation skipped action={0} authMode={1}" -f $resolvedAutomationAction, [string]$authContext.AuthMode)
        }
    }

    $inputBox.Focus()
})

[void]$form.Add_FormClosing({
    Write-Diagnostic "form closing"
    $refreshTimer.Stop()
})

& $setInteractiveState

try {
    [System.Windows.Forms.Application]::Run($form)
    Write-Diagnostic "application run completed"
}
catch {
    Write-Diagnostic ("application run exception: " + $_.Exception.ToString())
    throw
}
