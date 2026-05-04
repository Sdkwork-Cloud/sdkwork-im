import assert from 'node:assert/strict';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

import { resolvePortalAppRoot, resolvePortalWorkspaceRoot } from './helpers/portal-paths.mjs';

const appRoot = resolvePortalAppRoot(import.meta.url);
const workspaceRoot = resolvePortalWorkspaceRoot(import.meta.url);

test('craw-chat portal package exposes a dedicated user-center standard contract script', async () => {
  const packageJson = await import(pathToFileURL(path.join(appRoot, 'package.json')).href, {
    with: { type: 'json' },
  });

  assert.equal(
    packageJson.default.scripts['test:user-center-standard'],
    'node ../../scripts/run-user-center-standard.mjs',
  );
  assert.equal(
    packageJson.default.scripts.test,
    'node --test --experimental-test-isolation=none tests/*.test.mjs',
  );
});

test('craw-chat workspace relays the portal user-center standard contract as a first-class entrypoint', async () => {
  const packageJson = await import(pathToFileURL(path.join(workspaceRoot, 'package.json')).href, {
    with: { type: 'json' },
  });

  assert.equal(
    packageJson.default.scripts['test:user-center-standard'],
    'node scripts/run-user-center-standard.mjs',
  );
});
