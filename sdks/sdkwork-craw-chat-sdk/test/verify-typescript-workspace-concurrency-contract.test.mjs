import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const verifySdkSource = read('bin/verify-sdk.mjs');
const automationSource = read('bin/verify-sdk-automation.mjs');
const verificationMatrixSource = read('docs/verification-matrix.md');
const typescriptReadmeSource = read('sdkwork-craw-chat-sdk-typescript/README.md');

assert.match(
  verifySdkSource,
  /verify-typescript-workspace-concurrency\.mjs/,
  'verify-sdk.mjs must invoke verify-typescript-workspace-concurrency.mjs.',
);
assert.match(
  automationSource,
  /verify-typescript-workspace-concurrency\.mjs/,
  'verify-sdk-automation.mjs must guard verify-typescript-workspace-concurrency.mjs wiring.',
);
assert.match(
  verificationMatrixSource,
  /verify-typescript-workspace-concurrency\.mjs/,
  'docs/verification-matrix.md must document verify-typescript-workspace-concurrency.mjs.',
);
assert.match(
  typescriptReadmeSource,
  /verify-typescript-workspace-concurrency\.mjs/,
  'TypeScript workspace README must document verify-typescript-workspace-concurrency.mjs.',
);

console.log('typescript workspace concurrency contract test passed');
