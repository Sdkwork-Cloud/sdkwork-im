#!/usr/bin/env node
import fs from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const repoRoot = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const expectedDependencyIds = [
  'sdkwork-appbase',
  'sdkwork-core',
  'sdkwork-drive',
  'sdkwork-ui',
  'sdkwork-rtc',
  'sdkwork-kernel',
  'sdkwork-aiot',
  'sdkwork-notary',
  'sdkwork-sdk-commons',
  'sdkwork-sdk-generator',
];
const sourceDependencyFiles = [
  'package.json',
  'Cargo.toml',
  '.github/workflows/im-commercial-gates.yml',
  '.github/workflows/package.yml',
  'apps/sdkwork-chat-pc/package.json',
  'apps/sdkwork-chat-pc/packages/sdkwork-clawchat-pc-core/package.json',
  'apps/sdkwork-chat-pc/pnpm-lock.yaml',
  'apps/sdkwork-chat-pc/pnpm-workspace.yaml',
  'apps/sdkwork-chat-pc/tsconfig.json',
  'apps/sdkwork-chat-pc/vite.config.ts',
  'crates/im-domain-core/Cargo.toml',
  'crates/im-platform-contracts/Cargo.toml',
  'services/local-minimal-node/Cargo.toml',
  'services/local-minimal-node/tests/commercial_gate_contract_test.rs',
  'services/local-minimal-node/tests/openapi_im_v3_contract_test.rs',
  'scripts/prepare-ci-dependencies.mjs',
  'scripts/run-local-minimal.mjs',
  'artifacts/releases/sync-sdk-release-catalog.mjs',
  'sdks/sdkwork-im-app-sdk/bin/verify-flutter-composed-workspace.mjs',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-flutter/composed/pubspec_overrides.yaml',
  'sdks/sdkwork-im-app-sdk/sdkwork-im-app-sdk-flutter/composed/pubspec.lock',
];
const activeDocumentationFiles = [
  'README.md',
  'sdks/README.md',
  'specs/README.md',
  '.sdkwork/README.md',
  'docs/部署/兼容矩阵与SDK-CLI-operator验证索引.md',
];
const retiredDependencyRoot = ['.sdkwork', 'dependencies'].join('/');
const retiredLocalScript = ['prepare-local', 'dependencies.mjs'].join('-');
const retiredDepsLocalPrefix = ['deps', 'local'].join(':');
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

function listFilesRecursive(rootDir) {
  const files = [];
  if (!fs.existsSync(rootDir)) {
    return files;
  }
  for (const entry of fs.readdirSync(rootDir, { withFileTypes: true })) {
    const entryPath = path.join(rootDir, entry.name);
    if (entry.isDirectory()) {
      files.push(...listFilesRecursive(entryPath));
      continue;
    }
    files.push(entryPath);
  }
  return files;
}

function slashPath(value) {
  return String(value).replaceAll('\\', '/');
}

