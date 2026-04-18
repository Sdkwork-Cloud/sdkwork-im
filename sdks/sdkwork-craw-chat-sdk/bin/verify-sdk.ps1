param(
  [string[]]$Languages = @("typescript", "flutter", "rust", "java", "csharp", "swift", "kotlin", "go", "python"),
  [switch]$WithDart
)

$ErrorActionPreference = "Stop"
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

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$Languages = Normalize-LanguageList $Languages
$UnknownLanguages = @($Languages | Where-Object { $OfficialLanguages -notcontains $_.Trim().ToLowerInvariant() })
if ($UnknownLanguages.Count -gt 0) {
  throw "Unsupported language: $($UnknownLanguages -join ', ')"
}
$Args = @((Join-Path $ScriptDir "verify-sdk.mjs"))

foreach ($Language in $Languages) {
  $Args += @("--language", $Language)
}

if ($WithDart.IsPresent) {
  $Args += "--with-dart"
}

& node @Args
if ($LASTEXITCODE -ne 0) {
  throw "verify-sdk failed with exit code $LASTEXITCODE"
}
