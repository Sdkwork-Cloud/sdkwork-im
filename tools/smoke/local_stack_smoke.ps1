param(
    [string]$BaseUrl = "http://127.0.0.1:18090",
    [string]$PublicBearerSecret = "",
    [string]$BearerToken = ""
)

$ErrorActionPreference = 'Stop'

$DefaultHealthUrl = "http://127.0.0.1:18090/healthz"
$DefaultDockerPublicBearerSecret = "local-minimal-public-dev-secret"
$RepoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$LocalConfigFile = Join-Path $RepoRoot ".runtime\local-minimal\config\local-minimal.env"

function Read-ConfigValue {
    param(
        [string]$ConfigFile,
        [string]$Key
    )

    if (-not (Test-Path $ConfigFile)) {
        return $null
    }

    foreach ($line in Get-Content -Path $ConfigFile) {
        if ([string]::IsNullOrWhiteSpace($line) -or $line.TrimStart().StartsWith("#")) {
            continue
        }

        $parts = $line.Split("=", 2)
        if ($parts.Count -ne 2) {
            continue
        }

        if ($parts[0].Trim() -eq $Key) {
            $value = $parts[1].Trim()
            if (-not [string]::IsNullOrWhiteSpace($value)) {
                return $value
            }
        }
    }

    return $null
}

function ConvertTo-Base64Url {
    param([byte[]]$Bytes)

    return [Convert]::ToBase64String($Bytes).TrimEnd('=').Replace('+', '-').Replace('/', '_')
}

function New-Hs256BearerToken {
    param(
        [string]$Secret,
        [hashtable]$Claims
    )

    $headerJson = '{"alg":"HS256","typ":"JWT"}'
    $payloadJson = $Claims | ConvertTo-Json -Compress -Depth 6
    $headerSegment = ConvertTo-Base64Url -Bytes ([System.Text.Encoding]::UTF8.GetBytes($headerJson))
    $payloadSegment = ConvertTo-Base64Url -Bytes ([System.Text.Encoding]::UTF8.GetBytes($payloadJson))
    $signingInput = "$headerSegment.$payloadSegment"
    $hmac = [System.Security.Cryptography.HMACSHA256]::new([System.Text.Encoding]::UTF8.GetBytes($Secret))
    try {
        $signatureBytes = $hmac.ComputeHash([System.Text.Encoding]::UTF8.GetBytes($signingInput))
    }
    finally {
        $hmac.Dispose()
    }

    $signatureSegment = ConvertTo-Base64Url -Bytes $signatureBytes
    return "$signingInput.$signatureSegment"
}

function Resolve-PublicBearerSecret {
    param([string]$ExplicitSecret)

    if (-not [string]::IsNullOrWhiteSpace($ExplicitSecret)) {
        return $ExplicitSecret.Trim()
    }

    if (-not [string]::IsNullOrWhiteSpace($env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET)) {
        return $env:CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET.Trim()
    }

    $configSecret = Read-ConfigValue -ConfigFile $LocalConfigFile -Key "CRAW_CHAT_PUBLIC_BEARER_HS256_SECRET"
    if (-not [string]::IsNullOrWhiteSpace($configSecret)) {
        return $configSecret
    }

    return $DefaultDockerPublicBearerSecret
}

function Resolve-AuthorizationHeader {
    param(
        [string]$ExplicitBearerToken,
        [string]$ExplicitPublicBearerSecret
    )

    if (-not [string]::IsNullOrWhiteSpace($ExplicitBearerToken)) {
        $trimmed = $ExplicitBearerToken.Trim()
        if ($trimmed -match '^(?i)Bearer\s+') {
            return $trimmed
        }

        return "Bearer $trimmed"
    }

    $secret = Resolve-PublicBearerSecret -ExplicitSecret $ExplicitPublicBearerSecret
    $token = New-Hs256BearerToken -Secret $secret -Claims @{
        tenant_id = "t_demo"
        sub = "u_demo"
        actor_kind = "user"
        sid = "s_demo"
    }

    return "Bearer $token"
}

$headers = @{
    "Authorization" = (Resolve-AuthorizationHeader -ExplicitBearerToken $BearerToken -ExplicitPublicBearerSecret $PublicBearerSecret)
    "Content-Type" = "application/json"
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

$conversationId = "c_smoke_$([DateTimeOffset]::UtcNow.ToUnixTimeSeconds())"

$createBody = @{
    conversationId = $conversationId
    conversationType = "group"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/api/v1/conversations" -Headers $headers -Body $createBody

$messageBody = @{
    clientMsgId = "smoke_client"
    summary = "smoke"
    text = "smoke"
} | ConvertTo-Json -Depth 6

$null = Invoke-RestMethod -Method Post -Uri "$BaseUrl/api/v1/conversations/$conversationId/messages" -Headers $headers -Body $messageBody

$summary = Invoke-RestMethod -Method Get -Uri "$BaseUrl/api/v1/conversations/$conversationId" -Headers $headers
if ($summary.lastSummary -ne "smoke") {
    throw "Unexpected conversation summary payload"
}

Write-Host "local stack smoke check passed."
