import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const __dirname = path.dirname(fileURLToPath(import.meta.url));
const appRoot = path.resolve(__dirname, '..');
const repoRoot = path.resolve(appRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(repoRoot, relativePath), 'utf8');
}

const dashboardServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-console-dashboard/src/services/DashboardService.ts',
);
const securityServiceSource = read(
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-console-security/src/services/SecurityService.ts',
);

assert.match(
  dashboardServiceSource,
  /getAppSdkClientWithSession/u,
  'Console dashboard service must consume tenant portal data through the IM app SDK runtime wrapper.',
);
assert.match(
  dashboardServiceSource,
  /\.portal\.dashboard\.retrieve\s*\(/u,
  'Console dashboard service must retrieve dashboard metrics through client.portal.dashboard.retrieve().',
);
assert.match(
  dashboardServiceSource,
  /\.portal\.conversationSnapshot\.retrieve\s*\(/u,
  'Console dashboard service must retrieve conversation activity through client.portal.conversationSnapshot.retrieve().',
);
assert.doesNotMatch(
  dashboardServiceSource,
  /mock|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Console dashboard service must not keep mock data, fake delay, raw HTTP, or manual auth header logic.',
);

assert.match(
  securityServiceSource,
  /getBackendSdkClientWithSession/u,
  'Console security service must consume security and audit data through the IM backend SDK runtime wrapper.',
);
assert.match(
  securityServiceSource,
  /\.ops\.health\.retrieve\s*\(/u,
  'Console security service must retrieve security health through backend.ops.health.retrieve().',
);
assert.match(
  securityServiceSource,
  /\.audit\.records\.list\s*\(/u,
  'Console security service must retrieve audit activity through backend.audit.records.list().',
);
assert.doesNotMatch(
  securityServiceSource,
  /mock|mockConsoleFetch|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Console security service must not keep mock data, fake delay, raw HTTP, or manual auth header logic.',
);

console.log('sdkwork chat pc console SDK integration contract passed.');
