#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

const failures = [];

const ps1Source = read('bin/generate-sdk.ps1');
const shSource = read('bin/generate-sdk.sh');
const readmeSource = read('README.md');
const workspaceGitignoreSource = read('.gitignore');
const verifySdkSource = read('bin/verify-sdk.mjs');
const workspaceGitignorePath = path.join(workspaceRoot, '.gitignore');
const verifyEntrypointPath = path.join(workspaceRoot, 'bin', 'verify-sdk.mjs');
const verifyTypeScriptWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-typescript-workspace.mjs');
const verifyFlutterWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-flutter-workspace.mjs');
const verifyFlutterDartAnalysisPath = path.join(
  workspaceRoot,
  'bin',
  'verify-flutter-dart-analysis.dart',
);
const verifyTypeScriptGeneratedBuildConcurrencyPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-build-concurrency.mjs',
);
const verifyTypeScriptGeneratedPackagePath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-package.mjs',
);
const verifyTypeScriptGeneratedBuildDeterminismPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-build-determinism.mjs',
);
const verifyTypeScriptGeneratedPackageTempCleanupPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-package-temp-cleanup.mjs',
);
const verifyAuthSurfaceAlignmentPath = path.join(
  workspaceRoot,
  'bin',
  'verify-auth-surface-alignment.mjs',
);
const normalizeGeneratedAuthSurfacePath = path.join(
  workspaceRoot,
  'bin',
  'normalize-generated-auth-surface.mjs',
);
const typescriptReadmeSource = read('sdkwork-craw-chat-sdk-typescript/README.md');
const flutterReadmeSource = read('sdkwork-craw-chat-sdk-flutter/README.md');
const workspaceForwarders = [
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.sh',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.sh',
];

