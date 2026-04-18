import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const buildSource = read('bin/build-typescript-generated-package.mjs');
const verifyGeneratedPackageSource = read('bin/verify-typescript-generated-package.mjs');
const automationSource = read('bin/verify-sdk-automation.mjs');

assert.match(
  buildSource,
  /stable-typescript-generated-build\.lock/,
  'build-typescript-generated-package.mjs must keep the stable-typescript-generated-build.lock mutex.',
);
assert.match(
  verifyGeneratedPackageSource,
  /stable-typescript-generated-build\.lock/,
  'verify-typescript-generated-package.mjs must reuse stable-typescript-generated-build.lock while reading shared generated dist artifacts.',
);
assert.match(
  verifyGeneratedPackageSource,
  /acquireGeneratedArtifactsLock/,
  'verify-typescript-generated-package.mjs must explicitly acquire the generated artifacts lock.',
);
assert.match(
  verifyGeneratedPackageSource,
  /releaseGeneratedArtifactsLock/,
  'verify-typescript-generated-package.mjs must explicitly release the generated artifacts lock.',
);
assert.match(
  automationSource,
  /stable-typescript-generated-build\.lock|acquireGeneratedArtifactsLock|releaseGeneratedArtifactsLock/,
  'verify-sdk-automation.mjs must guard the shared generated artifacts lock contract.',
);

console.log('typescript generated package lock contract test passed');
