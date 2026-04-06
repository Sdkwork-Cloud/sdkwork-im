@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0inspect-runtime-local.ps1" %*
set "_exit_code=%ERRORLEVEL%"
endlocal & exit /b %_exit_code%
