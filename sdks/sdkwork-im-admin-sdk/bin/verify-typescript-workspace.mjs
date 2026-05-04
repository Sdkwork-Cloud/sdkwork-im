#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function read(workspaceRoot, relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

function expectedPackageTaskScript(task) {
  return `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;
}

export function verifyTypeScriptWorkspace(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = options.workspaceRoot || path.resolve(scriptDir, '..');
  const failures = [];
  const expectedGeneratedBuildScript = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';

  const generatedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  const composedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'package.json',
  );
  const composedSdkPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'src',
    'sdk.ts',
  );
  const composedIndexPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'src',
    'index.ts',
  );
  const composedTypesPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'src',
    'types.ts',
  );
  const composedGeneratedBackendTypesPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'src',
    'generated-backend-types.ts',
  );
  const composedPackageTaskPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'bin',
    'package-task.mjs',
  );
  const composedRunTscPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'bin',
    'run-tsc.mjs',
  );
  const generatorRuntimePath = path.join(
    workspaceRoot,
    'bin',
    'generator-runtime.mjs',
  );
  const npmRuntimePath = path.join(
    workspaceRoot,
    'bin',
    'npm-runtime.mjs',
  );
  const generatedBuildScriptPath = path.join(
    workspaceRoot,
    'bin',
    'build-typescript-generated-package.mjs',
  );
  const generatedVerifyScriptPath = path.join(
    workspaceRoot,
    'bin',
    'verify-typescript-generated-package.mjs',
  );
  const composedSmokeTestPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'test',
    'im-admin-sdk-client.test.mjs',
  );
  const publishCoreTestPath = path.join(
    workspaceRoot,
    'tests',
    'typescript-publish-core.test.mjs',
  );

  if (!existsSync(generatedPackagePath)) {
    failures.push('Missing generated TypeScript package.json.');
  } else {
    const generatedPackage = JSON.parse(readFileSync(generatedPackagePath, 'utf8'));
    if (generatedPackage.name !== '@sdkwork/im-admin-backend-sdk') {
      failures.push(
        `Generated TypeScript package name must be "@sdkwork/im-admin-backend-sdk", received "${generatedPackage.name}".`,
      );
    }
    if (generatedPackage.scripts?.build !== expectedGeneratedBuildScript) {
      failures.push('Generated TypeScript package must delegate build to the workspace-stable build script.');
    }
    if (generatedPackage.scripts?.prepublishOnly !== 'npm run build') {
      failures.push('Generated TypeScript package prepublishOnly must stay on "npm run build".');
    }
  }

  if (!existsSync(composedPackagePath)) {
    failures.push('Missing composed TypeScript package.json.');
  } else {
    const composedPackage = JSON.parse(readFileSync(composedPackagePath, 'utf8'));
    if (composedPackage.name !== '@sdkwork/im-admin-sdk') {
      failures.push(
        `Composed TypeScript package name must be "@sdkwork/im-admin-sdk", received "${composedPackage.name}".`,
      );
    }
    if (!String(composedPackage.dependencies?.['@sdkwork/im-admin-backend-sdk'] || '').includes('../generated/server-openapi')) {
      failures.push('Composed TypeScript package must depend on the local generated IM admin backend package.');
    }
    if (composedPackage.scripts?.typecheck !== expectedPackageTaskScript('typecheck')) {
      failures.push('Composed TypeScript package typecheck script must delegate to bin/package-task.mjs through npm-aware Node fallbacks.');
    }
    if (composedPackage.scripts?.build !== expectedPackageTaskScript('build')) {
      failures.push('Composed TypeScript package build script must delegate to bin/package-task.mjs through npm-aware Node fallbacks.');
    }
    if (composedPackage.scripts?.test !== expectedPackageTaskScript('test')) {
      failures.push('Composed TypeScript package test script must delegate to bin/package-task.mjs through npm-aware Node fallbacks.');
    }
    if (/sdkwork-sdk-generator/.test(JSON.stringify(composedPackage.scripts || {}))) {
      failures.push('Composed TypeScript package scripts must not hardcode sdkwork-sdk-generator-relative tool paths.');
    }
  }

  if (!existsSync(generatorRuntimePath)) {
    failures.push('Missing IM admin TypeScript generator runtime helper bin/generator-runtime.mjs.');
  }
  if (!existsSync(npmRuntimePath)) {
    failures.push('Missing IM admin npm runtime helper bin/npm-runtime.mjs.');
  }
  if (!existsSync(generatedBuildScriptPath)) {
    failures.push('Missing IM admin TypeScript generated-package build script.');
  }
  if (!existsSync(generatedVerifyScriptPath)) {
    failures.push('Missing IM admin TypeScript generated-package verification script.');
  }
  if (!existsSync(publishCoreTestPath)) {
    failures.push('Missing IM admin TypeScript publish-core regression test tests/typescript-publish-core.test.mjs.');
  }
  if (!existsSync(composedPackageTaskPath)) {
    failures.push('Missing IM admin composed TypeScript package task runner bin/package-task.mjs.');
  }
  if (!existsSync(composedRunTscPath)) {
    failures.push('Missing IM admin composed TypeScript run-tsc helper bin/run-tsc.mjs.');
  }

  if (!existsSync(composedSdkPath)) {
    failures.push('Missing composed src/sdk.ts.');
  } else {
    const composedSdkSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/src/sdk.ts',
    );
    if (!/export class ImAdminSdkClient/.test(composedSdkSource)) {
      failures.push('Composed src/sdk.ts must export ImAdminSdkClient.');
    }
    for (const domain of [
      'auth',
      'users',
      'marketing',
      'tenants',
      'access',
      'routing',
      'catalog',
      'usage',
      'billing',
      'operations',
      'storage',
    ]) {
      if (!new RegExp(`readonly ${domain}:`, 'm').test(composedSdkSource)) {
        failures.push(`ImAdminSdkClient must expose an "${domain}" domain.`);
      }
    }
    if (!/static async create/.test(composedSdkSource)) {
      failures.push('ImAdminSdkClient must expose a static async create factory.');
    }
  }

  if (!existsSync(composedIndexPath)) {
    failures.push('Missing composed src/index.ts.');
  } else {
    const composedIndexSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/src/index.ts',
    );
    if (!/ImAdminSdkClient/.test(composedIndexSource)) {
      failures.push('Composed src/index.ts must re-export ImAdminSdkClient.');
    }
  }

  const typeScriptWorkspaceReadmeSource = read(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript/README.md',
  );
  for (const requiredTerm of [
    'build-typescript-generated-package.mjs',
    'verify-typescript-generated-package.mjs',
    'composed/bin/package-task.mjs typecheck',
    'composed/bin/package-task.mjs build',
    'composed/bin/package-task.mjs test',
  ]) {
    if (!typeScriptWorkspaceReadmeSource.includes(requiredTerm)) {
      failures.push(`TypeScript workspace README must document ${requiredTerm}.`);
    }
  }
  if (typeScriptWorkspaceReadmeSource.includes('sdkwork-sdk-generator/node_modules/typescript/bin/tsc')) {
    failures.push('TypeScript workspace README must not document sdkwork-sdk-generator-relative TypeScript compiler paths.');
  }

  const composedReadmeSource = read(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript/composed/README.md',
  );
  for (const requiredTerm of [
    'package-task.mjs typecheck',
    'package-task.mjs build',
    'package-task.mjs test',
  ]) {
    if (!composedReadmeSource.includes(requiredTerm)) {
      failures.push(`Composed README must document ${requiredTerm}.`);
    }
  }
  if (composedReadmeSource.includes('sdkwork-sdk-generator/node_modules/typescript/bin/tsc')) {
    failures.push('Composed README must not document sdkwork-sdk-generator-relative TypeScript compiler paths.');
  }

  const composedTsconfigSource = read(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript/composed/tsconfig.build.json',
  );
  if (composedTsconfigSource.includes('generated/server-openapi/src/')) {
    failures.push('Composed tsconfig.build.json must not resolve @sdkwork/im-admin-backend-sdk to generated private source paths.');
  }
  if (!composedTsconfigSource.includes('generated/server-openapi/dist/index.d.ts')) {
    failures.push('Composed tsconfig.build.json must resolve @sdkwork/im-admin-backend-sdk to the generated dist type entrypoint.');
  }

  if (!existsSync(composedTypesPath)) {
    failures.push('Missing composed src/types.ts.');
  } else {
    const composedTypesSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/src/types.ts',
    );
    if (composedTypesSource.includes('generated/server-openapi/src/')) {
      failures.push('Composed src/types.ts must not import generated private source paths.');
    }
    if (!composedTypesSource.includes("@sdkwork/im-admin-backend-sdk")) {
      failures.push('Composed src/types.ts must import IM admin backend types from the generated package root.');
    }
  }

  if (!existsSync(composedGeneratedBackendTypesPath)) {
    failures.push('Missing composed src/generated-backend-types.ts.');
  } else {
    const composedGeneratedBackendTypesSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/src/generated-backend-types.ts',
    );
    if (composedGeneratedBackendTypesSource.includes('generated/server-openapi/src/')) {
      failures.push('Composed src/generated-backend-types.ts must not import generated private source paths.');
    }
    if (!composedGeneratedBackendTypesSource.includes("@sdkwork/im-admin-backend-sdk")) {
      failures.push('Composed src/generated-backend-types.ts must re-export types from the generated package root.');
    }
  }

  if (!existsSync(composedSmokeTestPath)) {
    failures.push('Missing composed smoke test test/im-admin-sdk-client.test.mjs.');
  }

  if (existsSync(composedPackageTaskPath)) {
    const composedPackageTaskSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/bin/package-task.mjs',
    );
    if (!/run-tsc\.mjs/.test(composedPackageTaskSource)) {
      failures.push('IM admin composed package-task.mjs must delegate TypeScript compilation through bin/run-tsc.mjs.');
    }
    if (/sdkwork-sdk-generator/.test(composedPackageTaskSource)) {
      failures.push('IM admin composed package-task.mjs must not hardcode sdkwork-sdk-generator-relative tool paths.');
    }
  }

  if (existsSync(composedRunTscPath)) {
    const composedRunTscSource = read(
      workspaceRoot,
      'sdkwork-im-admin-sdk-typescript/composed/bin/run-tsc.mjs',
    );
    if (!/generator-runtime\.mjs/.test(composedRunTscSource)) {
      failures.push('IM admin composed run-tsc.mjs must import ../../../bin/generator-runtime.mjs.');
    }
    if (!/resolveGeneratorModulePath/.test(composedRunTscSource)) {
      failures.push('IM admin composed run-tsc.mjs must resolve the TypeScript compiler through resolveGeneratorModulePath.');
    }
    if (/sdkwork-sdk-generator/.test(composedRunTscSource)) {
      failures.push('IM admin composed run-tsc.mjs must not hardcode sdkwork-sdk-generator-relative tool paths.');
    }
  }

  return failures;
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    stdio: 'inherit',
    shell: false,
    timeout: options.timeoutMs,
  });

  if (result.error) {
    throw new Error(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    throw new Error(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    throw new Error(`${options.step || command} terminated with signal ${result.signal}`);
  }
}

export function runTypeScriptWorkspaceVerification(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = options.workspaceRoot || path.resolve(scriptDir, '..');
  const packageTaskPath = path.join(
    workspaceRoot,
    'sdkwork-im-admin-sdk-typescript',
    'composed',
    'bin',
    'package-task.mjs',
  );
  const generatedBuildScriptPath = path.join(
    workspaceRoot,
    'bin',
    'build-typescript-generated-package.mjs',
  );
  const generatedVerifyScriptPath = path.join(
    workspaceRoot,
    'bin',
    'verify-typescript-generated-package.mjs',
  );
  const publishCoreTestPath = path.join(
    workspaceRoot,
    'tests',
    'typescript-publish-core.test.mjs',
  );

  run('node', [publishCoreTestPath], {
    cwd: workspaceRoot,
    step: 'typescript:publish-core',
  });
  run('node', [generatedBuildScriptPath], {
    cwd: workspaceRoot,
    step: 'typescript:generated-build',
  });
  run('node', [generatedVerifyScriptPath], {
    cwd: workspaceRoot,
    step: 'typescript:generated-package',
  });
  run('node', [packageTaskPath, 'typecheck'], {
    cwd: workspaceRoot,
    step: 'typescript:typecheck',
  });
  run('node', [packageTaskPath, 'build'], {
    cwd: workspaceRoot,
    step: 'typescript:build',
  });
  run('node', [packageTaskPath, 'test'], {
    cwd: workspaceRoot,
    step: 'typescript:smoke-test',
  });
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const failures = verifyTypeScriptWorkspace();
  if (failures.length > 0) {
    console.error('[sdkwork-im-admin-sdk] TypeScript workspace verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  try {
    runTypeScriptWorkspaceVerification();
  } catch (error) {
    console.error(`[sdkwork-im-admin-sdk] ${error.message}`);
    process.exit(1);
  }

  console.log('[sdkwork-im-admin-sdk] TypeScript workspace verification passed.');
}
