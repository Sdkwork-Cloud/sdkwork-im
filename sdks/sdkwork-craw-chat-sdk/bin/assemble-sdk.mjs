#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './sdk-generator-root.mjs';
import {
  detectAssemblyLanguages,
  parseAssemblyArgs,
  readAuthorityMeta,
  readJson,
  readYaml,
  renderLanguageAssembly,
  writeStableAssembly,
} from '../../workspace-assembly-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const args = parseAssemblyArgs(process.argv.slice(2), { prefix });
const authorityPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.openapi.yaml');
const derivedPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.sdkgen.yaml');
const flutterDerivedPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.flutter.sdkgen.yaml');
const assemblyPath = path.join(workspaceRoot, '.sdkwork-assembly.json');
const authority = readAuthorityMeta(authorityPath, yaml);

const languageConfigs = {
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

const languages = detectAssemblyLanguages({
  requestedLanguages: args.languages,
  workspaceRoot,
  generatedManifestPaths: {
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
  },
}).map((language) =>
  renderLanguageAssembly({
    workspaceRoot,
    language,
    yaml,
    languageConfigs,
    prefix,
  }),
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

process.stdout.write(
  writeStableAssembly({
    assemblyPath,
    assemblyBase,
    comparableKeys: [
      'workspace',
      'title',
      'apiVersion',
      'openapiVersion',
      'authoritySpec',
      'derivedSpec',
      'derivedSpecs',
      'websocketTransport',
      'languages',
    ],
  }),
);
