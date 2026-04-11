@echo off
setlocal
call powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0chat-cli-local.ps1" -- %*
exit /b %ERRORLEVEL%
