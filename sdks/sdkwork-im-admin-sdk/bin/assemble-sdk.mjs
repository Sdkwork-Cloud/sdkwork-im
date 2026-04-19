#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-im-admin-sdk] ${message}`);
  process.exit(1);
}

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readYamlScalar(filePath, key) {
  const source = readFileSync(filePath, 'utf8');
  const match = source.match(new RegExp(`^${key}:\\s*(.+)$`, 'm'));
  return match ? match[1].trim().replace(/^['"]|['"]$/g, '') : '';
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

function renderPackageAssembly(workspaceRoot, workspaceName, packageConfig) {
  const workspacePath = path.join(workspaceRoot, workspaceName);
  const manifestPath = path.join(workspacePath, ...packageConfig.manifest);
  if (!existsSync(manifestPath)) {
    return null;
  }

  return {
    layer: packageConfig.layer,
    packagePath: path.join(workspaceName, ...packageConfig.path).replaceAll('\\', '/'),
    manifestPath: path.relative(workspaceRoot, manifestPath).replaceAll('\\', '/'),
    ...packageConfig.readManifest(manifestPath),
  };
}

function buildTypeScriptConsumerSurface(discoverySurface, packages) {
  if (!packages.some((entry) => entry.layer === 'composed')) {
    return null;
  }

  const availableOperationGroups = new Set(
    (Array.isArray(discoverySurface.surfaceGroups) ? discoverySurface.surfaceGroups : [])
      .map((entry) => entry?.operationGroup)
      .filter((value) => typeof value === 'string' && value.length > 0),
  );
  const domains = [...availableOperationGroups]
    .sort()
    .map((operationGroup) => ({
      name: operationGroup,
      operationGroups: [operationGroup],
    }));

  return {
    primaryClient: 'ImAdminSdkClient',
    operationGroups: [...availableOperationGroups].sort(),
    domains,
    manualTransports: [],
  };
}

function buildFlutterConsumerSurface(discoverySurface, packages) {
  if (!packages.some((entry) => entry.layer === 'composed')) {
    return null;
  }

  const availableOperationGroups = new Set(
    (Array.isArray(discoverySurface.surfaceGroups) ? discoverySurface.surfaceGroups : [])
      .map((entry) => entry?.operationGroup)
      .filter((value) => typeof value === 'string' && value.length > 0),
  );
  const domains = [...availableOperationGroups]
    .sort()
    .map((operationGroup) => ({
      name: operationGroup,
      operationGroups: [operationGroup],
    }));

  return {
    primaryClient: 'ImAdminSdkClient',
    operationGroups: [...availableOperationGroups].sort(),
    domains,
    manualTransports: [],
  };
}

function renderLanguageAssembly(workspaceRoot, discoverySurface, language) {
  if (language === 'flutter') {
    const workspaceName = 'sdkwork-im-admin-sdk-flutter';
    const packages = [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'pubspec.yaml'],
        readManifest(manifestPath) {
          return {
            name: readYamlScalar(manifestPath, 'name'),
            version: readYamlScalar(manifestPath, 'version'),
            description: readYamlScalar(manifestPath, 'description'),
            entrypoints: {
              library: 'lib/im_admin_backend_sdk.dart',
            },
          };
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'pubspec.yaml'],
        readManifest(manifestPath) {
          return {
            name: readYamlScalar(manifestPath, 'name'),
            version: readYamlScalar(manifestPath, 'version'),
            description: readYamlScalar(manifestPath, 'description'),
            entrypoints: {
              library: 'lib/im_admin_sdk.dart',
            },
          };
        },
      },
    ]
      .map((packageConfig) => renderPackageAssembly(workspaceRoot, workspaceName, packageConfig))
      .filter(Boolean);

    const generatedPackage = packages.find((entry) => entry.layer === 'generated');
    return {
      language,
      workspace: workspaceName,
      generationState: generatedPackage ? 'materialized' : 'template_only_pending_generation',
      releaseState: 'not_published',
      generatedPath: generatedPackage?.packagePath || null,
      manifestPath: generatedPackage?.manifestPath || null,
      name: generatedPackage?.name || '',
      version: generatedPackage?.version || '',
      description: generatedPackage?.description || '',
      entrypoints: generatedPackage?.entrypoints || null,
      packages,
      consumerSurface: buildFlutterConsumerSurface(discoverySurface, packages),
    };
  }

  const workspaceName = 'sdkwork-im-admin-sdk-typescript';
  const packages = [
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
  ]
    .map((packageConfig) => renderPackageAssembly(workspaceRoot, workspaceName, packageConfig))
    .filter(Boolean);

  const generatedPackage = packages.find((entry) => entry.layer === 'generated');
  return {
    language,
    workspace: workspaceName,
    generationState: generatedPackage ? 'materialized' : 'template_only_pending_generation',
    releaseState: 'not_published',
    generatedPath: generatedPackage?.packagePath || null,
    manifestPath: generatedPackage?.manifestPath || null,
    name: generatedPackage?.name || '',
    version: generatedPackage?.version || '',
    description: generatedPackage?.description || '',
    entrypoints: generatedPackage?.entrypoints || null,
    packages,
    consumerSurface: buildTypeScriptConsumerSurface(discoverySurface, packages),
  };
}

export function assembleSdk(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = options.workspaceRoot || path.resolve(scriptDir, '..');
  const authorityPath = path.join(workspaceRoot, 'openapi', 'im-admin.openapi.json');
  const derivedPath = path.join(workspaceRoot, 'openapi', 'im-admin.sdkgen.json');
  const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');

  if (!existsSync(authorityPath)) {
    fail(`Missing authority contract: ${authorityPath}`);
  }
  if (!existsSync(derivedPath)) {
    fail(`Missing derived sdkgen contract: ${derivedPath}`);
  }

  const authority = readJson(authorityPath);
  const derived = readJson(derivedPath);
  const discoverySurface = derived?.['x-sdkwork-sdk-surface'];

  if (!discoverySurface || typeof discoverySurface !== 'object') {
    fail(`Derived sdkgen contract must expose x-sdkwork-sdk-surface: ${derivedPath}`);
  }

  const languageEntries = ['typescript', 'flutter']
    .filter((language) =>
      existsSync(
        path.join(
          workspaceRoot,
          language === 'typescript'
            ? 'sdkwork-im-admin-sdk-typescript'
            : 'sdkwork-im-admin-sdk-flutter',
        ),
      ),
    )
    .map((language) => renderLanguageAssembly(workspaceRoot, discoverySurface, language));

  const assemblyBase = {
    workspace: 'sdkwork-im-admin-sdk',
    title: authority?.info?.title || '',
    apiVersion: authority?.info?.version || '',
    openapiVersion: authority?.openapi || '',
    authoritySpec: path.relative(workspaceRoot, authorityPath).replaceAll('\\', '/'),
    derivedSpec: path.relative(workspaceRoot, derivedPath).replaceAll('\\', '/'),
    derivedSpecs: {
      default: path.relative(workspaceRoot, derivedPath).replaceAll('\\', '/'),
    },
    discoverySurface: cloneJson(discoverySurface),
    languages: languageEntries,
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

  return assemblyPath;
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  console.log(assembleSdk());
}
