param(
  [string[]]$Languages = @("typescript", "flutter", "rust", "java", "csharp", "swift", "kotlin", "go", "python"),
  [string]$RequestedVersion,
  [string]$BaseUrl = "http://127.0.0.1:18090",
  [string]$ApiPrefix = "/api/v1",
  [string]$SchemaUrl,
  [switch]$KeepServiceRunning
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
    return @("typescript", "flutter", "rust", "java", "csharp", "swift", "kotlin", "go", "python")
  }

  return $Normalized
}

function Get-HealthUrlFromBaseUrl {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Value
  )

  $uri = [Uri]$Value
  return ([Uri]::new($uri, "/healthz")).AbsoluteUri
}

function Get-SchemaUrlFromBaseUrl {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Value,
    [string]$ExplicitSchemaUrl
  )

  if (-not [string]::IsNullOrWhiteSpace($ExplicitSchemaUrl)) {
    return $ExplicitSchemaUrl.Trim()
  }

  $uri = [Uri]$Value
  return ([Uri]::new($uri, "/openapi/craw-chat-app.openapi.yaml")).AbsoluteUri
}

function Get-BindAddressFromBaseUrl {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Value
  )

  $uri = [Uri]$Value
  $port = if ($uri.IsDefaultPort) {
    if ($uri.Scheme -eq "https") { 443 } else { 80 }
  }
  else {
    $uri.Port
  }

  return "$($uri.Host):$port"
}

function Test-LoopbackBaseUrl {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Value
  )

  $uri = [Uri]$Value
  return @("127.0.0.1", "localhost", "0.0.0.0", "::1", "::", "[::1]", "[::]") -contains $uri.Host
}

function Test-ServiceHealthy {
  param(
    [Parameter(Mandatory = $true)]
    [string]$HealthUrl
  )

  try {
    $response = Invoke-WebRequest -Uri $HealthUrl -Method Get -TimeoutSec 2 -UseBasicParsing
    return $response.StatusCode -eq 200
  }
  catch {
    return $false
  }
}

function Invoke-PowerShellFileStep {
  param(
    [Parameter(Mandatory = $true)]
    [string]$Path,
    [Parameter(Mandatory = $true)]
    [string]$Step,
    [hashtable]$Parameters = @{}
  )

  if (-not (Test-Path $Path)) {
    throw "$Step script not found: $Path"
  }

  $global:LASTEXITCODE = 0
  & $Path @Parameters

  if ($LASTEXITCODE -ne 0) {
    throw "$Step failed with exit code $LASTEXITCODE"
  }
}

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceDir = (Resolve-Path (Join-Path $ScriptDir "..")).Path
$AppRoot = (Resolve-Path (Join-Path $WorkspaceDir "..\..")).Path
$GeneratorRoot = (& node (Join-Path $ScriptDir "sdk-paths.mjs") --workspace $WorkspaceDir).Trim()
Assert-LastExitCode "resolve-generator-root"
$BaseSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.openapi.yaml"
$SdkgenSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.sdkgen.yaml"
$FlutterSdkgenSpec = Join-Path $WorkspaceDir "openapi\craw-chat-app.flutter.sdkgen.yaml"
$ResolveVersionScript = Join-Path $GeneratorRoot "bin\resolve-sdk-version.js"
$SdkGeneratorScript = Join-Path $GeneratorRoot "bin\sdkgen.js"
$FlutterWorkspaceVerifyScript = Join-Path $ScriptDir "verify-flutter-workspace.mjs"
$TypeScriptWorkspaceVerifyScript = Join-Path $ScriptDir "verify-typescript-workspace.mjs"
$RustWorkspaceVerifyScript = Join-Path $ScriptDir "verify-rust-workspace.mjs"
$JavaWorkspaceVerifyScript = Join-Path $ScriptDir "verify-java-workspace.mjs"
$CsharpWorkspaceVerifyScript = Join-Path $ScriptDir "verify-csharp-workspace.mjs"
$SwiftWorkspaceVerifyScript = Join-Path $ScriptDir "verify-swift-workspace.mjs"
$KotlinWorkspaceVerifyScript = Join-Path $ScriptDir "verify-kotlin-workspace.mjs"
$GoWorkspaceVerifyScript = Join-Path $ScriptDir "verify-go-workspace.mjs"
$PythonWorkspaceVerifyScript = Join-Path $ScriptDir "verify-python-workspace.mjs"
$TypeScriptGeneratedBuildDeterminismVerifyScript = Join-Path $ScriptDir "verify-typescript-generated-build-determinism.mjs"
$NormalizeGeneratedAuthSurfaceScript = Join-Path $ScriptDir "normalize-generated-auth-surface.mjs"
$PrepareGeneratedOutputScript = Join-Path $ScriptDir "prepare-generated-output.mjs"
$NormalizeTypeScriptGeneratedPackageManifestScript = Join-Path $ScriptDir "normalize-typescript-generated-package-manifest.mjs"
$RefreshLiveOpenApiSourceScript = Join-Path $ScriptDir "refresh-live-openapi-source.mjs"
$StartLocalScript = Join-Path $AppRoot "bin\start-local.ps1"
$StopLocalScript = Join-Path $AppRoot "bin\stop-local.ps1"
$SdkName = "sdkwork-craw-chat-sdk"
$SdkType = "backend"
$Languages = Normalize-LanguageList $Languages
$OfficialLanguages = @(
  "typescript",
  "flutter",
  "rust",
  "java",
  "csharp",
  "swift",
  "kotlin",
  "go",
  "python"
)
$HealthUrl = Get-HealthUrlFromBaseUrl $BaseUrl
$ResolvedSchemaUrl = Get-SchemaUrlFromBaseUrl -Value $BaseUrl -ExplicitSchemaUrl $SchemaUrl
$StartedLocalService = $false
$PrimaryFailure = $null

