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
const assembly = JSON.parse(read('.sdkwork-assembly.json'));
const workspaceGitignoreSource = read('.gitignore');
const verifySdkSource = read('bin/verify-sdk.mjs');
const workspaceGitignorePath = path.join(workspaceRoot, '.gitignore');
const verifyEntrypointPath = path.join(workspaceRoot, 'bin', 'verify-sdk.mjs');
const verifyTypeScriptWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-typescript-workspace.mjs');
const verifyFlutterWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-flutter-workspace.mjs');
const verifyRustWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-rust-workspace.mjs');
const verifyJavaWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-java-workspace.mjs');
const verifyCsharpWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-csharp-workspace.mjs');
const verifySwiftWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-swift-workspace.mjs');
const verifyKotlinWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-kotlin-workspace.mjs');
const verifyGoWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-go-workspace.mjs');
const verifyPythonWorkspacePath = path.join(workspaceRoot, 'bin', 'verify-python-workspace.mjs');
const verifyInternalDocsPath = path.join(workspaceRoot, 'bin', 'verify-internal-docs.mjs');
const verifyDocsContractTestsPath = path.join(workspaceRoot, 'bin', 'verify-docs-contract-tests.mjs');
const verifyDocsContractTestsSource = read('bin/verify-docs-contract-tests.mjs');
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
const verifyTypeScriptWorkspaceConcurrencyPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-workspace-concurrency.mjs',
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
const verifyTypeScriptLiveContractPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-live-contract.mjs',
);
const verifyTypeScriptGeneratedPackageTempCleanupPath = path.join(
  workspaceRoot,
  'bin',
  'verify-typescript-generated-package-temp-cleanup.mjs',
);
const refreshLiveOpenApiSourcePath = path.join(
  workspaceRoot,
  'bin',
  'refresh-live-openapi-source.mjs',
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
const prepareGeneratedOutputPath = path.join(
  workspaceRoot,
  'bin',
  'prepare-generated-output.mjs',
);
const swiftGeneratedPackagePath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-swift',
  'generated',
  'server-openapi',
  'Package.swift',
);
const swiftGeneratedReadmePath = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-swift',
  'generated',
  'server-openapi',
  'README.md',
);
const workspaceForwarders = [
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-typescript/bin/sdk-verify.sh',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.ps1',
  'sdkwork-craw-chat-sdk-flutter/bin/sdk-verify.sh',
];
const officialLanguages = [
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
];
const officialLanguageWorkspaceBoundaries = [
  'sdkwork-craw-chat-sdk-typescript',
  'sdkwork-craw-chat-sdk-flutter',
  'sdkwork-craw-chat-sdk-rust',
  'sdkwork-craw-chat-sdk-java',
  'sdkwork-craw-chat-sdk-csharp',
  'sdkwork-craw-chat-sdk-swift',
  'sdkwork-craw-chat-sdk-kotlin',
  'sdkwork-craw-chat-sdk-go',
  'sdkwork-craw-chat-sdk-python',
];
const officialGenerationMappings = [
  {
    language: 'rust',
    psOutput: 'sdkwork-craw-chat-sdk-rust\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-rust/generated/server-openapi',
    packageName: 'sdkwork-craw-chat-backend-sdk',
    verifyScript: 'verify-rust-workspace.mjs',
  },
  {
    language: 'java',
    psOutput: 'sdkwork-craw-chat-sdk-java\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-java/generated/server-openapi',
    packageName: 'com.sdkwork:craw-chat-backend-sdk',
    verifyScript: 'verify-java-workspace.mjs',
  },
  {
    language: 'csharp',
    psOutput: 'sdkwork-craw-chat-sdk-csharp\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-csharp/generated/server-openapi',
    packageName: 'Sdkwork.CrawChat.BackendSdk',
    verifyScript: 'verify-csharp-workspace.mjs',
  },
  {
    language: 'swift',
    psOutput: 'sdkwork-craw-chat-sdk-swift\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-swift/generated/server-openapi',
    packageName: 'CrawChatBackendSdk',
    verifyScript: 'verify-swift-workspace.mjs',
  },
  {
    language: 'kotlin',
    psOutput: 'sdkwork-craw-chat-sdk-kotlin\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-kotlin/generated/server-openapi',
    packageName: 'com.sdkwork:craw-chat-backend-sdk',
    verifyScript: 'verify-kotlin-workspace.mjs',
  },
  {
    language: 'go',
    psOutput: 'sdkwork-craw-chat-sdk-go\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-go/generated/server-openapi',
    packageName: 'github.com/sdkwork/craw-chat-backend-sdk',
    verifyScript: 'verify-go-workspace.mjs',
  },
  {
    language: 'python',
    psOutput: 'sdkwork-craw-chat-sdk-python\\generated\\server-openapi',
    shOutput: 'sdkwork-craw-chat-sdk-python/generated/server-openapi',
    packageName: 'sdkwork-craw-chat-backend-sdk',
    verifyScript: 'verify-python-workspace.mjs',
  },
];

