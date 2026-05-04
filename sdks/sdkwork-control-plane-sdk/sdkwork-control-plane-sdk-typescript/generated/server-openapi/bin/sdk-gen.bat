@echo off
setlocal
node "%~dp0sdk-gen-core.mjs" %*
exit /b %errorlevel%
