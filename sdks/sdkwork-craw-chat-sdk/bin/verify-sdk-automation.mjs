#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  appendMissingPathFailures,
  finishVerification,
  makeRead,
  requireMatch,
} from '../../workspace-automation-shared.mjs';
import {
  appendAssemblyMetadataDocumentationFailures,
  appendGitignorePatternFailures,
  appendScriptInvocationFailures,
  appendVerificationFlowDocumentationFailures,
  appendVerifySdkAutomationEntrypointFailures,
} from '../../workspace-automation-policy-shared.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const read = makeRead(workspaceRoot);
const failures = [];

const ps1Source = read('bin/generate-sdk.ps1');
const shSource = read('bin/generate-sdk.sh');
const readmeSource = read('README.md');
const workspaceGitignoreSource = read('.gitignore');
const verifySdkSource = read('bin/verify-sdk.mjs');
const typescriptReadmeSource = read('sdkwork-craw-chat-sdk-typescript/README.md');
const flutterReadmeSource = read('sdkwork-craw-chat-sdk-flutter/README.md');
const verifyTypeScriptWorkspaceSource = read('bin/verify-typescript-workspace.mjs');
const verifyFlutterWorkspaceSource = read('bin/verify-flutter-workspace.mjs');
const verifyTypeScriptGeneratedPackageSource = read('bin/verify-typescript-generated-package.mjs');
const languageVerifySharedSource = read('../workspace-language-verify-shared.mjs');
const workspaceVerifySharedSource = read('../workspace-verify-shared.mjs');
const flutterWorkspaceInfrastructureSource = [
  verifyFlutterWorkspaceSource,
  languageVerifySharedSource,
].join('\n');
const verifySdkInfrastructureSource = [
  verifySdkSource,
  workspaceVerifySharedSource,
].join('\n');

appendMissingPathFailures({
  workspaceRoot,
  requiredPaths: [
    'bin/verify-typescript-workspace.mjs',
    'bin/verify-flutter-workspace.mjs',
    'tests/verify-sdk-automation.test.mjs',
    'tests/assemble-sdk.test.mjs',
    'bin/verify-flutter-dart-analysis.dart',
    'bin/verify-flutter-usage-surface.mjs',
    'bin/verify-typescript-generated-build-concurrency.mjs',
    'bin/verify-typescript-generated-package.mjs',
    'bin/verify-typescript-usage-surface.mjs',
    'bin/verify-typescript-generated-build-determinism.mjs',
    'bin/verify-typescript-generated-package-temp-cleanup.mjs',
    'bin/verify-auth-surface-alignment.mjs',
    'bin/normalize-generated-auth-surface.mjs',
    'bin/assemble-sdk.ps1',
    'bin/assemble-sdk.sh',
    'bin/verify-sdk.mjs',
    'bin/verify-shell-wrapper-args.mjs',
    '.gitignore',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-assemble.ps1',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-assemble.sh',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-gen.ps1',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-gen.sh',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.ps1',
    'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.sh',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-assemble.ps1',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-assemble.sh',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-gen.ps1',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-gen.sh',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.ps1',
    'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.sh',
  ],
  failures,
  formatFailure(relativePath) {
    if (relativePath === '.gitignore') {
      return 'Workspace root must provide .gitignore for transient SDK artifacts.';
    }
    if (
      relativePath.includes('/bin/sdk-verify.')
      || relativePath.includes('/bin/sdk-gen.')
      || relativePath.includes('/bin/sdk-assemble.')
    ) {
      return `Workspace forwarder is missing: ${relativePath}`;
    }
    return `Workspace root must provide ${relativePath}.`;
  },
});

appendScriptInvocationFailures({
  source: ps1Source,
  failures,
  label: 'PowerShell generator wrapper',
  invocations: [
    {
      pattern: /verify-typescript-workspace\.mjs/,
      description: 'verify-typescript-workspace.mjs',
    },
    {
      pattern: /normalize-generated-auth-surface\.mjs/,
      description: 'normalize-generated-auth-surface.mjs',
    },
    {
      pattern: /verify-typescript-generated-build-determinism\.mjs/,
      description: 'verify-typescript-generated-build-determinism.mjs',
    },
    {
      pattern: /verify-flutter-workspace\.mjs/,
      description: 'verify-flutter-workspace.mjs',
    },
  ],
});
appendScriptInvocationFailures({
  source: shSource,
  failures,
  label: 'Shell generator wrapper',
  invocations: [
    {
      pattern: /verify-typescript-workspace\.mjs/,
      description: 'verify-typescript-workspace.mjs',
    },
    {
      pattern: /normalize-generated-auth-surface\.mjs/,
      description: 'normalize-generated-auth-surface.mjs',
    },
    {
      pattern: /verify-typescript-generated-build-determinism\.mjs/,
      description: 'verify-typescript-generated-build-determinism.mjs',
    },
    {
      pattern: /verify-flutter-workspace\.mjs/,
      description: 'verify-flutter-workspace.mjs',
    },
  ],
});

