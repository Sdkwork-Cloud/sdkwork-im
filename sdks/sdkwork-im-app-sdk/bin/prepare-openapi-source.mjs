#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { loadGeneratorYaml } from '../../workspace-sdk-generator-root-shared.mjs';
import {
  cloneOpenApiJson,
  loadOpenApiDocument,
  parseOpenApiSourceArgs,
  writeOpenApiYamlDocument,
} from '../../workspace-openapi-source-shared.mjs';
import { applySdkworkV3OpenApiStandard } from '../../workspace-openapi-v3-standard.mjs';

const prefix = 'sdkwork-im-app-sdk';

function stripRealtimeWebsocketPath(document) {
  for (const pathKey of Object.keys(document.paths ?? {})) {
    if (pathKey.endsWith('/realtime/ws')) {
      delete document.paths[pathKey];
    }
  }
}

function isPrimitiveComponentSchema(schema) {
  return Boolean(
    schema
      && typeof schema === 'object'
      && !Array.isArray(schema)
      && (['string', 'integer', 'number', 'boolean'].includes(schema.type)
        || (schema.type === 'object' && schema.additionalProperties && !schema.properties)),
  );
}

function inlinePrimitiveRefs(node, primitiveRefMap) {
  if (Array.isArray(node)) {
    return node.map((item) => inlinePrimitiveRefs(item, primitiveRefMap));
  }
  if (!node || typeof node !== 'object') {
    return node;
  }
  if (typeof node.$ref === 'string' && primitiveRefMap.has(node.$ref)) {
    return inlinePrimitiveRefs({ ...cloneOpenApiJson(primitiveRefMap.get(node.$ref)), ...node, $ref: undefined }, primitiveRefMap);
  }
  for (const [key, value] of Object.entries(node)) {
    if (value === undefined) {
      delete node[key];
    } else {
      node[key] = inlinePrimitiveRefs(value, primitiveRefMap);
    }
  }
  return node;
}

function applyFlutterCompatibilityTransforms(document) {
  const schemas = document.components?.schemas;
  if (!schemas || typeof schemas !== 'object') return;
  const primitiveEntries = Object.entries(schemas).filter(([, schema]) => isPrimitiveComponentSchema(schema));
  const primitiveRefMap = new Map(primitiveEntries.map(([name, schema]) => [`#/components/schemas/${name}`, cloneOpenApiJson(schema)]));
  inlinePrimitiveRefs(document, primitiveRefMap);
  for (const [name] of primitiveEntries) {
    delete schemas[name];
  }
}

const args = parseOpenApiSourceArgs(process.argv.slice(2), {
  prefix,
  allowPreferDerived: true,
  allowTargetLanguage: true,
});
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const yaml = await loadGeneratorYaml(workspaceRoot);
const derived = cloneOpenApiJson(loadOpenApiDocument({ prefix, filePath: path.resolve(args.base), yaml }));
applySdkworkV3OpenApiStandard(derived);
stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
writeOpenApiYamlDocument({ filePath: path.resolve(args.derived), document: derived, yaml });
process.stdout.write(args.preferDerived ? path.resolve(args.derived) : path.resolve(args.base));
