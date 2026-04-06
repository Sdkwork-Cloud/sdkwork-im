@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0stop-local.ps1" %*
exit /b %ERRORLEVEL%
