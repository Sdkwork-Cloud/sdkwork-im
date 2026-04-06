$ErrorActionPreference = 'Stop'

$release = $false
$cliArgs = @()

foreach ($argument in $args) {
    if ($argument -eq "--") {
        continue
    }

    if ($argument -eq "-Release" -or $argument -eq "/release" -or $argument -eq "--release") {
        $release = $true
        continue
    }

    $cliArgs += $argument
}

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

$profileDir = if ($release) { "release" } else { "debug" }
$exePath = Join-Path $root "target\$profileDir\craw-chat-cli.exe"

if (-not (Test-Path $exePath)) {
    $cargoArgs = @("build", "-p", "craw-chat-cli")
    if ($release) {
        $cargoArgs += "--release"
    }

    & cargo @cargoArgs
    if ($LASTEXITCODE -ne 0) {
        exit $LASTEXITCODE
    }
}

if (-not (Test-Path $exePath)) {
    throw "craw-chat-cli binary was not found after build: $exePath"
}

& $exePath @cliArgs
exit $LASTEXITCODE
