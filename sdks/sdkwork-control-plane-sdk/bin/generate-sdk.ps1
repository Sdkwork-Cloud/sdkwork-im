param(
  [string[]]$Languages = @("typescript", "flutter")
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
        $Normalized += $Trimmed.ToLowerInvariant()
      }
    }
  }

  if ($Normalized.Count -eq 0) {
    return @("typescript", "flutter")
  }

  return $Normalized
}

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$WorkspaceDir = (Resolve-Path (Join-Path $ScriptDir "..")).Path
$AuthoritySpec = Join-Path $WorkspaceDir "openapi\control-plane.openapi.yaml"
$DerivedSpec = Join-Path $WorkspaceDir "openapi\control-plane.sdkgen.yaml"
$Languages = Normalize-LanguageList $Languages

& node (Join-Path $ScriptDir "fetch-openapi-source.mjs") --authority $AuthoritySpec
Assert-LastExitCode "fetch-openapi-source"

& node (Join-Path $ScriptDir "prepare-openapi-source.mjs") --base $AuthoritySpec --derived $DerivedSpec
Assert-LastExitCode "prepare-openapi-source"

$VerifyArgs = @((Join-Path $ScriptDir "verify-sdk.mjs"))
foreach ($Language in $Languages) {
  $VerifyArgs += @("--language", $Language)
}
& node @VerifyArgs
Assert-LastExitCode "verify-sdk"
