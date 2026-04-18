import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

const runnerPath = path.join(workspaceRoot, 'bin', 'verify-docs-contract-tests.mjs');
assert.equal(
  existsSync(runnerPath),
  true,
  'workspace must provide bin/verify-docs-contract-tests.mjs',
);

const runnerSource = read('bin/verify-docs-contract-tests.mjs');
assert.match(
  runnerSource,
  /verify-internal-docs-contract\.test\.mjs/,
  'verify-docs-contract-tests.mjs must run verify-internal-docs-contract.test.mjs',
);
assert.match(
  runnerSource,
  /verify-openapi-docs-contract\.test\.mjs/,
  'verify-docs-contract-tests.mjs must run verify-openapi-docs-contract.test.mjs',
);
assert.match(
  runnerSource,
  /verify-typescript-generated-package-stale-cleanup-contract\.test\.mjs/,
  'verify-docs-contract-tests.mjs must run verify-typescript-generated-package-stale-cleanup-contract.test.mjs',
);

const verifySdkSource = read('bin/verify-sdk.mjs');
assert.match(
  verifySdkSource,
  /verify-docs-contract-tests\.mjs/,
  'verify-sdk.mjs must invoke verify-docs-contract-tests.mjs',
);

const verificationMatrixSource = read('docs/verification-matrix.md');
assert.match(
  verificationMatrixSource,
  /verify-docs-contract-tests\.mjs/,
  'docs/verification-matrix.md must document verify-docs-contract-tests.mjs',
);

console.log('docs contract runner test passed');
