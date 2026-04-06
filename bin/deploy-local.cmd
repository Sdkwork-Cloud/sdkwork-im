@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0deploy-local.ps1" %*
exit /b %ERRORLEVEL%
