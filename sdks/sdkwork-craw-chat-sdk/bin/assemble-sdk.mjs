#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { resolveSdkworkGeneratorRoot } from './sdk-paths.mjs';

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
  const generatorRoot = resolveSdkworkGeneratorRoot(workspaceRoot);
  const yamlPath = path.join(generatorRoot, 'node_modules', 'js-yaml', 'dist', 'js-yaml.mjs');

  if (!existsSync(yamlPath)) {
    fail(`Unable to locate js-yaml from generator workspace: ${yamlPath}`);
  }

  const yamlModule = await import(pathToFileURL(yamlPath).href);
  return yamlModule.default;
}

function readJson(filePath) {
  return JSON.parse(readFileSync(filePath, 'utf8'));
}

function readYaml(filePath, yaml) {
  return yaml.load(readFileSync(filePath, 'utf8'));
}

function readCargoManifest(filePath) {
  const source = readFileSync(filePath, 'utf8');
  const packageSection = readTomlSection(source, 'package');
  const libSection = readTomlSection(source, 'lib');

  return {
    name: readTomlString(packageSection, 'name'),
    version: readTomlString(packageSection, 'version'),
    description: readTomlString(packageSection, 'description'),
    libName: readTomlString(libSection, 'name'),
    libPath: readTomlString(libSection, 'path') || 'src/lib.rs',
  };
}

function readTomlSection(source, sectionName) {
  const header = `[${sectionName}]`;
  const startIndex = source.indexOf(header);
  if (startIndex < 0) {
    return '';
  }

  const afterHeader = source.slice(startIndex + header.length);
  const nextHeaderMatch = afterHeader.match(/\r?\n\[[^\r\n]+\]/);
  if (!nextHeaderMatch || typeof nextHeaderMatch.index !== 'number') {
    return afterHeader;
  }

  return afterHeader.slice(0, nextHeaderMatch.index);
}

function readTomlString(sectionSource, key) {
  if (!sectionSource) {
    return '';
  }

  const pattern = new RegExp(`^\\s*${key}\\s*=\\s*"([^"]*)"\\s*$`, 'm');
  const match = sectionSource.match(pattern);
  return match ? match[1] : '';
}

function readAuthorityMeta(authorityPath, yaml) {
  const document = readYaml(authorityPath, yaml);
  return {
    title: document?.info?.title || '',
    apiVersion: document?.info?.version || '',
    openapiVersion: document?.openapi || '',
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
    rust: path.join(
      workspaceRoot,
      'sdkwork-craw-chat-sdk-rust',
      'generated',
      'server-openapi',
      'Cargo.toml',
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

function renderLanguageAssembly(workspaceRoot, language, yaml) {
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
    rust: {
      workspace: 'sdkwork-craw-chat-sdk-rust',
      packages: [
        {
          layer: 'generated',
          path: ['generated', 'server-openapi'],
          manifest: ['generated', 'server-openapi', 'Cargo.toml'],
          readManifest(manifestPath) {
            const manifest = readCargoManifest(manifestPath);
            return {
              name: manifest.name || '',
              version: manifest.version || '',
              description: manifest.description || '',
              entrypoints: {
                library: manifest.libPath || 'src/lib.rs',
              },
            };
          },
        },
        {
          layer: 'composed',
          path: ['composed'],
          manifest: ['composed', 'Cargo.toml'],
          readManifest(manifestPath) {
            const manifest = readCargoManifest(manifestPath);
            return {
              name: manifest.name || '',
              version: manifest.version || '',
              description: manifest.description || '',
              entrypoints: {
                library: manifest.libPath || 'src/lib.rs',
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
    generatedPath: generatedPackage.packagePath,
    manifestPath: generatedPackage.manifestPath,
    name: generatedPackage.name,
    version: generatedPackage.version,
    description: generatedPackage.description,
    entrypoints: generatedPackage.entrypoints,
    packages,
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
const languageSet = new Set(args.languages);
for (const language of ['typescript', 'flutter', 'rust']) {
  const manifestPath = generatedManifestPath(workspaceRoot, language);
  if (manifestPath && existsSync(manifestPath)) {
    languageSet.add(language);
  }
}
const requestedLanguages = languageSet.size > 0 ? [...languageSet] : ['typescript', 'flutter'];
const languages = requestedLanguages.map((language) =>
  renderLanguageAssembly(workspaceRoot, language, yaml),
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
