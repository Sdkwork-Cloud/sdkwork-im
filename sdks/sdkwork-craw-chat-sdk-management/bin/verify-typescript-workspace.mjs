#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { resolveGeneratorModulePath } from './generator-runtime.mjs';

function read(workspaceRoot, relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
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
    'sdkwork-craw-chat-sdk-management-typescript',
    'generated',
    'server-openapi',
    'package.json',
  );
  const composedPackagePath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'package.json',
  );
  const composedSdkPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'src',
    'sdk.ts',
  );
  const composedIndexPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'src',
    'index.ts',
  );
  const composedTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'src',
    'types.ts',
  );
  const composedGeneratedBackendTypesPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'src',
    'generated-backend-types.ts',
  );
  const generatorRuntimePath = path.join(
    workspaceRoot,
    'bin',
    'generator-runtime.mjs',
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
  const composedShimPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'src',
    'shims-sdk-common.d.ts',
  );
  const composedSmokeTestPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'test',
    'craw-chat-sdk-management-client.test.mjs',
  );

  if (!existsSync(generatedPackagePath)) {
    failures.push('Missing generated TypeScript package.json.');
  } else {
    const generatedPackage = JSON.parse(readFileSync(generatedPackagePath, 'utf8'));
    if (generatedPackage.name !== '@sdkwork/craw-chat-management-backend-sdk') {
      failures.push(
        `Generated TypeScript package name must be "@sdkwork/craw-chat-management-backend-sdk", received "${generatedPackage.name}".`,
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
    if (composedPackage.name !== '@sdkwork/craw-chat-sdk-management') {
      failures.push(
        `Composed TypeScript package name must be "@sdkwork/craw-chat-sdk-management", received "${composedPackage.name}".`,
      );
    }
    if (!String(composedPackage.dependencies?.['@sdkwork/craw-chat-management-backend-sdk'] || '').includes('../generated/server-openapi')) {
      failures.push('Composed TypeScript package must depend on the local generated management backend package.');
    }
    if (!String(composedPackage.scripts?.typecheck || '').includes('tsc -p tsconfig.build.json --noEmit')) {
      failures.push('Composed TypeScript package must expose a typecheck script backed by tsconfig.build.json.');
    }
    if (!String(composedPackage.scripts?.build || '').includes('tsc -p tsconfig.build.json')) {
      failures.push('Composed TypeScript package must expose a build script backed by tsconfig.build.json.');
    }
    if (!String(composedPackage.scripts?.test || '').includes('./test/craw-chat-sdk-management-client.test.mjs')) {
      failures.push('Composed TypeScript package must expose a smoke-test script.');
    }
  }

  if (!existsSync(generatorRuntimePath)) {
    failures.push('Missing management TypeScript generator runtime helper bin/generator-runtime.mjs.');
  }
  if (!existsSync(generatedBuildScriptPath)) {
    failures.push('Missing management TypeScript generated-package build script.');
  }
  if (!existsSync(generatedVerifyScriptPath)) {
    failures.push('Missing management TypeScript generated-package verification script.');
  }

  if (!existsSync(composedSdkPath)) {
    failures.push('Missing composed src/sdk.ts.');
  } else {
    const composedSdkSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/composed/src/sdk.ts',
    );
    if (!/export class CrawChatSdkManagementClient/.test(composedSdkSource)) {
      failures.push('Composed src/sdk.ts must export CrawChatSdkManagementClient.');
    }
    for (const domain of ['auth', 'users', 'tenants', 'access', 'catalog', 'operations']) {
      if (!new RegExp(`readonly ${domain}:`, 'm').test(composedSdkSource)) {
        failures.push(`CrawChatSdkManagementClient must expose an "${domain}" domain.`);
      }
    }
    if (!/static async create/.test(composedSdkSource)) {
      failures.push('CrawChatSdkManagementClient must expose a static async create factory.');
    }
  }

  if (!existsSync(composedIndexPath)) {
    failures.push('Missing composed src/index.ts.');
  } else {
    const composedIndexSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/composed/src/index.ts',
    );
    if (!/CrawChatSdkManagementClient/.test(composedIndexSource)) {
      failures.push('Composed src/index.ts must re-export CrawChatSdkManagementClient.');
    }
  }

  if (!existsSync(composedTypesPath)) {
    failures.push('Missing composed src/types.ts.');
  } else {
    const composedTypesSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/composed/src/types.ts',
    );
    if (composedTypesSource.includes('generated/server-openapi/src/')) {
      failures.push('Composed src/types.ts must not import generated private source paths.');
    }
    if (!composedTypesSource.includes("@sdkwork/craw-chat-management-backend-sdk")) {
      failures.push('Composed src/types.ts must import SdkworkBackendClient from the generated package root.');
    }
  }

  if (!existsSync(composedGeneratedBackendTypesPath)) {
    failures.push('Missing composed src/generated-backend-types.ts.');
  } else {
    const composedGeneratedBackendTypesSource = read(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-management-typescript/composed/src/generated-backend-types.ts',
    );
    if (composedGeneratedBackendTypesSource.includes('generated/server-openapi/src/')) {
      failures.push('Composed src/generated-backend-types.ts must not import generated private source paths.');
    }
    if (!composedGeneratedBackendTypesSource.includes("@sdkwork/craw-chat-management-backend-sdk")) {
      failures.push('Composed src/generated-backend-types.ts must re-export types from the generated package root.');
    }
  }

  if (!existsSync(composedShimPath)) {
    failures.push('Missing composed src/shims-sdk-common.d.ts.');
  }

  if (!existsSync(composedSmokeTestPath)) {
    failures.push('Missing composed smoke test test/craw-chat-sdk-management-client.test.mjs.');
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
  const tscPath = resolveGeneratorModulePath(workspaceRoot, 'typescript', 'bin', 'tsc');
  const tsconfigPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'tsconfig.build.json',
  );
  const cleanDistPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'bin',
    'clean-dist.mjs',
  );
  const smokeTestPath = path.join(
    workspaceRoot,
    'sdkwork-craw-chat-sdk-management-typescript',
    'composed',
    'test',
    'craw-chat-sdk-management-client.test.mjs',
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

  run('node', [generatedBuildScriptPath], {
    cwd: workspaceRoot,
    step: 'typescript:generated-build',
  });
  run('node', [generatedVerifyScriptPath], {
    cwd: workspaceRoot,
    step: 'typescript:generated-package',
  });
  run('node', [tscPath, '-p', tsconfigPath, '--noEmit'], {
    cwd: workspaceRoot,
    step: 'typescript:typecheck',
  });
  run('node', [tscPath, '-p', tsconfigPath], {
    cwd: workspaceRoot,
    step: 'typescript:build',
  });
  run('node', [cleanDistPath], {
    cwd: workspaceRoot,
    step: 'typescript:clean-dist',
  });
  run('node', [smokeTestPath], {
    cwd: workspaceRoot,
    step: 'typescript:smoke-test',
  });
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const failures = verifyTypeScriptWorkspace();
  if (failures.length > 0) {
    console.error('[sdkwork-craw-chat-sdk-management] TypeScript workspace verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  try {
    runTypeScriptWorkspaceVerification();
  } catch (error) {
    console.error(`[sdkwork-craw-chat-sdk-management] ${error.message}`);
    process.exit(1);
  }

  console.log('[sdkwork-craw-chat-sdk-management] TypeScript workspace verification passed.');
}
