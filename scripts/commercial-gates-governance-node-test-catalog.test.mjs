import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'commercial-gates-governance-node-test-catalog.mjs'),
    ).href,
  );
}

test('commercial gates governance node test catalog publishes the exact governed test file set', async () => {
  const module = await loadModule();

  assert.equal(typeof module.listCommercialGatesGovernanceNodeTestFiles, 'function');
  assert.deepEqual(
    module.listCommercialGatesGovernanceNodeTestFiles(),
    [
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/commercial-gates-governance-node-test-catalog.test.mjs',
      'scripts/run-commercial-gates-governance-node-tests.test.mjs',
      'scripts/im-commercial-gates-watch-catalog.test.mjs',
      'scripts/im-commercial-gates-step-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
      'scripts/run-user-center-standard.test.mjs',
      'scripts/server-user-center-entrypoint-contract.test.mjs',
      'scripts/user-center-upstream-sync-payload.test.mjs',
      'scripts/user-center-upstream-sync-workflow.test.mjs',
      'scripts/release/commercial-readiness.test.mjs',
      'apps/craw-chat-portal/tests/product-entrypoint-scripts.test.mjs',
    ],
  );
});

test('commercial gates governance node test catalog exposes strict file lookup helpers', async () => {
  const module = await loadModule();

  assert.equal(typeof module.findCommercialGatesGovernanceNodeTestFile, 'function');
  assert.equal(typeof module.listCommercialGatesGovernanceNodeTestFilesByPaths, 'function');

  assert.equal(
    module.findCommercialGatesGovernanceNodeTestFile('scripts/strict-contract-catalog.test.mjs'),
    'scripts/strict-contract-catalog.test.mjs',
  );
  assert.deepEqual(
    module.listCommercialGatesGovernanceNodeTestFilesByPaths([
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
    ]),
    [
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
    ],
  );
  assert.throws(
    () => module.findCommercialGatesGovernanceNodeTestFile('scripts/missing-commercial-governance.test.mjs'),
    /missing commercial gates governance node test file.*missing-commercial-governance/i,
  );
});
