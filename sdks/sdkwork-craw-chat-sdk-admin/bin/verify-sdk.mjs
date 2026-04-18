#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  ensureLanguageRequirements,
  normalizeLanguages,
  parseVerifyArgs,
  runVerifyCommand,
  runWorkspaceAssemblyStep,
  runWorkspaceVerificationPrelude,
} from '../../workspace-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk-admin';
const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const parsedArgs = parseVerifyArgs(process.argv.slice(2), { prefix });
const languages = normalizeLanguages({
  parsedArgs,
  defaultLanguages: ['typescript', 'flutter'],
  supportedLanguages: ['typescript', 'flutter'],
  prefix,
});

runWorkspaceVerificationPrelude({
  prefix,
  workspaceRoot,
  scriptDir,
  additionalSteps: [
    {
      step: 'workspace:shell-wrapper-args',
      args: [path.join(scriptDir, 'verify-shell-wrapper-args.mjs')],
    },
  ],
});

ensureLanguageRequirements({
  workspaceRoot,
  languages,
  prefix,
  requirements: {
    typescript: [
      'sdkwork-craw-chat-sdk-admin-typescript/README.md',
      'sdkwork-craw-chat-sdk-admin-typescript/bin',
      'sdkwork-craw-chat-sdk-admin-typescript/generated/server-openapi',
      'sdkwork-craw-chat-sdk-admin-typescript/composed/package.json',
    ],
    flutter: [
      'sdkwork-craw-chat-sdk-admin-flutter/README.md',
      'sdkwork-craw-chat-sdk-admin-flutter/bin',
      'sdkwork-craw-chat-sdk-admin-flutter/generated/server-openapi',
      'sdkwork-craw-chat-sdk-admin-flutter/composed/pubspec.yaml',
    ],
  },
});

if (languages.includes('typescript')) {
  runVerifyCommand({
    prefix,
    command: 'node',
    args: [path.join(scriptDir, 'verify-typescript-workspace.mjs')],
    cwd: workspaceRoot,
    step: 'typescript:workspace',
  });
}

if (languages.includes('flutter')) {
  const flutterArgs = [path.join(scriptDir, 'verify-flutter-workspace.mjs')];
  if (parsedArgs.withDart) {
    flutterArgs.push('--with-dart');
  }
  runVerifyCommand({
    prefix,
    command: 'node',
    args: flutterArgs,
    cwd: workspaceRoot,
    step: 'flutter:workspace',
  });
}

runWorkspaceAssemblyStep({
  prefix,
  workspaceRoot,
  scriptDir,
  languages,
});
