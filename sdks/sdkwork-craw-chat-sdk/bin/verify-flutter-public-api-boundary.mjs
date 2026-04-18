#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterComposedRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'composed',
  'lib',
);
const flutterGeneratedPublicEntrypoint = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'generated',
  'server-openapi',
  'lib',
  'backend_sdk.dart',
);

const files = [
  'craw_chat_sdk.dart',
  'src/auth_module.dart',
  'src/builders.dart',
  'src/conversations_module.dart',
  'src/device_module.dart',
  'src/inbox_module.dart',
  'src/media_module.dart',
  'src/messages_module.dart',
  'src/portal_module.dart',
  'src/presence_module.dart',
  'src/realtime_module.dart',
  'src/rtc_module.dart',
  'src/session_module.dart',
  'src/streams_module.dart',
  'src/types.dart',
];

const failures = [];
const generatedPublicSource = readFileSync(flutterGeneratedPublicEntrypoint, 'utf8');

if (!generatedPublicSource.includes("export 'src/models.dart';")) {
  failures.push('generated/server-openapi/lib/backend_sdk.dart must publicly export src/models.dart.');
}
if (!generatedPublicSource.includes("export 'backend_client.dart';")) {
  failures.push('generated/server-openapi/lib/backend_sdk.dart must publicly export backend_client.dart.');
}

for (const relativePath of files) {
  const source = readFileSync(path.join(flutterComposedRoot, relativePath), 'utf8');
  if (source.includes("package:backend_sdk/src/")) {
    failures.push(`${relativePath} imports or exports backend_sdk private src paths.`);
  }
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] Flutter public API boundary verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Flutter public API boundary verification passed.');
