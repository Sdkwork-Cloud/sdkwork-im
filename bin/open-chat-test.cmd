@echo off
setlocal
call powershell -NoProfile -ExecutionPolicy Bypass -File "%~dp0open-chat-test.ps1" %*
exit /b %ERRORLEVEL%
