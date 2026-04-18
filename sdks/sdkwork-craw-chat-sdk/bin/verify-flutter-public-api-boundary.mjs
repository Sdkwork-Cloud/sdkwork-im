#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  finishFileExpectationVerification,
  readWorkspaceSource,
  readWorkspaceSources,
} from '../../workspace-file-expectation-shared.mjs';

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
const { generatedPublicSource, flutterEntrypointSource, flutterTypesSource } = readWorkspaceSources({
  workspaceRoot,
  files: {
    generatedPublicSource: path.join(
      'sdkwork-craw-chat-sdk-flutter',
      'generated',
      'server-openapi',
      'lib',
      'backend_sdk.dart',
    ),
    flutterEntrypointSource: path.join(
      'sdkwork-craw-chat-sdk-flutter',
      'composed',
      'lib',
      'craw_chat_sdk.dart',
    ),
    flutterTypesSource: path.join(
      'sdkwork-craw-chat-sdk-flutter',
      'composed',
      'lib',
      'src',
      'types.dart',
    ),
  },
});

if (!generatedPublicSource.includes("export 'src/models.dart';")) {
  failures.push('generated/server-openapi/lib/backend_sdk.dart must publicly export src/models.dart.');
}
if (!generatedPublicSource.includes("export 'backend_client.dart';")) {
  failures.push('generated/server-openapi/lib/backend_sdk.dart must publicly export backend_client.dart.');
}
if (!/class CrawChatSdkClient\s*\{/.test(flutterEntrypointSource)) {
  failures.push('composed/lib/craw_chat_sdk.dart must define CrawChatSdkClient.');
}
if (!/class CrawChatSdkClientOptions\s*\{/.test(flutterTypesSource)) {
  failures.push('composed/lib/src/types.dart must define CrawChatSdkClientOptions.');
}
if (/class CrawChatClient\s*\{/.test(flutterEntrypointSource)) {
  failures.push('composed/lib/craw_chat_sdk.dart must not keep the legacy CrawChatClient name.');
}

for (const relativePath of files) {
  const source = readWorkspaceSource({
    workspaceRoot: flutterComposedRoot,
    relativePath,
  });
  if (source.includes("package:backend_sdk/src/")) {
    failures.push(`${relativePath} imports or exports backend_sdk private src paths.`);
  }
}

finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'Flutter public API boundary verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] Flutter public API boundary verification passed.',
});
