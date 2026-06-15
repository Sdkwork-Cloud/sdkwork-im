import { createStrictKeyedCatalog } from './strict-contract-catalog.mjs';

export const COMMERCIAL_GATES_GOVERNANCE_NODE_TEST_FILES = [
  'scripts/strict-contract-catalog.test.mjs',
  'scripts/commercial-gates-governance-node-test-catalog.test.mjs',
  'scripts/run-commercial-gates-governance-node-tests.test.mjs',
  'scripts/im-commercial-gates-watch-catalog.test.mjs',
  'scripts/im-commercial-gates-step-contract-catalog.test.mjs',
  'scripts/im-commercial-gates-workflow.test.mjs',
  'scripts/dev/sdkwork-im-pc-dev-command.test.mjs',
  'scripts/dev/sdkwork-im-pc-i18n.test.mjs',
  'scripts/dev/sdkwork-im-pc-sidebar-modules.test.mjs',
  'scripts/dev/sdkwork-im-pc-im-api-standard.test.mjs',
  'scripts/dev/sdkwork-im-pc-sdk-integration.test.mjs',
  'scripts/dev/sdkwork-im-sdk-websocket-contract-node.test.mjs',
  'apps/sdkwork-im-pc/scripts/auth-appbase-ui-contract.test.mjs',
  'apps/sdkwork-im-pc/scripts/notary-app-sdk-integration-contract.test.mjs',
  'scripts/release/commercial-readiness.test.mjs',
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
