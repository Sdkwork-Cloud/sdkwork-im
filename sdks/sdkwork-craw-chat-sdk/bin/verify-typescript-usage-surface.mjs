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
const packageRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
);

const { typesSource, sdkContextSource, readmeSource } = readWorkspaceSources({
  workspaceRoot: packageRoot,
  files: {
    typesSource: path.join('src', 'types.ts'),
    sdkContextSource: path.join('src', 'sdk-context.ts'),
    readmeSource: 'README.md',
  },
});
const publicOptionsDeclarationMatch = typesSource.match(
  /export interface CrawChatSdkClientOptions \{([\s\S]*?)\n\}/,
);
const publicOptionsSource = publicOptionsDeclarationMatch?.[0] || '';

const expectations = [
  {
    description: 'public constructor options expose flat baseUrl',
    pattern: /baseUrl\??:\s*string;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options expose flat apiBaseUrl',
    pattern: /apiBaseUrl\??:\s*string;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options expose flat websocketBaseUrl',
    pattern: /websocketBaseUrl\??:\s*string;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options expose flat authToken',
    pattern: /authToken\??:\s*string;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options expose tokenProvider',
    pattern: /tokenProvider\??:\s*CrawChatTokenProvider;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options expose webSocketFactory',
    pattern: /webSocketFactory\??:\s*CrawChatWebSocketFactory;/,
    source: publicOptionsSource,
  },
  {
    description: 'public constructor options do not expose backendConfig',
    pattern: /backendConfig\??:/,
    source: publicOptionsSource,
    negate: true,
  },
  {
    description: 'public constructor options do not expose generated transport tuning fields',
    pattern: /headers\??:|timeout\??:/,
    source: publicOptionsSource,
    negate: true,
  },
  {
    description: 'backend resolution accepts flat constructor options',
    pattern: /const apiBaseUrl = firstDefinedString\(\s*options\.apiBaseUrl,\s*options\.baseUrl,\s*\);[\s\S]*const authToken = firstDefinedString\(options\.authToken\);/,
    source: sdkContextSource,
  },
  {
    description: 'backend resolution does not read backendConfig from public constructor options',
    pattern: /options\.backendConfig/,
    source: sdkContextSource,
    negate: true,
  },
  {
    description: 'README uses constructor-based flat client options',
    pattern: /new CrawChatSdkClient\(\{\s*baseUrl:/,
    source: readmeSource,
  },
  {
    description: 'README does not document backendConfig as public consumer API',
    pattern: /backendConfig/,
    source: readmeSource,
    negate: true,
  },
  {
    description: 'README documents the upload helper',
    pattern: /sdk\.upload\(/,
    source: readmeSource,
  },
  {
    description: 'README explains CrawChatUploadedMediaAsset',
    pattern: /CrawChatUploadedMediaAsset/,
    source: readmeSource,
  },
];

const failures = collectExpectationFailures(expectations);
finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'TypeScript usage surface verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] TypeScript usage surface verification passed.',
});
