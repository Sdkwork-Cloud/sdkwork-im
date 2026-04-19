#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  finishFileExpectationVerification,
  readWorkspaceSource,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const modelsSource = readWorkspaceSource({
  workspaceRoot,
  relativePath: path.join(
    'sdkwork-control-plane-sdk-flutter',
    'generated',
    'server-openapi',
    'lib',
    'src',
    'models.dart',
  ),
});
const failures = [];

if (!modelsSource.includes('typedef JsonObject = Map<String, dynamic>;')) {
  failures.push('Flutter generated models.dart must define JsonObject as Map<String, dynamic>.');
}
if (!modelsSource.includes('typedef QueryParams = Map<String, dynamic>;')) {
  failures.push('Flutter generated models.dart must define QueryParams as Map<String, dynamic>.');
}
if (!modelsSource.includes('class ControlPlaneBackendConfig')) {
  failures.push('Flutter generated models.dart must define ControlPlaneBackendConfig.');
}
if (!modelsSource.includes('class AdminApiError implements Exception')) {
  failures.push('Flutter generated models.dart must expose AdminApiError.');
}

finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'Flutter generated model verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] Flutter generated model verification passed.',
});