requireMatch({
  source: readmeSource,
  pattern: /## Verification/,
  message: 'Workspace README must document a verification entrypoint.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /verify-sdk/,
  message: 'Workspace README must reference the verify-sdk command.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /assemble-sdk/,
  message: 'Workspace README must document assemble-sdk.',
  failures,
});
appendVerificationFlowDocumentationFailures({
  source: readmeSource,
  failures,
  label: 'Workspace README',
  requireAutomationMetaTest: true,
  requireAssemblyRegression: true,
  requireUsageSurface: true,
  requireMediaUploadSurface: true,
});
appendAssemblyMetadataDocumentationFailures({
  source: readmeSource,
  failures,
  label: 'Workspace README',
});
requireMatch({
  source: readmeSource,
  pattern: /runtime root-export validation/,
  message: 'Workspace README must document runtime root-export validation for the TypeScript package.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /dead-auth\/dead-residue cleanup/,
  message: 'Workspace README must document dead-auth and dead-residue cleanup checks.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /npm pack --dry-run/,
  message: 'Workspace README must document npm pack --dry-run verification for the TypeScript generated package.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /--with-dart|-WithDart/,
  message: 'Workspace README must document the native Dart verification opt-in path.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /\.sdkwork\/dart\/pub-cache/,
  message: 'Workspace README must document the local Dart pub-cache boundary.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /verify-flutter-dart-analysis\.dart/,
  message: 'Workspace README must document the Windows Flutter Dart analysis fallback entrypoint.',
  failures,
});
appendVerifySdkAutomationEntrypointFailures({
  source: verifySdkInfrastructureSource,
  failures,
  verb: 'run',
});
appendScriptInvocationFailures({
  source: verifySdkInfrastructureSource,
  failures,
  label: 'verify-sdk.mjs',
  verb: 'run',
  invocations: [
    {
      pattern: /verify-powershell-wrapper-args\.mjs/,
      description: 'verify-powershell-wrapper-args.mjs',
    },
    {
      pattern: /verify-typescript-workspace\.mjs/,
      description: 'verify-typescript-workspace.mjs',
    },
    {
      pattern: /verify-typescript-generated-build-concurrency\.mjs/,
      description: 'verify-typescript-generated-build-concurrency.mjs',
    },
    {
      pattern: /verify-typescript-generated-build-determinism\.mjs/,
      description: 'verify-typescript-generated-build-determinism.mjs',
    },
    {
      pattern: /verify-flutter-workspace\.mjs/,
      description: 'verify-flutter-workspace.mjs',
    },
  ],
});

requireMatch({
  source: typescriptReadmeSource,
  pattern: /sdk-verify/,
  message: 'TypeScript workspace README must reference sdk-verify.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /sdk-assemble/,
  message: 'TypeScript workspace README must document sdk-assemble.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /CrawChatSdkClient/,
  message: 'TypeScript workspace README must document CrawChatSdkClient.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /@sdkwork\/craw-chat-sdk/,
  message: 'TypeScript workspace README must document @sdkwork/craw-chat-sdk.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /@sdkwork\/craw-chat-backend-sdk/,
  message: 'TypeScript workspace README must document @sdkwork/craw-chat-backend-sdk.',
  failures,
});
appendAssemblyMetadataDocumentationFailures({
  source: typescriptReadmeSource,
  failures,
  label: 'TypeScript workspace README',
});
appendVerificationFlowDocumentationFailures({
  source: typescriptReadmeSource,
  failures,
  label: 'TypeScript workspace README',
  requireUsageSurface: true,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /npm pack --dry-run/,
  message: 'TypeScript workspace README must document npm pack --dry-run verification.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /verify-typescript-generated-build-determinism\.mjs/,
  message: 'TypeScript workspace README must document verify-typescript-generated-build-determinism.mjs.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /dead auth scaffolding plus stray `src\/index\.js` and `src\/index\.d\.ts` residue/,
  message: 'TypeScript workspace README must document dead auth and source residue cleanup.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /runtime root exports/,
  message: 'TypeScript workspace README must document runtime root-export verification.',
  failures,
});

requireMatch({
  source: flutterReadmeSource,
  pattern: /sdk-verify/,
  message: 'Flutter workspace README must reference sdk-verify.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /sdk-assemble/,
  message: 'Flutter workspace README must document sdk-assemble.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /craw_chat_sdk/,
  message: 'Flutter workspace README must document craw_chat_sdk.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /backend_sdk/,
  message: 'Flutter workspace README must document backend_sdk.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /CrawChatSdkClient/,
  message: 'Flutter workspace README must document CrawChatSdkClient.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /pubspec_overrides\.yaml/,
  message: 'Flutter workspace README must document pubspec_overrides.yaml.',
  failures,
});
appendAssemblyMetadataDocumentationFailures({
  source: flutterReadmeSource,
  failures,
  label: 'Flutter workspace README',
});
appendVerificationFlowDocumentationFailures({
  source: flutterReadmeSource,
  failures,
  label: 'Flutter workspace README',
  requireUsageSurface: true,
  requireMediaUploadSurface: true,
  requirePackageMetadata: true,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /WithDart|with-dart/,
  message: 'Flutter workspace README must document the WithDart verification path.',
  failures,
});

