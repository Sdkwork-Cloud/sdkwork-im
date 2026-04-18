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

function readAuthorityMeta(authorityPath, yaml) {
  const document = readYaml(authorityPath, yaml);
  return {
    title: document?.info?.title || '',
    apiVersion: document?.info?.version || '',
    openapiVersion: document?.openapi || '',
  };
}

const officialLanguages = [
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
];

function createJsonManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const manifest = readJson(manifestPath);
    return {
      name: manifest.name || '',
      version: manifest.version || '',
      description: manifest.description || '',
      entrypoints,
    };
  };
}

function createYamlManifestReader(entrypoints) {
  return function readManifest(manifestPath, yaml) {
    const manifest = readYaml(manifestPath, yaml);
    return {
      name: manifest?.name || '',
      version: manifest?.version || '',
      description: manifest?.description || '',
      entrypoints,
    };
  };
}

function readText(filePath) {
  return readFileSync(filePath, 'utf8');
}

function findMatch(source, pattern, index = 1) {
  const match = source.match(pattern);
  return match?.[index]?.trim() || '';
}

function findTomlSection(source, sectionName) {
  const escapedName = sectionName.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const headerPattern = new RegExp(`^\\[${escapedName}\\]\\s*$`, 'm');
  const headerMatch = headerPattern.exec(source);
  if (!headerMatch) {
    return '';
  }

  const sectionStart = headerMatch.index + headerMatch[0].length;
  const remaining = source.slice(sectionStart);
  const nextHeaderMatch = /^\[[^\]\r\n]+\]\s*$/m.exec(remaining);
  const sectionEnd = nextHeaderMatch ? sectionStart + nextHeaderMatch.index : source.length;
  return source.slice(sectionStart, sectionEnd).trim();
}

function createCargoManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    const packageSection = findTomlSection(raw, 'package');
    return {
      name: findMatch(packageSection, /^\s*name\s*=\s*"([^"]+)"/m),
      version: findMatch(packageSection, /^\s*version\s*=\s*"([^"]+)"/m),
      description: findMatch(packageSection, /^\s*description\s*=\s*"([^"]+)"/m),
      entrypoints,
    };
  };
}

function createPomManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    const groupId = findMatch(raw, /<groupId>([^<]+)<\/groupId>/);
    const artifactId = findMatch(raw, /<artifactId>([^<]+)<\/artifactId>/);
    return {
      name: groupId && artifactId ? `${groupId}:${artifactId}` : artifactId,
      version: findMatch(raw, /<version>([^<]+)<\/version>/),
      description: findMatch(raw, /<description>([^<]+)<\/description>/),
      entrypoints,
    };
  };
}

function createCsprojManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    return {
      name: findMatch(raw, /<PackageId>([^<]+)<\/PackageId>/) || findMatch(raw, /<AssemblyName>([^<]+)<\/AssemblyName>/),
      version: findMatch(raw, /<Version>([^<]+)<\/Version>/),
      description: findMatch(raw, /<Description>([^<]+)<\/Description>/),
      entrypoints,
    };
  };
}

function createSwiftPackageManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    return {
      name: findMatch(raw, /name:\s*"([^"]+)"/),
      version: '',
      description: '',
      entrypoints,
    };
  };
}

function createGradleManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    const groupId = findMatch(raw, /^\s*group\s*=\s*"([^"]+)"/m);
    const artifactId = findMatch(raw, /archiveBaseName\.set\("([^"]+)"\)/);
    return {
      name: groupId && artifactId ? `${groupId}:${artifactId}` : artifactId,
      version: findMatch(raw, /^\s*version\s*=\s*"([^"]+)"/m),
      description: '',
      entrypoints,
    };
  };
}

function createGoModManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    return {
      name: findMatch(raw, /^\s*module\s+([^\s]+)\s*$/m),
      version: '',
      description: '',
      entrypoints,
    };
  };
}

function createPyprojectManifestReader(entrypoints) {
  return function readManifest(manifestPath) {
    const raw = readText(manifestPath);
    const projectSection = findTomlSection(raw, 'project');
    return {
      name: findMatch(projectSection, /^\s*name\s*=\s*"([^"]+)"/m),
      version: findMatch(projectSection, /^\s*version\s*=\s*"([^"]+)"/m),
      description: findMatch(projectSection, /^\s*description\s*=\s*"([^"]+)"/m),
      entrypoints,
    };
  };
}

