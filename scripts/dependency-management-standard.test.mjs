#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const expectedDependencyIds = [
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-ui',
  'sdkwork-rtc',
  'sdkwork-kernel',
  'sdkwork-aiot',
  'sdkwork-sdk-commons',
  'sdkwork-sdk-generator',
];
const sourceDependencyFiles = [
  'package.json',
  'Cargo.toml',
  '.github/workflows/package.yml',
  'apps/sdkwork-chat-pc/package.json',
  'apps/sdkwork-chat-pc/pnpm-workspace.yaml',
  'apps/sdkwork-chat-pc/tsconfig.json',
  'apps/sdkwork-chat-pc/vite.config.ts',
  'services/local-minimal-node/Cargo.toml',
  'scripts/run-local-minimal.mjs',
  'artifacts/releases/sync-sdk-release-catalog.mjs',
];
const activeDocumentationFiles = [
  'README.md',
  'specs/README.md',
  '.sdkwork/README.md',
  'docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md',
];
const failures = [];

function readText(relativePath) {
  const absolutePath = path.join(repoRoot, relativePath);
  if (!fs.existsSync(absolutePath)) {
    failures.push(`${relativePath} must exist`);
    return '';
  }
  return fs.readFileSync(absolutePath, 'utf8');
}

function readJson(relativePath) {
  const text = readText(relativePath);
  if (!text) {
    return {};
  }
  return JSON.parse(text);
}

function assert(condition, message) {
  if (!condition) {
    failures.push(message);
  }
}

function assertNoSiblingDependencyPaths(relativePath) {
  const text = readText(relativePath);
  const matches = [...text.matchAll(/(?:\.\.\/|\.{2}\\)+(sdkwork-(?!specs)[A-Za-z0-9-]*)/g)];
  for (const match of matches) {
    failures.push(`${relativePath} uses sibling dependency path ${match[0]}; use .sdkwork/dependencies/${match[1]}`);
  }
}

function assertDependencyDeclaration() {
  const workflow = readJson('sdkwork.workflow.json');
  assert(Array.isArray(workflow.dependencies), 'sdkwork.workflow.json must declare dependencies[]');
  const dependencyIds = new Set((workflow.dependencies || []).map((dependency) => dependency.id));
  for (const expectedId of expectedDependencyIds) {
    assert(dependencyIds.has(expectedId), `sdkwork.workflow.json must declare ${expectedId}`);
  }
  for (const dependency of workflow.dependencies || []) {
    assert(typeof dependency.id === 'string' && expectedDependencyIds.includes(dependency.id), `unexpected dependency id ${dependency.id}`);
    assert(/^Sdkwork-Cloud\/sdkwork-[a-z0-9-]+$/.test(dependency.repository || ''), `${dependency.id} must use owner/repo repository form`);
    assert(Boolean(dependency.ref || dependency.refInput), `${dependency.id} must declare ref or refInput`);
    assert(dependency.tokenSecret === 'SDKWORK_RELEASE_TOKEN', `${dependency.id} must use SDKWORK_RELEASE_TOKEN`);
    assert(!Object.hasOwn(dependency, 'path'), `${dependency.id} must omit dependencies[].path`);
  }
}

function assertLocalMaterializer() {
  const packageJson = readJson('package.json');
  assert(packageJson.scripts?.['deps:local:link'] === 'node scripts/prepare-local-dependencies.mjs --apply', 'package.json must expose deps:local:link');
  assert(packageJson.scripts?.['deps:local:check'] === 'node scripts/prepare-local-dependencies.mjs --check', 'package.json must expose deps:local:check');
  assert(readText('.gitignore').includes('/.sdkwork/dependencies/'), '.gitignore must ignore /.sdkwork/dependencies/');
  const materializer = readText('scripts/prepare-local-dependencies.mjs');
  assert(materializer.includes('sdkwork.workflow.json'), 'prepare-local-dependencies must read sdkwork.workflow.json');
  assert(materializer.includes('.sdkwork/dependencies'), 'prepare-local-dependencies must materialize .sdkwork/dependencies');
  assert(!materializer.includes('const dependencyIds = ['), 'prepare-local-dependencies must not duplicate a hard-coded dependency id list');
}

function assertWorkflowRefs() {
  const workflowYaml = readText('.github/workflows/package.yml');
  assert(!workflowYaml.includes("dependency_refs_json: '{}'"), 'package workflow must not pass an empty dependency_refs_json');
  for (const dependencyId of expectedDependencyIds) {
    const inputName = `${dependencyId.replaceAll('-', '_')}_ref`;
    const envName = dependencyId.replaceAll('-', '_').toUpperCase();
    assert(workflowYaml.includes(inputName), `.github/workflows/package.yml must expose ${inputName}`);
    assert(workflowYaml.includes(envName), `.github/workflows/package.yml dependency_refs_json must include ${envName}`);
  }
}

function assertDocumentation() {
  for (const relativePath of activeDocumentationFiles) {
    assertNoSiblingDependencyPaths(relativePath);
  }
  const specsReadme = readText('specs/README.md');
  assert(specsReadme.includes('../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md'), 'specs/README.md must link DEPENDENCY_MANAGEMENT_SPEC.md via ../sdkwork-specs');
  assert(!specsReadme.includes('../../../specs/'), 'specs/README.md must not link the old ../../../specs standards path');
}

assertDependencyDeclaration();
assertLocalMaterializer();
assertWorkflowRefs();
for (const relativePath of sourceDependencyFiles) {
  assertNoSiblingDependencyPaths(relativePath);
}
assertDocumentation();

if (failures.length > 0) {
  process.stderr.write(`Dependency management standard failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}\n`);
  process.exit(1);
}

process.stdout.write('Dependency management standard passed\n');
