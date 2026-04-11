#!/usr/bin/env node
import { existsSync, mkdirSync, readFileSync, writeFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  const parsed = {
    base: '',
    derived: '',
    preferDerived: false,
    targetLanguage: '',
  };

  for (let index = 0; index < argv.length; index += 1) {
    const current = argv[index];
    if (current === '--base') {
      parsed.base = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--derived') {
      parsed.derived = argv[index + 1] || '';
      index += 1;
      continue;
    }
    if (current === '--prefer-derived') {
      parsed.preferDerived = true;
      continue;
    }
    if (current === '--target-language') {
      parsed.targetLanguage = (argv[index + 1] || '').trim().toLowerCase();
      index += 1;
      continue;
    }
    fail(`Unknown argument: ${current}`);
  }

  if (!parsed.base) {
    fail('Missing required argument: --base');
  }
  if (!parsed.derived) {
    fail('Missing required argument: --derived');
  }

  return parsed;
}

function cloneJson(value) {
  return JSON.parse(JSON.stringify(value));
}

async function loadYaml() {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  const workspaceRoot = path.resolve(scriptDir, '..');
  const generatorRoot = process.env.SDKWORK_GENERATOR_ROOT
    ? path.resolve(process.env.SDKWORK_GENERATOR_ROOT)
    : path.resolve(workspaceRoot, '..', '..', '..', '..', 'sdk', 'sdkwork-sdk-generator');
  const yamlPath = path.join(generatorRoot, 'node_modules', 'js-yaml', 'dist', 'js-yaml.mjs');

  if (!existsSync(yamlPath)) {
    fail(`Unable to locate js-yaml from generator workspace: ${yamlPath}`);
  }

  const yamlModule = await import(pathToFileURL(yamlPath).href);
  return yamlModule.default;
}

function loadOpenApiDocument(filePath, yaml) {
  if (!existsSync(filePath)) {
    fail(`OpenAPI file not found: ${filePath}`);
  }

  const raw = readFileSync(filePath, 'utf8');
  const trimmed = raw.trim();
  if (!trimmed) {
    fail(`OpenAPI file is empty: ${filePath}`);
  }

  const document = trimmed.startsWith('{') || trimmed.startsWith('[')
    ? JSON.parse(trimmed)
    : yaml.load(raw);

  if (!document || typeof document !== 'object') {
    fail(`OpenAPI file did not decode to an object: ${filePath}`);
  }
  if (typeof document.openapi !== 'string' || !document.openapi.startsWith('3.')) {
    fail(`Unsupported OpenAPI version in ${filePath}`);
  }

  return document;
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
    const replacement = cloneJson(primitiveRefMap.get(node.$ref));
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
    primitiveSchemaEntries.map(([name, schema]) => [`#/components/schemas/${name}`, cloneJson(schema)]),
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

function writeYamlDocument(filePath, document, yaml) {
  const nextContents = yaml.dump(document, {
    noRefs: true,
    sortKeys: false,
    lineWidth: 120,
  });

  if (existsSync(filePath)) {
    const currentContents = readFileSync(filePath, 'utf8');
    if (currentContents === nextContents) {
      return;
    }
  }

  mkdirSync(path.dirname(filePath), { recursive: true });
  writeFileSync(filePath, nextContents, 'utf8');
}

const args = parseArgs(process.argv.slice(2));
const yaml = await loadYaml();
const basePath = path.resolve(args.base);
const derivedPath = path.resolve(args.derived);
const authority = loadOpenApiDocument(basePath, yaml);
const derived = cloneJson(authority);

stripRealtimeWebsocketPath(derived);
if (args.targetLanguage === 'flutter') {
  applyFlutterCompatibilityTransforms(derived);
}
writeYamlDocument(derivedPath, derived, yaml);

process.stdout.write(args.preferDerived ? derivedPath : basePath);
