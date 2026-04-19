#!/usr/bin/env node
import { existsSync } from 'node:fs';
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
  'sdkwork-control-plane-sdk-flutter',
  'composed',
  'lib',
);
const flutterGeneratedPublicEntrypoint = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-flutter',
  'generated',
  'server-openapi',
  'lib',
  'control_plane_backend_sdk.dart',
);
const removedLegacyPublicEntrypoint = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-flutter',
  'composed',
  'lib',
  'craw_chat_sdk_admin.dart',
);

const files = [
  'control_plane_sdk.dart',
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
      'sdkwork-control-plane-sdk-flutter',
      'generated',
      'server-openapi',
      'lib',
      'control_plane_backend_sdk.dart',
    ),
    composedPublicSource: path.join(
      'sdkwork-control-plane-sdk-flutter',
      'composed',
      'lib',
      'control_plane_sdk.dart',
    ),
  },
});

if (!generatedPublicSource.includes("export 'src/models.dart';")) {
  failures.push(
    'generated/server-openapi/lib/control_plane_backend_sdk.dart must publicly export src/models.dart.',
  );
}
if (!generatedPublicSource.includes("export 'backend_client.dart';")) {
  failures.push(
    'generated/server-openapi/lib/control_plane_backend_sdk.dart must publicly export backend_client.dart.',
  );
}
if (existsSync(removedLegacyPublicEntrypoint)) {
  failures.push('composed/lib/craw_chat_sdk_admin.dart is a removed legacy public entrypoint and must not exist.');
}
if (!composedPublicSource.includes('class ControlPlaneSdkClient')) {
  failures.push('composed/lib/control_plane_sdk.dart must define ControlPlaneSdkClient.');
}
if (
  !composedPublicSource.includes(
    "export 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';",
  )
) {
  failures.push('composed/lib/control_plane_sdk.dart must re-export the generated package root.');
}

for (const relativePath of files) {
  const source = readWorkspaceSource({
    workspaceRoot: flutterComposedRoot,
    relativePath,
  });
  if (source.includes('package:control_plane_backend_sdk/src/')) {
    failures.push(`${relativePath} imports backend private src paths.`);
  }
  if (source.includes('../generated/') || source.includes('../../generated/')) {
    failures.push(`${relativePath} reaches into generated workspace paths directly.`);
  }
}

finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'Flutter public API boundary verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] Flutter public API boundary verification passed.',
});
