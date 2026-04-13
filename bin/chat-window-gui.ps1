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

if ($Help -or [string]::IsNullOrWhiteSpace($ConversationId) -or [string]::IsNullOrWhiteSpace($UserId)) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/chat-window-gui.ps1 -ConversationId <id> -UserId <id> [-BaseUrl <url>] [-TenantId <id>] [-SessionId <id>] [-DeviceId <id>] [-Label <name>] [-MessagePrefix <prefix>] [-DiagnosticsFile <path>] [-SkipConnect] [-Release]"
    Write-Host "Usage: cmd /c .\bin\chat-window-gui.cmd --conversation-id <id> --user-id <id> [--base-url <url>] [--tenant-id <id>] [--session-id <id>] [--device-id <id>] [--label <name>] [--message-prefix <prefix>] [--diagnostics-file <path>] [--skip-connect] [--release]"
    Write-Host "Open one visible GUI chat window backed by polling chat-cli timeline/send-message commands."
    if ($Help) {
        exit 0
    }
    exit 1
}

$resolvedBaseUrl = Resolve-BaseUrl -RequestedBaseUrl $BaseUrl
$resolvedLabel = if ([string]::IsNullOrWhiteSpace($Label)) { $UserId } else { $Label }
$resolvedSessionId = if ([string]::IsNullOrWhiteSpace($SessionId)) { "s_$UserId" } else { $SessionId }
$resolvedDeviceId = if ([string]::IsNullOrWhiteSpace($DeviceId)) { "d_$UserId" } else { $DeviceId }
$resolvedMessagePrefix = if ($PSBoundParameters.ContainsKey('MessagePrefix')) { $MessagePrefix } else { "[$resolvedLabel] " }
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

function Resolve-ChatCliExecutablePath {
    $root = Split-Path -Parent $PSScriptRoot
    $profileDir = if ($Release) { "release" } else { "debug" }
    $exePath = Join-Path $root "target\$profileDir\craw-chat-cli.exe"

    if (-not (Test-Path $exePath)) {
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

function New-ClientMessageId {
    return "gui_{0}_{1}_{2}" -f `
        ($resolvedLabel -replace '[^a-zA-Z0-9_-]', '_'), `
        (Get-Date -Format "yyyyMMddHHmmssfff"), `
        ([guid]::NewGuid().ToString("N").Substring(0, 8))
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
$form.Width = 900
$form.Height = 640
$form.StartPosition = 'CenterScreen'

$statusLabel = New-Object System.Windows.Forms.Label
$statusLabel.Location = New-Object System.Drawing.Point(12, 12)
$statusLabel.Size = New-Object System.Drawing.Size(860, 20)
$statusLabel.Text = "connecting: $resolvedLabel @ $resolvedBaseUrl"
$form.Controls.Add($statusLabel)

$transcriptBox = New-Object System.Windows.Forms.TextBox
$transcriptBox.Location = New-Object System.Drawing.Point(12, 40)
$transcriptBox.Size = New-Object System.Drawing.Size(860, 500)
$transcriptBox.Multiline = $true
$transcriptBox.ReadOnly = $true
$transcriptBox.ScrollBars = 'Vertical'
$transcriptBox.Font = New-Object System.Drawing.Font("Consolas", 10)
$form.Controls.Add($transcriptBox)

$inputBox = New-Object System.Windows.Forms.TextBox
$inputBox.Location = New-Object System.Drawing.Point(12, 552)
$inputBox.Size = New-Object System.Drawing.Size(740, 28)
$inputBox.Font = New-Object System.Drawing.Font("Consolas", 11)
$form.Controls.Add($inputBox)

$sendButton = New-Object System.Windows.Forms.Button
$sendButton.Location = New-Object System.Drawing.Point(760, 548)
$sendButton.Size = New-Object System.Drawing.Size(112, 34)
$sendButton.Text = "Send"
$form.Controls.Add($sendButton)

$footerLabel = New-Object System.Windows.Forms.Label
$footerLabel.Location = New-Object System.Drawing.Point(12, 590)
$footerLabel.Size = New-Object System.Drawing.Size(860, 20)
$footerLabel.Text = "Enter to send, /quit to close chat window."
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

    $script:refreshInProgress = $true
    try {
        $timeline = Invoke-ChatCliJson -Arguments @(
            "--base-url", $resolvedBaseUrl,
            "--tenant-id", $TenantId,
            "--user-id", $UserId,
            "--session-id", $resolvedSessionId,
            "--device-id", $resolvedDeviceId,
            "timeline",
            "--conversation-id", $ConversationId
        )
        & $renderTimeline $timeline
        $statusLabel.Text = "connected: $resolvedLabel @ $resolvedBaseUrl (last sync $(Get-Date -Format 'HH:mm:ss'))"
    }
    catch {
        $statusLabel.Text = "error: $resolvedLabel @ $resolvedBaseUrl"
        Write-Diagnostic ("timeline refresh failed: " + $_.Exception.ToString())
        if ($transcriptBox.Text -notlike "*[error]*$($_.Exception.Message)*") {
            & $appendLine ("[error] " + $_.Exception.Message)
        }
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

    $summary = if ([string]::IsNullOrWhiteSpace($resolvedMessagePrefix)) {
        $text
    }
    else {
        "$resolvedMessagePrefix$text"
    }

    try {
        $null = Invoke-ChatCliJson -Arguments @(
            "--base-url", $resolvedBaseUrl,
            "--tenant-id", $TenantId,
            "--user-id", $UserId,
            "--session-id", $resolvedSessionId,
            "--device-id", $resolvedDeviceId,
            "send-message",
            "--conversation-id", $ConversationId,
            "--summary", $summary,
            "--text", $summary,
            "--client-msg-id", (New-ClientMessageId)
        )
        Write-Diagnostic ("message sent: " + $summary)
        $inputBox.Clear()
        & $refreshTimeline
    }
    catch {
        $statusLabel.Text = "error: $resolvedLabel @ $resolvedBaseUrl"
        Write-Diagnostic ("send failed: " + $_.Exception.ToString())
        & $appendLine ("[error] " + $_.Exception.Message)
    }

    $inputBox.Focus()
}

[void]$refreshTimer.Add_Tick({
    & $refreshTimeline
})

[void]$sendButton.Add_Click($sendCurrent)
[void]$inputBox.Add_KeyDown({
    param($sender, $eventArgs)
    if ($eventArgs.KeyCode -eq [System.Windows.Forms.Keys]::Enter -and -not $eventArgs.Shift) {
        $eventArgs.SuppressKeyPress = $true
        & $sendCurrent
    }
})

[void]$form.Add_Shown({
    Write-Diagnostic "form shown"
    if ($SkipConnect) {
        $statusLabel.Text = "offline launch: $resolvedLabel @ $resolvedBaseUrl"
        Write-Diagnostic "skip-connect launch requested"
    }
    else {
        & $refreshTimeline
        $refreshTimer.Start()
    }
    $inputBox.Focus()
})

[void]$form.Add_FormClosing({
    Write-Diagnostic "form closing"
    $refreshTimer.Stop()
})

try {
    [System.Windows.Forms.Application]::Run($form)
    Write-Diagnostic "application run completed"
}
catch {
    Write-Diagnostic ("application run exception: " + $_.Exception.ToString())
    throw
}
