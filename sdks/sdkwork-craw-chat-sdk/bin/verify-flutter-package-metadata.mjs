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
const flutterRoot = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter');

const generatedPubspecName = readWorkspaceYamlScalar(
  {
    workspaceRoot: flutterRoot,
    relativePath: path.join('generated', 'server-openapi', 'pubspec.yaml'),
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

if (!generatedPubspecName) {
  failures.push('Flutter generated pubspec.yaml must declare a package name.');
}

if (!generatedMetadata.packageName) {
  failures.push('Flutter generated sdkwork-sdk.json must declare packageName.');
}

if (generatedPubspecName && generatedMetadata.packageName && generatedPubspecName !== generatedMetadata.packageName) {
  failures.push(
    `Flutter generated package metadata mismatch: pubspec name is "${generatedPubspecName}" but sdkwork-sdk.json packageName is "${generatedMetadata.packageName}".`,
  );
}

if (generatedPubspecName && !new RegExp(`\\n\\s{2}${generatedPubspecName}:\\s`).test(composedPubspec)) {
  failures.push(
    `Flutter composed pubspec.yaml must depend on the generated package name "${generatedPubspecName}".`,
  );
}

if (generatedPubspecName && !new RegExp(`\\n\\s{2}${generatedPubspecName}:\\s`).test(overridePubspec)) {
  failures.push(
    `Flutter pubspec_overrides.yaml must override the generated package name "${generatedPubspecName}".`,
  );
}

const generatedCommonFlutterOverride = readOverridePath(generatedOverridePubspec, 'sdkwork_common_flutter');
if (!generatedCommonFlutterOverride) {
  failures.push('Flutter generated pubspec_overrides.yaml must override sdkwork_common_flutter.');
}

const composedCommonFlutterOverride = readOverridePath(overridePubspec, 'sdkwork_common_flutter');
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
  prefix: 'sdkwork-craw-chat-sdk',
  failures,
  failureHeader: 'Flutter package metadata verification failed:',
  successMessage: '[sdkwork-craw-chat-sdk] Flutter package metadata verification passed.',
});
