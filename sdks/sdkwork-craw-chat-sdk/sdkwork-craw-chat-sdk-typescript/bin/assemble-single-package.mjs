#!/usr/bin/env node
import {
  cpSync,
  existsSync,
  mkdirSync,
  readFileSync,
  readdirSync,
  rmSync,
  writeFileSync,
} from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const packageRoot = path.resolve(scriptDir, '..');
const generatedPackageRoot = path.join(packageRoot, 'generated', 'server-openapi');
const composedPackageRoot = path.join(packageRoot, 'composed');
const generatedSourceRoot = path.join(generatedPackageRoot, 'src');
const composedSourceRoot = path.join(composedPackageRoot, 'src');
const composedTestRoot = path.join(composedPackageRoot, 'test');
const rootSourceRoot = path.join(packageRoot, 'src');
const rootGeneratedSourceRoot = path.join(rootSourceRoot, 'generated');
const rootTestRoot = path.join(packageRoot, 'test');
const rootNodeModulesRoot = path.join(packageRoot, 'node_modules');
const sourceSdkCommonRoot = path.join(generatedPackageRoot, 'node_modules', '@sdkwork', 'sdk-common');
const targetSdkCommonRoot = path.join(rootNodeModulesRoot, '@sdkwork', 'sdk-common');

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function expectedPackageTaskScript(task) {
  return `call "%npm_node_execpath%" ./bin/package-task.mjs ${task} || "$npm_node_execpath" ./bin/package-task.mjs ${task} || node ./bin/package-task.mjs ${task}`;
}

function ensureExists(absolutePath, description) {
  if (!existsSync(absolutePath)) {
    fail(`Missing ${description}: ${absolutePath}`);
  }
}

function resetDirectory(absolutePath) {
  rmSync(absolutePath, { recursive: true, force: true });
  mkdirSync(absolutePath, { recursive: true });
}

function copyDirectory(sourceRoot, targetRoot) {
  ensureExists(sourceRoot, 'source directory');
  rmSync(targetRoot, { recursive: true, force: true });
  mkdirSync(path.dirname(targetRoot), { recursive: true });
  cpSync(sourceRoot, targetRoot, { recursive: true });
}

function collectFiles(rootDirectory) {
  const files = [];
  const queue = [rootDirectory];

  while (queue.length > 0) {
    const currentDirectory = queue.shift();
    for (const entry of readdirSync(currentDirectory, { withFileTypes: true })) {
      const absolutePath = path.join(currentDirectory, entry.name);
      if (entry.isDirectory()) {
        queue.push(absolutePath);
        continue;
      }
      if (entry.isFile()) {
        files.push(absolutePath);
      }
    }
  }

  return files;
}

function resolveTypeScriptSpecifier(specifier, filePath) {
  if (!specifier.startsWith('.')) {
    return specifier;
  }
  if (/\.(?:[cm]?js|[cm]?ts|d\.ts|json)$/i.test(specifier)) {
    return specifier;
  }

  const absoluteBase = path.resolve(path.dirname(filePath), specifier);
  if (existsSync(`${absoluteBase}.ts`) || existsSync(`${absoluteBase}.mts`)) {
    return `${specifier}.js`;
  }
  if (existsSync(path.join(absoluteBase, 'index.ts')) || existsSync(path.join(absoluteBase, 'index.mts'))) {
    const normalized = specifier.replace(/\/+$/, '');
    return `${normalized}/index.js`;
  }

  return specifier;
}

