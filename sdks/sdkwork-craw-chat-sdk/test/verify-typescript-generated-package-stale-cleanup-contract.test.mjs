import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const buildSource = read('bin/build-typescript-generated-package.mjs');

assert.match(
  buildSource,
  /disposeStagedOutput/,
  'build-typescript-generated-package.mjs must centralize staged output disposal so Windows EPERM cleanup can be retried safely.',
);
assert.match(
  buildSource,
  /tryPruneRetainedStaleOutputs/,
  'build-typescript-generated-package.mjs must best-effort prune retained stale outputs from earlier runs.',
);
assert.match(
  buildSource,
  /Retaining staged TypeScript generated output for later cleanup/,
  'build-typescript-generated-package.mjs must document when a staged stale output is retained for later cleanup instead of failing immediately.',
);
assert.doesNotMatch(
  buildSource,
  /renameSync\(resolvedDistRoot,\s*staleDistRoot\);\s*rmSync\(staleDistRoot,\s*\{\s*recursive:\s*true,\s*force:\s*true\s*\}\);/s,
  'build-typescript-generated-package.mjs must not hard-fail on a one-shot rmSync immediately after staging the previous dist tree.',
);

console.log('typescript generated package stale cleanup contract test passed');
