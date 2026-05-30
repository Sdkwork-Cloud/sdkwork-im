param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$DataDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "data")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
    [string]$BindAddress = "0.0.0.0:18080",
    [string]$BaseUrl = "http://127.0.0.1:18080",
    [string]$ApiBaseUrl = "http://127.0.0.1:18080",
    [string]$WebsocketBaseUrl = "ws://127.0.0.1:18080/im/v3/api/realtime/ws",
    [switch]$NonInteractive,
    [switch]$Force,
    [switch]$Help
)

$ErrorActionPreference = "Stop"

if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("ConfigDir")) {
    $ConfigDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "config")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("DataDir")) {
    $DataDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "data")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("LogDir")) {
    $LogDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "logs")
}
if ($PSBoundParameters.ContainsKey("InstanceName") -and -not $PSBoundParameters.ContainsKey("RunDir")) {
    $RunDir = [System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", $InstanceName, "run")
}

if ($Help) {
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-config-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-DataDir <path>] [-LogDir <path>] [-RunDir <path>] [-BindAddress <host:port>] [-BaseUrl <url>] [-ApiBaseUrl <url>] [-WebsocketBaseUrl <url>] [-NonInteractive] [-Force]"
    Write-Host "Usage: cmd /c .\bin\init-config-server.cmd [--instance <name>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--bind-address <host:port>] [--base-url <url>] [--api-base-url <url>] [--websocket-base-url <url>] [--non-interactive] [--force]"
    Write-Host "Render craw-chat-server configuration files for the selected instance and preserve file-based PostgreSQL settings."
    exit 0
}

$storageDir = Join-Path $ConfigDir "storage"
$secretsDir = Join-Path $ConfigDir "secrets"
foreach ($path in @($ConfigDir, $DataDir, $LogDir, $RunDir, $storageDir, $secretsDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$serverYamlPath = Join-Path $ConfigDir "server.yaml"
$serverEnvPath = Join-Path $ConfigDir "server.env"
$postgresqlPath = Join-Path $storageDir "postgresql.yaml"
$passwordFilePath = Join-Path $secretsDir "postgresql.password"

if ((-not (Test-Path $serverYamlPath)) -or $Force) {
    @"
instance:
  name: "$InstanceName"

network:
  bindAddress: "$BindAddress"

publicEndpoints:
  baseUrl: "$BaseUrl"
  apiBaseUrl: "$ApiBaseUrl"
  websocketBaseUrl: "$WebsocketBaseUrl"
  docsBaseUrl: "$BaseUrl/docs"

runtime:
  configDir: "$ConfigDir"
  dataDir: "$DataDir"
  logDir: "$LogDir"
  runDir: "$RunDir"

storage:
  postgresqlConfig: "$postgresqlPath"
"@ | Set-Content -Path $serverYamlPath -Encoding utf8
}

if ((-not (Test-Path $serverEnvPath)) -or $Force) {
    @"
CRAW_CHAT_SERVER_INSTANCE=$InstanceName
CRAW_CHAT_SERVER_CONFIG_DIR=$ConfigDir
CRAW_CHAT_SERVER_DATA_DIR=$DataDir
CRAW_CHAT_SERVER_LOG_DIR=$LogDir
CRAW_CHAT_SERVER_RUN_DIR=$RunDir
CRAW_CHAT_SERVER_BIND_ADDRESS=$BindAddress
CRAW_CHAT_SERVER_BASE_URL=$BaseUrl
CRAW_CHAT_SERVER_API_BASE_URL=$ApiBaseUrl
CRAW_CHAT_SERVER_WEBSOCKET_BASE_URL=$WebsocketBaseUrl
"@ | Set-Content -Path $serverEnvPath -Encoding utf8
}

if ((-not (Test-Path $postgresqlPath)) -or $Force) {
    @"
provider: postgresql

connection:
  host: 127.0.0.1
  port: 5432
  database: craw_chat
  username: craw_chat_app
  passwordFile: "$passwordFilePath"
  sslmode: prefer
  applicationName: craw-chat-server
  connectTimeoutSeconds: 10

schema:
  name: craw_chat
  provisioningMode: none
  migrationMode: apply
  expectedVersion: latest

pool:
  minConnections: 5
  maxConnections: 30
  idleTimeoutSeconds: 300
  maxLifetimeSeconds: 1800

# init-storage-server modes:
# - verify-only
# - bootstrap-schema
# - create-db-and-schema
"@ | Set-Content -Path $postgresqlPath -Encoding utf8
}

if ((-not (Test-Path $passwordFilePath)) -or $Force) {
    "replace-me" | Set-Content -Path $passwordFilePath -Encoding utf8
}

Write-Host "Rendered craw-chat-server configuration for instance '$InstanceName'."
Write-Host "server.yaml: $serverYamlPath"
Write-Host "server.env: $serverEnvPath"
Write-Host "storage/postgresql.yaml: $postgresqlPath"
