#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';

import { materializeFlutterWorkspace } from '../../_shared/flutter-workspace-tools.mjs';

function resolveWorkspaceRoot(options = {}) {
  const scriptDir = path.dirname(fileURLToPath(import.meta.url));
  return options.workspaceRoot || path.resolve(scriptDir, '..');
}

function buildConfig(workspaceRoot) {
  return {
    workspaceRoot,
    workspaceName: 'sdkwork-control-plane-sdk',
    flutterWorkspaceName: 'sdkwork-control-plane-sdk-flutter',
    derivedSpecRelativePath: 'openapi/control-plane.sdkgen.json',
    familyLabel: 'Control Plane API',
    consumerLabel: 'control-plane',
    generatedPackageName: 'control_plane_backend_sdk',
    generatedPackageDescription: 'Generated Flutter transport package for the control-plane API',
    generatedLibraryFile: 'control_plane_backend_sdk.dart',
    composedPackageName: 'control_plane_sdk',
    composedPackageDescription:
      'Composed control-plane Flutter SDK built on the generated control_plane_backend_sdk package',
    composedLibraryFile: 'control_plane_sdk.dart',
    composedClientClassName: 'ControlPlaneSdkClient',
    defaultBaseUrl: 'http://127.0.0.1:18081',
    quickStartInvocation: "final registry = await client.protocol.getApiV1ControlProtocolRegistry();\nprint(registry);",
    endpointTargeting: [
      'For standalone governance development, point baseUrl directly at control-plane-api, which defaults to http://127.0.0.1:18081.',
      'For packaged installs, point the same client at the unified craw-chat-server / web-gateway public origin.',
      'Do not mix direct control-plane origins and packaged single-port gateway assumptions in the same client instance.',
    ],
  };
}

export function materializeControlPlaneFlutterWorkspace(options = {}) {
  const workspaceRoot = resolveWorkspaceRoot(options);
  return materializeFlutterWorkspace(buildConfig(workspaceRoot));
}

const isCli = process.argv[1]
  && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url);

if (isCli) {
  const result = materializeControlPlaneFlutterWorkspace();
  console.log(`Materialized control-plane Flutter workspace at ${result.flutterRoot}`);
}
