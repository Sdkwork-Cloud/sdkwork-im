#!/usr/bin/env node
import { existsSync, readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { verifySdkSiteDocs } from '../../../docs/sites/sdk/verify-sdk-site-docs.mjs';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const repoRoot = path.resolve(workspaceRoot, '..', '..');

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

const failures = [];
failures.push(...verifySdkSiteDocs({ rootDir: repoRoot }));

const ps1Source = read('bin/generate-sdk.ps1');
const shSource = read('bin/generate-sdk.sh');
const readmeSource = read('README.md');
const workspaceGitignoreSource = read('.gitignore');
const verifySdkSource = read('bin/verify-sdk.mjs');
const workspaceGitignorePath = path.join(workspaceRoot, '.gitignore');
const verifyEntrypointPath = path.join(workspaceRoot, 'bin', 'verify-sdk.mjs');
const verifyTypeScriptWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-typescript-workspace.mjs');
const verifyFlutterWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-flutter-workspace.mjs');
const verifyFlutterDiscoveryAlignmentPath = path.join(
  workspaceRoot,
  'bin',
  'verify-flutter-discovery-alignment.mjs',
);
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
const verifyTypeScriptDiscoveryAlignmentPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-discovery-alignment.mjs',
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
const verifyDerivedDiscoveryMetadataPath = path.join(
  workspaceRoot,
  'bin',
  'verify-derived-discovery-metadata.mjs',
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
if (!existsSync(verifyFlutterDiscoveryAlignmentPath)) {
  failures.push('Workspace root must provide bin/verify-flutter-discovery-alignment.mjs.');
}
if (!existsSync(verifyFlutterDartAnalysisPath)) {
  failures.push('Workspace root must provide bin/verify-flutter-dart-analysis.dart.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildConcurrencyPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-concurrency.mjs.');
}
if (!existsSync(verifyTypeScriptDiscoveryAlignmentPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-discovery-alignment.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedPackagePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-package.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildDeterminismPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-determinism.mjs.');
}
if (!existsSync(verifyDerivedDiscoveryMetadataPath)) {
  failures.push('Workspace root must provide bin/verify-derived-discovery-metadata.mjs.');
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
if (/## Release Placeholder Boundary/.test(readmeSource)
  || /generationStatus = template_only_pending_generation/.test(readmeSource)) {
  failures.push(
    'Workspace README must not describe the app SDK workspace as template-only after TypeScript and Flutter materialization.',
  );
}
if (!/x-sdkwork-sdk-surface|derived discovery metadata/.test(readmeSource)) {
  failures.push('Workspace README must document the derived discovery metadata contract.');
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
if (!/verify-derived-discovery-metadata\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-derived-discovery-metadata.mjs.');
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
if (/## Release Placeholder Boundary/.test(typescriptReadmeSource)
  || /template_only_pending_generation/.test(typescriptReadmeSource)) {
  failures.push(
    'TypeScript workspace README must not describe the workspace as template-only after generated and composed packages are materialized.',
  );
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
  failures.push(
    'TypeScript workspace README must direct manual layers to consume the generated transport package through @sdkwork/craw-chat-backend-sdk.',
  );
}
if (!/sdk-verify/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must reference sdk-verify.');
}
if (/## Release Placeholder Boundary/.test(flutterReadmeSource)
  || /template_only_pending_generation/.test(flutterReadmeSource)) {
  failures.push(
    'Flutter workspace README must not describe the workspace as template-only after generated and composed packages are materialized.',
  );
}
if (!/WithDart|with-dart/.test(flutterReadmeSource)) {
  failures.push('Flutter workspace README must document the WithDart verification path.');
}
if (!/package:backend_sdk\/backend_sdk\.dart/.test(flutterReadmeSource)) {
  failures.push(
    'Flutter workspace README must direct manual layers to consume the generated transport package through package:backend_sdk/backend_sdk.dart.',
  );
}

const generatedTypeScriptPackagePath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
  'package.json',
);
if (existsSync(generatedTypeScriptPackagePath)) {
  const generatedTypeScriptPackage = JSON.parse(
    read('sdkwork-craw-chat-sdk-typescript/generated/server-openapi/package.json'),
  );
  const expectedGeneratedBuildScript = process.platform === 'win32'
    ? '..\\..\\..\\bin\\build-typescript-generated-package.cmd'
    : '../../../bin/build-typescript-generated-package';
  if (!/Craw Chat app API/i.test(String(generatedTypeScriptPackage.description || ''))) {
    failures.push('Generated TypeScript package description must describe the Craw Chat app API.');
  }
  if (generatedTypeScriptPackage.scripts?.build !== expectedGeneratedBuildScript) {
    failures.push('Generated TypeScript package must delegate build to the workspace-stable build script.');
  }
  if (generatedTypeScriptPackage.scripts?.prepublishOnly !== 'npm run build') {
    failures.push('Generated TypeScript package prepublishOnly must stay on "npm run build".');
  }
  const generatedKeywords = Array.isArray(generatedTypeScriptPackage.keywords)
    ? generatedTypeScriptPackage.keywords
    : [];
  for (const expectedKeyword of ['craw-chat', 'app', 'chat']) {
    if (!generatedKeywords.includes(expectedKeyword)) {
      failures.push(`Generated TypeScript package keywords must include "${expectedKeyword}".`);
    }
  }
}

const generatedTypeScriptReadmePath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
  'generated',
  'server-openapi',
  'README.md',
);
if (existsSync(generatedTypeScriptReadmePath)) {
  const generatedTypeScriptReadmeSource = read(
    'sdkwork-craw-chat-sdk-typescript/generated/server-openapi/README.md',
  );
  for (const requiredSection of [
    '## Package Role',
    '## Quick Start',
    '## Authentication',
    '## Endpoint Targeting',
    '## Surface Groups',
    '## Package Boundary',
  ]) {
    if (!generatedTypeScriptReadmeSource.includes(requiredSection)) {
      failures.push(`Generated TypeScript package README must include a "${requiredSection}" section.`);
    }
  }
  if (!/@sdkwork\/craw-chat-backend-sdk/.test(generatedTypeScriptReadmeSource)) {
    failures.push('Generated TypeScript package README must document the generated package root entrypoint.');
  }
  if (!/@sdkwork\/craw-chat-sdk/.test(generatedTypeScriptReadmeSource)) {
    failures.push('Generated TypeScript package README must direct business consumers to the composed app package.');
  }
  if (!/generated\/server-openapi\/src\/\*/.test(generatedTypeScriptReadmeSource)) {
    failures.push('Generated TypeScript package README must forbid importing generated private source paths.');
  }
}

const generatedFlutterPubspecPath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'generated',
  'server-openapi',
  'pubspec.yaml',
);
if (existsSync(generatedFlutterPubspecPath)) {
  const generatedFlutterPubspecSource = read(
    'sdkwork-craw-chat-sdk-flutter/generated/server-openapi/pubspec.yaml',
  );
  if (!/description:\s*Generated Flutter transport package for the Craw Chat app API/.test(generatedFlutterPubspecSource)) {
    failures.push('Generated Flutter pubspec description must describe the Craw Chat app API.');
  }
}

const generatedFlutterReadmePath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-flutter',
  'generated',
  'server-openapi',
  'README.md',
);
if (existsSync(generatedFlutterReadmePath)) {
  const generatedFlutterReadmeSource = read(
    'sdkwork-craw-chat-sdk-flutter/generated/server-openapi/README.md',
  );
  for (const requiredSection of [
    '## Package Role',
    '## Quick Start',
    '## Authentication',
    '## Endpoint Targeting',
    '## Surface Groups',
    '## Package Boundary',
  ]) {
    if (!generatedFlutterReadmeSource.includes(requiredSection)) {
      failures.push(`Generated Flutter package README must include a "${requiredSection}" section.`);
    }
  }
  if (!/package:backend_sdk\/backend_sdk\.dart/.test(generatedFlutterReadmeSource)) {
    failures.push('Generated Flutter package README must document the generated package root entrypoint.');
  }
  if (!/sdkwork-craw-chat-sdk-flutter\/composed|composed Flutter layers/.test(generatedFlutterReadmeSource)) {
    failures.push('Generated Flutter package README must direct business consumers to the composed Flutter layer.');
  }
  if (!/generated `src\/` imports|generated `lib\/src\/` imports/.test(generatedFlutterReadmeSource)) {
    failures.push('Generated Flutter package README must forbid importing generated private source paths.');
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
if (!/verify-typescript-discovery-alignment\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-discovery-alignment.mjs.');
}
if (!/verify-auth-surface-alignment\.mjs/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must run verify-auth-surface-alignment.mjs.');
}
if (!/verify-flutter-discovery-alignment\.mjs/.test(verifyFlutterWorkspaceSource)) {
  failures.push('verify-flutter-workspace.mjs must run verify-flutter-discovery-alignment.mjs.');
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
