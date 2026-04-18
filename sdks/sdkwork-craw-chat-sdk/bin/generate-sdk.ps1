param(
  [string[]]$Languages = @("typescript", "flutter", "rust"),
  [string]$RequestedVersion,
  [string]$BaseUrl = "http://127.0.0.1:18090",
  [string]$ApiPrefix = "/api/v1"
)

$ErrorActionPreference = "Stop"

function Assert-LastExitCode {
  param(
    [string]$Step
  )

  if ($LASTEXITCODE -ne 0) {
    throw "$Step failed with exit code $LASTEXITCODE"
  }
}

function Normalize-LanguageList {
  param(
    [string[]]$Values
  )

  $Normalized = @()
  foreach ($Value in $Values) {
    if ([string]::IsNullOrWhiteSpace($Value)) {
      continue
    }

    foreach ($Segment in ($Value -split ",")) {
      $Trimmed = $Segment.Trim()
      if (-not [string]::IsNullOrWhiteSpace($Trimmed)) {
        $Normalized += $Trimmed
      }
    }
  }

  if ($Normalized.Count -eq 0) {
    return @("typescript", "flutter", "rust")
  }

  return $Normalized
}

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceDir = (Resolve-Path (Join-Path $ScriptDir "..")).Path
$GeneratorRoot = (& node (Join-Path $ScriptDir "sdk-paths.mjs") --workspace $WorkspaceDir).Trim()
Assert-LastExitCode "resolve-generator-root"
$BaseSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.openapi.yaml"
$SdkgenSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.sdkgen.yaml"
$FlutterSdkgenSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.flutter.sdkgen.yaml"
$ResolveVersionScript = Join-Path $GeneratorRoot "bin\resolve-sdk-version.js"
$SdkGeneratorScript = Join-Path $GeneratorRoot "bin\sdkgen.js"
$FlutterWorkspaceVerifyScript = Join-Path $ScriptDir "verify-flutter-workspace.mjs"
$TypeScriptWorkspaceVerifyScript = Join-Path $ScriptDir "verify-typescript-workspace.mjs"
$TypeScriptGeneratedBuildDeterminismVerifyScript = Join-Path $ScriptDir "verify-typescript-generated-build-determinism.mjs"
$NormalizeGeneratedAuthSurfaceScript = Join-Path $ScriptDir "normalize-generated-auth-surface.mjs"
$SdkName = "sdkwork-craw-chat-sdk"
$SdkType = "backend"
$Languages = Normalize-LanguageList $Languages

$PreparedInput = (& node (Join-Path $ScriptDir "prepare-openapi-source.mjs") `
  --base $BaseSpec `
  --derived $SdkgenSpec `
  --prefer-derived).Trim()
Assert-LastExitCode "prepare-openapi-source"

$PreparedFlutterInput = (& node (Join-Path $ScriptDir "prepare-openapi-source.mjs") `
  --base $BaseSpec `
  --derived $FlutterSdkgenSpec `
  --prefer-derived `
  --target-language flutter).Trim()
Assert-LastExitCode "prepare-openapi-source:flutter"

$VersionMatch = Select-String -Path $BaseSpec -Pattern '^\s{2}version:\s*["'']?([^"''\r\n]+)["'']?' | Select-Object -First 1
if (-not $VersionMatch) {
  throw "Unable to resolve authority OpenAPI version from $BaseSpec"
}
$AuthorityVersion = $VersionMatch.Matches[0].Groups[1].Value.Trim()
$RequestedSdkVersion = if ([string]::IsNullOrWhiteSpace($RequestedVersion)) {
  $AuthorityVersion
} else {
  $RequestedVersion.Trim()
}

$ResolvedSdkVersion = (& node $ResolveVersionScript `
  --sdk-root $WorkspaceDir `
  --sdk-name $SdkName `
  --sdk-type $SdkType `
  --requested-version $RequestedSdkVersion `
  --package-name "@sdkwork/craw-chat-backend-sdk" `
  --no-sync-published-version).Trim()
Assert-LastExitCode "resolve-sdk-version"

if ([string]::IsNullOrWhiteSpace($ResolvedSdkVersion)) {
  throw "Failed to resolve SDK version"
}

$LanguageConfigurations = @{
  typescript = @{
    OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-typescript\generated\server-openapi"
    PackageName = "@sdkwork/craw-chat-backend-sdk"
    Input = $PreparedInput
  }
  flutter = @{
    OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-flutter\generated\server-openapi"
    PackageName = "backend_sdk"
    Input = $PreparedFlutterInput
  }
  rust = @{
    OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-rust\generated\server-openapi"
    PackageName = "sdkwork-craw-chat-backend-sdk"
    Input = $PreparedInput
  }
}

foreach ($Language in $Languages) {
  $NormalizedLanguage = $Language.Trim().ToLowerInvariant()
  if ([string]::IsNullOrWhiteSpace($NormalizedLanguage)) {
    continue
  }

  if (-not $LanguageConfigurations.ContainsKey($NormalizedLanguage)) {
    throw "Unsupported language: $Language"
  }

  $Configuration = $LanguageConfigurations[$NormalizedLanguage]
  New-Item -ItemType Directory -Force -Path $Configuration.OutputDir | Out-Null

  & node $SdkGeneratorScript generate `
    --input $Configuration.Input `
    --output $Configuration.OutputDir `
    --name $SdkName `
    --type $SdkType `
    --language $NormalizedLanguage `
    --base-url $BaseUrl `
    --api-prefix $ApiPrefix `
    --fixed-sdk-version $ResolvedSdkVersion `
    --sdk-root $WorkspaceDir `
    --sdk-name $SdkName `
    --package-name $Configuration.PackageName
  Assert-LastExitCode "sdkgen:$NormalizedLanguage"

  & node $NormalizeGeneratedAuthSurfaceScript --language $NormalizedLanguage
  Assert-LastExitCode "normalize-generated-auth-surface:$NormalizedLanguage"

  if ($NormalizedLanguage -eq "flutter") {
    & node $FlutterWorkspaceVerifyScript
    Assert-LastExitCode "verify:flutter-workspace"
  }

  if ($NormalizedLanguage -eq "typescript") {
    & node $TypeScriptWorkspaceVerifyScript
    Assert-LastExitCode "verify:typescript-workspace"

    & node $TypeScriptGeneratedBuildDeterminismVerifyScript
    Assert-LastExitCode "verify:typescript-generated-build-determinism"
  }
}

$AssembleArgs = @((Join-Path $ScriptDir "assemble-sdk.mjs"))
foreach ($Language in $Languages) {
  $NormalizedLanguage = $Language.Trim().ToLowerInvariant()
  if ([string]::IsNullOrWhiteSpace($NormalizedLanguage)) {
    continue
  }
  $AssembleArgs += @("--language", $NormalizedLanguage)
}
& node @AssembleArgs
Assert-LastExitCode "assemble-sdk"
