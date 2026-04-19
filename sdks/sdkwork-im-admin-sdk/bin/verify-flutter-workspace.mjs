#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { verifyFlutterWorkspaceShape } from '../../_shared/flutter-workspace-tools.mjs';

function resolveWorkspaceRoot(options = {}) {
  return options.workspaceRoot || path.resolve(import.meta.dirname, '..');
}

function buildConfig(workspaceRoot) {
  return {
    workspaceRoot,
    flutterWorkspaceName: 'sdkwork-im-admin-sdk-flutter',
    generatedPackageName: 'im_admin_backend_sdk',
    generatedLibraryFile: 'im_admin_backend_sdk.dart',
    composedPackageName: 'im_admin_sdk',
    composedLibraryFile: 'im_admin_sdk.dart',
    composedClientClassName: 'ImAdminSdkClient',
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
    console.error('[sdkwork-im-admin-sdk] Flutter workspace verification failed:');
    for (const failure of failures) {
      console.error(`- ${failure}`);
    }
    process.exit(1);
  }

  console.log('[sdkwork-im-admin-sdk] Flutter workspace verification passed.');
}
