$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceDir = (Resolve-Path (Join-Path $scriptDir '..\..')).Path

node (Join-Path $workspaceDir 'bin\verify-sdk.mjs')
exit $LASTEXITCODE
