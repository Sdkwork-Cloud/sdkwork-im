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
  'sdkwork-craw-chat-sdk-flutter',
  'composed',
);

const { sdkSource, typesSource, readmeSource } = readWorkspaceSources({
  workspaceRoot: composedRoot,
  files: {
    sdkSource: path.join('lib', 'craw_chat_sdk.dart'),
    typesSource: path.join('lib', 'src', 'types.dart'),
    readmeSource: 'README.md',
  },
});

const expectations = [
  {
    description: 'public client uses package-aligned CrawChatSdkClient naming',
    pattern: /class CrawChatSdkClient\s*\{/,
    source: sdkSource,
  },
  {
    description: 'sdk options use CrawChatSdkClientOptions naming',
    pattern: /class CrawChatSdkClientOptions\s*\{/,
    source: typesSource,
  },
  {
    description: 'create exposes flat baseUrl',
    pattern: /factory CrawChatSdkClient\.create\(\{[\s\S]*String\?\s+baseUrl,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat authToken',
    pattern: /factory CrawChatSdkClient\.create\(\{[\s\S]*String\?\s+authToken,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat headers',
    pattern: /factory CrawChatSdkClient\.create\(\{[\s\S]*Map<String,\s*String>\?\s+headers,/,
    source: sdkSource,
  },
  {
    description: 'create exposes flat timeout',
    pattern: /factory CrawChatSdkClient\.create\(\{[\s\S]*int\s+timeout\s*=\s*30000,/,
    source: sdkSource,
  },
  {
    description: 'create does not expose backendConfig',
    pattern: /factory CrawChatSdkClient\.create\(\{[\s\S]*SdkworkBackendConfig\?\s+backendConfig,/,
    source: sdkSource,
    negate: true,
  },
  {
    description: 'README uses package-aligned CrawChatSdkClient naming',
    pattern: /CrawChatSdkClient/,
    source: readmeSource,
  },
  {
    description: 'README uses flat create options',
    pattern: /CrawChatSdkClient\.create\(\s*baseUrl:/,
    source: readmeSource,
  },
  {
    description: 'README documents the upload helper',
    pattern: /sdk\.media\.upload\(/,
    source: readmeSource,
  },
  {
    description: 'README documents MediaUploadMutationResponse',
    pattern: /MediaUploadMutationResponse/,
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
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'Flutter usage surface verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] Flutter usage surface verification passed.',
});
