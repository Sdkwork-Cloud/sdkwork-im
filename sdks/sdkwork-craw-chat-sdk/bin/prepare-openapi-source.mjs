#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './sdk-generator-root.mjs';
import {
  cloneOpenApiJson,
  failOpenApiSource,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  return loadGeneratorYaml(workspaceRoot);
}

function stripRealtimeWebsocketPath(document) {
  if (!document.paths || typeof document.paths !== 'object') {
    return;
  }

  delete document.paths['/api/v1/realtime/ws'];
  if (document.info && typeof document.info === 'object') {
    const description = typeof document.info.description === 'string'
      ? document.info.description.trim()
      : '';
    const suffix =
      'Derived sdkgen input excludes the realtime websocket upgrade route. Websocket transport stays manual-owned.';
    document.info.description = description ? `${description}\n${suffix}` : suffix;
  }
}

function isPrimitiveComponentSchema(schema) {
  if (!schema || typeof schema !== 'object' || Array.isArray(schema)) {
    return false;
  }

  if (['string', 'integer', 'number', 'boolean'].includes(schema.type)) {
    return true;
  }

  return schema.type === 'object' && schema.additionalProperties && !schema.properties;
}

function inlinePrimitiveComponentRefs(node, primitiveRefMap) {
  if (Array.isArray(node)) {
    for (let index = 0; index < node.length; index += 1) {
      node[index] = inlinePrimitiveComponentRefs(node[index], primitiveRefMap);
    }
    return node;
  }

  if (!node || typeof node !== 'object') {
    return node;
  }

  if (typeof node.$ref === 'string' && primitiveRefMap.has(node.$ref)) {
    const replacement = cloneOpenApiJson(primitiveRefMap.get(node.$ref));
    for (const [key, value] of Object.entries(node)) {
      if (key === '$ref') {
        continue;
      }
      replacement[key] = inlinePrimitiveComponentRefs(value, primitiveRefMap);
    }
    return inlinePrimitiveComponentRefs(replacement, primitiveRefMap);
  }

  for (const [key, value] of Object.entries(node)) {
    node[key] = inlinePrimitiveComponentRefs(value, primitiveRefMap);
  }

  return node;
}

function applyFlutterCompatibilityTransforms(document) {
  const schemas = document?.components?.schemas;
  if (!schemas || typeof schemas !== 'object') {
    return;
  }

  const primitiveSchemaEntries = Object.entries(schemas).filter(([, schema]) =>
    isPrimitiveComponentSchema(schema),
  );
  if (primitiveSchemaEntries.length === 0) {
    return;
  }

  const primitiveRefMap = new Map(
    primitiveSchemaEntries.map(([name, schema]) => [`#/components/schemas/${name}`, cloneOpenApiJson(schema)]),
  );

  inlinePrimitiveComponentRefs(document, primitiveRefMap);

  for (const [name] of primitiveSchemaEntries) {
    delete schemas[name];
  }

  if (document.info && typeof document.info === 'object') {
    const description = typeof document.info.description === 'string'
      ? document.info.description.trim()
      : '';
    const suffix =
      'Flutter-compatible derived sdkgen input expands primitive component refs so the generated Dart models stay strongly typed.';
    document.info.description = description ? `${description}\n${suffix}` : suffix;
  }
}

const args = parseOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  allowPreferDerived: true,
  allowTargetLanguage: true,
});
const yaml = await loadYaml();
const basePath = path.resolve(args.base);
const derivedPath = path.resolve(args.derived);
const authority = loadOpenApiDocument({ prefix, filePath: basePath, yaml });
const derived = cloneOpenApiJson(authority);

stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
writeOpenApiYamlDocument({ filePath: derivedPath, document: derived, yaml });

process.stdout.write(args.preferDerived ? derivedPath : basePath);
