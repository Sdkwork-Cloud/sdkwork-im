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
        id: 'pc-auth-appbase-ui-contract-uses-sdkwork-chat-pc-workspace',
        patternSource: String.raw`Test SDKWork Chat PC auth appbase UI contract[\s\S]*?working-directory:\s*apps\/sdkwork-chat-pc[\s\S]*?run:\s*node scripts\/auth-appbase-ui-contract\.test\.mjs`,
        message: 'im commercial gates workflow must execute the SDKWork Chat PC auth appbase UI contract inside apps/sdkwork-chat-pc',
      },
      {
        id: 'pc-notary-app-sdk-contract-uses-sdkwork-chat-pc-workspace',
        patternSource: String.raw`Test SDKWork Chat PC notary app SDK integration contract[\s\S]*?working-directory:\s*apps\/sdkwork-chat-pc[\s\S]*?run:\s*node scripts\/notary-app-sdk-integration-contract\.test\.mjs`,
        message: 'im commercial gates workflow must execute the SDKWork Chat PC notary app SDK integration contract inside apps/sdkwork-chat-pc',
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
      'pc-notary-app-sdk-contract-uses-sdkwork-chat-pc-workspace',
    ]).map(({ id }) => id),
    [
      'setup-node-v5-before-governance-runner',
      'pc-notary-app-sdk-contract-uses-sdkwork-chat-pc-workspace',
    ],
  );

  assert.throws(
    () => module.findImCommercialGatesWorkflowStepContract('missing-commercial-gates-contract'),
    /missing im commercial gates workflow step contract.*missing-commercial-gates-contract/i,
  );
});
