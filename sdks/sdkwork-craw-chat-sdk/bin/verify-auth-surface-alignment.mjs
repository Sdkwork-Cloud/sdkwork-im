#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    languages: [],
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim().toLowerCase();
      if (!value) {
        fail('Missing value for --language');
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

function readIfExists(relativePath, failures, missingMessage) {
  const absolutePath = path.join(workspaceRoot, relativePath);
  if (!existsSync(absolutePath)) {
    failures.push(missingMessage);
    return null;
  }
  return readFileSync(absolutePath, 'utf8');
}

function assertAbsent(failures, source, pattern, message) {
  if (pattern.test(source)) {
    failures.push(message);
  }
}

function assertPresent(failures, source, pattern, message) {
  if (!pattern.test(source)) {
    failures.push(message);
  }
}

function assertExactValues(failures, actualValues, expectedValues, message) {
  const actual = [...actualValues].sort();
  const expected = [...expectedValues].sort();
  if (actual.length !== expected.length || actual.some((value, index) => value !== expected[index])) {
    failures.push(
      `${message} Expected [${expected.join(', ')}] but found [${actual.join(', ')}].`,
    );
  }
}

const args = parseArgs(process.argv.slice(2));
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const require = createRequire(import.meta.url);
const workspaceRoot = path.resolve(scriptDir, '..');
const languageSet = new Set(args.languages.length > 0 ? args.languages : ['typescript', 'flutter']);

for (const language of languageSet) {
  if (!['typescript', 'flutter', 'rust'].includes(language)) {
    fail(`Unsupported language: ${language}`);
  }
}

const failures = [];

if (languageSet.has('typescript')) {
  const expectedGeneratedRuntimeExports = [
    'AuthApi',
    'BaseApi',
    'ConversationApi',
    'DEFAULT_TIMEOUT',
    'DeviceApi',
    'InboxApi',
    'MediaApi',
    'MessageApi',
    'PortalApi',
    'PresenceApi',
    'RealtimeApi',
    'RtcApi',
    'SUCCESS_CODES',
    'SdkworkBackendClient',
    'SessionApi',
    'StreamApi',
    'backendApiPath',
    'createAuthApi',
    'createClient',
    'createConversationApi',
    'createDeviceApi',
    'createInboxApi',
    'createMediaApi',
    'createMessageApi',
    'createPortalApi',
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
  const composedShimPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-typescript',
    'composed',
    'src',
    'shims-sdk-common.d.ts',
  );

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
    /\bpublic readonly auth:\s*AuthApi;/,
    'TypeScript generated backend client must expose auth API wiring.',
  );
  assertPresent(
    failures,
    generatedSdkSource,
    /\bpublic readonly portal:\s*PortalApi;/,
    'TypeScript generated backend client must expose portal API wiring.',
  );
  assertPresent(
    failures,
    generatedSdkSource,
    /\bthis\.auth = createAuthApi\(this\.httpClient\);/,
    'TypeScript generated backend client must construct auth API from the shared http client.',
  );
  assertPresent(
    failures,
    generatedSdkSource,
    /\bthis\.portal = createPortalApi\(this\.httpClient\);/,
    'TypeScript generated backend client must construct portal API from the shared http client.',
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
  assertAbsent(
    failures,
    composedSdkSource,
    /\bsetAuthToken\s*\(/,
    'TypeScript composed client must not expose root-level setAuthToken(...); auth must live under sdk.auth.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\breadonly auth:/,
    'TypeScript composed client must expose an auth domain module.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\bconnect\s*\(/,
    'TypeScript composed client must expose connect(...) as the live runtime entrypoint.',
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
  if (existsSync(composedShimPath)) {
    failures.push(
      'TypeScript composed layer must not ship the legacy shims-sdk-common.d.ts override once real @sdkwork/sdk-common runtime types are available.',
    );
  }
}

if (languageSet.has('flutter')) {
  const generatedBackendClientSource = read(
    'sdkwork-craw-chat-sdk-flutter/generated/server-openapi/lib/backend_client.dart',
  );
  const generatedReadmeSource = read('sdkwork-craw-chat-sdk-flutter/generated/server-openapi/README.md');
  const composedSdkSource = read('sdkwork-craw-chat-sdk-flutter/composed/lib/craw_chat_sdk.dart');
  const composedContextSource = read('sdkwork-craw-chat-sdk-flutter/composed/lib/src/context.dart');
  const composedAuthModuleSource = read(
    'sdkwork-craw-chat-sdk-flutter/composed/lib/src/auth_module.dart',
  );
  const composedPortalModuleSource = read(
    'sdkwork-craw-chat-sdk-flutter/composed/lib/src/portal_module.dart',
  );

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
  assertPresent(
    failures,
    generatedBackendClientSource,
    /\blate final AuthApi auth;/,
    'Flutter generated backend client must expose auth API wiring.',
  );
  assertPresent(
    failures,
    generatedBackendClientSource,
    /\blate final PortalApi portal;/,
    'Flutter generated backend client must expose portal API wiring.',
  );
  assertPresent(
    failures,
    generatedBackendClientSource,
    /\bauth = AuthApi\(_httpClient\);/,
    'Flutter generated backend client must construct auth API from the shared http client.',
  );
  assertPresent(
    failures,
    generatedBackendClientSource,
    /\bportal = PortalApi\(_httpClient\);/,
    'Flutter generated backend client must construct portal API from the shared http client.',
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
  assertPresent(
    failures,
    generatedReadmeSource,
    /`client\.auth`/,
    'Flutter generated README must document client.auth as a mounted generated API group.',
  );
  assertPresent(
    failures,
    generatedReadmeSource,
    /`client\.portal`/,
    'Flutter generated README must document client.portal as a mounted generated API group.',
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
  assertPresent(
    failures,
    composedSdkSource,
    /\bString\?\s+authToken\b/,
    'Flutter composed client factory must expose a flat authToken parameter.',
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
  assertPresent(
    failures,
    composedSdkSource,
    /\blate final CrawChatAuthModule auth;/,
    'Flutter composed client must expose an auth domain module.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\blate final CrawChatPortalModule portal;/,
    'Flutter composed client must expose a portal domain module.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\bauth = CrawChatAuthModule\(_context\);/,
    'Flutter composed client must construct its auth domain from the shared context.',
  );
  assertPresent(
    failures,
    composedSdkSource,
    /\bportal = CrawChatPortalModule\(_context\);/,
    'Flutter composed client must construct its portal domain from the shared context.',
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
  assertPresent(
    failures,
    composedContextSource,
    /\bclearAuthToken\s*\(/,
    'Flutter composed context must expose clearAuthToken(...).',
  );
  assertPresent(
    failures,
    composedContextSource,
    /\bsetAuthToken\(''\);/,
    'Flutter composed context clearAuthToken() must clear bearer auth through setAuthToken(\'\').',
  );
  assertPresent(
    failures,
    composedAuthModuleSource,
    /\bclass CrawChatAuthModule\b/,
    'Flutter composed layer must define CrawChatAuthModule.',
  );
  assertPresent(
    failures,
    composedAuthModuleSource,
    /\blogin\s*\(PortalLoginRequest body\)/,
    'Flutter auth module must expose login(PortalLoginRequest body).',
  );
  assertPresent(
    failures,
    composedAuthModuleSource,
    /\bme\s*\(\)/,
    'Flutter auth module must expose me().',
  );
  assertPresent(
    failures,
    composedAuthModuleSource,
    /\buseToken\s*\(String token\)/,
    'Flutter auth module must expose useToken(String token).',
  );
  assertPresent(
    failures,
    composedAuthModuleSource,
    /\bclearToken\s*\(\)/,
    'Flutter auth module must expose clearToken().',
  );
  assertPresent(
    failures,
    composedPortalModuleSource,
    /\bclass CrawChatPortalModule\b/,
    'Flutter composed layer must define CrawChatPortalModule.',
  );
  for (const portalMethod of [
    'getHome',
    'getAuth',
    'getWorkspace',
    'getDashboard',
    'getConversations',
    'getRealtime',
    'getMedia',
    'getAutomation',
    'getGovernance',
  ]) {
    assertPresent(
      failures,
      composedPortalModuleSource,
      new RegExp(`\\b${portalMethod}\\s*\\(`),
      `Flutter portal module must expose ${portalMethod}().`,
    );
  }
}

if (languageSet.has('rust')) {
  const generatedClientSource = readIfExists(
    'sdkwork-craw-chat-sdk-rust/generated/server-openapi/src/client.rs',
    failures,
    'Rust generated backend client source is missing: sdkwork-craw-chat-sdk-rust/generated/server-openapi/src/client.rs.',
  );
  const generatedHttpClientSource = readIfExists(
    'sdkwork-craw-chat-sdk-rust/generated/server-openapi/src/http/client.rs',
    failures,
    'Rust generated http client source is missing: sdkwork-craw-chat-sdk-rust/generated/server-openapi/src/http/client.rs.',
  );
  const generatedReadmeSource = readIfExists(
    'sdkwork-craw-chat-sdk-rust/generated/server-openapi/README.md',
    failures,
    'Rust generated README is missing: sdkwork-craw-chat-sdk-rust/generated/server-openapi/README.md.',
  );
  const generatedCargoTomlSource = readIfExists(
    'sdkwork-craw-chat-sdk-rust/generated/server-openapi/Cargo.toml',
    failures,
    'Rust generated Cargo manifest is missing: sdkwork-craw-chat-sdk-rust/generated/server-openapi/Cargo.toml.',
  );

  if (generatedClientSource) {
    assertAbsent(
      failures,
      generatedClientSource,
      /\bset_api_key\s*\(/,
      'Rust generated backend client must not expose set_api_key(...).',
    );
    assertAbsent(
      failures,
      generatedClientSource,
      /\bset_access_token\s*\(/,
      'Rust generated backend client must not expose set_access_token(...).',
    );
    assertPresent(
      failures,
      generatedClientSource,
      /\bset_auth_token\s*\(/,
      'Rust generated backend client must expose set_auth_token(...).',
    );
  }

  if (generatedHttpClientSource) {
    assertAbsent(
      failures,
      generatedHttpClientSource,
      /\bset_api_key\s*\(/,
      'Rust generated http client must not expose set_api_key(...).',
    );
    assertAbsent(
      failures,
      generatedHttpClientSource,
      /\bset_access_token\s*\(/,
      'Rust generated http client must not expose set_access_token(...).',
    );
    assertAbsent(
      failures,
      generatedHttpClientSource,
      /\bDEFAULT_API_KEY_HEADER\b|\bDEFAULT_API_KEY_USE_BEARER\b|Access-Token|api_key/,
      'Rust generated http client must stay bearer-only internally.',
    );
    assertPresent(
      failures,
      generatedHttpClientSource,
      /\bset_auth_token\s*\(/,
      'Rust generated http client must expose set_auth_token(...).',
    );
  }

  if (generatedReadmeSource) {
    assertAbsent(
      failures,
      generatedReadmeSource,
      /set_api_key|set_access_token|Access-Token|Mode A: API Key|Mode B: Dual Token/,
      'Rust generated README must not document API key or dual-token auth flows.',
    );
    assertPresent(
      failures,
      generatedReadmeSource,
      /set_auth_token/,
      'Rust generated README must document bearer auth through set_auth_token(...).',
    );
  }

  if (generatedCargoTomlSource) {
    assertPresent(
      failures,
      generatedCargoTomlSource,
      /^name = "sdkwork-craw-chat-backend-sdk"$/m,
      'Rust generated Cargo manifest must keep package name sdkwork-craw-chat-backend-sdk.',
    );
    assertPresent(
      failures,
      generatedCargoTomlSource,
      /^\[lib\]$/m,
      'Rust generated Cargo manifest must declare a lib target.',
    );
    assertPresent(
      failures,
      generatedCargoTomlSource,
      /^path = "src\/lib\.rs"$/m,
      'Rust generated Cargo manifest lib target must point to src/lib.rs.',
    );
  }
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Auth surface alignment verification failed:');
  for (const entry of failures) {
    console.error(`- ${entry}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Auth surface alignment verification passed.');
