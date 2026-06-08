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
    "x-sdkwork-permission-scope" = "chat.write"
    "Content-Type" = "application/json"
}

$defaultBaseUrl = "http://127.0.0.1:18090"
$defaultHealthUrl = "http://127.0.0.1:18090/healthz"
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
    if ($Url -eq $defaultBaseUrl) {
        $healthUrl = $defaultHealthUrl
    }
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

$conversationId = "c_smoke_$([DateTimeOffset]::UtcNow.ToUnixTimeSeconds())"

$createBody = @{
    conversationId = $conversationId
    conversationType = "group"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations" -Headers $headers -Body $createBody

$messageBody = @{
    clientMsgId = "smoke_client"
    summary = "smoke"
    text = "smoke"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId/messages" -Headers $headers -Body $messageBody

$summary = Invoke-RestMethod -Method Get -Uri "$BaseUrl/im/v3/api/chat/conversations/$conversationId" -Headers $headers
if ($summary.lastSummary -ne "smoke") {
    throw "Unexpected conversation summary payload"
}

Write-Host "local stack smoke check passed."
