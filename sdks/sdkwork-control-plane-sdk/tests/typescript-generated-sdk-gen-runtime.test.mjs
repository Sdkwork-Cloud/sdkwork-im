import assert from 'node:assert/strict';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const workspaceRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const verifyScriptPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-sdk-gen-runtime.mjs',
);

assert.ok(
  existsSync(verifyScriptPath),
  'workspace must provide bin/verify-typescript-generated-sdk-gen-runtime.mjs',
);

const result = spawnSync(process.execPath, [verifyScriptPath], {
  cwd: workspaceRoot,
  encoding: 'utf8',
  shell: false,
});

const output = `${result.stdout || ''}${result.stderr || ''}`;
assert.equal(
  result.status,
  0,
  `generated sdk-gen runtime verifier must succeed.\n${output}`,
);
assert.match(output, /verify-typescript-generated-sdk-gen-runtime passed/);

console.log('typescript generated sdk-gen runtime tests passed');
