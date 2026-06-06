param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")),
    [string]$ReleaseGatePath = "",
    [ValidateSet("text", "json")]
    [string]$OutputFormat = "text",
    [switch]$Help
)

$ErrorActionPreference = "Stop"

function Get-ServerConfigDirForInstance {
    param([string]$Name)

    $root = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "sdkwork", "chat")
    if ($Name -eq "default") {
        return $root
    }
    return [System.IO.Path]::Combine($root, "instances", $Name)
}

if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = Get-ServerConfigDirForInstance $InstanceName
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/verify-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-ReleaseGatePath <path-to-release-gate.json>] [-OutputFormat <text|json>]"
    Write-Host "Validate config, storage wiring, and ready state for craw-chat-server, and optionally audit the machine-readable release-gate bundle for semantic contract drift, decisionStatus, and semanticIssues."
    exit 0
}

function Get-ReleaseContractReport {
    param(
        [Parameter(Mandatory = $true)]
        [string]$GatePath
    )

    $verifierPath = Join-Path $PSScriptRoot "verify-server-release-contracts.mjs"
    $json = & node $verifierPath --release-gate-path $GatePath --format json
    if ($LASTEXITCODE -ne 0) {
        throw "verify-server-release-contracts.mjs failed for gate path: $GatePath"
    }

    return $json | ConvertFrom-Json
}

$serverYamlPath = Join-Path $ConfigDir "chat.toml"
$postgresqlPath = Join-Path $ConfigDir "postgresql.yaml"
$passwordFilePath = Join-Path $ConfigDir "database.secret"
$missing = New-Object System.Collections.Generic.List[string]

if (-not (Test-Path $serverYamlPath)) { $missing.Add("chat.toml") }
if (-not (Test-Path $postgresqlPath)) { $missing.Add("postgresql.yaml") }
if (-not (Test-Path $passwordFilePath)) { $missing.Add("database.secret") }

$serverContent = if (Test-Path $serverYamlPath) { Get-Content -Path $serverYamlPath -Raw } else { "" }
$storageContent = if (Test-Path $postgresqlPath) { Get-Content -Path $postgresqlPath -Raw } else { "" }

foreach ($contract in @("[runtime]", "deployment_mode = ""server""", "app_code = ""chat""", "[server]", "bind_address =")) {
    if (-not $serverContent.Contains($contract)) { $missing.Add($contract) }
}
foreach ($contract in @("provider: postgresql", "passwordFile:", "migrationMode:")) {
    if (-not $storageContent.Contains($contract)) { $missing.Add($contract) }
}

$releaseContracts = [ordered]@{
    enabled = $false
}

if (-not [string]::IsNullOrWhiteSpace($ReleaseGatePath)) {
    $releaseContracts = Get-ReleaseContractReport -GatePath $ReleaseGatePath
}

$ready = ($missing.Count -eq 0)
if ($releaseContracts.enabled) {
    $ready = $ready -and [bool]$releaseContracts.contractsValid
}

$result = [ordered]@{
    product = "craw-chat-server"
    instance = $InstanceName
    config = $ConfigDir
    configValid = ($missing.Count -eq 0)
    storageValid = ($missing.Count -eq 0)
    ready = $ready
    output = $OutputFormat
    missing = @($missing)
    releaseContracts = $releaseContracts
}

$json = $result | ConvertTo-Json -Depth 5
if ($OutputFormat -eq "json") {
    Write-Output $json
}
else {
    Write-Host "craw-chat-server verify report"
    Write-Host "config: $ConfigDir"
    Write-Host "configValid: $($result.configValid)"
    Write-Host "storageValid: $($result.storageValid)"
    Write-Host "ready: $($result.ready)"
    if ($missing.Count -gt 0) {
        Write-Host "missing: $($missing -join ', ')"
    }
    if ($releaseContracts.enabled) {
        Write-Host "releaseGate: $($releaseContracts.gatePath)"
        Write-Host "releaseBundle: $($releaseContracts.bundleId)"
        Write-Host "releaseDecisionStatus: $($releaseContracts.decisionStatus)"
        Write-Host "releasePlatforms: $($releaseContracts.platforms -join ', ')"
        Write-Host "releaseContractsValid: $($releaseContracts.contractsValid)"
        Write-Host "releaseSemanticIssueCount: $($releaseContracts.semanticIssueCount)"
        if ($releaseContracts.missing.Count -gt 0) {
            Write-Host "releaseMissing: $($releaseContracts.missing -join ', ')"
        }
        if ($releaseContracts.semanticIssues.Count -gt 0) {
            Write-Host "releaseSemanticIssues: $($releaseContracts.semanticIssues -join ', ')"
        }
    }
}