function sdkworkSiblingDependencyIdsFromCargo(relativePath) {
  const text = readText(relativePath);
  return [...new Set(
    [...text.matchAll(/path\s*=\s*"[^"]*\.\.\/(sdkwork-[A-Za-z0-9-]+)(?:[\/\\]|")/g)]
      .map((match) => match[1]),
  )].sort();
}

function assertNoRetiredDependencyModel(relativePath) {
  const text = readText(relativePath);
  assert(!text.includes(retiredDependencyRoot), `${relativePath} must not reference the retired SDKWork dependency root`);
  assert(!text.includes(retiredLocalScript.replace(/\.mjs$/u, '')), `${relativePath} must not reference the retired local dependency script`);
  assert(!text.includes(retiredDepsLocalPrefix), `${relativePath} must not reference retired local dependency scripts`);
}

function assertPnpmWorkspaceOnlyProtocol(relativePath) {
  if (!relativePath.endsWith('package.json') || relativePath === 'package.json') {
    return;
  }
  const text = readText(relativePath);
  const linkMatches = [...text.matchAll(/['"](link:[^'"]+)['"]/g)];
  for (const match of linkMatches) {
    const specifier = match[1];
    assert(
      !specifier.includes('sdkwork-'),
      `${relativePath} must not use ${specifier}; SDKWork cross-workspace sources must use the workspace: protocol declared in pnpm-workspace.yaml packages:`,
    );
  }
}

function assertCargoWorkspaceOnlyProtocol(relativePath) {
  if (!relativePath.endsWith('Cargo.toml') || relativePath === 'Cargo.toml') {
    return;
  }
  const text = readText(relativePath);
  const pathMatches = [...text.matchAll(/path\s*=\s*"((?:\.\.\/)+sdkwork-[A-Za-z0-9-]+[^"]*)"/g)];
  for (const match of pathMatches) {
    const path = match[1];
    assert(
      false,
      `${relativePath} must not redeclare cross-workspace SDKWork source path "${path}"; declare it in root [workspace.dependencies] and consume with \`crate_name.workspace = true\``,
    );
  }
}

function assertSiblingDependencyPathsAreKnown(relativePath) {
  const text = readText(relativePath);
  const absolutePath = path.join(repoRoot, relativePath);
  const sourceDir = path.dirname(absolutePath);
  const matches = [...text.matchAll(/(?:\.\.\/|\.{2}\\)+(sdkwork-[A-Za-z0-9-]*)/g)];
  for (const match of matches) {
    const dependencyId = match[1];
    if (dependencyId === 'sdkwork-specs') {
      continue;
    }
    const resolvedTarget = path.resolve(sourceDir, match[0].replaceAll('\\', path.sep));
    const relativeToRepoRoot = path.relative(repoRoot, resolvedTarget);
    if (relativeToRepoRoot && !relativeToRepoRoot.startsWith('..') && !path.isAbsolute(relativeToRepoRoot)) {
      continue;
    }
    assert(
      expectedDependencyIds.includes(dependencyId),
      `${relativePath} uses undeclared SDKWork sibling dependency path ${match[0]}`,
    );
  }
}

function assertNativeDependencyFile(relativePath) {
  assertNoRetiredDependencyModel(relativePath);
  assertSiblingDependencyPathsAreKnown(relativePath);
  assertPnpmWorkspaceOnlyProtocol(relativePath);
  assertCargoWorkspaceOnlyProtocol(relativePath);
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

function assertNoLocalMaterializer() {
  const packageJson = readJson('package.json');
  assert(packageJson.scripts?.[[retiredDepsLocalPrefix, 'link'].join(':')] === undefined, 'package.json must not expose retired local link script');
  assert(packageJson.scripts?.[[retiredDepsLocalPrefix, 'check'].join(':')] === undefined, 'package.json must not expose retired local check script');
  assert(!readText('.gitignore').includes(retiredDependencyRoot), 'gitignore must not keep the retired SDKWork dependency ignore rule');
  assert(!fs.existsSync(path.join(repoRoot, 'scripts', retiredLocalScript)), 'retired local dependency script must not exist');
  assert(!fs.existsSync(path.join(repoRoot, ...retiredDependencyRoot.split('/'))), 'retired SDKWork dependency directory must not exist');
}

function assertCiMaterializer() {
  const materializer = readText('scripts/prepare-ci-dependencies.mjs');
  assert(materializer.includes('sdkwork.workflow.json'), 'prepare-ci-dependencies must read sdkwork.workflow.json');
  assert(materializer.includes("path.resolve(repoRoot, '..')"), 'prepare-ci-dependencies must use the workspace sibling repository root');
  assert(!materializer.includes(retiredDependencyRoot), 'prepare-ci-dependencies must not use the retired SDKWork dependency root');
  assert(materializer.includes('dependencies'), 'prepare-ci-dependencies must process dependency entries');
  assert(materializer.includes('tokenSecret'), 'prepare-ci-dependencies must honor dependency tokenSecret declarations');
  assert(!materializer.includes('const dependencyIds = ['), 'prepare-ci-dependencies must not duplicate a hard-coded dependency id list');

  const workflowYaml = readText('.github/workflows/im-commercial-gates.yml');
  assert(workflowYaml.includes('node scripts/prepare-ci-dependencies.mjs'), 'im commercial gates workflow must prepare SDKWork dependencies through the repository CI materializer');
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

function assertReleaseLifecycleDependencyGate() {
  const workflow = readJson('sdkwork.workflow.json');
  const buildLifecycleSource = (workflow.lifecycle?.build || [])
    .map((step) => step.run || '')
    .join('\n');
  assert(
    /pnpm\s+(?:run\s+)?check:dependency-management/u.test(buildLifecycleSource),
    'sdkwork.workflow.json build lifecycle must run pnpm check:dependency-management before release packaging so sdkwork-notary and sdkwork-drive app SDK dependency refs are verified in package jobs',
  );
}

function assertSharedGatewayFoundationIntegration() {
  const componentSpec = readJson('specs/component.spec.json');
  const foundationGateway = componentSpec.integration?.foundationApiGateway;

  assert(
    foundationGateway?.targetApplication === 'sdkwork-api-gateway',
    'specs/component.spec.json must declare sdkwork-api-gateway as the shared foundation API gateway target',
  );
  assert(
    foundationGateway?.targetMode === 'shared-gateway',
    'specs/component.spec.json foundationApiGateway.targetMode must be shared-gateway',
  );
  assert(
    foundationGateway?.commonSdkRootEnv === 'SDKWORK_CHAT_SERVER_API_BASE_URL',
    'specs/component.spec.json must use SDKWORK_CHAT_SERVER_API_BASE_URL as the server common SDK root',
  );
  assert(
    foundationGateway?.browserSdkRootEnv === 'VITE_CRAW_CHAT_APP_API_BASE_URL',
    'specs/component.spec.json must use VITE_CRAW_CHAT_APP_API_BASE_URL as the browser app-api gateway root',
  );
  assert(
    foundationGateway?.authority === 'cargo-workspace',
    'Craw Chat shared gateway integration must use Cargo workspace metadata as build authority',
  );
  assert(
    foundationGateway?.catalogPolicy === 'no-dedicated-gateway-catalog',
    'Craw Chat must not introduce a standalone gateway catalog',
  );
  assert(
    foundationGateway?.productApiPolicy === 'Craw Chat IM APIs remain product-owned SDKWork API surfaces',
    'Craw Chat component spec must keep IM APIs product-owned',
  );
  assert(
    foundationGateway?.migrationState === 'legacy-compatible',
    'Craw Chat local foundation aggregation must be marked as legacy-compatible during migration',
  );

  const compatibilityComponents = foundationGateway?.legacyCompatibilityComponents ?? [];
  for (const component of [
    'services/web-gateway',
    'crates/craw-chat-gateway-config',
    'services/local-minimal-node',
  ]) {
    assert(
      compatibilityComponents.includes(component),
      `Craw Chat gateway migration compatibility must name ${component}`,
    );
  }

  const directCargoDependencyIds = sdkworkSiblingDependencyIdsFromCargo('Cargo.toml');
  const declaredLegacyIds = (foundationGateway?.legacyDirectFoundationRuntimeDependencies ?? [])
    .map((dependency) => dependency.id)
    .sort();
  assert(
    JSON.stringify(directCargoDependencyIds) === JSON.stringify(declaredLegacyIds),
    `direct Cargo foundation dependencies must be declared as migration exceptions: Cargo=${directCargoDependencyIds.join(',')} spec=${declaredLegacyIds.join(',')}`,
  );

  for (const dependency of foundationGateway?.legacyDirectFoundationRuntimeDependencies ?? []) {
    assert(
      dependency.authority === 'Cargo.toml',
      `${dependency.id} migration exception must point back to Cargo.toml`,
    );
    assert(
      typeof dependency.migrationTarget === 'string'
        && dependency.migrationTarget.startsWith('sdkwork-api-gateway foundation-'),
      `${dependency.id} must name a sdkwork-api-gateway foundation-* migration target`,
    );
  }

  const dependencyApiSurfaces = componentSpec.contracts?.dependencyApiSurfaces ?? [];
  const sharedGatewaySurfaceIds = dependencyApiSurfaces
    .filter((surface) => surface.targetRuntimeIntegration?.gatewayApplication === 'sdkwork-api-gateway')
    .map((surface) => surface.apiAuthority)
    .sort();
  const expectedSharedGatewaySurfaceIds = [
    'sdkwork-appbase-app-api',
    'sdkwork-agent-app-api',
    'sdkwork-agent-backend-api',
    'sdkwork-agent-open-api',
    'sdkwork-aiot-app-api',
    'sdkwork-aiot-backend-api',
    'sdkwork-drive-app-api',
    'sdkwork-notary-app-api',
    'sdkwork-rtc-app-api',
    'sdkwork-rtc-backend-api',
  ].sort();
  assert(
    JSON.stringify(sharedGatewaySurfaceIds) === JSON.stringify(expectedSharedGatewaySurfaceIds),
    `component spec must declare the current shared-gateway dependency API surface targets, got ${sharedGatewaySurfaceIds.join(',')}`,
  );
  for (const surface of dependencyApiSurfaces) {
    assert(
      surface.targetRuntimeIntegration?.catalogPolicy === 'no-dedicated-gateway-catalog',
      `${surface.apiAuthority} must use existing Cargo/spec evidence instead of a standalone gateway catalog`,
    );
    assert(
      surface.currentCompatibility?.mode === 'legacy-web-gateway',
      `${surface.apiAuthority} must mark the current web-gateway aggregation as migration compatibility`,
    );
  }

  const forbiddenGatewayCatalogs = listFilesRecursive(path.join(repoRoot, 'specs'))
    .map((filePath) => slashPath(path.relative(repoRoot, filePath)))
    .filter((relativePath) =>
      /(^|\/)(sdkwork-api-gateway-catalog|api-gateway-catalog|gateway-catalog|foundation-api-catalog)\.(json|ya?ml|toml)$/iu.test(relativePath)
    );
  assert(
    forbiddenGatewayCatalogs.length === 0,
    `gateway integration must not add standalone gateway catalog files: ${forbiddenGatewayCatalogs.join(', ')}`,
  );
}

function assertDocumentation() {
  for (const relativePath of activeDocumentationFiles) {
    assertNativeDependencyFile(relativePath);
  }
  const specsReadme = readText('specs/README.md');
  assert(specsReadme.includes('../sdkwork-specs/DEPENDENCY_MANAGEMENT_SPEC.md'), 'specs/README.md must link DEPENDENCY_MANAGEMENT_SPEC.md via ../sdkwork-specs');
  assert(!specsReadme.includes('../../../specs/'), 'specs/README.md must not link the old ../../../specs standards path');
}

assertDependencyDeclaration();
assertNoLocalMaterializer();
assertCiMaterializer();
assertWorkflowRefs();
assertReleaseLifecycleDependencyGate();
assertSharedGatewayFoundationIntegration();
for (const relativePath of sourceDependencyFiles) {
  assertNativeDependencyFile(relativePath);
}
assertDocumentation();

if (failures.length > 0) {
  process.stderr.write(`Dependency management standard failed:\n${failures.map((failure) => `- ${failure}`).join('\n')}\n`);
  process.exit(1);
}

process.stdout.write('Dependency management standard passed\n');
