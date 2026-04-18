#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { verifyFlutterWorkspaceShape } from '../../_shared/flutter-workspace-tools.mjs';

function resolveWorkspaceRoot(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  return options.workspaceRoot || path.resolve(scriptDir, '..');
}

function buildConfig(workspaceRoot) {
  return {
    workspaceRoot,
    flutterWorkspaceName: 'sdkwork-craw-chat-sdk-admin-flutter',
    generatedPackageName: 'craw_chat_admin_backend_sdk',
    generatedLibraryFile: 'craw_chat_admin_backend_sdk.dart',
    composedPackageName: 'craw_chat_sdk_admin',
    composedLibraryFile: 'craw_chat_sdk_admin.dart',
    composedClientClassName: 'CrawChatAdminClient',
  };
}

export function verifyFlutterWorkspace(options = {}) {
  const workspaceRoot = resolveWorkspaceRoot(options);
  return verifyFlutterWorkspaceShape(buildConfig(workspaceRoot));
}

export function runFlutterWorkspaceVerification(options = {}) {
  const failures = verifyFlutterWorkspace(options);
  if (failures.length > 0) {
    throw new Error(failures.join('\n'));
  }
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const failures = verifyFlutterWorkspace();
  if (failures.length > 0) {
    console.error('[sdkwork-craw-chat-sdk-admin] Flutter workspace verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[sdkwork-craw-chat-sdk-admin] Flutter workspace verification passed.');
}
