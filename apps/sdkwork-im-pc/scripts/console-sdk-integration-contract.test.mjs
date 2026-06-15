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
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-dashboard/src/services/DashboardService.ts',
);
const securityServiceSource = read(
  'apps/sdkwork-im-pc/packages/sdkwork-im-console-security/src/services/SecurityService.ts',
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
  /getAppSdkClientWithSession/u,
  'Console security service must consume user-facing governance data through the IM app SDK runtime wrapper.',
);
assert.match(
  securityServiceSource,
  /\.portal\.governance\.retrieve\s*\(/u,
  'Console security service must retrieve governance health through client.portal.governance.retrieve().',
);
assert.match(
  securityServiceSource,
  /\.portal\.access\.retrieve\s*\(/u,
  'Console security service must retrieve user-facing access activity through client.portal.access.retrieve().',
);
assert.doesNotMatch(
  securityServiceSource,
  /getBackendSdkClientWithSession|\.ops\.|\.audit\.records/u,
  'Console security service must not consume backend SDK operations; backend security/audit records belong to admin.',
);
assert.doesNotMatch(
  securityServiceSource,
  /mock|mockConsoleFetch|setTimeout|new Promise\s*\(|\bfetch\s*\(|\b(Authorization|Access-Token|X-API-Key)\b/u,
  'Console security service must not keep mock data, fake delay, raw HTTP, or manual auth header logic.',
);

console.log('sdkwork im pc console SDK integration contract passed.');
