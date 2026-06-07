import { readFileSync } from 'node:fs';
import { fileURLToPath } from 'node:url';
import path from 'node:path';
import assert from 'node:assert/strict';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const backendSdkClientSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/sdk/backendSdkClient.ts',
);
const coreIndexSource = read('apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/src/index.ts');
const adminDashboardServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-dashboard/src/services/AdminDashboardService.ts',
);
const infraStatusServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-infrastructure/src/services/InfraStatusService.ts',
);
const adminBillingServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-infrastructure/src/services/AdminBillingService.ts',
);
const adminComplianceServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-admin-operations/src/services/AdminComplianceService.ts',
);
const messageAuditServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-console-communications/src/services/MessageAuditService.ts',
);

assert.match(
  backendSdkClientSource,
  /from ['"]@sdkwork-internal\/im-backend-api-generated['"]/u,
  'PC core backend SDK wrapper must import the generated IM backend SDK package.',
);
assert.match(
  backendSdkClientSource,
  /SdkworkImBackendClient/u,
  'PC core backend SDK wrapper must expose the product-scoped SdkworkImBackendClient.',
);
assert.match(
  backendSdkClientSource,
  /getSdkworkChatGlobalTokenManager/u,
  'PC backend SDK wrapper must share the runtime global TokenManager.',
);
assert.match(
  backendSdkClientSource,
  /createSdkworkChatRequestContextInterceptors/u,
  'PC backend SDK wrapper must attach dynamic SDKWork AppContext request interceptors.',
);
assert.match(
  backendSdkClientSource,
  /VITE_CRAW_CHAT_BACKEND_API_BASE_URL/u,
  'PC backend SDK wrapper must resolve a backend API base URL surface explicitly.',
);
assert.doesNotMatch(
  backendSdkClientSource,
  /\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'PC backend SDK wrapper must not use raw HTTP or assemble auth headers manually.',
);
assert.match(
  coreIndexSource,
  /export \* from '\.\/sdk\/backendSdkClient'/u,
  'PC core package must export the backend SDK wrapper.',
);

for (const [label, source] of [
  ['admin dashboard service', adminDashboardServiceSource],
  ['infrastructure status service', infraStatusServiceSource],
  ['admin billing service', adminBillingServiceSource],
  ['admin compliance service', adminComplianceServiceSource],
  ['message audit service', messageAuditServiceSource],
]) {
  assert.match(
    source,
    /getBackendSdkClientWithSession/u,
    `${label} must receive backend/operator data through the generated IM backend SDK wrapper.`,
  );
  assert.doesNotMatch(
    source,
    /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
    `${label} must not keep mock data, fake delay, raw HTTP, or manual auth header logic.`,
  );
}

assert.match(adminDashboardServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(adminDashboardServiceSource, /\.ops\.cluster\.retrieve\s*\(/u);
assert.match(adminDashboardServiceSource, /\.audit\.records\.list\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.cluster\.retrieve\s*\(/u);
assert.match(infraStatusServiceSource, /\.ops\.diagnostics\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.summary\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.events\.summary\.retrieve\s*\(/u);
assert.match(adminBillingServiceSource, /\.admin\.billing\.events\.list\s*\(/u);
assert.match(adminComplianceServiceSource, /\.ops\.health\.retrieve\s*\(/u);
assert.match(adminComplianceServiceSource, /\.audit\.records\.list\s*\(/u);
assert.match(messageAuditServiceSource, /\.audit\.records\.list\s*\(/u);

console.log('sdkwork chat pc backend SDK integration contract passed.');
