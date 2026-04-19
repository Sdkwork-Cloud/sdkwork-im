param(
  [switch]$WithDart
)

$ErrorActionPreference = "Stop"

function Resolve-RootWorkspaceDir {
  $Candidates = @(
    (Join-Path (Get-Location).Path ".."),
    (Join-Path (Get-Location).Path "..\..")
  )

  if ($PSCommandPath) {
    $Candidates += (Join-Path (Split-Path -Parent $PSCommandPath) "..\..")
  }

  foreach ($Candidate in ($Candidates | Select-Object -Unique)) {
    try {
      $Resolved = (Resolve-Path $Candidate -ErrorAction Stop).Path
    } catch {
      continue
    }

    if (Test-Path (Join-Path $Resolved "bin\verify-sdk.ps1")) {
      return $Resolved
    }
  }

  throw "Unable to resolve root SDK workspace directory"
}

$WorkspaceDir = Resolve-RootWorkspaceDir
$ForwardedArgs = @{
  Languages = @("flutter")
}
if ($WithDart.IsPresent) {
  $ForwardedArgs["WithDart"] = $true
}
& (Join-Path $WorkspaceDir "bin\verify-sdk.ps1") @ForwardedArgs
exit $LASTEXITCODE