if (!existsSync(verifyTypeScriptWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-workspace.mjs.');
}
if (!existsSync(verifyFlutterWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-flutter-workspace.mjs.');
}
if (!existsSync(verifyFlutterDartAnalysisPath)) {
  failures.push('Workspace root must provide bin/verify-flutter-dart-analysis.dart.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildConcurrencyPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-concurrency.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedPackagePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-package.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildDeterminismPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-determinism.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedPackageTempCleanupPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-package-temp-cleanup.mjs.');
}
if (!existsSync(verifyAuthSurfaceAlignmentPath)) {
  failures.push('Workspace root must provide bin/verify-auth-surface-alignment.mjs.');
}
if (!existsSync(normalizeGeneratedAuthSurfacePath)) {
  failures.push('Workspace root must provide bin/normalize-generated-auth-surface.mjs.');
}
if (!/verify-typescript-workspace\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-typescript-workspace.mjs.');
}
if (!/normalize-generated-auth-surface\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke normalize-generated-auth-surface.mjs.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-flutter-workspace.mjs.');
}
if (!/verify-typescript-workspace\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-typescript-workspace.mjs.');
}
if (!/normalize-generated-auth-surface\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke normalize-generated-auth-surface.mjs.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-flutter-workspace.mjs.');
}
if (!existsSync(verifyEntrypointPath)) {
  failures.push('Workspace root must provide bin/verify-sdk.mjs.');
}
if (!existsSync(workspaceGitignorePath)) {
  failures.push('Workspace root must provide .gitignore for transient SDK artifacts.');
}
if (!/## Verification/.test(readmeSource)) {
  failures.push('Workspace README must document a verification entrypoint.');
}
if (!/verify-sdk/.test(readmeSource)) {
  failures.push('Workspace README must reference the verify-sdk command.');
}
if (!/runtime root-export validation/.test(readmeSource)) {
  failures.push('Workspace README must document runtime root-export validation for the TypeScript package.');
}
if (!/dead-auth\/dead-residue cleanup/.test(readmeSource)) {
  failures.push('Workspace README must document dead-auth and dead-residue cleanup checks.');
}
if (!/npm pack --dry-run/.test(readmeSource)) {
  failures.push('Workspace README must document npm pack --dry-run verification for the TypeScript generated package.');
}
if (!/--with-dart|-WithDart/.test(readmeSource)) {
  failures.push('Workspace README must document the native Dart verification opt-in path.');
}
if (!/\.sdkwork\/dart\/pub-cache/.test(readmeSource)) {
  failures.push('Workspace README must document the local Dart pub-cache boundary.');
}
if (!/verify-flutter-dart-analysis\.dart/.test(readmeSource)) {
  failures.push('Workspace README must document the Windows Flutter Dart analysis fallback entrypoint.');
}
if (!/verify-sdk-automation\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-sdk-automation.mjs.');
}
if (!/verify-powershell-wrapper-args\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-powershell-wrapper-args.mjs.');
}
if (!/verify-typescript-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-workspace.mjs.');
}
if (!/verify-typescript-generated-build-concurrency\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-generated-build-concurrency.mjs.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-flutter-workspace.mjs.');
}
for (const relativePath of workspaceForwarders) {
  if (!existsSync(path.join(workspaceRoot, relativePath))) {
    failures.push(`Workspace forwarder is missing: ${relativePath}`);
  }
}
if (!/sdk-verify/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must reference sdk-verify.');
}
if (!/npm pack --dry-run/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document npm pack --dry-run verification.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document verify-typescript-generated-build-determinism.mjs.');
}
if (!/dead auth scaffolding plus stray `src\/index\.js` and `src\/index\.d\.ts` residue/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document dead auth and source residue cleanup.');
}
if (!/runtime root exports/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document runtime root-export verification.');
}
if (!/sdk-verify/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must reference sdk-verify.');
}
if (!/WithDart|with-dart/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must document the WithDart verification path.');
}

const verifyTypeScriptWorkspaceSource = read('bin/verify-typescript-workspace.mjs');
const verifyFlutterWorkspaceSource = read('bin/verify-flutter-workspace.mjs');
const verifyTypeScriptGeneratedPackageSource = read('bin/verify-typescript-generated-package.mjs');
if (!/pack --dry-run --json|['"]pack['"], ['"]--dry-run['"], ['"]--json['"]/.test(verifyTypeScriptGeneratedPackageSource)) {
  failures.push('verify-typescript-generated-package.mjs must run npm pack --dry-run --json.');
}
if (!/verify-typescript-generated-package\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-generated-package.mjs.');
}
if (!/verify-typescript-generated-package-temp-cleanup\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-generated-package-temp-cleanup.mjs.');
}
if (!/verify-auth-surface-alignment\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-auth-surface-alignment.mjs.');
}
if (!/verify-auth-surface-alignment\.mjs/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must run verify-auth-surface-alignment.mjs.');
}
if (!/verify-flutter-dart-analysis\.dart/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must reference verify-flutter-dart-analysis.dart for Windows Dart analysis.');
}
if (!/\.sdkwork['"]?, ['"]dart['"]?, ['"]pub-cache|\.sdkwork\\dart\\pub-cache/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must isolate the Dart pub cache under .sdkwork/dart/pub-cache.');
}
if (!/DART_SUPPRESS_ANALYTICS/.test(verifyFlutterWorkspaceSource) || !/FLUTTER_SUPPRESS_ANALYTICS/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must suppress Dart and Flutter analytics during native verification.');
}
if (!/bin['"]?, ['"]cache['"]?, ['"]dart-sdk['"]?, ['"]bin['"]?, ['"]dart\.exe|bin\\cache\\dart-sdk\\bin\\dart\.exe/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must resolve Flutter bundled dart.exe on Windows.');
}
for (const requiredPattern of [
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
]) {
  if (!workspaceGitignoreSource.includes(requiredPattern)) {
    failures.push(`Workspace .gitignore must ignore ${requiredPattern}.`);
  }
}

if (failures.length > 0) {
  console.error('[sdkwork-craw-chat-sdk] SDK automation verification failed:');
  for (const failure of failures) {
    console.error(`- ${failure}`);
  }
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] SDK automation verification passed.');
