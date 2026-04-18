param(
  [string[]]$Languages = @("typescript", "flutter"),
  [switch]$WithDart
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
$Languages = Normalize-LanguageList $Languages
$Args = @((Join-Path $ScriptDir "verify-sdk.mjs"))
foreach ($Language in $Languages) {
  $Args += @("--language", $Language)
}
if ($WithDart.IsPresent) {
  $Args += "--with-dart"
}

& node @Args
Assert-LastExitCode "verify-sdk"
