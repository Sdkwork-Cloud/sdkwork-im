#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { materializeFlutterWorkspace } from '../../_shared/flutter-workspace-tools.mjs';

function resolveWorkspaceRoot(options = {}) {
  return options.workspaceRoot || path.resolve(import.meta.dirname, '..');
}

function buildConfig(workspaceRoot) {
  return {
    workspaceRoot,
    workspaceName: 'sdkwork-craw-chat-sdk-management',
    flutterWorkspaceName: 'sdkwork-craw-chat-sdk-management-flutter',
    derivedSpecRelativePath: 'openapi/craw-chat-management.sdkgen.json',
    familyLabel: 'Craw Chat operator-console management backend',
    consumerLabel: 'management',
    generatedPackageName: 'craw_chat_management_backend_sdk',
    generatedPackageDescription:
      'Generated Flutter transport package for the Craw Chat operator-console management backend',
    generatedLibraryFile: 'craw_chat_management_backend_sdk.dart',
    composedPackageName: 'craw_chat_sdk_management',
    composedPackageDescription:
      'Composed Craw Chat management Flutter SDK built on the generated craw_chat_management_backend_sdk package',
    composedLibraryFile: 'craw_chat_sdk_management.dart',
    composedClientClassName: 'CrawChatManagementClient',
    defaultBaseUrl: 'http://127.0.0.1:18080',
    quickStartInvocation: "final tenants = await client.tenants.listTenants();\nprint(tenants);",
    endpointTargeting: [
      'Point baseUrl at the deployed surface that serves the checked-in /api/admin/* contract.',
      'In packaged installs, that surface is the unified craw-chat-server / web-gateway public origin.',
      'In direct backend development, use the environment-specific origin that already owns /api/admin/*.',
    ],
  };
}

export function materializeManagementFlutterWorkspace(options = {}) {
  const workspaceRoot = resolveWorkspaceRoot(options);
  return materializeFlutterWorkspace(buildConfig(workspaceRoot));
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const result = materializeManagementFlutterWorkspace();
  console.log(`Materialized management Flutter workspace at ${result.flutterRoot}`);
}
