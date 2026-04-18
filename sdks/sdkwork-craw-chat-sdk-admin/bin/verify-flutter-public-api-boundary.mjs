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
  'sdkwork-craw-chat-sdk-admin-flutter',
  'composed',
  'lib',
);
const flutterGeneratedPublicEntrypoint = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-admin-flutter',
  'generated',
  'server-openapi',
  'lib',
  'craw_chat_admin_backend_sdk.dart',
);

const files = [
  'craw_chat_admin_sdk.dart',
  'src/context.dart',
  'src/meta_module.dart',
  'src/nodes_module.dart',
  'src/protocol_module.dart',
  'src/providers_module.dart',
  'src/social_module.dart',
  'src/social_runtime_module.dart',
  'src/types.dart',
];

const failures = [];
const { generatedPublicSource, composedPublicSource } = readWorkspaceSources({
  workspaceRoot,
  files: {
    generatedPublicSource: path.join(
      'sdkwork-craw-chat-sdk-admin-flutter',
      'generated',
      'server-openapi',
      'lib',
      'craw_chat_admin_backend_sdk.dart',
    ),
    composedPublicSource: path.join(
      'sdkwork-craw-chat-sdk-admin-flutter',
      'composed',
      'lib',
      'craw_chat_admin_sdk.dart',
    ),
  },
});

if (!generatedPublicSource.includes("export 'src/models.dart';")) {
  failures.push(
    'generated/server-openapi/lib/craw_chat_admin_backend_sdk.dart must publicly export src/models.dart.',
  );
}
if (!generatedPublicSource.includes("export 'backend_client.dart';")) {
  failures.push(
    'generated/server-openapi/lib/craw_chat_admin_backend_sdk.dart must publicly export backend_client.dart.',
  );
}
if (!composedPublicSource.includes('class CrawChatAdminSdkClient')) {
  failures.push('composed/lib/craw_chat_admin_sdk.dart must define CrawChatAdminSdkClient.');
}
if (
  !composedPublicSource.includes(
    "export 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';",
  )
) {
  failures.push('composed/lib/craw_chat_admin_sdk.dart must re-export the generated package root.');
}

for (const relativePath of files) {
  const source = readWorkspaceSource({
    workspaceRoot: flutterComposedRoot,
    relativePath,
  });
  if (source.includes('package:craw_chat_admin_backend_sdk/src/')) {
    failures.push(`${relativePath} imports backend private src paths.`);
  }
  if (source.includes('../generated/') || source.includes('../../generated/')) {
    failures.push(`${relativePath} reaches into generated workspace paths directly.`);
  }
}

finishFileExpectationVerification({
  prefix: 'sdkwork-craw-chat-sdk-admin',
  failures,
  failureHeader: 'Flutter public API boundary verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk-admin] Flutter public API boundary verification passed.',
});
