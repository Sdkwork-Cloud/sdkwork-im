import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

const internalDocsVerifierPath = path.join(workspaceRoot, 'bin', 'verify-internal-docs.mjs');
assert.equal(
  existsSync(internalDocsVerifierPath),
  true,
  'workspace must provide bin/verify-internal-docs.mjs',
);

const verifySdkSource = read('bin/verify-sdk.mjs');
assert.match(
  verifySdkSource,
  /verify-internal-docs\.mjs/,
  'verify-sdk.mjs must invoke verify-internal-docs.mjs',
);

const internalDocsVerifierSource = read('bin/verify-internal-docs.mjs');
for (const marker of [
  '/openapi/craw-chat-app.openapi.yaml',
  'live service schema',
  '--with-dart',
  '-WithDart',
  'verify-typescript-live-contract.mjs',
  '.sdkwork/dart/pub-cache',
  'verify-flutter-dart-analysis.dart',
  'single-package',
  'src/generated/**',
  'CrawChatSdkClient',
  'bin/start-local.ps1',
  'bin/start-local.sh',
  'runtime root-export validation',
  'dead-auth/dead-residue cleanup',
  'npm pack --dry-run',
  'workspace-internal documentation map',
  'verify-sdk-automation.mjs',
  'bin/verify-sdk.mjs',
  'two-package consumer model',
  'generated-versus-composed',
  'live.messages',
  'connected`, `error`, and `closed`',
  'verify-api-docs.mjs',
  'verify-sdk-docs.mjs',
]) {
  assert.match(
    internalDocsVerifierSource,
    new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
    `verify-internal-docs.mjs must enforce internal-doc marker ${marker}`,
  );
}
assert.match(
  internalDocsVerifierSource,
  /TypeScript composed package:/,
  'verify-internal-docs.mjs must guard against the removed TypeScript composed-package wording in the workspace README',
);

const verificationMatrixSource = read('docs/verification-matrix.md');
assert.match(
  verificationMatrixSource,
  /verify-internal-docs\.mjs/,
  'docs/verification-matrix.md must document verify-internal-docs.mjs',
);

const realtimeBoundarySource = read('docs/realtime-extension-boundary.md');
assert.match(
  realtimeBoundarySource,
  /\/api\/v1\/realtime\/ws/,
  'docs/realtime-extension-boundary.md must document the realtime WebSocket handshake route explicitly',
);
assert.match(
  realtimeBoundarySource,
  /sites\/README\.md/,
  'docs/realtime-extension-boundary.md must link to sites/README.md',
);

console.log('internal docs contract test passed');
