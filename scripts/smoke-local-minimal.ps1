$ErrorActionPreference = 'Stop'

$root = Split-Path -Parent $PSScriptRoot
Set-Location $root

Write-Host "Building local-minimal-node..."
cargo build -p local-minimal-node --offline | Out-Host

$exe = Join-Path $root "target\debug\local-minimal-node.exe"
if (-not (Test-Path $exe)) {
    throw "Binary not found: $exe"
}

Write-Host "Starting local-minimal-node for smoke check..."
$process = Start-Process -FilePath $exe -PassThru -WindowStyle Hidden

try {
    Start-Sleep -Seconds 2
    $response = Invoke-WebRequest -UseBasicParsing "http://127.0.0.1:18090/healthz"
    if ($response.StatusCode -ne 200) {
        throw "Unexpected status code: $($response.StatusCode)"
    }
    Write-Host "Smoke check passed."
}
finally {
    if ($null -ne $process -and -not $process.HasExited) {
        Stop-Process -Id $process.Id -Force
    }
}
