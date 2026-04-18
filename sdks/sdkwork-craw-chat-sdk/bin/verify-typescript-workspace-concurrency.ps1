param()

$ErrorActionPreference = "Stop"

function Resolve-WorkspaceDir {
  $ScriptDir = if ($PSScriptRoot) {
    $PSScriptRoot
  } elseif ($PSCommandPath) {
    Split-Path -Parent $PSCommandPath
  } else {
    throw "Unable to resolve script directory"
  }
  return (Resolve-Path (Join-Path $ScriptDir "..")).Path
}

function New-LogFilePath {
  param(
    [string]$Root,
    [string]$Name
  )

  return Join-Path $Root "${Name}.log"
}

function Read-LogTail {
  param(
    [string]$Path
  )

  if (-not (Test-Path $Path)) {
    return ""
  }

  return ((Get-Content $Path -Tail 40) -join [Environment]::NewLine)
}

function Test-LogContains {
  param(
    [string]$Path,
    [string]$Pattern
  )

  if (-not (Test-Path $Path)) {
    return $false
  }

  $Content = Get-Content $Path -Raw
  return $Content -match $Pattern
}

$WorkspaceDir = Resolve-WorkspaceDir
$VerifyScript = Join-Path $WorkspaceDir "bin\verify-typescript-workspace.mjs"
$RunId = "run-$PID-$([DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds())"
$LogRoot = Join-Path $WorkspaceDir ".sdkwork\tmp\verify-typescript-workspace-concurrency"
$LogDir = Join-Path $LogRoot $RunId
$Succeeded = $false

if (Test-Path $LogRoot) {
  Remove-Item -LiteralPath $LogRoot -Recurse -Force
}
New-Item -ItemType Directory -Force -Path $LogDir | Out-Null

$Run1Out = New-LogFilePath -Root $LogDir -Name "run1.stdout"
$Run1Err = New-LogFilePath -Root $LogDir -Name "run1.stderr"
$Run2Out = New-LogFilePath -Root $LogDir -Name "run2.stdout"
$Run2Err = New-LogFilePath -Root $LogDir -Name "run2.stderr"

try {
  $Process1 = Start-Process node `
    -ArgumentList @($VerifyScript) `
    -WorkingDirectory $WorkspaceDir `
    -PassThru `
    -WindowStyle Hidden `
    -RedirectStandardOutput $Run1Out `
    -RedirectStandardError $Run1Err

  $Process2 = Start-Process node `
    -ArgumentList @($VerifyScript) `
    -WorkingDirectory $WorkspaceDir `
    -PassThru `
    -WindowStyle Hidden `
    -RedirectStandardOutput $Run2Out `
    -RedirectStandardError $Run2Err

  $Process1Completed = $Process1.WaitForExit(600000)
  $Process2Completed = $Process2.WaitForExit(600000)

  $Failures = @()
  if (-not $Process1Completed) {
    $Failures += "Concurrent TypeScript workspace verification run 1 did not exit before timeout."
  }
  if (-not $Process2Completed) {
    $Failures += "Concurrent TypeScript workspace verification run 2 did not exit before timeout."
  }

  $Process1.Refresh()
  $Process2.Refresh()
  $Process1ExitCode = if ($Process1Completed -and $Process1.HasExited) { $Process1.ExitCode } else { $null }
  $Process2ExitCode = if ($Process2Completed -and $Process2.HasExited) { $Process2.ExitCode } else { $null }

  if (($null -ne $Process1ExitCode) -and ($Process1ExitCode -ne 0)) {
    $Failures += @(
      "Concurrent TypeScript workspace verification run 1 exited with code $Process1ExitCode.",
      (Read-LogTail -Path $Run1Out),
      (Read-LogTail -Path $Run1Err)
    )
  }
  if (($null -ne $Process2ExitCode) -and ($Process2ExitCode -ne 0)) {
    $Failures += @(
      "Concurrent TypeScript workspace verification run 2 exited with code $Process2ExitCode.",
      (Read-LogTail -Path $Run2Out),
      (Read-LogTail -Path $Run2Err)
    )
  }

  if ($Process1Completed -and -not (Test-LogContains -Path $Run1Out -Pattern 'typescript workspace verification passed\.')) {
    $Failures += @(
      "Concurrent TypeScript workspace verification run 1 did not report the success marker.",
      (Read-LogTail -Path $Run1Out),
      (Read-LogTail -Path $Run1Err)
    )
  }
  if ($Process2Completed -and -not (Test-LogContains -Path $Run2Out -Pattern 'typescript workspace verification passed\.')) {
    $Failures += @(
      "Concurrent TypeScript workspace verification run 2 did not report the success marker.",
      (Read-LogTail -Path $Run2Out),
      (Read-LogTail -Path $Run2Err)
    )
  }

  if ($Failures.Count -gt 0) {
    Write-Error ($Failures | Where-Object { -not [string]::IsNullOrWhiteSpace($_) } | Out-String)
  }

  $Succeeded = $true
  Write-Host "[sdkwork-craw-chat-sdk] TypeScript workspace concurrency verification passed."
} finally {
  if ($Succeeded -and (Test-Path $LogRoot)) {
    Remove-Item -LiteralPath $LogRoot -Recurse -Force
  }
}
