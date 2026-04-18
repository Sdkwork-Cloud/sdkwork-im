$ErrorActionPreference = 'Stop'

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$workspaceRoot = Resolve-Path (Join-Path $scriptDir '..')

node (Join-Path $workspaceRoot 'bin\\materialize-management-authority.mjs')
node (Join-Path $workspaceRoot 'bin\\materialize-management-typescript-workspace.mjs')
node (Join-Path $workspaceRoot 'bin\\materialize-management-flutter-workspace.mjs')
node (Join-Path $workspaceRoot 'bin\\assemble-sdk.mjs')
node (Join-Path $workspaceRoot 'bin\\verify-sdk.mjs')
node (Join-Path $workspaceRoot 'bin\\verify-typescript-workspace.mjs')
