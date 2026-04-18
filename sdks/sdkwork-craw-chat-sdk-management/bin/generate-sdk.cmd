@echo off
setlocal
set SCRIPT_DIR=%~dp0
set WORKSPACE_ROOT=%SCRIPT_DIR%..

node "%WORKSPACE_ROOT%\bin\materialize-management-authority.mjs"
if errorlevel 1 exit /b %errorlevel%
node "%WORKSPACE_ROOT%\bin\materialize-management-typescript-workspace.mjs"
if errorlevel 1 exit /b %errorlevel%
node "%WORKSPACE_ROOT%\bin\materialize-management-flutter-workspace.mjs"
if errorlevel 1 exit /b %errorlevel%
node "%WORKSPACE_ROOT%\bin\assemble-sdk.mjs"
if errorlevel 1 exit /b %errorlevel%
node "%WORKSPACE_ROOT%\bin\verify-sdk.mjs"
if errorlevel 1 exit /b %errorlevel%
node "%WORKSPACE_ROOT%\bin\verify-typescript-workspace.mjs"
