param(
    [string]$BaseUrl = "http://127.0.0.1:18090"
)

$ErrorActionPreference = 'Stop'

$headers = @{
    "x-sdkwork-tenant-id" = "t_demo"
    "x-sdkwork-user-id" = "u_demo"
    "x-sdkwork-actor-id" = "u_demo"
    "x-sdkwork-actor-kind" = "user"
    "x-sdkwork-session-id" = "s_demo"
    "x-sdkwork-device-id" = "d_demo"
    "x-sdkwork-permission-scope" = "automation.execute automation.read audit.read ops.read"
    "Content-Type" = "application/json"
}
$defaultBaseUrl = "http://127.0.0.1:18090"
$signedAppContextHeaderNames = @(
    "x-sdkwork-app-id",
    "x-sdkwork-tenant-id",
    "x-sdkwork-organization-id",
    "x-sdkwork-user-id",
    "x-sdkwork-session-id",
    "x-sdkwork-environment",
    "x-sdkwork-deployment-mode",
    "x-sdkwork-auth-level",
    "x-sdkwork-data-scope",
    "x-sdkwork-permission-scope",
    "x-sdkwork-actor-id",
    "x-sdkwork-actor-kind",
    "x-sdkwork-device-id"
)

function Test-Truthy {
    param([string]$Value)

    if ([string]::IsNullOrWhiteSpace($Value)) {
        return $false
    }

    return @("1", "true", "yes", "on").Contains($Value.Trim().ToLowerInvariant())
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

function Resolve-LocalConfigValue {
    param([string]$Key)

    foreach ($configFile in @(
        ".runtime\local-minimal\config\local-minimal.env",
        ".runtime\local-default\config\local-default.env"
    )) {
        $value = Read-ConfigValue -ConfigFile $configFile -Key $Key
        if (-not [string]::IsNullOrWhiteSpace($value)) {
            return $value
        }
    }

    return $null
}

function Resolve-DefaultComposeSignatureSecret {
    $composeFile = "deployments\docker-compose\local-minimal.yml"
    if ($BaseUrl -ne $defaultBaseUrl -or -not (Test-Path $composeFile)) {
        return $null
    }

    foreach ($line in Get-Content -Path $composeFile) {
        $trimmed = $line.Trim()
        if ($trimmed.StartsWith("CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET:")) {
            return ($trimmed -split ':', 2)[1].Trim().Trim('"').Trim("'")
        }
    }

    return $null
}

function Get-AppContextHeaderValue {
    param([string]$HeaderName)

    if ($headers.ContainsKey($HeaderName)) {
        $value = $headers[$HeaderName]
        if ($null -ne $value) {
            return $value.ToString().Trim()
        }
    }

    return ""
}

function Get-CanonicalAppContextHeaders {
    $lines = foreach ($headerName in $signedAppContextHeaderNames) {
        "${headerName}:$(Get-AppContextHeaderValue -HeaderName $headerName)"
    }

    return ($lines -join "`n")
}

function New-AppContextSignature {
    param([Parameter(Mandatory = $true)][string]$Secret)

    $secretBytes = [System.Text.Encoding]::UTF8.GetBytes($Secret)
    $payloadBytes = [System.Text.Encoding]::UTF8.GetBytes((Get-CanonicalAppContextHeaders))
    $hmac = [System.Security.Cryptography.HMACSHA256]::new($secretBytes)
    try {
        $digest = $hmac.ComputeHash($payloadBytes)
    }
    finally {
        $hmac.Dispose()
    }

    return ([Convert]::ToBase64String($digest)).TrimEnd('=').Replace('+', '-').Replace('/', '_')
}

function Set-AppContextSignatureHeader {
    $requireSignature = $env:CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE
    $signatureSecret = $env:CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET

    if ([string]::IsNullOrWhiteSpace($requireSignature)) {
        $requireSignature = Resolve-LocalConfigValue -Key "CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE"
    }
    if ([string]::IsNullOrWhiteSpace($signatureSecret)) {
        $signatureSecret = Resolve-LocalConfigValue -Key "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET"
    }
    if ([string]::IsNullOrWhiteSpace($signatureSecret)) {
        $signatureSecret = Resolve-DefaultComposeSignatureSecret
    }

    if (-not [string]::IsNullOrWhiteSpace($signatureSecret)) {
        $headers["x-sdkwork-context-signature"] = New-AppContextSignature -Secret $signatureSecret
        return
    }

    if (Test-Truthy -Value $requireSignature) {
        throw "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET is required when CRAW_CHAT_APP_CONTEXT_REQUIRE_SIGNATURE=true."
    }
}

function Wait-Healthy {
    param([string]$Url)

    $healthUrl = "$Url/healthz"
    for ($attempt = 0; $attempt -lt 20; $attempt++) {
        try {
            $response = Invoke-WebRequest -UseBasicParsing $healthUrl
            if ($response.StatusCode -eq 200) {
                return
            }
        }
        catch {
            Start-Sleep -Seconds 2
        }
    }

    throw "Timed out waiting for $Url/healthz"
}

Wait-Healthy -Url $BaseUrl
Set-AppContextSignatureHeader

$suffix = [DateTimeOffset]::UtcNow.ToUnixTimeSeconds()
$conversationId = "c_e2e_$suffix"
$executionId = "ae_e2e_$suffix"

$createConversationBody = @{
    conversationId = $conversationId
    conversationType = "group"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations" -Headers $headers -Body $createConversationBody

$messageBody = @{
    clientMsgId = "e2e_client_$suffix"
    summary = "e2e hello"
    text = "e2e hello"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId/messages" -Headers $headers -Body $messageBody

$notifications = Invoke-RestMethod -Method Get -Uri "$BaseUrl/app/v3/api/notifications" -Headers $headers
if ($notifications.items.Count -lt 1) {
    throw "Expected at least one notification"
}
if ($notifications.items[0].sourceEventType -ne "message.posted") {
    throw "Unexpected notification source event type"
}

$automationBody = @{
    executionId = $executionId
    triggerType = "webhook.manual"
    targetKind = "workflow"
    targetRef = "wf_e2e_demo"
    inputPayload = "{""conversationId"":""$conversationId""}"
} | ConvertTo-Json -Depth 6

$automation = Invoke-RestMethod -Method Post -Uri "$BaseUrl/app/v3/api/automation/executions" -Headers $headers -Body $automationBody
if ($automation.state -ne "succeeded") {
    throw "Unexpected automation execution state"
}

$auditExport = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/audit/export" -Headers $headers
if ($auditExport.total -lt 2) {
    throw "Expected at least two audit records"
}

$opsCluster = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/ops/cluster" -Headers $headers
if ($opsCluster.nodes[0].profile -ne "local-minimal") {
    throw "Unexpected ops cluster profile"
}

$opsDiagnostics = Invoke-RestMethod -Method Get -Uri "$BaseUrl/backend/v3/api/ops/diagnostics" -Headers $headers
if (-not $opsDiagnostics.nodeId) {
    throw "Missing ops diagnostic nodeId"
}

Write-Host "end-to-end smoke check passed."
