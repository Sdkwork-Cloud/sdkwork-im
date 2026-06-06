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