function rewriteRelativeSpecifiers(rootDirectory) {
  for (const absolutePath of collectFiles(rootDirectory)) {
    if (!absolutePath.endsWith('.ts') && !absolutePath.endsWith('.mts')) {
      continue;
    }

    const source = readFileSync(absolutePath, 'utf8');
    const rewritten = source
      .replace(/(\bfrom\s*['"])(\.{1,2}\/[^'"]+)(['"])/g, (_match, prefix, specifier, suffix) => {
        return `${prefix}${resolveTypeScriptSpecifier(specifier, absolutePath)}${suffix}`;
      })
      .replace(/(\bimport\s*['"])(\.{1,2}\/[^'"]+)(['"])/g, (_match, prefix, specifier, suffix) => {
        return `${prefix}${resolveTypeScriptSpecifier(specifier, absolutePath)}${suffix}`;
      });

    if (rewritten !== source) {
      writeFileSync(absolutePath, rewritten, 'utf8');
    }
  }
}

function patchGeneratedTypeBridge() {
  const bridgePath = path.join(rootSourceRoot, 'generated-backend-types.ts');
  ensureExists(bridgePath, 'root generated type bridge');
  const source = readFileSync(bridgePath, 'utf8')
    .replace(
      /\/\/ This bridge keeps generated type-path knowledge in one place[\s\S]+?generator-owned internals\./,
      '// This bridge keeps generated type-path knowledge in one place so the root SDK can expose generated contracts without leaking private source paths.',
    )
    .replaceAll("../../generated/server-openapi/src/types/index", "./generated/types/index.js")
    .replaceAll("../../generated/server-openapi/src/types/common", "./generated/types/common.js")
    .replaceAll("../../generated/server-openapi/src/types/string-map", "./generated/types/string-map.js")
    .replace(
      /\s*QueryParams,\s*SdkworkBackendConfig,\s*StringMap,\s*} from '@sdkwork\/craw-chat-backend-sdk';/,
      `\n} from './generated/types/index.js';\nimport type { QueryParams, SdkworkBackendConfig } from './generated/types/common.js';\nimport type { StringMap } from './generated/types/string-map.js';`,
    );
  writeFileSync(bridgePath, source, 'utf8');
}

function patchSdkContext() {
  const composedSdkContextPath = path.join(composedSourceRoot, 'sdk-context.ts');
  const sdkContextPath = path.join(rootSourceRoot, 'sdk-context.ts');
  ensureExists(composedSdkContextPath, 'composed sdk context');
  const source = readFileSync(composedSdkContextPath, 'utf8').replace(
    "import { createClient as createGeneratedClient } from '@sdkwork/craw-chat-backend-sdk';",
    "import { createClient as createGeneratedClient } from './generated/sdk.js';",
  );
  writeFileSync(sdkContextPath, source, 'utf8');
}

function patchSdkTypes() {
  const composedSdkPath = path.join(composedSourceRoot, 'sdk.ts');
  const sdkPath = path.join(rootSourceRoot, 'sdk.ts');
  ensureExists(composedSdkPath, 'composed sdk entrypoint');
  const source = readFileSync(composedSdkPath, 'utf8').replace(
    "import type { SdkworkBackendClient } from '@sdkwork/craw-chat-backend-sdk';",
    "import type { SdkworkBackendClient } from './generated/sdk.js';",
  );
  writeFileSync(sdkPath, source, 'utf8');
}

function patchRootIndex() {
  const indexPath = path.join(rootSourceRoot, 'index.ts');
  writeFileSync(
    indexPath,
    `export * from './errors.js';
export * from './sdk.js';
export * from './types.js';
export {
  SdkworkBackendClient,
  createClient as createGeneratedBackendClient,
} from './generated/sdk.js';
export * as generated from './generated/index.js';
`,
    'utf8',
  );
}

