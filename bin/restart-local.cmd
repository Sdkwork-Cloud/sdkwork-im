@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0restart-local.ps1" %*
exit /b %ERRORLEVEL%