const languageConfigs = {
  typescript: {
    workspace: 'sdkwork-craw-chat-sdk-typescript',
    maturityTier: 'tier-a',
    primaryClient: 'CrawChatSdkClient',
    consumerLayer: 'root',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'package.json'],
        readManifest: createJsonManifestReader({
          main: './dist/index.cjs',
          module: './dist/index.js',
          types: './dist/index.d.ts',
        }),
        placeholder: {
          name: '@sdkwork/craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned TypeScript transport boundary for Craw Chat',
          entrypoints: {
            main: './dist/index.cjs',
            module: './dist/index.js',
            types: './dist/index.d.ts',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'package.json'],
        readManifest: createJsonManifestReader({
          main: './dist/index.js',
          module: './dist/index.js',
          types: './dist/index.d.ts',
        }),
        placeholder: {
          name: '@sdkwork/craw-chat-sdk',
          version: '',
          description: 'Manual-owned TypeScript semantic SDK boundary for Craw Chat',
          entrypoints: {
            main: './dist/index.js',
            module: './dist/index.js',
            types: './dist/index.d.ts',
          },
        },
      },
      {
        layer: 'root',
        path: [],
        manifest: ['package.json'],
        readManifest: createJsonManifestReader({
          main: './dist/index.js',
          module: './dist/index.js',
          types: './dist/index.d.ts',
        }),
        placeholder: {
          name: '@sdkwork/craw-chat-sdk',
          version: '',
          description: 'Published TypeScript single-package SDK for Craw Chat',
          entrypoints: {
            main: './dist/index.js',
            module: './dist/index.js',
            types: './dist/index.d.ts',
          },
        },
      },
    ],
  },
  flutter: {
    workspace: 'sdkwork-craw-chat-sdk-flutter',
    maturityTier: 'tier-a',
    primaryClient: 'CrawChatClient',
    consumerLayer: 'composed',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'pubspec.yaml'],
        readManifest: createYamlManifestReader({
          library: 'lib/',
        }),
        placeholder: {
          name: 'backend_sdk',
          version: '',
          description: 'Generator-owned Flutter transport boundary for Craw Chat',
          entrypoints: {
            library: 'lib/',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'pubspec.yaml'],
        readManifest: createYamlManifestReader({
          library: 'lib/',
        }),
        placeholder: {
          name: 'craw_chat_sdk',
          version: '',
          description: 'Manual-owned Flutter semantic SDK boundary for Craw Chat',
          entrypoints: {
            library: 'lib/',
          },
        },
      },
    ],
  },
  rust: {
    workspace: 'sdkwork-craw-chat-sdk-rust',
    maturityTier: 'tier-a',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'Cargo.toml'],
        readManifest: createCargoManifestReader({
          library: 'src/lib.rs',
        }),
        placeholder: {
          name: 'sdkwork-craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned Rust transport boundary for Craw Chat',
          entrypoints: {
            library: 'src/lib.rs',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'Cargo.toml'],
        readManifest: createCargoManifestReader({
          library: 'src/lib.rs',
        }),
        placeholder: {
          name: 'craw-chat-sdk',
          version: '',
          description: 'Manual-owned Rust semantic SDK boundary for Craw Chat',
          entrypoints: {
            library: 'src/lib.rs',
          },
        },
      },
    ],
  },
  java: {
    workspace: 'sdkwork-craw-chat-sdk-java',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'pom.xml'],
        readManifest: createPomManifestReader({
          sourceRoot: 'src/main/java/',
        }),
        placeholder: {
          name: 'com.sdkwork:craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned Java transport boundary for Craw Chat',
          entrypoints: {
            sourceRoot: 'src/main/java/',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'pom.xml'],
        readManifest: createPomManifestReader({
          sourceRoot: 'src/main/java/',
        }),
        placeholder: {
          name: 'com.sdkwork:craw-chat-sdk',
          version: '',
          description: 'Manual-owned Java semantic SDK boundary for Craw Chat',
          entrypoints: {
            sourceRoot: 'src/main/java/',
          },
        },
      },
    ],
  },
  csharp: {
    workspace: 'sdkwork-craw-chat-sdk-csharp',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'Sdkwork.CrawChat.BackendSdk.csproj'],
        readManifest: createCsprojManifestReader({
          sourceRoot: 'src/',
        }),
        placeholder: {
          name: 'Sdkwork.CrawChat.BackendSdk',
          version: '',
          description: 'Generator-owned C# transport boundary for Craw Chat',
          entrypoints: {
            sourceRoot: 'src/',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'CrawChatSdk.csproj'],
        readManifest: createCsprojManifestReader({
          sourceRoot: 'src/',
        }),
        placeholder: {
          name: 'Sdkwork.CrawChat.Sdk',
          version: '',
          description: 'Manual-owned C# semantic SDK boundary for Craw Chat',
          entrypoints: {
            sourceRoot: 'src/',
          },
        },
      },
    ],
  },
  swift: {
    workspace: 'sdkwork-craw-chat-sdk-swift',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'Package.swift'],
        readManifest: createSwiftPackageManifestReader({
          packageManifest: 'Package.swift',
        }),
        placeholder: {
          name: 'CrawChatBackendSdk',
          version: '',
          description: 'Generator-owned Swift transport boundary for Craw Chat',
          entrypoints: {
            packageManifest: 'Package.swift',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'Package.swift'],
        readManifest: createSwiftPackageManifestReader({
          packageManifest: 'Package.swift',
        }),
        placeholder: {
          name: 'CrawChatSdk',
          version: '',
          description: 'Manual-owned Swift semantic SDK boundary for Craw Chat',
          entrypoints: {
            packageManifest: 'Package.swift',
          },
        },
      },
    ],
  },
  kotlin: {
    workspace: 'sdkwork-craw-chat-sdk-kotlin',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'build.gradle.kts'],
        readManifest: createGradleManifestReader({
          buildFile: 'build.gradle.kts',
        }),
        placeholder: {
          name: 'com.sdkwork:craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned Kotlin transport boundary for Craw Chat',
          entrypoints: {
            buildFile: 'build.gradle.kts',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'build.gradle.kts'],
        readManifest: createGradleManifestReader({
          buildFile: 'build.gradle.kts',
        }),
        placeholder: {
          name: 'com.sdkwork:craw-chat-sdk',
          version: '',
          description: 'Manual-owned Kotlin semantic SDK boundary for Craw Chat',
          entrypoints: {
            buildFile: 'build.gradle.kts',
          },
        },
      },
    ],
  },
  go: {
    workspace: 'sdkwork-craw-chat-sdk-go',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'go.mod'],
        readManifest: createGoModManifestReader({
          moduleRoot: '.',
        }),
        placeholder: {
          name: 'github.com/sdkwork/craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned Go transport boundary for Craw Chat',
          entrypoints: {
            moduleRoot: '.',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'go.mod'],
        readManifest: createGoModManifestReader({
          moduleRoot: '.',
        }),
        placeholder: {
          name: 'github.com/sdkwork/craw-chat-sdk',
          version: '',
          description: 'Manual-owned Go semantic SDK boundary for Craw Chat',
          entrypoints: {
            moduleRoot: '.',
          },
        },
      },
    ],
  },
  python: {
    workspace: 'sdkwork-craw-chat-sdk-python',
    maturityTier: 'tier-b',
    primaryClient: 'CrawChatSdkClient',
    packages: [
      {
        layer: 'generated',
        path: ['generated', 'server-openapi'],
        manifest: ['generated', 'server-openapi', 'pyproject.toml'],
        readManifest: createPyprojectManifestReader({
          packageRoot: 'src/',
        }),
        placeholder: {
          name: 'sdkwork-craw-chat-backend-sdk',
          version: '',
          description: 'Generator-owned Python transport boundary for Craw Chat',
          entrypoints: {
            packageRoot: 'src/',
          },
        },
      },
      {
        layer: 'composed',
        path: ['composed'],
        manifest: ['composed', 'pyproject.toml'],
        readManifest: createPyprojectManifestReader({
          packageRoot: 'src/',
        }),
        placeholder: {
          name: 'sdkwork-craw-chat-sdk',
          version: '',
          description: 'Manual-owned Python semantic SDK boundary for Craw Chat',
          entrypoints: {
            packageRoot: 'src/',
          },
        },
      },
    ],
  },
};

