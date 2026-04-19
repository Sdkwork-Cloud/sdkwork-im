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
    workspaceName: 'sdkwork-im-admin-sdk',
    flutterWorkspaceName: 'sdkwork-im-admin-sdk-flutter',
    derivedSpecRelativePath: 'openapi/im-admin.sdkgen.json',
    familyLabel: 'IM admin backend',
    consumerLabel: 'admin',
    generatedPackageName: 'im_admin_backend_sdk',
    generatedPackageDescription:
      'Generated Flutter transport package for the IM admin backend',
    generatedLibraryFile: 'im_admin_backend_sdk.dart',
    composedPackageName: 'im_admin_sdk',
    composedPackageDescription:
      'Composed IM admin Flutter SDK built on the generated im_admin_backend_sdk package',
    composedLibraryFile: 'im_admin_sdk.dart',
    composedClientClassName: 'ImAdminSdkClient',
    defaultBaseUrl: 'http://127.0.0.1:18080',
    quickStartInvocation: "final tenants = await client.tenants.listTenants();\nprint(tenants);",
    endpointTargeting: [
      'Point baseUrl at the deployed surface that serves the checked-in /api/admin/* contract.',
      'In packaged installs, that surface is the unified public origin that fronts the admin gateway.',
      'In direct backend development, use the environment-specific origin that already owns /api/admin/*.',
    ],
  };
}

export function materializeImAdminFlutterWorkspace(options = {}) {
  const workspaceRoot = resolveWorkspaceRoot(options);
  return materializeFlutterWorkspace(buildConfig(workspaceRoot));
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const result = materializeImAdminFlutterWorkspace();
  console.log(`Materialized IM admin Flutter workspace at ${result.flutterRoot}`);
}
