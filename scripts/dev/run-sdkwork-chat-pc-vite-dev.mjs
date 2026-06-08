#!/usr/bin/env node

import path from 'node:path';
import process from 'node:process';
import { fileURLToPath, pathToFileURL } from 'node:url';

import {
  DEFAULT_SDKWORK_CHAT_PC_DEV_HOST,
  SDKWORK_CHAT_PC_DEV_HOST_ENV,
  SDKWORK_CHAT_PC_DEV_PORT_ENV,
  resolveAvailableSdkworkChatPcDevPort,
  resolveSdkworkChatPcDevServer,
} from './run-sdkwork-chat-pc-dev.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const viteRunnerPath = path.join(scriptDir, 'run-vite-cli.mjs');

const configuredServer = resolveSdkworkChatPcDevServer({
  env: process.env,
  host: process.env[SDKWORK_CHAT_PC_DEV_HOST_ENV] ?? DEFAULT_SDKWORK_CHAT_PC_DEV_HOST,
});
const selectedPort = process.env[SDKWORK_CHAT_PC_DEV_PORT_ENV]
  ? configuredServer.port
  : await resolveAvailableSdkworkChatPcDevPort({
    env: process.env,
    host: configuredServer.host,
  });

process.env[SDKWORK_CHAT_PC_DEV_HOST_ENV] = process.env[SDKWORK_CHAT_PC_DEV_HOST_ENV]
  ?? configuredServer.host;
process.env[SDKWORK_CHAT_PC_DEV_PORT_ENV] = String(selectedPort);

process.argv = [
  process.argv[0],
  viteRunnerPath,
  '--host',
  process.env[SDKWORK_CHAT_PC_DEV_HOST_ENV],
  '--port',
  String(selectedPort),
  ...process.argv.slice(2),
];

await import(pathToFileURL(viteRunnerPath).href);
