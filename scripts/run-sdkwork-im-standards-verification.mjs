#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const pnpmExecutable = process.platform === 'win32' ? 'pnpm.cmd' : 'pnpm';

const standardChecks = [
  'test:sdkwork-workspace-structure-standard',
  'test:web-framework-standard',
  'test:database-framework-standard',
  'test:utils-standard',
  'test:runtime-standard',
  'test:runtime-id-standard',
  'test:deprecated-service-boundary',
  'test:topology-baggage',
  'test:rtc-signaling-boundary',
  'test:rpc-contract',
  'test:database-naming-standard',
  'test:component-spec-consistency',
  'test:apis-authority-standard',
  'check:agent-workflow-standard',
  'check:pnpm-script-standard',
  'check:dependency-management',
  'test:workflow-commercial-gates',
];

for (const scriptName of standardChecks) {
  const result = spawnSync(pnpmExecutable, ['run', scriptName], {
    cwd: repoRoot,
    stdio: 'inherit',
    shell: process.platform === 'win32',
  });
  if (result.status !== 0) {
    process.exit(result.status ?? 1);
  }
}

process.stdout.write('sdkwork-im standards verification passed\n');
