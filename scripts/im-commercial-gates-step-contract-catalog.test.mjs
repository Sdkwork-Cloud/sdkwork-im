import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const repoRoot = path.resolve(import.meta.dirname, '..');

async function loadModule() {
  return import(
    pathToFileURL(
      path.join(repoRoot, 'scripts', 'im-commercial-gates-step-contract-catalog.mjs'),
    ).href,
  );
}

test('im commercial gates workflow step contract catalog publishes the exact governed workflow step assertions', async () => {
  const module = await loadModule();

  assert.equal(typeof module.listImCommercialGatesWorkflowStepContracts, 'function');
  assert.deepEqual(
    module.listImCommercialGatesWorkflowStepContracts(),
    [
      {
        id: 'setup-node-v5-before-governance-runner',
        patternSource: String.raw`Setup Node[\s\S]*?uses:\s*actions\/setup-node@v5[\s\S]*?Run commercial gates governance node tests[\s\S]*?run:\s*node scripts\/run-commercial-gates-governance-node-tests\.mjs`,
        message: 'im commercial gates workflow must provision Node via actions/setup-node@v5 before delegating governance node tests to the repository-owned runner',
      },
      {
        id: 'admin-release-safety-uses-craw-chat-admin-workspace',
        patternSource: String.raw`Test admin architecture release safety[\s\S]*?working-directory:\s*apps\/craw-chat-admin[\s\S]*?run:\s*node --test --experimental-test-isolation=none tests\/admin-architecture\.test\.mjs`,
        message: 'im commercial gates workflow must execute the admin release safety contract inside apps/craw-chat-admin instead of the retired control-plane path',
      },
      {
        id: 'portal-release-safety-uses-craw-chat-portal-workspace',
        patternSource: String.raw`Test portal dist release safety[\s\S]*?working-directory:\s*apps\/craw-chat-portal[\s\S]*?run:\s*node --test tests\/portal-build-smoke\.test\.mjs`,
        message: 'im commercial gates workflow must execute the portal release safety smoke inside apps/craw-chat-portal',
      },
    ],
  );
});

test('im commercial gates workflow step contract catalog exposes strict id-based lookup helpers', async () => {
  const module = await loadModule();

  assert.equal(typeof module.findImCommercialGatesWorkflowStepContract, 'function');
  assert.equal(typeof module.listImCommercialGatesWorkflowStepContractsByIds, 'function');

  const governanceRunnerContract = module.findImCommercialGatesWorkflowStepContract(
    'setup-node-v5-before-governance-runner',
  );
  assert.deepEqual(
    governanceRunnerContract,
    module
      .listImCommercialGatesWorkflowStepContracts()
      .find(({ id }) => id === 'setup-node-v5-before-governance-runner'),
  );

  governanceRunnerContract.message = 'mutated locally';
  assert.notEqual(
    module.findImCommercialGatesWorkflowStepContract('setup-node-v5-before-governance-runner').message,
    'mutated locally',
  );

  assert.deepEqual(
    module.listImCommercialGatesWorkflowStepContractsByIds([
      'setup-node-v5-before-governance-runner',
      'portal-release-safety-uses-craw-chat-portal-workspace',
    ]).map(({ id }) => id),
    [
      'setup-node-v5-before-governance-runner',
      'portal-release-safety-uses-craw-chat-portal-workspace',
    ],
  );

  assert.throws(
    () => module.findImCommercialGatesWorkflowStepContract('missing-commercial-gates-contract'),
    /missing im commercial gates workflow step contract.*missing-commercial-gates-contract/i,
  );
});
