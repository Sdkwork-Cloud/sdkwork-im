import { createStrictContractCatalog } from './strict-contract-catalog.mjs';

export const IM_COMMERCIAL_GATES_WORKFLOW_STEP_CONTRACTS = [
  {
    id: 'setup-node-v5-before-governance-runner',
    patternSource: String.raw`Setup Node[\s\S]*?uses:\s*actions\/setup-node@v5[\s\S]*?Run commercial gates governance node tests[\s\S]*?run:\s*node scripts\/run-commercial-gates-governance-node-tests\.mjs`,
    message: 'im commercial gates workflow must provision Node via actions/setup-node@v5 before delegating governance node tests to the repository-owned runner',
  },
  {
    id: 'governance-runner-before-portal-user-center-standard',
    patternSource: String.raw`Run commercial gates governance node tests[\s\S]*?run:\s*node scripts\/run-commercial-gates-governance-node-tests\.mjs[\s\S]*?Test portal user-center standard[\s\S]*?run:\s*node scripts\/run-user-center-standard\.mjs`,
    message: 'im commercial gates workflow must run the repository-owned governance node test runner before executing the portal user-center standard gate',
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
