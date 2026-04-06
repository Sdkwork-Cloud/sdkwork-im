param(
    [string]$BaseUrl = "http://127.0.0.1:18090"
)

$ErrorActionPreference = 'Stop'

$DefaultHealthUrl = "http://127.0.0.1:18090/healthz"

$headers = @{
    "Authorization" = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ."
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
