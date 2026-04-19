@echo off
setlocal

if defined npm_node_execpath (
  "%npm_node_execpath%" "%~dp0build-typescript-generated-package.mjs" %*
  exit /b %errorlevel%
)

if defined NODE (
  "%NODE%" "%~dp0build-typescript-generated-package.mjs" %*
  exit /b %errorlevel%
)

node "%~dp0build-typescript-generated-package.mjs" %*
