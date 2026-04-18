#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  normalizeLanguages,
  parseVerifyArgs,
  runVerifyCommand,
  runWorkspaceAssemblyStep,
  runWorkspaceVerificationPrelude,
} from '../../workspace-verify-shared.mjs';

const prefix = 'sdkwork-craw-chat-sdk';
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

if (languages.includes('typescript')) {
  runVerifyCommand({
    prefix,
    command: 'node',
    args: [path.join(scriptDir, 'verify-typescript-workspace.mjs')],
    cwd: workspaceRoot,
    step: 'typescript:workspace',
  });
  runVerifyCommand({
    prefix,
    command: 'node',
    args: [path.join(scriptDir, 'verify-typescript-generated-build-determinism.mjs')],
    cwd: workspaceRoot,
    step: 'typescript:generated-build-determinism',
  });
  runVerifyCommand({
    prefix,
    command: 'node',
    args: [path.join(scriptDir, 'verify-typescript-generated-build-concurrency.mjs')],
    cwd: workspaceRoot,
    step: 'typescript:generated-build-concurrency',
  });
}

if (languages.includes('flutter')) {
  const flutterWorkspaceArgs = [path.join(scriptDir, 'verify-flutter-workspace.mjs')];
  if (parsedArgs.withDart) {
    flutterWorkspaceArgs.push('--with-dart');
  }
  runVerifyCommand({
    prefix,
    command: 'node',
    args: flutterWorkspaceArgs,
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
