#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  collectExpectationFailures,
  finishFileExpectationVerification,
  readWorkspaceSources,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const composedRoot = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-flutter',
  'composed',
);

const { sdkSource, typesSource, readmeSource } = readWorkspaceSources({
  workspaceRoot: composedRoot,
  files: {
    sdkSource: path.join('lib', 'control_plane_sdk.dart'),
    typesSource: path.join('lib', 'src', 'types.dart'),
    readmeSource: 'README.md',
  },
});

const expectations = [
  {
    description: 'public client uses ControlPlaneSdkClient naming',
    pattern: /class ControlPlaneSdkClient\s*\{/,
    source: sdkSource,
  },
  {
    description: 'sdk options use ControlPlaneSdkClientOptions naming',
    pattern: /class ControlPlaneSdkClientOptions\s*\{/,
    source: typesSource,
  },
  {
    description: 'create exposes flat baseUrl',
    pattern: /factory ControlPlaneSdkClient\.create\(\{[\s\S]*String\?\s+baseUrl,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat authToken',
    pattern: /factory ControlPlaneSdkClient\.create\(\{[\s\S]*String\?\s+authToken,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat headers',
    pattern: /factory ControlPlaneSdkClient\.create\(\{[\s\S]*Map<String,\s*String>\?\s+headers,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat timeout',
    pattern: /factory ControlPlaneSdkClient\.create\(\{[\s\S]*int\s+timeout\s*=\s*defaultTimeoutMs,/,
    source: sdkSource,
  },
  {
    description: 'create does not expose backendConfig',
    pattern: /factory ControlPlaneSdkClient\.create\(\{[\s\S]*ControlPlaneBackendConfig\?\s+backendConfig,/,
    source: sdkSource,
    negate: true,
  },
  {
    description: 'README uses package-aligned ControlPlaneSdkClient naming',
    pattern: /ControlPlaneSdkClient/,
    source: readmeSource,
  },
  {
    description: 'README uses flat create options',
    pattern: /ControlPlaneSdkClient\.create\(\s*baseUrl:/,
    source: readmeSource,
  },
  {
    description: 'README does not document backendConfig as public consumer API',
    pattern: /backendConfig/,
    source: readmeSource,
    negate: true,
  },
  {
    description: 'factory error message does not mention backendConfig',
    pattern: /baseUrl\/backendConfig/,
    source: sdkSource,
    negate: true,
  },
];

const failures = collectExpectationFailures(expectations);
finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'Flutter usage surface verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] Flutter usage surface verification passed.',
});
