@echo off
setlocal
call powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0chat-window-gui.ps1" %*
exit /b %ERRORLEVEL%
