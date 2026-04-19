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
  'sdkwork-control-plane-sdk-typescript',
  'composed',
);

const { typesSource, sdkContextSource, readmeSource } = readWorkspaceSources({
  workspaceRoot: composedRoot,
  files: {
    typesSource: path.join('src', 'types.ts'),
    sdkContextSource: path.join('src', 'sdk-context.ts'),
    readmeSource: 'README.md',
  },
});

const expectations = [
  {
    description: 'create options expose flat baseUrl',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*baseUrl\??:\s*string;/,
    source: typesSource,
  },
  {
    description: 'create options expose flat authToken',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*authToken\??:\s*string;/,
    source: typesSource,
  },
  {
    description: 'create options expose flat headers',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*headers\??:\s*Record<string,\s*string>;/,
    source: typesSource,
  },
  {
    description: 'create options expose flat timeout',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*timeout\??:\s*number;/,
    source: typesSource,
  },
  {
    description: 'create options expose flat fetch',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*fetch\??:\s*FetchLike;/,
    source: typesSource,
  },
  {
    description: 'create options do not expose backendConfig',
    pattern: /interface ControlPlaneSdkClientCreateOptions[\s\S]*backendConfig\??:/,
    source: typesSource,
    negate: true,
  },
  {
    description: 'backend resolution accepts flat create options',
    pattern: /if \(options\.baseUrl\) \{[\s\S]*baseUrl:\s*options\.baseUrl[\s\S]*authToken:\s*options\.authToken/,
    source: sdkContextSource,
  },
  {
    description: 'backend resolution does not read backendConfig from public create options',
    pattern: /options\.backendConfig/,
    source: sdkContextSource,
    negate: true,
  },
  {
    description: 'README uses flat create options',
    pattern: /ControlPlaneSdkClient\.create\(\{\s*baseUrl:/,
    source: readmeSource,
  },
  {
    description: 'README does not document backendConfig as public consumer API',
    pattern: /backendConfig/,
    source: readmeSource,
    negate: true,
  },
  {
    description: 'README documents protocol module',
    pattern: /sdk\.protocol\.getRegistry\(\)/,
    source: readmeSource,
  },
  {
    description: 'README documents socialRuntime module',
    pattern: /`socialRuntime`/,
    source: readmeSource,
  },
];

const failures = collectExpectationFailures(expectations);
finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'TypeScript usage surface verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] TypeScript usage surface verification passed.',
});
