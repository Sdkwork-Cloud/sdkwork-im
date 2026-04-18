import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const testDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(testDir, '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

const internalDocsVerifierSource = read('bin/verify-internal-docs.mjs');
assert.match(
  internalDocsVerifierSource,
  /openapi\/README\.md/,
  'verify-internal-docs.mjs must verify openapi/README.md',
);
assert.match(
  internalDocsVerifierSource,
  /const workspaceReadmePath = 'README\.md'/,
  'verify-internal-docs.mjs must verify the workspace README.md documentation contract',
);

const workspaceReadmeSource = read('README.md');
assert.match(
  workspaceReadmeSource,
  /## Documentation Standards/,
  'workspace README must include a Documentation Standards section',
);
assert.match(
  workspaceReadmeSource,
  /docs\/README\.md/,
  'workspace README Documentation Standards section must link to docs/README.md',
);
assert.match(
  workspaceReadmeSource,
  /docs\/sites\/README\.md/,
  'workspace README Documentation Standards section must link to docs/sites/README.md',
);

const openApiReadmeSource = read('openapi/README.md');
assert.match(
  openApiReadmeSource,
  /\/openapi\/craw-chat-app\.openapi\.yaml/,
  'openapi/README.md must document the live service schema endpoint',
);
assert.match(
  openApiReadmeSource,
  /CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH/,
  'openapi/README.md must document CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH precedence',
);
assert.match(
  openApiReadmeSource,
  /last successful live schema snapshot|last successful live snapshot/i,
  'openapi/README.md must describe the checked-in authority file as the last successful live snapshot',
);

console.log('openapi docs contract test passed');