if (!existsSync(verifyTypeScriptWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-workspace.mjs.');
}
if (!existsSync(verifyFlutterWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-flutter-workspace.mjs.');
}
if (!existsSync(verifyRustWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-rust-workspace.mjs.');
}
if (!existsSync(verifyJavaWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-java-workspace.mjs.');
}
if (!existsSync(verifyCsharpWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-csharp-workspace.mjs.');
}
if (!existsSync(verifySwiftWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-swift-workspace.mjs.');
}
if (!existsSync(verifyKotlinWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-kotlin-workspace.mjs.');
}
if (!existsSync(verifyGoWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-go-workspace.mjs.');
}
if (!existsSync(verifyPythonWorkspacePath)) {
  failures.push('Workspace root must provide bin/verify-python-workspace.mjs.');
}
if (!existsSync(verifyInternalDocsPath)) {
  failures.push('Workspace root must provide bin/verify-internal-docs.mjs.');
}
if (!existsSync(verifyDocsContractTestsPath)) {
  failures.push('Workspace root must provide bin/verify-docs-contract-tests.mjs.');
}
for (const requiredDocsContractTest of [
  'verify-language-workspace-readme-terms-contract.test.mjs',
  'verify-multilanguage-workspace-readmes-contract.test.mjs',
]) {
  if (!verifyDocsContractTestsSource.includes(requiredDocsContractTest)) {
    failures.push(`verify-docs-contract-tests.mjs must require ${requiredDocsContractTest}.`);
  }
}
if (!existsSync(verifyFlutterDartAnalysisPath)) {
  failures.push('Workspace root must provide bin/verify-flutter-dart-analysis.dart.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildConcurrencyPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-concurrency.mjs.');
}
if (!existsSync(verifyTypeScriptWorkspaceConcurrencyPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-workspace-concurrency.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedPackagePath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-package.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedBuildDeterminismPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-build-determinism.mjs.');
}
if (!existsSync(verifyTypeScriptLiveContractPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-live-contract.mjs.');
}
if (!existsSync(verifyTypeScriptGeneratedPackageTempCleanupPath)) {
  failures.push('Workspace root must provide bin/verify-typescript-generated-package-temp-cleanup.mjs.');
}
if (!existsSync(refreshLiveOpenApiSourcePath)) {
  failures.push('Workspace root must provide bin/refresh-live-openapi-source.mjs.');
}
if (!existsSync(verifyAuthSurfaceAlignmentPath)) {
  failures.push('Workspace root must provide bin/verify-auth-surface-alignment.mjs.');
}
if (!existsSync(normalizeGeneratedAuthSurfacePath)) {
  failures.push('Workspace root must provide bin/normalize-generated-auth-surface.mjs.');
}
if (!existsSync(prepareGeneratedOutputPath)) {
  failures.push('Workspace root must provide bin/prepare-generated-output.mjs.');
}
for (const language of officialLanguages) {
  const languageLiteral = new RegExp(`['"\`]${language}['"\`]`);
  if (!languageLiteral.test(read('bin/normalize-generated-auth-surface.mjs'))) {
    failures.push(`normalize-generated-auth-surface.mjs must register ${language} as an official normalization target.`);
  }
}
if (!/verify-typescript-workspace\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-typescript-workspace.mjs.');
}
if (!/normalize-generated-auth-surface\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke normalize-generated-auth-surface.mjs.');
}
if (!/prepare-generated-output\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke prepare-generated-output.mjs before generation.');
}
if (!/refresh-live-openapi-source\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must refresh the authority contract from the live service schema before generation.');
}
if (!/start-local\.ps1/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must be able to start the local service before fetching the live schema.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(ps1Source)) {
  failures.push('PowerShell generator wrapper must invoke verify-flutter-workspace.mjs.');
}
for (const mapping of officialGenerationMappings) {
  if (!ps1Source.includes(mapping.psOutput)) {
    failures.push(`PowerShell generator wrapper must map ${mapping.language} into ${mapping.psOutput}.`);
  }
  if (!ps1Source.includes(mapping.packageName)) {
    failures.push(`PowerShell generator wrapper must define the ${mapping.language} package identity ${mapping.packageName}.`);
  }
  if (!ps1Source.includes(mapping.verifyScript)) {
    failures.push(`PowerShell generator wrapper must invoke ${mapping.verifyScript} after ${mapping.language} generation.`);
  }
}
if (!/verify-typescript-workspace\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-typescript-workspace.mjs.');
}
if (!/normalize-generated-auth-surface\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke normalize-generated-auth-surface.mjs.');
}
if (!/prepare-generated-output\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke prepare-generated-output.mjs before generation.');
}
if (!/refresh-live-openapi-source\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must refresh the authority contract from the live service schema before generation.');
}
if (!/start-local\.sh/.test(shSource)) {
  failures.push('Shell generator wrapper must be able to start the local service before fetching the live schema.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(shSource)) {
  failures.push('Shell generator wrapper must invoke verify-flutter-workspace.mjs.');
}
for (const mapping of officialGenerationMappings) {
  if (!shSource.includes(mapping.shOutput)) {
    failures.push(`Shell generator wrapper must map ${mapping.language} into ${mapping.shOutput}.`);
  }
  if (!shSource.includes(mapping.packageName)) {
    failures.push(`Shell generator wrapper must define the ${mapping.language} package identity ${mapping.packageName}.`);
  }
  if (!shSource.includes(mapping.verifyScript)) {
    failures.push(`Shell generator wrapper must invoke ${mapping.verifyScript} after ${mapping.language} generation.`);
  }
}
if (!/cleanup\(\)[\s\S]*local final_exit_code=\"\$\?\"/.test(shSource)) {
  failures.push('Shell generator wrapper must preserve the primary exit code inside cleanup before attempting stop-local.sh.');
}
if (!/warning: stop-local cleanup failed after an earlier generation failure/.test(shSource)) {
  failures.push('Shell generator wrapper must warn instead of masking an earlier generation failure when stop-local.sh cleanup fails.');
}
if (!/exit \"\$final_exit_code\"/.test(shSource)) {
  failures.push('Shell generator wrapper cleanup must restore the original exit code after stop-local.sh handling.');
}
if (!existsSync(verifyEntrypointPath)) {
  failures.push('Workspace root must provide bin/verify-sdk.mjs.');
}
if (verifySdkSource.includes(".flatMap((language) => ['--language', language])")) {
  failures.push('verify-sdk.mjs must refresh the full assembly matrix instead of shrinking it to the requested verification subset.');
}
if (ps1Source.includes('$AssembleArgs += @("--language", $NormalizedLanguage)')) {
  failures.push('PowerShell generator wrapper must refresh the full assembly matrix instead of shrinking it to the requested generation subset.');
}
if (shSource.includes('ASSEMBLE_ARGS+=(--language "${NORMALIZED_LANGUAGE}")')) {
  failures.push('Shell generator wrapper must refresh the full assembly matrix instead of shrinking it to the requested generation subset.');
}
if (!Array.isArray(assembly.languages)) {
  failures.push('.sdkwork-assembly.json must define a languages array.');
} else {
  for (const language of officialLanguages) {
    const entry = assembly.languages.find((candidate) => candidate.language === language);
    if (!entry) {
      failures.push(`.sdkwork-assembly.json must include a language entry for ${language}.`);
      continue;
    }
    if (!entry.workspace) {
      failures.push(`.sdkwork-assembly.json must record the workspace path for ${language}.`);
    }
    if (!entry.primaryClient) {
      failures.push(`.sdkwork-assembly.json must record the primary client for ${language}.`);
    }
    if (!entry.maturityTier) {
      failures.push(`.sdkwork-assembly.json must record the maturity tier for ${language}.`);
    }
    if (!Array.isArray(entry.packages)) {
      failures.push(`.sdkwork-assembly.json must record package layers for ${language}.`);
    }
  }

  const typescriptAssembly = assembly.languages.find((candidate) => candidate.language === 'typescript');
  if (!typescriptAssembly?.consumerPackage) {
    failures.push('.sdkwork-assembly.json must record consumerPackage metadata for typescript.');
  } else {
    if (typescriptAssembly.consumerPackage.name !== '@sdkwork/craw-chat-sdk') {
      failures.push('TypeScript consumerPackage name in .sdkwork-assembly.json must be @sdkwork/craw-chat-sdk.');
    }
    if (typescriptAssembly.consumerPackage.packagePath !== 'sdkwork-craw-chat-sdk-typescript') {
      failures.push('TypeScript consumerPackage packagePath in .sdkwork-assembly.json must point at sdkwork-craw-chat-sdk-typescript.');
    }
    if (typescriptAssembly.consumerPackage.manifestPath !== 'sdkwork-craw-chat-sdk-typescript/package.json') {
      failures.push('TypeScript consumerPackage manifestPath in .sdkwork-assembly.json must point at sdkwork-craw-chat-sdk-typescript/package.json.');
    }
  }
  if (!typescriptAssembly?.packages?.some((pkg) => pkg.layer === 'root')) {
    failures.push('.sdkwork-assembly.json must record the assembled TypeScript root package as a root layer.');
  }

  const flutterAssembly = assembly.languages.find((candidate) => candidate.language === 'flutter');
  if (!flutterAssembly?.consumerPackage) {
    failures.push('.sdkwork-assembly.json must record consumerPackage metadata for flutter.');
  } else {
    if (flutterAssembly.consumerPackage.name !== 'craw_chat_sdk') {
      failures.push('Flutter consumerPackage name in .sdkwork-assembly.json must be craw_chat_sdk.');
    }
    if (flutterAssembly.consumerPackage.packagePath !== 'sdkwork-craw-chat-sdk-flutter/composed') {
      failures.push('Flutter consumerPackage packagePath in .sdkwork-assembly.json must point at sdkwork-craw-chat-sdk-flutter/composed.');
    }
  }

  for (const language of ['rust', 'java', 'csharp', 'kotlin', 'python']) {
    const entry = assembly.languages.find((candidate) => candidate.language === language);
    if (!entry || !entry.manifestPath) {
      continue;
    }
    const manifestPath = path.join(workspaceRoot, entry.manifestPath);
    if (existsSync(manifestPath) && !entry.version) {
      failures.push(`.sdkwork-assembly.json must record the detected generated package version for ${language} when ${entry.manifestPath} exists.`);
    }
  }
}
if (existsSync(swiftGeneratedPackagePath)) {
  const swiftGeneratedPackageSource = readFileSync(swiftGeneratedPackagePath, 'utf8');
  if (!/name:\s*"CrawChatBackendSdk"/.test(swiftGeneratedPackageSource)) {
    failures.push('Swift generated Package.swift must expose CrawChatBackendSdk as the package and product name.');
  }
  if (!/targets:\s*\["CrawChatBackendSdk"\]/.test(swiftGeneratedPackageSource)) {
    failures.push('Swift generated Package.swift must expose CrawChatBackendSdk as the library target.');
  }
}
if (existsSync(swiftGeneratedReadmePath)) {
  const swiftGeneratedReadmeSource = readFileSync(swiftGeneratedReadmePath, 'utf8');
  if (!/import CrawChatBackendSdk/.test(swiftGeneratedReadmeSource)) {
    failures.push('Swift generated README must import CrawChatBackendSdk instead of the generic BackendSDK module name.');
  }
}
if (!existsSync(workspaceGitignorePath)) {
  failures.push('Workspace root must provide .gitignore for transient SDK artifacts.');
}
if (!/verify-sdk-automation\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-sdk-automation.mjs.');
}
for (const language of officialLanguages) {
  const unsupportedLanguagePattern = new RegExp(`['"]${language}['"]`);
  if (!unsupportedLanguagePattern.test(verifySdkSource)) {
    failures.push(`verify-sdk.mjs must recognize ${language} as a supported language.`);
  }
}
if (!/verify-powershell-wrapper-args\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-powershell-wrapper-args.mjs.');
}
if (!/verify-internal-docs\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-internal-docs.mjs.');
}
if (!/verify-docs-contract-tests\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-docs-contract-tests.mjs.');
}
if (!/verify-typescript-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-workspace.mjs.');
}
if (!/verify-typescript-generated-build-concurrency\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-generated-build-concurrency.mjs.');
}
if (!/verify-typescript-workspace-concurrency\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-workspace-concurrency.mjs.');
}
if (!/verify-typescript-generated-build-determinism\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-typescript-generated-build-determinism.mjs.');
}
if (!/verify-flutter-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-flutter-workspace.mjs.');
}
if (!/verify-rust-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-rust-workspace.mjs.');
}
if (!/verify-java-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-java-workspace.mjs.');
}
if (!/verify-csharp-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-csharp-workspace.mjs.');
}
if (!/verify-swift-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-swift-workspace.mjs.');
}
if (!/verify-kotlin-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-kotlin-workspace.mjs.');
}
if (!/verify-go-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-go-workspace.mjs.');
}
if (!/verify-python-workspace\.mjs/.test(verifySdkSource)) {
  failures.push('verify-sdk.mjs must run verify-python-workspace.mjs.');
}
for (const relativePath of workspaceForwarders) {
  if (!existsSync(path.join(workspaceRoot, relativePath))) {
    failures.push(`Workspace forwarder is missing: ${relativePath}`);
  }
}
for (const workspaceName of officialLanguageWorkspaceBoundaries) {
  for (const relativePath of [
    `${workspaceName}/README.md`,
    `${workspaceName}/bin/sdk-gen.ps1`,
    `${workspaceName}/bin/sdk-gen.sh`,
    `${workspaceName}/bin/sdk-verify.ps1`,
    `${workspaceName}/bin/sdk-verify.sh`,
    `${workspaceName}/generated/server-openapi`,
    `${workspaceName}/composed`,
  ]) {
    if (!existsSync(path.join(workspaceRoot, relativePath))) {
      failures.push(`Official language workspace boundary is missing: ${relativePath}`);
    }
  }
}

const verifyTypeScriptWorkspaceSource = read('bin/verify-typescript-workspace.mjs');
const verifyFlutterWorkspaceSource = read('bin/verify-flutter-workspace.mjs');
const verifyTypeScriptGeneratedPackageSource = read('bin/verify-typescript-generated-package.mjs');
const buildTypeScriptGeneratedPackageSource = read('bin/build-typescript-generated-package.mjs');
if (!/pack --dry-run --json|['"]pack['"], ['"]--dry-run['"], ['"]--json['"]/.test(verifyTypeScriptGeneratedPackageSource)) {
  failures.push('verify-typescript-generated-package.mjs must run npm pack --dry-run --json.');
}
if (!/stable-typescript-generated-build\.lock/.test(buildTypeScriptGeneratedPackageSource)) {
  failures.push('build-typescript-generated-package.mjs must keep stable-typescript-generated-build.lock as the shared generated artifacts mutex.');
}
if (!/stable-typescript-generated-build\.lock/.test(verifyTypeScriptGeneratedPackageSource)) {
  failures.push('verify-typescript-generated-package.mjs must reuse stable-typescript-generated-build.lock while validating shared generated dist artifacts.');
}
if (!/acquireGeneratedArtifactsLock/.test(verifyTypeScriptGeneratedPackageSource) || !/releaseGeneratedArtifactsLock/.test(verifyTypeScriptGeneratedPackageSource)) {
  failures.push('verify-typescript-generated-package.mjs must explicitly acquire and release the generated artifacts lock.');
}
if (!/verify-typescript-generated-package\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-generated-package.mjs.');
}
if (!/verify-typescript-generated-package-temp-cleanup\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-generated-package-temp-cleanup.mjs.');
}
if (!/verify-typescript-live-contract\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-typescript-live-contract.mjs.');
}
if (!/verify-auth-surface-alignment\.mjs/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run verify-auth-surface-alignment.mjs.');
}
if (/runNpm\(/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must avoid npm script indirection and invoke TypeScript verification commands directly.');
}
if (!/typescriptCompilerCliPath/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must resolve the generator-owned TypeScript compiler CLI directly.');
}
if (!/typescript:clean/.test(verifyTypeScriptWorkspaceSource) || !/typescript:build/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run explicit clean and build verification steps for the single-package TypeScript SDK.');
}
if (
  !/typescript:composed-typecheck/.test(verifyTypeScriptWorkspaceSource)
  || !/typescript:composed-build/.test(verifyTypeScriptWorkspaceSource)
  || !/typescript:composed-live-contract/.test(verifyTypeScriptWorkspaceSource)
  || !/typescript:composed-test/.test(verifyTypeScriptWorkspaceSource)
) {
  failures.push('verify-typescript-workspace.mjs must run explicit typecheck, build, live-contract, and test verification steps for the composed TypeScript package.');
}
if (!/typescript:live-contract/.test(verifyTypeScriptWorkspaceSource)) {
  failures.push('verify-typescript-workspace.mjs must run an explicit live-contract verification step for the single-package TypeScript SDK.');
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
