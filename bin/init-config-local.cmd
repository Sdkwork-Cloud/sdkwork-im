@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0init-config-local.ps1" %*
exit /b %ERRORLEVEL%
