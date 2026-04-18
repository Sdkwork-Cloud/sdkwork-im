import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { test } from 'node:test';

import {
  resolvePortalAppRoot,
  resolvePortalPackagesRoot,
  resolvePortalWorkspaceRoot,
} from './helpers/portal-paths.mjs';

test('portal test path helpers resolve stable roots without depending on cwd', () => {
  const originalCwd = process.cwd();
  const expectedAppRoot = path.resolve(import.meta.dirname, '..');
  const expectedPackagesRoot = path.join(expectedAppRoot, 'packages');
  const expectedWorkspaceRoot = path.resolve(expectedAppRoot, '..', '..');

  try {
    process.chdir(path.join(expectedAppRoot, 'packages'));

    assert.equal(resolvePortalAppRoot(import.meta.url), expectedAppRoot);
    assert.equal(resolvePortalPackagesRoot(import.meta.url), expectedPackagesRoot);
    assert.equal(resolvePortalWorkspaceRoot(import.meta.url), expectedWorkspaceRoot);
    assert.equal(existsSync(path.join(resolvePortalAppRoot(import.meta.url), 'package.json')), true);
  } finally {
    process.chdir(originalCwd);
  }
});
