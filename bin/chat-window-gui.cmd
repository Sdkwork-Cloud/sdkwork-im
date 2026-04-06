@echo off
setlocal
call "%~dp0_cmd-forward-powershell.cmd" "%~dp0chat-window-gui.ps1" %*
exit /b %ERRORLEVEL%
