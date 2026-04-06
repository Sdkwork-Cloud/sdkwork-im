@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0repair-runtime-local.ps1" %*
