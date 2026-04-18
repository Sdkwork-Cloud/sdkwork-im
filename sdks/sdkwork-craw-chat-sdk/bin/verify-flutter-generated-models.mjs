#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { resolveSdkworkGeneratorRoot } from './sdk-paths.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const modelsPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'generated',
  'server-openapi',
  'lib',
  'src',
  'models.dart',
);
const generatorRoot = resolveSdkworkGeneratorRoot(workspaceRoot);
const authoritySpecPath = path.join(workspaceRoot, 'openapi', 'craw-chat-app.openapi.yaml');

const models = readFileSync(modelsPath, 'utf8');
const yamlModule = await import(
  pathToFileURL(path.join(generatorRoot, 'node_modules', 'js-yaml', 'dist', 'js-yaml.mjs')).href
);
const yaml = yamlModule.default;
const authority = yaml.load(readFileSync(authoritySpecPath, 'utf8'));
const primitiveRefTypes = Object.entries(authority?.components?.schemas ?? {})
  .filter(([, schema]) => {
    if (!schema || typeof schema !== 'object') {
      return false;
    }

    if (['string', 'integer', 'number', 'boolean'].includes(schema.type)) {
      return true;
    }

    return schema.type === 'object' && schema.additionalProperties && !schema.properties;
  })
  .map(([name]) => name)
  .sort();

const failures = [];

for (const typeName of primitiveRefTypes) {
  const emptyClassPattern = new RegExp(
    String.raw`class ${typeName} \{\s*${typeName}\(\);\s*factory ${typeName}\.fromJson\(Map<String, dynamic> json\) \{\s*return ${typeName}\(\);\s*\}\s*Map<String, dynamic> toJson\(\) \{\s*return <String, dynamic>\{\};\s*\}\s*\}`,
    's',
  );

  if (emptyClassPattern.test(models)) {
    failures.push(
      `${typeName} is still generated as an empty object class in Flutter output, which breaks enum/map serialization.`,
    );
  }
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Flutter generated model verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Flutter generated model verification passed.');
