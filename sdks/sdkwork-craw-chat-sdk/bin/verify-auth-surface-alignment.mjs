#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import {
  assertAbsent,
  assertExactValues,
  assertPresent,
  finishAuthSurfaceVerification,
  parseAuthSurfaceLanguageArgs,
} from '../../workspace-auth-surface-shared.mjs';

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}
const prefix = 'sdkwork-craw-chat-sdk';
const supportedLanguages = ['typescript', 'flutter'];
const languageSet = parseAuthSurfaceLanguageArgs(process.argv.slice(2), {
  prefix,
  supportedLanguages,
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const workspaceRoot = path.resolve(scriptDir, '..');
const failures = [];

if (languageSet.has('typescript')) {
  const expectedGeneratedRuntimeExports = [
    'BaseApi',
    'ConversationApi',
    'DEFAULT_TIMEOUT',
    'DeviceApi',
    'InboxApi',
    'MediaApi',
    'MessageApi',
    'PresenceApi',
    'RealtimeApi',
    'RtcApi',
    'SUCCESS_CODES',
    'SdkworkBackendClient',
    'SessionApi',
    'StreamApi',
    'backendApiPath',
    'createClient',
    'createConversationApi',
    'createDeviceApi',
    'createInboxApi',
    'createMediaApi',
    'createMessageApi',
    'createPresenceApi',
    'createRealtimeApi',
    'createRtcApi',
    'createSessionApi',
    'createStreamApi',
  ];
  const generatedIndexSource = read('sdkwork-craw-chat-sdk-typescript/generated/server-openapi/src/index.ts');
  const generatedSdkSource = read('sdkwork-craw-chat-sdk-typescript/generated/server-openapi/src/sdk.ts');
  const generatedHttpClientSource = read(
    'sdkwork-craw-chat-sdk-typescript/generated/server-openapi/src/http/client.ts',
  );
  const generatedCommonTypesSource = read(
    'sdkwork-craw-chat-sdk-typescript/generated/server-openapi/src/types/common.ts',
  );
  const generatedReadmeSource = read('sdkwork-craw-chat-sdk-typescript/generated/server-openapi/README.md');
  const generatedDistIndexTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'index.d.ts',
  );
  const generatedDistSdkTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'sdk.d.ts',
  );
  const generatedDistCommonTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'types',
    'common.d.ts',
  );
  const generatedDistHttpClientTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'http',
    'client.d.ts',
  );
  const generatedDistRuntimeIndexPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'index.cjs',
  );
  const generatedDistRuntimeEsmPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'index.js',
  );
  const generatedSourceRuntimeEsmPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'src',
    'index.js',
  );
  const generatedSourceRuntimeTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'src',
    'index.d.ts',
  );
  const generatedSourceAuthIndexPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'src',
    'auth',
    'index.ts',
  );
  const generatedDistAuthIndexTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'dist',
    'auth',
    'index.d.ts',
  );
  const generatedPackageJsonPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  const composedSdkSource = read('sdkwork-craw-chat-sdk-typescript/composed/src/sdk.ts');
  const composedContextSource = read('sdkwork-craw-chat-sdk-typescript/composed/src/sdk-context.ts');
  const composedTypesSource = read('sdkwork-craw-chat-sdk-typescript/composed/src/types.ts');
  const composedShimSource = read('sdkwork-craw-chat-sdk-typescript/composed/src/shims-sdk-common.d.ts');

  assertAbsent(
    failures,
    generatedIndexSource,
    /export \* from '\.\/http';/,
    'TypeScript generated root entrypoint must not re-export ./http.',
  );
  assertAbsent(
    failures,
    generatedIndexSource,
    /export \* from '\.\/auth';/,
    'TypeScript generated root entrypoint must not re-export ./auth.',
  );
  if (existsSync(generatedSourceRuntimeEsmPath)) {
    failures.push('TypeScript generated source tree must not contain src/index.js build residue.');
  }
  if (existsSync(generatedSourceRuntimeTypesPath)) {
    failures.push('TypeScript generated source tree must not contain src/index.d.ts build residue.');
  }
  if (existsSync(generatedSourceAuthIndexPath)) {
    failures.push('TypeScript generated source tree must not contain src/auth/index.ts dead auth module.');
  }
  assertAbsent(
    failures,
    generatedSdkSource,
    /\bsetApiKey\s*\(/,
    'TypeScript generated backend client must not expose setApiKey(...).',
  );
  assertAbsent(
    failures,
    generatedSdkSource,
    /\bsetAccessToken\s*\(/,
    'TypeScript generated backend client must not expose setAccessToken(...).',
  );
  assertAbsent(
    failures,
    generatedSdkSource,
    /\bget http\s*\(/,
    'TypeScript generated backend client must not expose the raw http getter.',
  );
  assertPresent(
    failures,
    generatedSdkSource,
    /\bsetAuthToken\s*\(/,
    'TypeScript generated backend client must expose setAuthToken(...).',
  );
  assertAbsent(
    failures,
    generatedHttpClientSource,
    /\bsetApiKey\s*\(/,
    'TypeScript generated http client must not define setApiKey(...).',
  );
  assertAbsent(
    failures,
    generatedHttpClientSource,
    /\bsetAccessToken\s*\(/,
    'TypeScript generated http client must not define setAccessToken(...).',
  );
  assertAbsent(
    failures,
    generatedHttpClientSource,
    /\bAPI_KEY_HEADER\b|\bAPI_KEY_USE_BEARER\b|Access-Token|authMode|apiKey/,
    'TypeScript generated http client must stay bearer-only internally.',
  );
  assertPresent(
    failures,
    generatedHttpClientSource,
    /\bsetAuthToken\s*\(/,
    'TypeScript generated http client must define setAuthToken(...).',
  );
  assertAbsent(
    failures,
    generatedCommonTypesSource,
    /\bapiKey\?:/,
    'TypeScript generated backend config must not expose apiKey.',
  );
  assertAbsent(
    failures,
    generatedCommonTypesSource,
    /\baccessToken\?:/,
    'TypeScript generated backend config must not expose accessToken.',
  );
  assertAbsent(
    failures,
    generatedCommonTypesSource,
    /\bauthMode\?:/,
    'TypeScript generated backend config must not expose authMode.',
  );
  assertPresent(
    failures,
    generatedCommonTypesSource,
    /\bauthToken\?:/,
    'TypeScript generated backend config must expose authToken.',
  );
  assertAbsent(
    failures,
    generatedReadmeSource,
    /Mode A: API Key|Mode B: Dual Token|setApiKey|setAccessToken|Access-Token/,
    'TypeScript generated README must not document API key or dual-token auth flows.',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /setAuthToken/,
    'TypeScript generated README must document bearer auth through setAuthToken(...).',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /Use only the package root entrypoint: `@sdkwork\/craw-chat-backend-sdk`\./,
    'TypeScript generated README must document the package root entrypoint boundary.',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /Internal generator subpaths are not part of the supported public API\./,
    'TypeScript generated README must forbid internal generator subpaths as public API.',
  );
  if (existsSync(generatedDistIndexTypesPath)) {
    const generatedDistIndexTypesSource = readFileSync(generatedDistIndexTypesPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistIndexTypesSource,
      /export \* from '\.\/http';/,
      'TypeScript generated dist/index.d.ts must not re-export ./http.',
    );
    assertAbsent(
      failures,
      generatedDistIndexTypesSource,
      /export \* from '\.\/auth';/,
      'TypeScript generated dist/index.d.ts must not re-export ./auth.',
    );
  }
  if (existsSync(generatedDistSdkTypesPath)) {
    const generatedDistSdkTypesSource = readFileSync(generatedDistSdkTypesPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistSdkTypesSource,
      /\bsetApiKey\s*\(/,
      'TypeScript generated dist/sdk.d.ts must not expose setApiKey(...).',
    );
    assertAbsent(
      failures,
      generatedDistSdkTypesSource,
      /\bsetAccessToken\s*\(/,
      'TypeScript generated dist/sdk.d.ts must not expose setAccessToken(...).',
    );
    assertAbsent(
      failures,
      generatedDistSdkTypesSource,
      /\bget http\s*\(/,
      'TypeScript generated dist/sdk.d.ts must not expose the raw http getter.',
    );
    assertPresent(
      failures,
      generatedDistSdkTypesSource,
      /\bsetAuthToken\s*\(/,
      'TypeScript generated dist/sdk.d.ts must expose setAuthToken(...).',
    );
  }
  if (existsSync(generatedDistCommonTypesPath)) {
    const generatedDistCommonTypesSource = readFileSync(generatedDistCommonTypesPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistCommonTypesSource,
      /\bapiKey\?:/,
      'TypeScript generated dist/types/common.d.ts must not expose apiKey.',
    );
    assertAbsent(
      failures,
      generatedDistCommonTypesSource,
      /\baccessToken\?:/,
      'TypeScript generated dist/types/common.d.ts must not expose accessToken.',
    );
    assertAbsent(
      failures,
      generatedDistCommonTypesSource,
      /\bauthMode\?:/,
      'TypeScript generated dist/types/common.d.ts must not expose authMode.',
    );
    assertPresent(
      failures,
      generatedDistCommonTypesSource,
      /\bauthToken\?:/,
      'TypeScript generated dist/types/common.d.ts must expose authToken.',
    );
  }
  if (existsSync(generatedDistHttpClientTypesPath)) {
    const generatedDistHttpClientTypesSource = readFileSync(generatedDistHttpClientTypesPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistHttpClientTypesSource,
      /\bsetApiKey\s*\(/,
      'TypeScript generated dist/http/client.d.ts must not expose setApiKey(...).',
    );
    assertAbsent(
      failures,
      generatedDistHttpClientTypesSource,
      /\bsetAccessToken\s*\(/,
      'TypeScript generated dist/http/client.d.ts must not expose setAccessToken(...).',
    );
    assertPresent(
      failures,
      generatedDistHttpClientTypesSource,
      /\bsetAuthToken\s*\(/,
      'TypeScript generated dist/http/client.d.ts must expose setAuthToken(...).',
    );
  }
  if (existsSync(generatedDistRuntimeIndexPath)) {
    const generatedDistRuntimeIndexSource = readFileSync(generatedDistRuntimeIndexPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistRuntimeIndexSource,
      /\bsetApiKey\s*\(/,
      'TypeScript generated dist/index.cjs must not contain setApiKey(...).',
    );
    assertAbsent(
      failures,
      generatedDistRuntimeIndexSource,
      /\bsetAccessToken\s*\(/,
      'TypeScript generated dist/index.cjs must not contain setAccessToken(...).',
    );
    try {
      const generatedDistRuntimeCjs = require(generatedDistRuntimeIndexPath);
      assertExactValues(
        failures,
        Object.keys(generatedDistRuntimeCjs),
        expectedGeneratedRuntimeExports,
        'TypeScript generated CommonJS root export set drifted.',
      );
    } catch (error) {
      failures.push(
        `TypeScript generated CommonJS root runtime import failed: ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  if (existsSync(generatedDistRuntimeEsmPath)) {
    try {
      const generatedDistRuntimeEsm = await import(pathToFileURL(generatedDistRuntimeEsmPath).href);
      assertExactValues(
        failures,
        Object.keys(generatedDistRuntimeEsm),
        expectedGeneratedRuntimeExports,
        'TypeScript generated ESM root export set drifted.',
      );
    } catch (error) {
      failures.push(
        `TypeScript generated ESM root runtime import failed: ${error instanceof Error ? error.message : String(error)}`,
      );
    }
  }
  if (existsSync(generatedDistAuthIndexTypesPath)) {
    const generatedDistAuthIndexTypesSource = readFileSync(generatedDistAuthIndexTypesPath, 'utf8');
    assertAbsent(
      failures,
      generatedDistAuthIndexTypesSource,
      /\bAuthMode\b/,
      'TypeScript generated dist/auth/index.d.ts must not re-export AuthMode.',
    );
    failures.push('TypeScript generated dist/auth/index.d.ts must not exist.');
  }
  if (existsSync(generatedPackageJsonPath)) {
    const generatedPackageJson = JSON.parse(readFileSync(generatedPackageJsonPath, 'utf8'));
    assertExactValues(
      failures,
      Object.keys(generatedPackageJson.exports || {}),
      ['.'],
      'TypeScript generated package exports map must only expose the root entrypoint.',
    );
  }
  assertAbsent(
    failures,
    composedSdkSource,
    /\bsetApiKey\s*\(/,
    'TypeScript composed client must not expose setApiKey(...).',
  );
  assertAbsent(
    failures,
    composedSdkSource,
    /\bsetAccessToken\s*\(/,
    'TypeScript composed client must not expose setAccessToken(...).',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\bsetAuthToken\s*\(/,
    'TypeScript composed client must expose setAuthToken(...).',
  );
  assertAbsent(
    failures,
    composedContextSource,
    /\bsetApiKey\s*\(/,
    'TypeScript composed context must not proxy setApiKey(...).',
  );
  assertAbsent(
    failures,
    composedContextSource,
    /\bsetAccessToken\s*\(/,
    'TypeScript composed context must not proxy setAccessToken(...).',
  );
  assertAbsent(
    failures,
    composedTypesSource,
    /\bsetApiKey\?\s*\(/,
    'TypeScript composed backend client contract must not include setApiKey(...).',
  );
  assertAbsent(
    failures,
    composedTypesSource,
    /\bsetAccessToken\?\s*\(/,
    'TypeScript composed backend client contract must not include setAccessToken(...).',
  );
  assertAbsent(
    failures,
    composedShimSource,
    /\baccessToken\?:/,
    'TypeScript composed sdk-common shim must not expose accessToken.',
  );
  assertAbsent(
    failures,
    composedShimSource,
    /\bapiKey\b/,
    'TypeScript composed sdk-common shim must not expose apiKey auth mode.',
  );
  assertPresent(
    failures,
    composedShimSource,
    /\bauthToken\?:/,
    'TypeScript composed sdk-common shim must expose authToken.',
  );
  assertAbsent(
    failures,
    composedShimSource,
    /\bAuthMode\b/,
    'TypeScript composed sdk-common shim must not expose AuthMode.',
  );
}

if (languageSet.has('flutter')) {
  const generatedBackendClientSource = read(
    'sdkwork-craw-chat-sdk-flutter/generated/server-openapi/lib/backend_client.dart',
  );
  const generatedReadmeSource = read('sdkwork-craw-chat-sdk-flutter/generated/server-openapi/README.md');
  const composedSdkSource = read('sdkwork-craw-chat-sdk-flutter/composed/lib/craw_chat_sdk.dart');
  const composedContextSource = read('sdkwork-craw-chat-sdk-flutter/composed/lib/src/context.dart');

  assertPresent(
    failures,
    generatedBackendClientSource,
    /\bclass SdkworkBackendConfig\b/,
    'Flutter generated backend client must define a bearer-only SdkworkBackendConfig.',
  );
  assertAbsent(
    failures,
    generatedBackendClientSource,
    /\bapiKey\b/,
    'Flutter generated backend client must not expose apiKey fields or parameters.',
  );
  assertAbsent(
    failures,
    generatedBackendClientSource,
    /\baccessToken\b/,
    'Flutter generated backend client must not expose accessToken fields or parameters.',
  );
  assertAbsent(
    failures,
    generatedBackendClientSource,
    /\bsetApiKey\s*\(/,
    'Flutter generated backend client must not expose setApiKey(...).',
  );
  assertAbsent(
    failures,
    generatedBackendClientSource,
    /\bsetAccessToken\s*\(/,
    'Flutter generated backend client must not expose setAccessToken(...).',
  );
  assertPresent(
    failures,
    generatedBackendClientSource,
    /\bsetAuthToken\s*\(/,
    'Flutter generated backend client must expose setAuthToken(...).',
  );
  assertAbsent(
    failures,
    generatedReadmeSource,
    /Mode A: API Key|Mode B: Dual Token|setApiKey|setAccessToken|Access-Token/,
    'Flutter generated README must not document API key or dual-token auth flows.',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /setAuthToken/,
    'Flutter generated README must document bearer auth through setAuthToken(...).',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /Use only the package root entrypoint: `package:backend_sdk\/backend_sdk\.dart`\./,
    'Flutter generated README must document the package root entrypoint boundary.',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /Generated `src\/` imports are not part of the supported public API\./,
    'Flutter generated README must forbid generated src imports as public API.',
  );
  assertAbsent(
    failures,
    composedSdkSource,
    /\bString\?\s+apiKey\b|\bString\?\s+accessToken\b|\bString\s+apiKeyHeader\b|\bbool\s+apiKeyAsBearer\b/,
    'Flutter composed client factory must not expose apiKey or accessToken creation parameters.',
  );
  assertAbsent(
    failures,
    composedSdkSource,
    /\bsetApiKey\s*\(/,
    'Flutter composed client must not expose setApiKey(...).',
  );
  assertAbsent(
    failures,
    composedSdkSource,
    /\bsetAccessToken\s*\(/,
    'Flutter composed client must not expose setAccessToken(...).',
  );
  assertAbsent(
    failures,
    composedSdkSource,
    /\bSdkworkBackendConfig\?\s+backendConfig\b/,
    'Flutter composed client factory must not expose SdkworkBackendConfig? backendConfig.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\bsetAuthToken\s*\(/,
    'Flutter composed client must expose setAuthToken(...).',
  );
  assertAbsent(
    failures,
    composedContextSource,
    /\bsetApiKey\s*\(/,
    'Flutter composed context must not proxy setApiKey(...).',
  );
  assertAbsent(
    failures,
    composedContextSource,
    /\bsetAccessToken\s*\(/,
    'Flutter composed context must not proxy setAccessToken(...).',
  );
}

finishAuthSurfaceVerification({
  prefix,
  failures,
});
