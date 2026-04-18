import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';
import { pathToFileURL } from 'node:url';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const assembleScript = path.join(workspaceRoot, 'bin', 'assemble-sdk.mjs');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
const assembleSource = readFileSync(assembleScript, 'utf8');

async function runAssemble() {
  const originalWrite = process.stdout.write.bind(process.stdout);
  process.stdout.write = () => true;
  try {
    await import(`${pathToFileURL(assembleScript).href}?run=${Date.now()}-${Math.random()}`);
  } finally {
    process.stdout.write = originalWrite;
  }
}

function readAssembly() {
  return JSON.parse(readFileSync(assemblyPath, 'utf8'));
}

test('assemble-sdk imports the shared workspace assembly helper', () => {
  assert.match(
    assembleSource,
    /workspace-assembly-shared\.mjs/,
  );
});

test('assemble-sdk emits manifest-backed language package details', async () => {
  await runAssemble();
  const assembly = readAssembly();

  assert.equal(assembly.workspace, 'sdkwork-craw-chat-sdk-admin');
  assert.equal(assembly.authoritySpec, 'openapi/admin-control-plane.openapi.yaml');
  assert.equal(assembly.derivedSpec, 'openapi/admin-control-plane.sdkgen.yaml');
  assert.ok(Array.isArray(assembly.languages));

  const typescript = assembly.languages.find((entry) => entry.language === 'typescript');
  assert.ok(typescript, 'missing TypeScript assembly');
  assert.equal(
    typescript.manifestPath,
    'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi/package.json',
  );
  assert.equal(typescript.name, '@sdkwork/craw-chat-admin-backend-sdk');
  assert.ok(Array.isArray(typescript.packages));
  assert.deepEqual(
    typescript.packages.map((entry) => entry.layer),
    ['generated', 'composed'],
  );
  assert.equal(
    typescript.packages[1].name,
    '@sdkwork/craw-chat-admin-sdk',
  );
  assert.equal(typescript.entrypoints.types, './dist/index.d.ts');

  const flutter = assembly.languages.find((entry) => entry.language === 'flutter');
  assert.ok(flutter, 'missing Flutter assembly');
  assert.equal(
    flutter.manifestPath,
    'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi/pubspec.yaml',
  );
  assert.equal(flutter.name, 'craw_chat_admin_backend_sdk');
  assert.ok(Array.isArray(flutter.packages));
  assert.deepEqual(
    flutter.packages.map((entry) => entry.layer),
    ['generated', 'composed'],
  );
  assert.equal(flutter.packages[1].name, 'craw_chat_admin_sdk');
  assert.equal(flutter.entrypoints.library, 'lib/');
});

test('assemble-sdk preserves generatedAt when assembly content is unchanged', async () => {
  await runAssemble();
  const firstAssembly = readAssembly();

  await runAssemble();
  const secondAssembly = readAssembly();

  assert.equal(secondAssembly.generatedAt, firstAssembly.generatedAt);
});
