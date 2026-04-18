#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadYamlFromGenerator } from './generator-runtime.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  try {
    return await loadYamlFromGenerator(workspaceRoot);
  } catch (error) {
    fail(error instanceof Error ? error.message : String(error));
  }
}

function readYaml(filePath, yaml) {
  return yaml.load(readFileSync(filePath, 'utf8'));
}

function collectFailures(document, label) {
  const failures = [];
  const discovery = document?.['x-sdkwork-sdk-surface'];
  if (!discovery || typeof discovery !== 'object') {
    failures.push(`${label} must define x-sdkwork-sdk-surface.`);
    return failures;
  }

  if (discovery.sdkTarget !== 'crawChatAppSdk') {
    failures.push(`${label} x-sdkwork-sdk-surface.sdkTarget must equal crawChatAppSdk.`);
  }
  if (JSON.stringify(discovery.generatedProtocols || []) !== JSON.stringify(['http'])) {
    failures.push(`${label} x-sdkwork-sdk-surface.generatedProtocols must equal [http].`);
  }
  const manualTransports = Array.isArray(discovery.manualTransports) ? discovery.manualTransports : [];
  if (
    !manualTransports.some(
      (entry) =>
        entry?.path === '/api/v1/realtime/ws'
        && entry?.method === 'get'
        && entry?.protocol === 'websocket'
        && entry?.serviceId === 'session-gateway'
        && entry?.operationGroup === 'realtime',
    )
  ) {
    failures.push(
      `${label} x-sdkwork-sdk-surface.manualTransports must include the session-gateway realtime websocket binding.`,
    );
  }

  const operationBindings = Array.isArray(discovery.operationBindings)
    ? discovery.operationBindings
    : [];
  if (
    !operationBindings.some(
      (binding) =>
        binding?.operationId === 'registerDevice'
        && binding?.path === '/api/v1/devices/register'
        && binding?.method === 'post'
        && binding?.serviceId === 'projection-service'
        && binding?.operationGroup === 'devices',
    )
  ) {
    failures.push(
      `${label} x-sdkwork-sdk-surface.operationBindings must include projection-service device registration binding.`,
    );
  }

  const resumeOperation = document?.paths?.['/api/v1/sessions/resume']?.post;
  if (resumeOperation?.['x-sdkwork-service'] !== 'session-gateway') {
    failures.push(`${label} /api/v1/sessions/resume post must declare x-sdkwork-service=session-gateway.`);
  }
  if (resumeOperation?.['x-sdkwork-operation-group'] !== 'sessions') {
    failures.push(`${label} /api/v1/sessions/resume post must declare x-sdkwork-operation-group=sessions.`);
  }
  if (resumeOperation?.['x-sdkwork-sdk-target'] !== 'crawChatAppSdk') {
    failures.push(`${label} /api/v1/sessions/resume post must declare x-sdkwork-sdk-target=crawChatAppSdk.`);
  }
  if (resumeOperation?.['x-sdkwork-protocol'] !== 'http') {
    failures.push(`${label} /api/v1/sessions/resume post must declare x-sdkwork-protocol=http.`);
  }

  const deviceSyncFeedOperation = document?.paths?.['/api/v1/devices/{deviceId}/sync-feed']?.get;
  if (deviceSyncFeedOperation?.['x-sdkwork-service'] !== 'projection-service') {
    failures.push(
      `${label} /api/v1/devices/{deviceId}/sync-feed get must declare x-sdkwork-service=projection-service.`,
    );
  }
  if (deviceSyncFeedOperation?.['x-sdkwork-operation-group'] !== 'devices') {
    failures.push(
      `${label} /api/v1/devices/{deviceId}/sync-feed get must declare x-sdkwork-operation-group=devices.`,
    );
  }

  if (document?.paths?.['/api/v1/realtime/ws']) {
    failures.push(`${label} must not keep /api/v1/realtime/ws in the derived sdkgen surface.`);
  }

  return failures;
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadYaml();
const defaultDerived = readYaml(path.join(workspaceRoot, 'openapi', 'craw-chat-app.sdkgen.yaml'), yaml);
const flutterDerived = readYaml(
  path.join(workspaceRoot, 'openapi', 'craw-chat-app.flutter.sdkgen.yaml'),
  yaml,
);

const failures = [
  ...collectFailures(defaultDerived, 'craw-chat-app.sdkgen.yaml'),
  ...collectFailures(flutterDerived, 'craw-chat-app.flutter.sdkgen.yaml'),
];

if (failures.length > 0) {
  fail(failures.join('\n'));
}