function patchGeneratedSdkAliases() {
  const generatedSdkPath = path.join(rootGeneratedSourceRoot, 'sdk.ts');
  const generatedIndexPath = path.join(rootGeneratedSourceRoot, 'index.ts');
  const generatedCommonTypesPath = path.join(rootGeneratedSourceRoot, 'types', 'common.ts');

  let generatedSdkSource = readFileSync(generatedSdkPath, 'utf8');
  if (!generatedSdkSource.includes('export { SdkworkBackendClient as CrawChatSdkClient };')) {
    generatedSdkSource = `${generatedSdkSource.trimEnd()}\n\nexport { SdkworkBackendClient as CrawChatSdkClient };\n`;
    writeFileSync(generatedSdkPath, generatedSdkSource, 'utf8');
  }

  let generatedIndexSource = readFileSync(generatedIndexPath, 'utf8');
  if (!generatedIndexSource.includes("export { CrawChatSdkClient } from './sdk.js';")) {
    generatedIndexSource = `${generatedIndexSource.trimEnd()}\nexport { CrawChatSdkClient } from './sdk.js';\nexport type { CrawChatSdkConfig } from './types/common.js';\n`;
    writeFileSync(generatedIndexPath, generatedIndexSource, 'utf8');
  }

  let generatedCommonTypesSource = readFileSync(generatedCommonTypesPath, 'utf8');
  if (!generatedCommonTypesSource.includes('export type CrawChatSdkConfig = SdkworkBackendConfig;')) {
    generatedCommonTypesSource = `${generatedCommonTypesSource.trimEnd()}\n\nexport type CrawChatSdkConfig = SdkworkBackendConfig;\n`;
    writeFileSync(generatedCommonTypesPath, generatedCommonTypesSource, 'utf8');
  }
}

function writeRootPackageManifest() {
  const generatedPackageJsonPath = path.join(generatedPackageRoot, 'package.json');
  const composedPackageJsonPath = path.join(composedPackageRoot, 'package.json');
  const generatedPackageJson = JSON.parse(readFileSync(generatedPackageJsonPath, 'utf8'));
  const composedPackageJson = JSON.parse(readFileSync(composedPackageJsonPath, 'utf8'));

  const manifest = {
    name: '@sdkwork/craw-chat-sdk',
    version: composedPackageJson.version || generatedPackageJson.version || '0.1.0',
    description: 'Unified Craw Chat SDK package with embedded OpenAPI-generated transport and handwritten realtime/business modules',
    type: 'module',
    main: './dist/index.js',
    module: './dist/index.js',
    types: './dist/index.d.ts',
    exports: {
      '.': {
        types: './dist/index.d.ts',
        import: './dist/index.js',
        default: './dist/index.js',
      },
    },
    sideEffects: false,
    files: [
      'dist',
    ],
    dependencies: {
      '@sdkwork/sdk-common': generatedPackageJson.dependencies?.['@sdkwork/sdk-common'] || '^1.0.2',
    },
    scripts: {
      assemble: expectedPackageTaskScript('assemble'),
      clean: expectedPackageTaskScript('clean'),
      typecheck: expectedPackageTaskScript('typecheck'),
      build: expectedPackageTaskScript('build'),
      smoke: expectedPackageTaskScript('smoke'),
      test: expectedPackageTaskScript('test'),
    },
  };

  writeFileSync(path.join(packageRoot, 'package.json'), `${JSON.stringify(manifest, null, 2)}\n`, 'utf8');
}

function copySdkCommonRuntime() {
  if (existsSync(targetSdkCommonRoot)) {
    return;
  }

  ensureExists(sourceSdkCommonRoot, 'sdk-common runtime package');
  mkdirSync(path.dirname(targetSdkCommonRoot), { recursive: true });
  cpSync(sourceSdkCommonRoot, targetSdkCommonRoot, { recursive: true });
}

resetDirectory(rootSourceRoot);
resetDirectory(rootTestRoot);
copyDirectory(composedSourceRoot, rootSourceRoot);
copyDirectory(generatedSourceRoot, rootGeneratedSourceRoot);
copyDirectory(composedTestRoot, rootTestRoot);
rewriteRelativeSpecifiers(rootGeneratedSourceRoot);
patchGeneratedTypeBridge();
patchSdkContext();
patchSdkTypes();
patchRootIndex();
patchGeneratedSdkAliases();
writeRootPackageManifest();
copySdkCommonRuntime();

console.log('[sdkwork-craw-chat-sdk] TypeScript single-package assembly completed.');
