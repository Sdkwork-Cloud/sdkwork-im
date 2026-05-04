param(
    [string]$InstanceName = "default",
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$DataDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "data")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
    [string]$BindAddress = "0.0.0.0:18080",
    [string]$BaseUrl = "http://127.0.0.1:18080",
    [string]$ApiBaseUrl = "http://127.0.0.1:18080",
    [string]$WebsocketBaseUrl = "ws://127.0.0.1:18080/api/v1/realtime/ws",
    [string]$UserCenterMode = "builtin-local",
    [string]$UserCenterProviderKey = "craw-chat-server-local",
    [string]$UserCenterLocalApiBasePath = "/api/app/v1/user-center",
    [string]$UserCenterAuthorizationHeaderName = "Authorization",
    [string]$UserCenterAccessTokenHeaderName = "Access-Token",
    [string]$UserCenterRefreshTokenHeaderName = "Refresh-Token",
    [string]$UserCenterSessionHeaderName = "x-sdkwork-user-center-session-id",
    [string]$UserCenterAuthorizationScheme = "Bearer",
    [string]$UserCenterAllowAuthorizationFallbackToAccessToken = "true",
    [string]$UserCenterAppId = "craw-chat-server",
    [string]$UserCenterAppApiBaseUrl = "",
    [string]$UserCenterSecretId = "",
    [string]$UserCenterSharedSecret = "",
    [string]$UserCenterExternalBaseUrl = "",
    [string]$UserCenterDatabaseUrl = "",
    [string]$UserCenterSchemaName = "",
    [string]$UserCenterSqlitePath = "",
    [string]$UserCenterTablePrefix = "cc_uc_",
    [string]$UserCenterHandshakeFreshnessWindowMs = "",
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/init-config-server.ps1 [-InstanceName <name>] [-ConfigDir <path>] [-DataDir <path>] [-LogDir <path>] [-RunDir <path>] [-BindAddress <host:port>] [-BaseUrl <url>] [-ApiBaseUrl <url>] [-WebsocketBaseUrl <url>] [-UserCenterMode <builtin-local|sdkwork-cloud-app-api|external-user-center>] [-UserCenterProviderKey <key>] [-UserCenterLocalApiBasePath <path>] [-UserCenterAppApiBaseUrl <url>] [-UserCenterSecretId <id>] [-UserCenterSharedSecret <secret>] [-UserCenterExternalBaseUrl <url>] [-NonInteractive] [-Force]"
    Write-Host "Usage: cmd /c .\bin\init-config-server.cmd [--instance <name>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--bind-address <host:port>] [--base-url <url>] [--api-base-url <url>] [--websocket-base-url <url>] [--user-center-mode <builtin-local|sdkwork-cloud-app-api|external-user-center>] [--user-center-provider-key <key>] [--user-center-local-api-base-path <path>] [--user-center-app-api-base-url <url>] [--user-center-secret-id <id>] [--user-center-shared-secret <secret>] [--user-center-external-base-url <url>] [--non-interactive] [--force]"
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
CRAW_CHAT_SERVER_USER_CENTER_MODE=$UserCenterMode
CRAW_CHAT_SERVER_USER_CENTER_PROVIDER_KEY=$UserCenterProviderKey
CRAW_CHAT_SERVER_USER_CENTER_LOCAL_API_BASE_PATH=$UserCenterLocalApiBasePath
SDKWORK_USER_CENTER_MODE=$UserCenterMode
SDKWORK_USER_CENTER_PROVIDER_KEY=$UserCenterProviderKey
SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH=$UserCenterLocalApiBasePath
CRAW_CHAT_USER_CENTER_MODE=$UserCenterMode
CRAW_CHAT_USER_CENTER_PROVIDER_KEY=$UserCenterProviderKey
CRAW_CHAT_USER_CENTER_LOCAL_API_BASE_PATH=$UserCenterLocalApiBasePath
CRAW_CHAT_SERVER_USER_CENTER_AUTHORIZATION_HEADER_NAME=$UserCenterAuthorizationHeaderName
CRAW_CHAT_SERVER_USER_CENTER_ACCESS_TOKEN_HEADER_NAME=$UserCenterAccessTokenHeaderName
CRAW_CHAT_SERVER_USER_CENTER_REFRESH_TOKEN_HEADER_NAME=$UserCenterRefreshTokenHeaderName
CRAW_CHAT_SERVER_USER_CENTER_SESSION_HEADER_NAME=$UserCenterSessionHeaderName
CRAW_CHAT_SERVER_USER_CENTER_AUTHORIZATION_SCHEME=$UserCenterAuthorizationScheme
CRAW_CHAT_SERVER_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN=$UserCenterAllowAuthorizationFallbackToAccessToken
CRAW_CHAT_SERVER_USER_CENTER_APP_ID=$UserCenterAppId
CRAW_CHAT_SERVER_USER_CENTER_APP_API_BASE_URL=$UserCenterAppApiBaseUrl
CRAW_CHAT_SERVER_USER_CENTER_SECRET_ID=$UserCenterSecretId
CRAW_CHAT_SERVER_USER_CENTER_SHARED_SECRET=$UserCenterSharedSecret
CRAW_CHAT_SERVER_USER_CENTER_EXTERNAL_BASE_URL=$UserCenterExternalBaseUrl
CRAW_CHAT_SERVER_USER_CENTER_DATABASE_URL=$UserCenterDatabaseUrl
CRAW_CHAT_SERVER_USER_CENTER_SCHEMA_NAME=$UserCenterSchemaName
CRAW_CHAT_SERVER_USER_CENTER_SQLITE_PATH=$UserCenterSqlitePath
CRAW_CHAT_SERVER_USER_CENTER_TABLE_PREFIX=$UserCenterTablePrefix
CRAW_CHAT_SERVER_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS=$UserCenterHandshakeFreshnessWindowMs
SDKWORK_USER_CENTER_AUTHORIZATION_HEADER_NAME=$UserCenterAuthorizationHeaderName
SDKWORK_USER_CENTER_ACCESS_TOKEN_HEADER_NAME=$UserCenterAccessTokenHeaderName
SDKWORK_USER_CENTER_REFRESH_TOKEN_HEADER_NAME=$UserCenterRefreshTokenHeaderName
SDKWORK_USER_CENTER_SESSION_HEADER_NAME=$UserCenterSessionHeaderName
SDKWORK_USER_CENTER_AUTHORIZATION_SCHEME=$UserCenterAuthorizationScheme
SDKWORK_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN=$UserCenterAllowAuthorizationFallbackToAccessToken
SDKWORK_USER_CENTER_APP_ID=$UserCenterAppId
SDKWORK_USER_CENTER_APP_API_BASE_URL=$UserCenterAppApiBaseUrl
SDKWORK_USER_CENTER_SECRET_ID=$UserCenterSecretId
SDKWORK_USER_CENTER_SHARED_SECRET=$UserCenterSharedSecret
SDKWORK_USER_CENTER_EXTERNAL_BASE_URL=$UserCenterExternalBaseUrl
SDKWORK_USER_CENTER_DATABASE_URL=$UserCenterDatabaseUrl
SDKWORK_USER_CENTER_SCHEMA_NAME=$UserCenterSchemaName
SDKWORK_USER_CENTER_SQLITE_PATH=$UserCenterSqlitePath
SDKWORK_USER_CENTER_TABLE_PREFIX=$UserCenterTablePrefix
SDKWORK_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS=$UserCenterHandshakeFreshnessWindowMs
CRAW_CHAT_USER_CENTER_AUTHORIZATION_HEADER_NAME=$UserCenterAuthorizationHeaderName
CRAW_CHAT_USER_CENTER_ACCESS_TOKEN_HEADER_NAME=$UserCenterAccessTokenHeaderName
CRAW_CHAT_USER_CENTER_REFRESH_TOKEN_HEADER_NAME=$UserCenterRefreshTokenHeaderName
CRAW_CHAT_USER_CENTER_SESSION_HEADER_NAME=$UserCenterSessionHeaderName
CRAW_CHAT_USER_CENTER_AUTHORIZATION_SCHEME=$UserCenterAuthorizationScheme
CRAW_CHAT_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN=$UserCenterAllowAuthorizationFallbackToAccessToken
CRAW_CHAT_USER_CENTER_APP_ID=$UserCenterAppId
CRAW_CHAT_USER_CENTER_APP_API_BASE_URL=$UserCenterAppApiBaseUrl
CRAW_CHAT_USER_CENTER_SECRET_ID=$UserCenterSecretId
CRAW_CHAT_USER_CENTER_SHARED_SECRET=$UserCenterSharedSecret
CRAW_CHAT_USER_CENTER_EXTERNAL_BASE_URL=$UserCenterExternalBaseUrl
CRAW_CHAT_USER_CENTER_DATABASE_URL=$UserCenterDatabaseUrl
CRAW_CHAT_USER_CENTER_SCHEMA_NAME=$UserCenterSchemaName
CRAW_CHAT_USER_CENTER_SQLITE_PATH=$UserCenterSqlitePath
CRAW_CHAT_USER_CENTER_TABLE_PREFIX=$UserCenterTablePrefix
CRAW_CHAT_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS=$UserCenterHandshakeFreshnessWindowMs
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
