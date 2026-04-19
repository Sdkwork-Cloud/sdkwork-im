$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

if ($env:npm_node_execpath) {
  & $env:npm_node_execpath (Join-Path $scriptDir 'build-typescript-generated-package.mjs') @args
  exit $LASTEXITCODE
}

if ($env:NODE) {
  & $env:NODE (Join-Path $scriptDir 'build-typescript-generated-package.mjs') @args
  exit $LASTEXITCODE
}

node (Join-Path $scriptDir 'build-typescript-generated-package.mjs') @args
exit $LASTEXITCODE
