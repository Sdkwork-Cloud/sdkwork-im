import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import assert from 'node:assert/strict';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const coreSdkRoot = path.join(
  appRoot,
  'packages',
  'sdkwork-clawchat-pc-core',
  'src',
  'sdk',
);

function readSdkSource(fileName) {
  return readFileSync(path.join(coreSdkRoot, fileName), 'utf8');
}

const sessionSource = readSdkSource('session.ts');
const sdkClientFiles = [
  'appSdkClient.ts',
  'appbaseAppSdkClient.ts',
  'agentAppSdkClient.ts',
  'imSdkClient.ts',
];

assert.match(
  sessionSource,
  /export function getSdkworkChatGlobalTokenManager\(/u,
  'PC SDK runtime must expose one shared TokenManager for all authenticated SDK clients.',
);

assert.match(
  sessionSource,
  /export function syncSdkworkChatGlobalTokenManager\(/u,
  'Session persistence must synchronize the shared TokenManager after login, refresh, restore, and logout.',
);

assert.match(
  sessionSource,
  /syncSdkworkChatGlobalTokenManager\(normalizedSession\)/u,
  'persistAppSdkSessionTokens must sync normalized session state into the shared TokenManager.',
);

assert.match(
  sessionSource,
  /syncSdkworkChatGlobalTokenManager\(null\)/u,
  'clearAppSdkSessionTokens must clear the shared TokenManager.',
);

for (const fileName of sdkClientFiles) {
  const source = readSdkSource(fileName);
  assert.match(
    source,
    /getSdkworkChatGlobalTokenManager/u,
    `${fileName} must inject the PC runtime shared TokenManager.`,
  );
  assert.doesNotMatch(
    source,
    /createSdkworkChatSessionTokenManager\(currentSession\)/u,
    `${fileName} must not create a per-client TokenManager snapshot.`,
  );
}

console.log('sdkwork chat pc SDK runtime TokenManager contract passed.');
