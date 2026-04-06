@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0open-chat-test.ps1" %*
exit /b %ERRORLEVEL%
