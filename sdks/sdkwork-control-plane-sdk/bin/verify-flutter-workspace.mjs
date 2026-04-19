#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  buildDartEnv,
  parseWithDartArgs,
  runWorkspaceCommand,
} from '../../workspace-language-verify-shared.mjs';

const prefix = 'sdkwork-control-plane-sdk';
const args = parseWithDartArgs(process.argv.slice(2), { prefix });
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatedDir = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-flutter',
  'generated',
  'server-openapi',
);
const composedDir = path.join(
  workspaceRoot,
  'sdkwork-control-plane-sdk-flutter',
  'composed',
);
const flutterDartAnalysisScript = path.join(scriptDir, 'verify-flutter-dart-analysis.dart');
const dartEnv = buildDartEnv(workspaceRoot);

runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-flutter-generated-models.mjs')],
  cwd: workspaceRoot,
  step: 'flutter:generated-regression',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-flutter-public-api-boundary.mjs')],
  cwd: workspaceRoot,
  step: 'flutter:public-api-boundary',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-flutter-usage-surface.mjs')],
  cwd: workspaceRoot,
  step: 'flutter:usage-surface',
});
runWorkspaceCommand({
  prefix,
  command: 'node',
  args: [path.join(scriptDir, 'verify-flutter-package-metadata.mjs')],
  cwd: workspaceRoot,
  step: 'flutter:package-metadata',
});

if (args.withDart) {
  runWorkspaceCommand({
    prefix,
    command: 'dart',
    args: ['--version'],
    cwd: workspaceRoot,
    env: dartEnv,
    step: 'flutter:dart-version',
    timeoutMs: 10000,
  });
  runWorkspaceCommand({
    prefix,
    command: 'dart',
    args: ['pub', 'get'],
    cwd: generatedDir,
    env: dartEnv,
    step: 'flutter:generated-pub-get',
    timeoutMs: 600000,
  });
  runWorkspaceCommand({
    prefix,
    command: 'dart',
    args: ['pub', 'get'],
    cwd: composedDir,
    env: dartEnv,
    step: 'flutter:composed-pub-get',
    timeoutMs: 600000,
  });
  if (process.platform === 'win32') {
    const generatedPackageConfig = path.join(generatedDir, '.dart_tool', 'package_config.json');
    runWorkspaceCommand({
      prefix,
      command: 'dart',
      args: [`--packages=${generatedPackageConfig}`, flutterDartAnalysisScript, generatedDir],
      cwd: workspaceRoot,
      env: dartEnv,
      step: 'flutter:generated-analyze',
      timeoutMs: 300000,
    });
    runWorkspaceCommand({
      prefix,
      command: 'dart',
      args: [`--packages=${generatedPackageConfig}`, flutterDartAnalysisScript, composedDir],
      cwd: workspaceRoot,
      env: dartEnv,
      step: 'flutter:composed-analyze',
      timeoutMs: 300000,
    });
  } else {
    runWorkspaceCommand({
      prefix,
      command: 'dart',
      args: ['analyze'],
      cwd: generatedDir,
      env: dartEnv,
      step: 'flutter:generated-analyze',
      timeoutMs: 300000,
    });
    runWorkspaceCommand({
      prefix,
      command: 'dart',
      args: ['analyze'],
      cwd: composedDir,
      env: dartEnv,
      step: 'flutter:composed-analyze',
      timeoutMs: 300000,
    });
  }
}
