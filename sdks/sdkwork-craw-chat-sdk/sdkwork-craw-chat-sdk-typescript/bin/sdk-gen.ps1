param(
  [string]$RequestedVersion,
  [string]$BaseUrl = "http://127.0.0.1:18090",
  [string]$ApiPrefix = "/api/v1"
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

    if (Test-Path (Join-Path $Resolved "bin\generate-sdk.ps1")) {
      return $Resolved
    }
  }

  throw "Unable to resolve root SDK workspace directory"
}

$WorkspaceDir = Resolve-RootWorkspaceDir
$ForwardedArgs = @{
  Languages = @("typescript")
  BaseUrl = $BaseUrl
  ApiPrefix = $ApiPrefix
}
if ($PSBoundParameters.ContainsKey("RequestedVersion")) {
  $ForwardedArgs["RequestedVersion"] = $RequestedVersion
}
& (Join-Path $WorkspaceDir "bin\generate-sdk.ps1") @ForwardedArgs
exit $LASTEXITCODE
