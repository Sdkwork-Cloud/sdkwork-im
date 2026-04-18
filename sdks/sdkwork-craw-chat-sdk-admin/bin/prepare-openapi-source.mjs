#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './sdk-generator-root.mjs';
import {
  cloneOpenApiJson,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';
const args = parseOpenApiSourceArgs(process.argv.slice(2), { prefix });
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const authority = loadOpenApiDocument({ prefix, filePath: path.resolve(args.base), yaml });
const derived = cloneOpenApiJson(authority);

if (derived.info && typeof derived.info === 'object') {
  const description = typeof derived.info.description === 'string'
    ? derived.info.description.trim()
    : '';
  const suffix =
    'Derived sdkgen input mirrors the live authority snapshot for admin control-plane SDK generation.';
  derived.info.description = description ? `${description}\n${suffix}` : suffix;
}

writeOpenApiYamlDocument({ filePath: path.resolve(args.derived), document: derived, yaml });
process.stdout.write(path.resolve(args.derived));
