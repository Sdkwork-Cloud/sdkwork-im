import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'im-commercial-gates-watch-catalog.mjs'),
    ).href,
  );
}

test('im commercial gates workflow watch catalog publishes the exact governed watch surface', async () => {
  const module = await loadModule();

  assert.equal(typeof module.listImCommercialGatesWorkflowWatchPaths, 'function');
  assert.deepEqual(
    module.listImCommercialGatesWorkflowWatchPaths(),
    [
      '.github/workflows/im-commercial-gates.yml',
      '.github/workflows/user-center-upstream-sync.yml',
      'Cargo.toml',
      'Cargo.lock',
      'package.json',
      'README.md',
      'apps/craw-chat-admin/**',
      'apps/craw-chat-portal/**',
      'crates/**',
      'deployments/**',
      'docs/**',
      'scripts/build-craw-chat-desktop-assets.mjs',
      'scripts/commercial-gates-governance-node-test-catalog.mjs',
      'scripts/commercial-gates-governance-node-test-catalog.test.mjs',
      'scripts/im-commercial-gates-contracts.mjs',
      'scripts/im-commercial-gates-step-contract-catalog.mjs',
      'scripts/im-commercial-gates-step-contract-catalog.test.mjs',
      'scripts/im-commercial-gates-watch-catalog.mjs',
      'scripts/im-commercial-gates-watch-catalog.test.mjs',
      'scripts/im-commercial-gates-workflow.test.mjs',
      'scripts/run-commercial-gates-governance-node-tests.mjs',
      'scripts/run-commercial-gates-governance-node-tests.test.mjs',
      'scripts/run-user-center-standard.mjs',
      'scripts/run-user-center-standard.test.mjs',
      'scripts/server-user-center-entrypoint-contract.test.mjs',
      'scripts/user-center-upstream-sync-payload.mjs',
      'scripts/user-center-upstream-sync-payload.test.mjs',
      'scripts/user-center-upstream-sync-workflow.test.mjs',
      'scripts/strict-contract-catalog.mjs',
      'scripts/strict-contract-catalog.test.mjs',
      'scripts/dev/**',
      'scripts/release/**',
      'services/**',
      'sdks/**',
      'vendor/**',
    ],
  );
});

test('im commercial gates workflow watch catalog exposes strict path lookup helpers', async () => {
  const module = await loadModule();

  assert.equal(typeof module.findImCommercialGatesWorkflowWatchRequirement, 'function');
  assert.equal(typeof module.listImCommercialGatesWorkflowWatchRequirementsByPaths, 'function');

  const strictHelperRequirement = module.findImCommercialGatesWorkflowWatchRequirement(
    'scripts/strict-contract-catalog.mjs',
  );
  assert.deepEqual(
    strictHelperRequirement,
    module
      .listImCommercialGatesWorkflowWatchRequirements()
      .find(({ path }) => path === 'scripts/strict-contract-catalog.mjs'),
  );

  strictHelperRequirement.message = 'mutated locally';
  assert.notEqual(
    module.findImCommercialGatesWorkflowWatchRequirement('scripts/strict-contract-catalog.mjs').message,
    'mutated locally',
  );

  assert.deepEqual(
    module.listImCommercialGatesWorkflowWatchRequirementsByPaths([
      'scripts/strict-contract-catalog.mjs',
      'scripts/run-user-center-standard.mjs',
    ]).map(({ path }) => path),
    [
      'scripts/strict-contract-catalog.mjs',
      'scripts/run-user-center-standard.mjs',
    ],
  );
});