function renderPackageAssembly(workspaceRoot, workspaceName, packageConfig, yaml) {
  const workspacePath = path.join(workspaceRoot, workspaceName);
  const manifestPath = path.join(workspacePath, ...packageConfig.manifest);
  const relativeManifestPath = path.relative(workspaceRoot, manifestPath).replaceAll('\\', '/');

  const assembly = {
    layer: packageConfig.layer,
    packagePath: path.join(workspaceName, ...packageConfig.path).replaceAll('\\', '/'),
    manifestPath: relativeManifestPath,
    ...(existsSync(manifestPath)
      ? packageConfig.readManifest
        ? packageConfig.readManifest(manifestPath, yaml)
        : packageConfig.placeholder
      : packageConfig.placeholder),
  };

  if (!existsSync(manifestPath)) {
    assembly.status = 'reserved';
  }

  return assembly;
}

function renderLanguageAssembly(workspaceRoot, language, yaml) {
  const config = languageConfigs[language];
  if (!config) {
    fail(`Unsupported language for assembly: ${language}`);
  }

  const workspacePath = path.join(workspaceRoot, config.workspace);
  const packages = config.packages.map((packageConfig) =>
    renderPackageAssembly(workspaceRoot, config.workspace, packageConfig, yaml),
  );
  const generatedPackage = packages.find((entry) => entry.layer === 'generated');
  if (!generatedPackage) {
    fail(`Missing generated package assembly for ${language}`);
  }
  const consumerPackage = config.consumerLayer
    ? packages.find((entry) => entry.layer === config.consumerLayer)
    : null;
  if (config.consumerLayer && !consumerPackage) {
    fail(`Missing ${config.consumerLayer} consumer package assembly for ${language}`);
  }

  return {
    language,
    workspace: config.workspace,
    maturityTier: config.maturityTier,
    primaryClient: config.primaryClient,
    generatedPath: generatedPackage.packagePath,
    manifestPath: generatedPackage.manifestPath,
    name: generatedPackage.name,
    version: generatedPackage.version,
    description: generatedPackage.description,
    entrypoints: generatedPackage.entrypoints,
    consumerPackage,
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
const requestedLanguages = new Set(args.languages.length > 0 ? args.languages : officialLanguages);
for (const language of requestedLanguages) {
  if (!officialLanguages.includes(language)) {
    fail(`Unsupported language for assembly: ${language}`);
  }
}
const languages = officialLanguages.map((language) =>
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
