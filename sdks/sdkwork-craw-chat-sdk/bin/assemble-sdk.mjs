#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadYamlFromGenerator } from './generator-runtime.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    languages: [],
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--language') {
      const value = (argv[index + 1] || '').trim();
      if (!value) {
        fail('Missing value for --language');
      }
      parsed.languages.push(value);
      index += 1;
      continue;
    }

    fail(`Unknown argument: ${current}`);
  }

  return parsed;
}

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  try {
    return await loadYamlFromGenerator(workspaceRoot);
  } catch (error) {
    fail(error instanceof Error ? error.message : String(error));
  }
}

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readYaml(filePath, yaml) {
  return yaml.load(readFileSync(filePath, 'utf8'));
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function readAuthorityMeta(authorityPath, yaml) {
  const document = readYaml(authorityPath, yaml);
  return {
    title: document?.info?.title || '',
    apiVersion: document?.info?.version || '',
    openapiVersion: document?.openapi || '',
  };
}

function readDiscoverySurface(derivedPath, yaml) {
  const document = readYaml(derivedPath, yaml);
  const discoverySurface = document?.['x-sdkwork-sdk-surface'];
  if (!discoverySurface || typeof discoverySurface !== 'object') {
    fail(
      `Derived sdkgen contract must expose x-sdkwork-sdk-surface: ${derivedPath}`,
    );
  }

  return cloneJson(discoverySurface);
}

const consumerSurfaceProfiles = {
  typescript: {
    primaryClient: 'CrawChatClient',
    domains: [
      { name: 'session', operationGroups: ['sessions'] },
      { name: 'presence', operationGroups: ['presence'] },
      { name: 'realtime', operationGroups: ['realtime'] },
      { name: 'devices', operationGroups: ['devices'] },
      { name: 'inbox', operationGroups: ['conversations'] },
      { name: 'conversations', operationGroups: ['conversations'] },
      { name: 'messages', operationGroups: ['conversations'] },
      { name: 'media', operationGroups: ['media'] },
      { name: 'streams', operationGroups: ['streams'] },
      { name: 'rtc', operationGroups: ['rtc'] },
    ],
  },
  flutter: {
    primaryClient: 'CrawChatClient',
    domains: [
      { name: 'session', operationGroups: ['sessions'] },
      { name: 'presence', operationGroups: ['presence'] },
      { name: 'realtime', operationGroups: ['realtime'] },
      { name: 'devices', operationGroups: ['devices'] },
      { name: 'inbox', operationGroups: ['conversations'] },
      { name: 'conversations', operationGroups: ['conversations'] },
      { name: 'messages', operationGroups: ['conversations'] },
      { name: 'media', operationGroups: ['media'] },
      { name: 'streams', operationGroups: ['streams'] },
      { name: 'rtc', operationGroups: ['rtc'] },
    ],
  },
};

function sortedUnique(values) {
  return [...new Set(values)].sort();
}

function buildConsumerSurface(language, discoverySurface, packages) {
  const profile = consumerSurfaceProfiles[language];
  if (!profile || !packages.some((entry) => entry.layer === 'composed')) {
    return null;
  }

  const availableOperationGroups = new Set(
    sortedUnique(
      (Array.isArray(discoverySurface.surfaceGroups) ? discoverySurface.surfaceGroups : [])
        .map((entry) => entry?.operationGroup)
        .filter((value) => typeof value === 'string' && value.length > 0),
    ),
  );
  const manualTransports = (Array.isArray(discoverySurface.manualTransports)
    ? discoverySurface.manualTransports
    : []
  )
    .map((entry) => ({
      operationGroup: entry?.operationGroup || '',
      protocol: entry?.protocol || '',
      path: entry?.path || '',
      serviceId: entry?.serviceId || '',
    }))
    .filter((entry) => entry.operationGroup && entry.protocol && entry.path);

  return {
    primaryClient: profile.primaryClient,
    operationGroups: [...availableOperationGroups],
    domains: profile.domains
      .filter((entry) =>
        entry.operationGroups.every((operationGroup) =>
          availableOperationGroups.has(operationGroup),
        ),
      )
      .map((entry) => cloneJson(entry)),
    manualTransports,
  };
}

function generatedManifestPath(workspaceRoot, language) {
  const map = {
    typescript: path.join(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-typescript',
      'generated',
      'server-openapi',
      'package.json',
    ),
    flutter: path.join(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-flutter',
      'generated',
      'server-openapi',
      'pubspec.yaml',
    ),
  };

  return map[language] || '';
}

function renderPackageAssembly(workspaceRoot, workspaceName, packageConfig, yaml) {
  const workspacePath = path.join(workspaceRoot, workspaceName);
  const manifestPath = path.join(workspacePath, ...packageConfig.manifest);
  if (!existsSync(manifestPath)) {
    return null;
  }

  return {
    layer: packageConfig.layer,
    packagePath: path.join(workspaceName, ...packageConfig.path).replaceAll('\\', '/'),
    manifestPath: path.relative(workspaceRoot, manifestPath).replaceAll('\\', '/'),
    ...packageConfig.readManifest(manifestPath, yaml),
  };
}

function renderLanguageAssembly(workspaceRoot, language, yaml, discoverySurface) {
  const map = {
    typescript: {
      workspace: 'sdkwork-craw-chat-sdk-typescript',
      packages: [
        {
          layer: 'generated',
          path: ['generated', 'server-openapi'],
          manifest: ['generated', 'server-openapi', 'package.json'],
          readManifest(manifestPath) {
            const manifest = readJson(manifestPath);
            return {
              name: manifest.name || '',
              version: manifest.version || '',
              description: manifest.description || '',
              entrypoints: {
                main: manifest.main || '',
                module: manifest.module || '',
                types: manifest.types || '',
              },
            };
          },
        },
        {
          layer: 'composed',
          path: ['composed'],
          manifest: ['composed', 'package.json'],
          readManifest(manifestPath) {
            const manifest = readJson(manifestPath);
            return {
              name: manifest.name || '',
              version: manifest.version || '',
              description: manifest.description || '',
              entrypoints: {
                main: manifest.main || '',
                module: manifest.module || '',
                types: manifest.types || '',
              },
            };
          },
        },
      ],
    },
    flutter: {
      workspace: 'sdkwork-craw-chat-sdk-flutter',
      packages: [
        {
          layer: 'generated',
          path: ['generated', 'server-openapi'],
          manifest: ['generated', 'server-openapi', 'pubspec.yaml'],
          readManifest(manifestPath) {
            const manifest = readYaml(manifestPath, yaml);
            return {
              name: manifest?.name || '',
              version: manifest?.version || '',
              description: manifest?.description || '',
              entrypoints: {
                library: 'lib/',
              },
            };
          },
        },
        {
          layer: 'composed',
          path: ['composed'],
          manifest: ['composed', 'pubspec.yaml'],
          readManifest(manifestPath) {
            const manifest = readYaml(manifestPath, yaml);
            return {
              name: manifest?.name || '',
              version: manifest?.version || '',
              description: manifest?.description || '',
              entrypoints: {
                library: 'lib/',
              },
            };
          },
        },
      ],
    },
  };

  const config = map[language];
  if (!config) {
    fail(`Unsupported language for assembly: ${language}`);
  }

  const workspacePath = path.join(workspaceRoot, config.workspace);
  const generatedManifestPath = path.join(workspacePath, ...config.packages[0].manifest);
  if (!existsSync(generatedManifestPath)) {
    fail(`Missing generated manifest for ${language}: ${generatedManifestPath}`);
  }

  const packages = config.packages
    .map((packageConfig) =>
      renderPackageAssembly(workspaceRoot, config.workspace, packageConfig, yaml),
    )
    .filter(Boolean);
  const generatedPackage = packages.find((entry) => entry.layer === 'generated');
  if (!generatedPackage) {
    fail(`Missing generated package assembly for ${language}`);
  }

  return {
    language,
    workspace: config.workspace,
    generationState: 'materialized',
    releaseState: 'not_published',
    generatedPath: generatedPackage.packagePath,
    manifestPath: generatedPackage.manifestPath,
    name: generatedPackage.name,
    version: generatedPackage.version,
    description: generatedPackage.description,
    entrypoints: generatedPackage.entrypoints,
    packages,
    consumerSurface: buildConsumerSurface(language, discoverySurface, packages),
  };
}

const args = parseArgs(process.argv.slice(2));
const yaml = await loadYaml();
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const authorityPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.openapi.yaml');
const derivedPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.sdkgen.yaml');
const flutterDerivedPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.flutter.sdkgen.yaml');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
const authority = readAuthorityMeta(authorityPath, yaml);
const discoverySurface = readDiscoverySurface(derivedPath, yaml);
const languageSet = new Set(args.languages);
for (const language of ['typescript', 'flutter']) {
  const manifestPath = generatedManifestPath(workspaceRoot, language);
  if (manifestPath && existsSync(manifestPath)) {
    languageSet.add(language);
  }
}
const requestedLanguages = languageSet.size > 0 ? [...languageSet] : ['typescript', 'flutter'];
const languages = requestedLanguages.map((language) =>
  renderLanguageAssembly(workspaceRoot, language, yaml, discoverySurface),
);
const assemblyBase = {
  workspace: 'sdkwork-craw-chat-sdk',
  title: authority.title,
  apiVersion: authority.apiVersion,
  openapiVersion: authority.openapiVersion,
  authoritySpec: path.relative(workspaceRoot, authorityPath).replaceAll('\\', '/'),
  derivedSpec: path.relative(workspaceRoot, derivedPath).replaceAll('\\', '/'),
  derivedSpecs: {
    default: path.relative(workspaceRoot, derivedPath).replaceAll('\\', '/'),
    flutter: path.relative(workspaceRoot, flutterDerivedPath).replaceAll('\\', '/'),
  },
  websocketTransport: {
    documented: true,
    generated: false,
  },
  discoverySurface,
  languages,
};

mkdirSync(path.dirname(assemblyPath), { recursive: true });
let currentAssembly = null;
if (existsSync(assemblyPath)) {
  currentAssembly = readJson(assemblyPath);
}

const currentComparable = currentAssembly
  ? {
      workspace: currentAssembly.workspace,
      title: currentAssembly.title,
      apiVersion: currentAssembly.apiVersion,
      openapiVersion: currentAssembly.openapiVersion,
      authoritySpec: currentAssembly.authoritySpec,
      derivedSpec: currentAssembly.derivedSpec,
      derivedSpecs: currentAssembly.derivedSpecs,
      websocketTransport: currentAssembly.websocketTransport,
      discoverySurface: currentAssembly.discoverySurface,
      languages: currentAssembly.languages,
    }
  : null;

const nextAssemblyObject = {
  ...assemblyBase,
  generatedAt:
    currentComparable && JSON.stringify(currentComparable) === JSON.stringify(assemblyBase)
      ? currentAssembly.generatedAt
      : new Date().toISOString(),
};

const nextAssembly = `${JSON.stringify(nextAssemblyObject, null, 2)}\n`;
if (!existsSync(assemblyPath) || readFileSync(assemblyPath, 'utf8') !== nextAssembly) {
  writeFileSync(assemblyPath, nextAssembly, 'utf8');
}

process.stdout.write(assemblyPath);