try {
  if (-not (Test-ServiceHealthy -HealthUrl $HealthUrl)) {
    if (-not (Test-LoopbackBaseUrl $BaseUrl)) {
      throw "SDK generation requires a healthy service at $HealthUrl. Automatic start is only supported for local base URLs."
    }
    Invoke-PowerShellFileStep `
      -Path $StartLocalScript `
      -Step "start-local" `
      -Parameters @{
        ProfileName = "local-minimal"
        BindAddress = Get-BindAddressFromBaseUrl $BaseUrl
      }
    $StartedLocalService = $true
  }

  & node $RefreshLiveOpenApiSourceScript `
    --schema-url $ResolvedSchemaUrl `
    --output $BaseSpec | Out-Null
  Assert-LastExitCode "refresh-live-openapi-source"

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
      VerifyScript = $TypeScriptWorkspaceVerifyScript
    }
    flutter = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-flutter\generated\server-openapi"
      PackageName = "backend_sdk"
      Input = $PreparedFlutterInput
      VerifyScript = $FlutterWorkspaceVerifyScript
    }
    rust = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-rust\generated\server-openapi"
      PackageName = "sdkwork-craw-chat-backend-sdk"
      Input = $PreparedInput
      VerifyScript = $RustWorkspaceVerifyScript
    }
    java = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-java\generated\server-openapi"
      PackageName = "com.sdkwork:craw-chat-backend-sdk"
      Input = $PreparedInput
      VerifyScript = $JavaWorkspaceVerifyScript
    }
    csharp = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-csharp\generated\server-openapi"
      PackageName = "Sdkwork.CrawChat.BackendSdk"
      Namespace = "Sdkwork.CrawChat.BackendSdk"
      Input = $PreparedInput
      VerifyScript = $CsharpWorkspaceVerifyScript
    }
    swift = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-swift\generated\server-openapi"
      PackageName = "CrawChatBackendSdk"
      Input = $PreparedInput
      VerifyScript = $SwiftWorkspaceVerifyScript
    }
    kotlin = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-kotlin\generated\server-openapi"
      PackageName = "com.sdkwork:craw-chat-backend-sdk"
      Input = $PreparedInput
      VerifyScript = $KotlinWorkspaceVerifyScript
    }
    go = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-go\generated\server-openapi"
      PackageName = "github.com/sdkwork/craw-chat-backend-sdk"
      Input = $PreparedInput
      VerifyScript = $GoWorkspaceVerifyScript
    }
    python = @{
      OutputDir = Join-Path $WorkspaceDir "sdkwork-craw-chat-sdk-python\generated\server-openapi"
      PackageName = "sdkwork-craw-chat-backend-sdk"
      Input = $PreparedInput
      VerifyScript = $PythonWorkspaceVerifyScript
    }
  }

  foreach ($Language in $Languages) {
    $NormalizedLanguage = $Language.Trim().ToLowerInvariant()
    if ([string]::IsNullOrWhiteSpace($NormalizedLanguage)) {
      continue
    }

    if (-not ($OfficialLanguages -contains $NormalizedLanguage)) {
      throw "Unsupported language: $Language"
    }

    if (-not $LanguageConfigurations.ContainsKey($NormalizedLanguage)) {
      throw "Generation mapping not implemented yet for official language: $Language"
    }

    $Configuration = $LanguageConfigurations[$NormalizedLanguage]
    & node $PrepareGeneratedOutputScript --language $NormalizedLanguage
    Assert-LastExitCode "prepare-generated-output:$NormalizedLanguage"
    New-Item -ItemType Directory -Force -Path $Configuration.OutputDir | Out-Null

    $GeneratorArgs = @(
      $SdkGeneratorScript,
      "generate",
      "--input", $Configuration.Input,
      "--output", $Configuration.OutputDir,
      "--name", $SdkName,
      "--type", $SdkType,
      "--language", $NormalizedLanguage,
      "--base-url", $BaseUrl,
      "--api-prefix", $ApiPrefix,
      "--fixed-sdk-version", $ResolvedSdkVersion,
      "--sdk-root", $WorkspaceDir,
      "--sdk-name", $SdkName,
      "--package-name", $Configuration.PackageName
    )
    if ($Configuration.ContainsKey("Namespace") -and -not [string]::IsNullOrWhiteSpace($Configuration.Namespace)) {
      $GeneratorArgs += @("--namespace", $Configuration.Namespace)
    }

    & node @GeneratorArgs
    Assert-LastExitCode "sdkgen:$NormalizedLanguage"

    & node $NormalizeGeneratedAuthSurfaceScript --language $NormalizedLanguage
    Assert-LastExitCode "normalize-generated-auth-surface:$NormalizedLanguage"

    if ($NormalizedLanguage -eq "typescript") {
      & node $NormalizeTypeScriptGeneratedPackageManifestScript
      Assert-LastExitCode "normalize:typescript-generated-package-manifest"
    }

    if ($Configuration.ContainsKey("VerifyScript") -and -not [string]::IsNullOrWhiteSpace($Configuration.VerifyScript)) {
      & node $Configuration.VerifyScript
      Assert-LastExitCode "verify:$NormalizedLanguage-workspace"
    }

    if ($NormalizedLanguage -eq "typescript") {
      & node $TypeScriptGeneratedBuildDeterminismVerifyScript
      Assert-LastExitCode "verify:typescript-generated-build-determinism"
    }
  }

  & node (Join-Path $ScriptDir "assemble-sdk.mjs")
  Assert-LastExitCode "assemble-sdk"
}
catch {
  $PrimaryFailure = $_
  throw
}
finally {
  if ($StartedLocalService -and -not $KeepServiceRunning) {
    try {
      Invoke-PowerShellFileStep `
        -Path $StopLocalScript `
        -Step "stop-local" `
        -Parameters @{
          ProfileName = "local-minimal"
        }
    }
    catch {
      if ($null -ne $PrimaryFailure) {
        Write-Warning "stop-local cleanup failed after an earlier generation failure: $($_.Exception.Message)"
      }
      else {
        throw
      }
    }
  }
}
