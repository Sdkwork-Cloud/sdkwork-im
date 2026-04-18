param(
  [string]$RequestedVersion,
  [string]$BaseUrl = 'http://127.0.0.1:18081'
)

$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir '..\..')).Path
$command = Join-Path $workspaceDir 'bin\generate-sdk.ps1'

if ($PSBoundParameters.ContainsKey('RequestedVersion')) {
  & $command -RequestedVersion $RequestedVersion -BaseUrl $BaseUrl
} else {
  & $command -BaseUrl $BaseUrl
}

exit $LASTEXITCODE
