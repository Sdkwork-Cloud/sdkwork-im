param(
  [string]$RequestedVersion,
  [string]$BaseUrl = "http://127.0.0.1:18081"
)

$ErrorActionPreference = "Stop"

function Resolve-GeneratorRoot {
  param(
    [string]$WorkspaceRoot
  )

  if ($env:SDKWORK_GENERATOR_ROOT) {
    $ExplicitRoot = (Resolve-Path $env:SDKWORK_GENERATOR_ROOT -ErrorAction SilentlyContinue)
    if ($ExplicitRoot) {
      return $ExplicitRoot.Path
    }
  }

  $Current = $WorkspaceRoot
  while ($true) {
    $Candidate = Join-Path $Current "sdk\\sdkwork-sdk-generator"
    if (Test-Path $Candidate) {
      return (Resolve-Path $Candidate).Path
    }

    $Parent = Split-Path -Parent $Current
    if ($Parent -eq $Current) {
      break
    }
    $Current = $Parent
  }

  throw "Unable to locate sdkwork-sdk-generator. Set SDKWORK_GENERATOR_ROOT to an explicit path."
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceRoot = (Resolve-Path (Join-Path $scriptDir "..")).Path
$generatorRoot = Resolve-GeneratorRoot $workspaceRoot
$sdkGeneratorScript = Join-Path $generatorRoot "bin\\sdkgen.js"
$tmpDir = Join-Path $workspaceRoot ".tmp"
$tmpExportPath = Join-Path $tmpDir "control-plane.openapi.json"
$authorityPath = Join-Path $workspaceRoot "openapi\\craw-chat-control-plane.openapi.json"
$derivedPath = Join-Path $workspaceRoot "openapi\\craw-chat-control-plane.sdkgen.json"
$generatedOutputDir = Join-Path $workspaceRoot "sdkwork-craw-chat-sdk-admin-typescript\\generated\\server-openapi"

New-Item -ItemType Directory -Force -Path $tmpDir | Out-Null
cargo run -q -p control-plane-api --bin export-openapi | Out-File -FilePath $tmpExportPath -Encoding utf8
node (Join-Path $scriptDir "refresh-openapi-source.mjs") --source-file $tmpExportPath
node (Join-Path $scriptDir "prepare-openapi-source.mjs") --base $authorityPath --derived $derivedPath

$AuthorityVersion = (
  Get-Content $authorityPath -Raw | ConvertFrom-Json
).info.version
$ResolvedVersion = if ([string]::IsNullOrWhiteSpace($RequestedVersion)) {
  $AuthorityVersion
} else {
  $RequestedVersion.Trim()
}

node $sdkGeneratorScript generate `
  --input $derivedPath `
  --output $generatedOutputDir `
  --name "sdkwork-craw-chat-sdk-admin" `
  --type "backend" `
  --language "typescript" `
  --base-url $BaseUrl `
  --fixed-sdk-version $ResolvedVersion `
  --sdk-root $workspaceRoot `
  --sdk-name "sdkwork-craw-chat-sdk-admin" `
  --package-name "@sdkwork/craw-chat-admin-backend-sdk"

node (Join-Path $scriptDir "normalize-generated-transport-package.mjs")
node (Join-Path $scriptDir "materialize-admin-flutter-workspace.mjs")
node (Join-Path $scriptDir "assemble-sdk.mjs")
node (Join-Path $scriptDir "verify-sdk.mjs")
