@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0archive-runtime-backup-local.ps1" %*
set "_exit_code=%ERRORLEVEL%"
endlocal & exit /b %_exit_code%
