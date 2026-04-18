param(
    [string]$InstanceName = "default",
    [string]$InstallRoot = ([System.IO.Path]::Combine([Environment]::GetFolderPath("ProgramFiles"), "CrawChat")),
    [string]$ConfigDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "config")),
    [string]$DataDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "data")),
    [string]$LogDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "logs")),
    [string]$RunDir = ([System.IO.Path]::Combine([Environment]::GetFolderPath("CommonApplicationData"), "CrawChat", "default", "run")),
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
    Write-Host "Usage: powershell -ExecutionPolicy Bypass -File bin/install-server.ps1 [-InstanceName <name>] [-InstallRoot <path>] [-ConfigDir <path>] [-DataDir <path>] [-LogDir <path>] [-RunDir <path>] [-NonInteractive] [-Force]"
    Write-Host "Usage: cmd /c .\bin\install-server.cmd [--instance <name>] [--install-root <path>] [--config-dir <path>] [--data-dir <path>] [--log-dir <path>] [--run-dir <path>] [--non-interactive] [--force]"
    Write-Host "Create the craw-chat-server install/config/data/log/run directory skeleton and stage canonical payload examples."
    exit 0
}

$root = Split-Path -Parent $PSScriptRoot
$templateRoot = Join-Path $root "deployments\templates"
$storageDir = Join-Path $ConfigDir "storage"
$secretsDir = Join-Path $ConfigDir "secrets"
$installMetadataPath = Join-Path $ConfigDir "install.json"

foreach ($path in @($InstallRoot, $ConfigDir, $DataDir, $LogDir, $RunDir, $storageDir, $secretsDir)) {
    if (-not (Test-Path $path)) {
        New-Item -ItemType Directory -Path $path -Force | Out-Null
    }
}

$copies = @(
    @{ Source = (Join-Path $templateRoot "server.yaml.example"); Destination = (Join-Path $ConfigDir "server.yaml.example") },
    @{ Source = (Join-Path $templateRoot "server.env.example"); Destination = (Join-Path $ConfigDir "server.env.example") },
    @{ Source = (Join-Path $templateRoot "postgresql.yaml.example"); Destination = (Join-Path $storageDir "postgresql.yaml.example") }
)

foreach ($copy in $copies) {
    if ((-not (Test-Path $copy.Destination)) -or $Force) {
        Copy-Item -LiteralPath $copy.Source -Destination $copy.Destination -Force
    }
}

$installMetadata = [ordered]@{
    product = "craw-chat-server"
    instance = $InstanceName
    installRoot = $InstallRoot
    configDir = $ConfigDir
    dataDir = $DataDir
    logDir = $LogDir
    runDir = $RunDir
    nonInteractive = [bool]$NonInteractive
}

$installMetadata | ConvertTo-Json -Depth 4 | Set-Content -Path $installMetadataPath -Encoding utf8

Write-Host "Prepared craw-chat-server directories for instance '$InstanceName'."
Write-Host "ConfigDir: $ConfigDir"
Write-Host "DataDir: $DataDir"
Write-Host "LogDir: $LogDir"
Write-Host "RunDir: $RunDir"
