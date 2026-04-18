#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { assembleSdk } from './assemble-sdk.mjs';
import { runFlutterWorkspaceVerification, verifyFlutterWorkspace } from './verify-flutter-workspace.mjs';
import { verifySdkAutomation } from './verify-sdk-automation.mjs';
import {
  runTypeScriptWorkspaceVerification,
  verifyTypeScriptWorkspace,
} from './verify-typescript-workspace.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk-admin] ${message}`);
  process.exit(1);
}

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

const failures = verifySdkAutomation({ workspaceRoot });
if (failures.length > 0) {
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  fail('workspace:automation failed');
}

const typeScriptFailures = verifyTypeScriptWorkspace({ workspaceRoot });
if (typeScriptFailures.length > 0) {
  for (const failure of typeScriptFailures) {
    console.error(`- ${failure}`);
  }
  fail('workspace:typescript failed');
}

try {
  runTypeScriptWorkspaceVerification({ workspaceRoot });
} catch (error) {
  fail(`workspace:typescript-runtime failed: ${error.message}`);
}

const flutterFailures = verifyFlutterWorkspace({ workspaceRoot });
if (flutterFailures.length > 0) {
  for (const failure of flutterFailures) {
    console.error(`- ${failure}`);
  }
  fail('workspace:flutter failed');
}

try {
  runFlutterWorkspaceVerification({ workspaceRoot });
} catch (error) {
  fail(`workspace:flutter-runtime failed: ${error.message}`);
}

assembleSdk({ workspaceRoot });
console.log('[sdkwork-craw-chat-sdk-admin] Workspace verification passed.');