requireMatch({
  source: verifyTypeScriptGeneratedPackageSource,
  pattern: /pack --dry-run --json|['"]pack['"], ['"]--dry-run['"], ['"]--json['"]/,
  message: 'verify-typescript-generated-package.mjs must run npm pack --dry-run --json.',
  failures,
});
requireMatch({
  source: verifyTypeScriptWorkspaceSource,
  pattern: /verify-typescript-generated-package\.mjs/,
  message: 'verify-typescript-workspace.mjs must run verify-typescript-generated-package.mjs.',
  failures,
});
requireMatch({
  source: verifyTypeScriptWorkspaceSource,
  pattern: /verify-typescript-usage-surface\.mjs/,
  message: 'verify-typescript-workspace.mjs must run verify-typescript-usage-surface.mjs.',
  failures,
});
requireMatch({
  source: verifyTypeScriptWorkspaceSource,
  pattern: /verify-typescript-generated-package-temp-cleanup\.mjs/,
  message: 'verify-typescript-workspace.mjs must run verify-typescript-generated-package-temp-cleanup.mjs.',
  failures,
});
requireMatch({
  source: verifyTypeScriptWorkspaceSource,
  pattern: /verify-auth-surface-alignment\.mjs/,
  message: 'verify-typescript-workspace.mjs must run verify-auth-surface-alignment.mjs.',
  failures,
});
requireMatch({
  source: verifyFlutterWorkspaceSource,
  pattern: /verify-auth-surface-alignment\.mjs/,
  message: 'verify-flutter-workspace.mjs must run verify-auth-surface-alignment.mjs.',
  failures,
});
requireMatch({
  source: verifyFlutterWorkspaceSource,
  pattern: /verify-flutter-usage-surface\.mjs/,
  message: 'verify-flutter-workspace.mjs must run verify-flutter-usage-surface.mjs.',
  failures,
});
requireMatch({
  source: flutterWorkspaceInfrastructureSource,
  pattern: /verify-flutter-dart-analysis\.dart/,
  message: 'verify-flutter-workspace.mjs must reference verify-flutter-dart-analysis.dart for Windows Dart analysis.',
  failures,
});
requireMatch({
  source: flutterWorkspaceInfrastructureSource,
  pattern: /\.sdkwork['"]?, ['"]dart['"]?, ['"]pub-cache|\.sdkwork\\dart\\pub-cache/,
  message: 'verify-flutter-workspace.mjs must isolate the Dart pub cache under .sdkwork/dart/pub-cache.',
  failures,
});
requireMatch({
  source: flutterWorkspaceInfrastructureSource,
  pattern: /DART_SUPPRESS_ANALYTICS/,
  message: 'verify-flutter-workspace.mjs must suppress Dart and Flutter analytics during native verification.',
  failures,
});
requireMatch({
  source: flutterWorkspaceInfrastructureSource,
  pattern: /FLUTTER_SUPPRESS_ANALYTICS/,
  message: 'verify-flutter-workspace.mjs must suppress Dart and Flutter analytics during native verification.',
  failures,
});
requireMatch({
  source: flutterWorkspaceInfrastructureSource,
  pattern: /bin['"]?, ['"]cache['"]?, ['"]dart-sdk['"]?, ['"]bin['"]?, ['"]dart\.exe|bin\\cache\\dart-sdk\\bin\\dart\.exe/,
  message: 'verify-flutter-workspace.mjs must resolve Flutter bundled dart.exe on Windows.',
  failures,
});

appendGitignorePatternFailures({
  source: workspaceGitignoreSource,
  failures,
  label: 'Workspace .gitignore',
  patterns: [
  '/.tmp/',
  '/.sdkwork/dart/',
  '/.sdkwork/tmp/',
  '/.sdkwork-assembly.json',
  '**/node_modules/',
  '**/.npm-cache/',
  '**/.dart_tool/',
  '**/.sdkwork/sdkwork-generator-changes.json',
  '**/.sdkwork/sdkwork-generator-manifest.json',
  '**/.sdkwork/sdkwork-generator-report.json',
  '**/.sdkwork/tmp/',
  '**/.sdkwork/locks/',
  '**/.sdkwork/manual-backups/',
  '**/*.tgz',
  ],
});

finishVerification({ prefix: 'sdkwork-craw-chat-sdk', failures });
