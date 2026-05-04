import { createStrictKeyedCatalog } from './strict-contract-catalog.mjs';

export const COMMERCIAL_GATES_GOVERNANCE_NODE_TEST_FILES = [
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
];

const commercialGatesGovernanceNodeTestCatalog = createStrictKeyedCatalog({
  entries: COMMERCIAL_GATES_GOVERNANCE_NODE_TEST_FILES,
  getKey: (filePath) => filePath,
  duplicateKeyMessagePrefix: 'duplicate commercial gates governance node test file',
  missingKeyMessagePrefix: 'missing commercial gates governance node test file',
});

export function listCommercialGatesGovernanceNodeTestFiles() {
  return commercialGatesGovernanceNodeTestCatalog.list();
}

export function findCommercialGatesGovernanceNodeTestFile(filePath) {
  return commercialGatesGovernanceNodeTestCatalog.find(filePath);
}

export function listCommercialGatesGovernanceNodeTestFilesByPaths(filePaths = []) {
  return commercialGatesGovernanceNodeTestCatalog.listByKeys(filePaths);
}
