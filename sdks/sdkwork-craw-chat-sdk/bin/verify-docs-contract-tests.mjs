#!/usr/bin/env node
import { readdirSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const testsDir = path.join(workspaceRoot, 'test');

const requiredTests = [
  'verify-internal-docs-contract.test.mjs',
  'verify-language-workspace-readme-terms-contract.test.mjs',
  'verify-multilanguage-workspace-readmes-contract.test.mjs',
  'verify-openapi-docs-contract.test.mjs',
  'verify-sdk-automation-scope-contract.test.mjs',
  'verify-typescript-generated-package-stale-cleanup-contract.test.mjs',
];

const availableTests = new Set(
  readdirSync(testsDir).filter((entry) => /^verify-.*\.test\.mjs$/.test(entry)),
);

for (const testFile of requiredTests) {
  if (!availableTests.has(testFile)) {
    console.error(`[sdkwork-craw-chat-sdk] Missing docs contract test: ${testFile}`);
    process.exit(1);
  }
}

for (const testFile of [...availableTests].sort()) {
  const result = spawnSync('node', [path.join(testsDir, testFile)], {
    cwd: workspaceRoot,
    stdio: 'inherit',
    shell: false,
  });

  if (result.error) {
    console.error(`[sdkwork-craw-chat-sdk] docs contract test failed to start: ${testFile}`);
    console.error(result.error.message);
    process.exit(1);
  }

  if (typeof result.status === 'number' && result.status !== 0) {
    console.error(
      `[sdkwork-craw-chat-sdk] docs contract test failed with exit code ${result.status}: ${testFile}`,
    );
    process.exit(result.status || 1);
  }

  if (result.signal) {
    console.error(
      `[sdkwork-craw-chat-sdk] docs contract test terminated with signal ${result.signal}: ${testFile}`,
    );
    process.exit(1);
  }
}

console.log('[sdkwork-craw-chat-sdk] Docs contract tests passed.');
