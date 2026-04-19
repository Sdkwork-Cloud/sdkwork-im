#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  finishFileExpectationVerification,
  readWorkspaceJson,
  readWorkspaceSource,
  readWorkspaceYamlScalar,
} from '../../workspace-file-expectation-shared.mjs';
import {
  escapeRegExp,
  readOverridePath,
} from '../../workspace-flutter-package-metadata-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const flutterRoot = path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter');

const generatedPubspecName = readWorkspaceYamlScalar(
  {
    workspaceRoot: flutterRoot,
    relativePath: path.join('generated', 'server-openapi', 'pubspec.yaml'),
    key: 'name',
  },
);
const composedPubspecName = readWorkspaceYamlScalar(
  {
    workspaceRoot: flutterRoot,
    relativePath: path.join('composed', 'pubspec.yaml'),
    key: 'name',
  },
);
const generatedMetadata = readWorkspaceJson({
  workspaceRoot: flutterRoot,
  relativePath: path.join('generated', 'server-openapi', 'sdkwork-sdk.json'),
});
const composedPubspec = readWorkspaceSource({
  workspaceRoot: flutterRoot,
  relativePath: path.join('composed', 'pubspec.yaml'),
});
const overridePubspec = readWorkspaceSource({
  workspaceRoot: flutterRoot,
  relativePath: path.join('composed', 'pubspec_overrides.yaml'),
});
const generatedOverridePubspec = readWorkspaceSource({
  workspaceRoot: flutterRoot,
  relativePath: path.join('generated', 'server-openapi', 'pubspec_overrides.yaml'),
});

const failures = [];

if (generatedPubspecName !== 'control_plane_backend_sdk') {
  failures.push('Flutter generated pubspec.yaml name must stay on control_plane_backend_sdk.');
}
if (generatedMetadata.packageName !== 'control_plane_backend_sdk') {
  failures.push('Flutter generated sdkwork-sdk.json packageName must stay on control_plane_backend_sdk.');
}
if (composedPubspecName !== 'control_plane_sdk') {
  failures.push('Flutter composed pubspec.yaml name must stay on control_plane_sdk.');
}
if (!new RegExp(`\\n\\s{2}${escapeRegExp(generatedPubspecName)}:\\s`).test(composedPubspec)) {
  failures.push(
    `Flutter composed pubspec.yaml must depend on the generated package name "${generatedPubspecName}".`,
  );
}
if (!new RegExp(`\\n\\s{2}${escapeRegExp(generatedPubspecName)}:\\s`).test(overridePubspec)) {
  failures.push(
    `Flutter pubspec_overrides.yaml must override the generated package name "${generatedPubspecName}".`,
  );
}

const generatedCommonFlutterOverride = readOverridePath(generatedOverridePubspec, 'sdkwork_common_flutter');
const composedCommonFlutterOverride = readOverridePath(overridePubspec, 'sdkwork_common_flutter');
if (!generatedCommonFlutterOverride) {
  failures.push('Flutter generated pubspec_overrides.yaml must override sdkwork_common_flutter.');
}
if (!composedCommonFlutterOverride) {
  failures.push('Flutter composed pubspec_overrides.yaml must override sdkwork_common_flutter.');
}

if (generatedCommonFlutterOverride && composedCommonFlutterOverride) {
  const generatedCommonFlutterAbsolute = path.resolve(
    flutterRoot,
    'generated',
    'server-openapi',
    generatedCommonFlutterOverride,
  );
  const composedCommonFlutterAbsolute = path.resolve(
    flutterRoot,
    'composed',
    composedCommonFlutterOverride,
  );
  if (generatedCommonFlutterAbsolute !== composedCommonFlutterAbsolute) {
    failures.push(
      `Flutter composed pubspec_overrides.yaml must point sdkwork_common_flutter to ${path.relative(path.join(flutterRoot, 'composed'), generatedCommonFlutterAbsolute).replaceAll('\\', '/')}.`,
    );
  }
}

finishFileExpectationVerification({
  prefix: 'sdkwork-control-plane-sdk',
  failures,
  failureHeader: 'Flutter package metadata verification failed:',
  successMessage: '[sdkwork-control-plane-sdk] Flutter package metadata verification passed.',
});
