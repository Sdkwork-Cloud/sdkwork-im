param(
  [string[]]$Languages = @("typescript", "flutter")
)

$ErrorActionPreference = "Stop"

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
$Args = @((Join-Path $ScriptDir "assemble-sdk.mjs"))

foreach ($Language in $Languages) {
  $Args += @("--language", $Language)
}

& node @Args
if ($LASTEXITCODE -ne 0) {
  throw "assemble-sdk failed with exit code $LASTEXITCODE"
}
