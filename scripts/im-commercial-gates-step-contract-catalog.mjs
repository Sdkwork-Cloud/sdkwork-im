import { createStrictContractCatalog } from './strict-contract-catalog.mjs';

export const IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS = [
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
];

const imCommercialGatesWorkflowStepContractCatalog = createStrictContractCatalog({
  contracts: IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS,
  duplicateIdMessagePrefix: 'duplicate im commercial gates workflow step contract id',
  missingIdMessagePrefix: 'missing im commercial gates workflow step contract',
});

export function listImCommercialGatesWorkflowStepContracts() {
  return imCommercialGatesWorkflowStepContractCatalog.list();
}

export function findImCommercialGatesWorkflowStepContract(contractId) {
  return imCommercialGatesWorkflowStepContractCatalog.find(contractId);
}

export function listImCommercialGatesWorkflowStepContractsByIds(contractIds = []) {
  return imCommercialGatesWorkflowStepContractCatalog.listByIds(contractIds);
}
