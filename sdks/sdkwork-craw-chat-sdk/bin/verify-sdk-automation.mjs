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
const assemblySource = read('.sdkwork-assembly.json');
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
const syncFlutterPubspecOverridesPath = path.join(
  workspaceRoot,
  'bin',
  'sync-flutter-pubspec-overrides.mjs',
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
const assembly = JSON.parse(assemblySource);
const typescriptReadmeSource = read('sdkwork-craw-chat-sdk-typescript/README.md');
const flutterReadmeSource = read('sdkwork-craw-chat-sdk-flutter/README.md');
const rustReadmeSource = read('sdkwork-craw-chat-sdk-rust/README.md');
const rustComposedReadmeSource = read('sdkwork-craw-chat-sdk-rust/composed/README.md');
const rustWorkspaceReadmePath = path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-rust', 'README.md');
const rustWorkspaceComposedCargoTomlPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-rust',
  'composed',
  'Cargo.toml',
);
const workspaceForwarders = [
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.sh',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.sh',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-gen.ps1',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-gen.sh',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-verify.sh',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-assemble.ps1',
  'sdkwork-craw-chat-sdk-rust/bin/sdk-assemble.sh',
];

if (!existsSync(verifyTypeScriptWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-workspace.mjs.');
}
if (!existsSync(verifyFlutterWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-flutter-workspace.mjs.');
}
if (!existsSync(rustWorkspaceReadmePath)) {
  failures.push('Workspace root must provide sdkwork-craw-chat-sdk-rust/README.md.');
}
if (!existsSync(rustWorkspaceComposedCargoTomlPath)) {
  failures.push('Workspace root must provide sdkwork-craw-chat-sdk-rust/composed/Cargo.toml.');
}
if (!existsSync(verifyFlutterDartAnalysisPath)) {
  failures.push('Workspace root must provide bin/verify-flutter-dart-analysis.dart.');
}
if (!existsSync(syncFlutterPubspecOverridesPath)) {
  failures.push('Workspace root must provide bin/sync-flutter-pubspec-overrides.mjs.');
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
if (!/sdkwork-craw-chat-sdk-rust\//.test(readmeSource)) {
  failures.push('Workspace README must list sdkwork-craw-chat-sdk-rust/ in the workspace layout.');
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
if (!/Rust composed (package|workspace path): `sdkwork-craw-chat-sdk-rust\/composed`/.test(readmeSource)) {
  failures.push('Workspace README must document the Rust composed workspace path.');
}
if (!/Rust generated package: `sdkwork-craw-chat-backend-sdk`/.test(readmeSource)) {
  failures.push('Workspace README must document the Rust generated package name.');
}
if (!/-Languages typescript,flutter,rust/.test(readmeSource)) {
  failures.push('Workspace README must document the comma-separated PowerShell generation example with rust.');
}
if (!/-Languages typescript,flutter,rust -WithDart/.test(readmeSource)) {
  failures.push('Workspace README must document the comma-separated PowerShell verification example with rust.');
}
if (!/\.\/bin\/generate-sdk\.sh --language typescript --language flutter --language rust/.test(readmeSource)) {
  failures.push('Workspace README must document the shell generation example with rust.');
}
if (!/\.\/bin\/verify-sdk\.sh --language typescript --language flutter --language rust --with-dart/.test(readmeSource)) {
  failures.push('Workspace README must document the shell verification example with rust.');
}
for (const requiredReadmePattern of [
  /TypeScript generated package: `@sdkwork\/craw-chat-backend-sdk`/,
  /TypeScript composed package: `@sdkwork\/craw-chat-sdk`/,
  /Flutter generated package: `backend_sdk`/,
  /Flutter composed package: `craw_chat_sdk`/,
  /Rust generated package: `sdkwork-craw-chat-backend-sdk`/,
  /Rust composed package: `craw-chat-sdk`/,
]) {
  if (!requiredReadmePattern.test(readmeSource)) {
    failures.push(`Workspace README is missing package contract: ${requiredReadmePattern}`);
  }
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
if (!/@sdkwork\/craw-chat-backend-sdk/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document the generated package name @sdkwork/craw-chat-backend-sdk.');
}
if (!/The websocket transport is documented at the workspace root but is not implemented/.test(typescriptReadmeSource)) {
  failures.push('TypeScript workspace README must document that websocket transport is not implemented in this round.');
}
if (!/sdk-verify/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must reference sdk-verify.');
}
if (!/WithDart|with-dart/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must document the WithDart verification path.');
}
if (!/package:backend_sdk\/backend_sdk\.dart/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must document the generated backend_sdk package entrypoint.');
}
if (!/The websocket transport is documented at the workspace root but is not implemented/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must document that websocket transport is not implemented in this round.');
}
if (!/- Rust: \[sdkwork-craw-chat-sdk-rust\]/.test(readmeSource)) {
  failures.push('Workspace README must list the Rust language workspace link.');
}
if (!/sdkwork-craw-chat-backend-sdk/.test(rustReadmeSource)) {
  failures.push('Rust workspace README must document the generated crate name sdkwork-craw-chat-backend-sdk.');
}
if (!/craw-chat-sdk/.test(rustReadmeSource)) {
  failures.push('Rust workspace README must document the composed crate name craw-chat-sdk.');
}
if (!/The websocket transport is documented at the workspace root but is not implemented/.test(rustReadmeSource)) {
  failures.push('Rust workspace README must document that websocket transport is not implemented in this round.');
}
if (!/generated\/server-openapi/.test(rustComposedReadmeSource) || !/Generator-owned transport crate/.test(rustComposedReadmeSource)) {
  failures.push('Rust composed README must document the generated/server-openapi generator-owned crate boundary.');
}
if (!/sdkwork-craw-chat-backend-sdk/.test(rustComposedReadmeSource)) {
  failures.push('Rust composed README must document the generated crate name sdkwork-craw-chat-backend-sdk.');
}
if (!/craw-chat-sdk/.test(rustComposedReadmeSource)) {
  failures.push('Rust composed README must document the composed crate name craw-chat-sdk.');
}
if (!/CrawChatClient/.test(rustComposedReadmeSource)) {
  failures.push('Rust composed README must document CrawChatClient as the primary consumer entrypoint.');
}
if (!/websocket transport is documented at the workspace root but is not implemented/i.test(rustComposedReadmeSource)) {
  failures.push('Rust composed README must document that websocket transport is not implemented in this round.');
}
const assemblyLanguages = new Set((assembly.languages ?? []).map((entry) => entry.language));
for (const requiredLanguage of ['typescript', 'flutter', 'rust']) {
  if (!assemblyLanguages.has(requiredLanguage)) {
    failures.push(`.sdkwork-assembly.json must include language metadata for ${requiredLanguage}.`);
  }
}
for (const requiredPackage of [
  '@sdkwork/craw-chat-backend-sdk',
  '@sdkwork/craw-chat-sdk',
  'backend_sdk',
  'craw_chat_sdk',
  'sdkwork-craw-chat-backend-sdk',
  'craw-chat-sdk',
]) {
  if (!assemblySource.includes(`"${requiredPackage}"`)) {
    failures.push(`.sdkwork-assembly.json must include package metadata for ${requiredPackage}.`);
  }
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
if (!/sync-flutter-pubspec-overrides\.mjs/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must synchronize Flutter pubspec_overrides.yaml before metadata verification.');
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
