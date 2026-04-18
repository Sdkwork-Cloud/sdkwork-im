#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { loadGeneratorYaml } from './sdk-generator-root.mjs';
import {
  finishFileExpectationVerification,
  readWorkspaceSource,
} from '../../workspace-file-expectation-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const models = readWorkspaceSource({
  workspaceRoot,
  relativePath: path.join(
    'sdkwork-craw-chat-sdk-flutter',
    'generated',
    'server-openapi',
    'lib',
    'src',
    'models.dart',
  ),
});
const yaml = await loadGeneratorYaml(workspaceRoot);
const authority = yaml.load(
  readWorkspaceSource({
    workspaceRoot,
    relativePath: path.join('openapi', 'craw-chat-app.openapi.yaml'),
  }),
);
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

finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'Flutter generated model verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] Flutter generated model verification passed.',
});
