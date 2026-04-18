import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import path from 'node:path';
import test from 'node:test';

const workspaceRoot = path.resolve(import.meta.dirname, '..');
const automationVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-sdk-automation.mjs'),
  'utf8',
);
const rootVerifySource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-sdk.mjs'),
  'utf8',
);
const prepareOpenApiSourceScript = readFileSync(
  path.join(workspaceRoot, 'bin', 'prepare-openapi-source.mjs'),
  'utf8',
);
const sdkGeneratorRootSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'sdk-generator-root.mjs'),
  'utf8',
);
const automationPolicySharedSource = readFileSync(
  path.join(workspaceRoot, '..', 'workspace-automation-policy-shared.mjs'),
  'utf8',
);
const openApiSourceSharedSource = readFileSync(
  path.join(workspaceRoot, '..', 'workspace-openapi-source-shared.mjs'),
  'utf8',
);
const workspaceVerifySharedSource = readFileSync(
  path.join(workspaceRoot, '..', 'workspace-verify-shared.mjs'),
  'utf8',
);
const normalizeGeneratedAuthSurfaceScript = readFileSync(
  path.join(workspaceRoot, 'bin', 'normalize-generated-auth-surface.mjs'),
  'utf8',
);
const typescriptGeneratedBuildSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'build-typescript-generated-package.mjs'),
  'utf8',
);
const typescriptGeneratedBuildDeterminismVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-generated-build-determinism.mjs'),
  'utf8',
);
const typescriptGeneratedBuildConcurrencyVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-generated-build-concurrency.mjs'),
  'utf8',
);
const typescriptGeneratedBuildConcurrencyCleanupVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-generated-build-concurrency-log-cleanup.mjs'),
  'utf8',
);
const typescriptGeneratedPackageTempCleanupVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-generated-package-temp-cleanup.mjs'),
  'utf8',
);
const typescriptWorkspaceVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-workspace.mjs'),
  'utf8',
);
const typescriptPublicApiBoundaryVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-public-api-boundary.mjs'),
  'utf8',
);
const flutterWorkspaceVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-workspace.mjs'),
  'utf8',
);
const flutterPublicApiBoundaryVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-public-api-boundary.mjs'),
  'utf8',
);
const flutterPackageMetadataVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-package-metadata.mjs'),
  'utf8',
);
const flutterGeneratedModelsVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-generated-models.mjs'),
  'utf8',
);
const powershellWrapperVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-powershell-wrapper-args.mjs'),
  'utf8',
);
const shellWrapperVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-shell-wrapper-args.mjs'),
  'utf8',
);
const authSurfaceAlignmentVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-auth-surface-alignment.mjs'),
  'utf8',
);
const typescriptGeneratedPackageVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-generated-package.mjs'),
  'utf8',
);
const typescriptUsageSurfaceVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-typescript-usage-surface.mjs'),
  'utf8',
);
const flutterUsageSurfaceVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-usage-surface.mjs'),
  'utf8',
);
const flutterMediaUploadSurfaceVerifierSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'verify-flutter-media-upload-surface.mjs'),
  'utf8',
);
const typescriptSdkVerifyShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'bin', 'sdk-verify.sh'),
  'utf8',
);
const flutterSdkVerifyShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter', 'bin', 'sdk-verify.sh'),
  'utf8',
);
const typescriptSdkGenShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'bin', 'sdk-gen.sh'),
  'utf8',
);
const flutterSdkGenShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter', 'bin', 'sdk-gen.sh'),
  'utf8',
);
const typescriptSdkGenPowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'bin', 'sdk-gen.ps1'),
  'utf8',
);
const flutterSdkGenPowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter', 'bin', 'sdk-gen.ps1'),
  'utf8',
);
const typescriptSdkAssembleShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'bin', 'sdk-assemble.sh'),
  'utf8',
);
const flutterSdkAssembleShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter', 'bin', 'sdk-assemble.sh'),
  'utf8',
);
const typescriptSdkAssemblePowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-typescript', 'bin', 'sdk-assemble.ps1'),
  'utf8',
);
const flutterSdkAssemblePowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-craw-chat-sdk-flutter', 'bin', 'sdk-assemble.ps1'),
  'utf8',
);

test('automation verifier requires the app workspace meta-tests to exist', () => {
  assert.match(
    automationVerifierSource,
    /tests\/verify-sdk-automation\.test\.mjs/,
  );
  assert.match(
    automationVerifierSource,
    /tests\/assemble-sdk\.test\.mjs/,
  );
});

test('automation verifier imports the shared workspace-automation helper', () => {
  assert.match(
    automationVerifierSource,
    /workspace-automation-shared\.mjs/,
  );
});

test('automation verifier imports the shared automation policy helper', () => {
  assert.match(
    automationVerifierSource,
    /workspace-automation-policy-shared\.mjs/,
  );
});

test('language workspace verifiers import the shared language verify helper', () => {
  assert.match(
    typescriptWorkspaceVerifierSource,
    /workspace-language-verify-shared\.mjs/,
  );
  assert.match(
    flutterWorkspaceVerifierSource,
    /workspace-language-verify-shared\.mjs/,
  );
});

test('PowerShell wrapper verifier imports the shared PowerShell wrapper helper', () => {
  assert.match(
    powershellWrapperVerifierSource,
    /workspace-powershell-wrapper-verify-shared\.mjs/,
  );
});

test('PowerShell wrapper verifier validates language-local sdk-gen and sdk-verify forwarders', () => {
  assert.match(
    powershellWrapperVerifierSource,
    /sdk-gen\.ps1/,
  );
  assert.match(
    powershellWrapperVerifierSource,
    /sdk-verify\.ps1/,
  );
  assert.match(
    powershellWrapperVerifierSource,
    /sdk-assemble\.ps1/,
  );
});

test('root verify entrypoint executes shell wrapper verification', () => {
  assert.match(
    rootVerifySource,
    /verify-shell-wrapper-args\.mjs/,
  );
});

test('automation verifier requires the shell wrapper verification entrypoint', () => {
  assert.match(
    automationVerifierSource,
    /bin\/verify-shell-wrapper-args\.mjs/,
  );
});

test('automation verifier requires language sdk-gen forwarders', () => {
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-typescript\/bin\/sdk-gen\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-typescript\/bin\/sdk-gen\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-flutter\/bin\/sdk-gen\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-flutter\/bin\/sdk-gen\.sh/,
  );
});

test('automation verifier requires root and language sdk-assemble forwarders', () => {
  assert.match(
    automationVerifierSource,
    /bin\/assemble-sdk\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /bin\/assemble-sdk\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-typescript\/bin\/sdk-assemble\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-typescript\/bin\/sdk-assemble\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-flutter\/bin\/sdk-assemble\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-craw-chat-sdk-flutter\/bin\/sdk-assemble\.sh/,
  );
});

test('language shell sdk-verify wrappers delegate to the root verify-sdk.sh entrypoint', () => {
  assert.match(
    typescriptSdkVerifyShellSource,
    /bin\/verify-sdk\.sh/,
  );
  assert.match(
    typescriptSdkVerifyShellSource,
    /--language typescript/,
  );
  assert.match(
    typescriptSdkVerifyShellSource,
    /"\$@"/,
  );
  assert.match(
    flutterSdkVerifyShellSource,
    /bin\/verify-sdk\.sh/,
  );
  assert.match(
    flutterSdkVerifyShellSource,
    /--language flutter/,
  );
  assert.match(
    flutterSdkVerifyShellSource,
    /"\$@"/,
  );
});

test('language shell sdk-gen wrappers delegate to the root generate-sdk.sh entrypoint', () => {
  assert.match(
    typescriptSdkGenShellSource,
    /bin\/generate-sdk\.sh/,
  );
  assert.match(
    typescriptSdkGenShellSource,
    /--language typescript/,
  );
  assert.match(
    typescriptSdkGenShellSource,
    /"\$@"/,
  );
  assert.match(
    flutterSdkGenShellSource,
    /bin\/generate-sdk\.sh/,
  );
  assert.match(
    flutterSdkGenShellSource,
    /--language flutter/,
  );
  assert.match(
    flutterSdkGenShellSource,
    /"\$@"/,
  );
});

test('language PowerShell sdk-gen wrappers delegate to the root generate-sdk.ps1 entrypoint', () => {
  assert.match(
    typescriptSdkGenPowerShellSource,
    /bin\\generate-sdk\.ps1/,
  );
  assert.match(
    typescriptSdkGenPowerShellSource,
    /Languages\s*=\s*@\("typescript"\)/,
  );
  assert.match(
    flutterSdkGenPowerShellSource,
    /bin\\generate-sdk\.ps1/,
  );
  assert.match(
    flutterSdkGenPowerShellSource,
    /Languages\s*=\s*@\("flutter"\)/,
  );
});

test('language shell sdk-assemble wrappers delegate to the root assemble-sdk.sh entrypoint', () => {
  assert.match(
    typescriptSdkAssembleShellSource,
    /bin\/assemble-sdk\.sh/,
  );
  assert.match(
    typescriptSdkAssembleShellSource,
    /--language typescript/,
  );
  assert.match(
    typescriptSdkAssembleShellSource,
    /"\$@"/,
  );
  assert.match(
    flutterSdkAssembleShellSource,
    /bin\/assemble-sdk\.sh/,
  );
  assert.match(
    flutterSdkAssembleShellSource,
    /--language flutter/,
  );
  assert.match(
    flutterSdkAssembleShellSource,
    /"\$@"/,
  );
});

test('language PowerShell sdk-assemble wrappers delegate to the root assemble-sdk.ps1 entrypoint', () => {
  assert.match(
    typescriptSdkAssemblePowerShellSource,
    /bin\\assemble-sdk\.ps1/,
  );
  assert.match(
    typescriptSdkAssemblePowerShellSource,
    /Languages\s*=\s*@\("typescript"\)/,
  );
  assert.match(
    flutterSdkAssemblePowerShellSource,
    /bin\\assemble-sdk\.ps1/,
  );
  assert.match(
    flutterSdkAssemblePowerShellSource,
    /Languages\s*=\s*@\("flutter"\)/,
  );
});

test('shell wrapper verifier validates sdk-gen and sdk-assemble forwarders', () => {
  assert.match(
    shellWrapperVerifierSource,
    /sdk-gen\.sh/,
  );
  assert.match(
    shellWrapperVerifierSource,
    /sdk-assemble\.sh/,
  );
});

test('TypeScript usage-surface verifier imports the shared file expectation helper', () => {
  assert.match(
    typescriptUsageSurfaceVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('TypeScript generated-package verifier imports the shared TypeScript package helper', () => {
  assert.match(
    typescriptGeneratedPackageVerifierSource,
    /workspace-typescript-package-verify-shared\.mjs/,
  );
});

test('TypeScript generated-package build script imports the shared TypeScript build helper', () => {
  assert.match(
    typescriptGeneratedBuildSource,
    /workspace-typescript-build-shared\.mjs/,
  );
});

test('TypeScript generated build verification scripts import the shared generated-build verify helper', () => {
  assert.match(
    typescriptGeneratedBuildDeterminismVerifierSource,
    /workspace-typescript-generated-build-verify-shared\.mjs/,
  );
  assert.match(
    typescriptGeneratedBuildConcurrencyVerifierSource,
    /workspace-typescript-generated-build-verify-shared\.mjs/,
  );
  assert.match(
    typescriptGeneratedBuildConcurrencyCleanupVerifierSource,
    /workspace-typescript-generated-build-verify-shared\.mjs/,
  );
  assert.match(
    typescriptGeneratedPackageTempCleanupVerifierSource,
    /workspace-typescript-generated-build-verify-shared\.mjs/,
  );
});

test('prepare-openapi-source imports the shared OpenAPI source helper', () => {
  assert.match(
    prepareOpenApiSourceScript,
    /workspace-openapi-source-shared\.mjs/,
  );
});

test('prepare-openapi-source delegates CLI parsing to the shared OpenAPI source helper', () => {
  assert.match(
    openApiSourceSharedSource,
    /\bparseOpenApiSourceArgs\b/,
  );
  assert.match(
    prepareOpenApiSourceScript,
    /\bparseOpenApiSourceArgs\b/,
  );
});

test('sdk-generator-root delegates generator root resolution to the shared helper', () => {
  assert.match(
    sdkGeneratorRootSource,
    /workspace-sdk-generator-root-shared\.mjs/,
  );
});

test('auth-surface scripts import the shared auth-surface helper', () => {
  assert.match(
    authSurfaceAlignmentVerifierSource,
    /workspace-auth-surface-shared\.mjs/,
  );
  assert.match(
    normalizeGeneratedAuthSurfaceScript,
    /workspace-auth-surface-shared\.mjs/,
  );
});

test('TypeScript public API boundary verifier imports the shared file expectation helper', () => {
  assert.match(
    typescriptPublicApiBoundaryVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('Flutter file-based surface verifiers import the shared file expectation helper', () => {
  assert.match(
    flutterUsageSurfaceVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
  assert.match(
    flutterMediaUploadSurfaceVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('Flutter boundary and package metadata verifiers import the shared file expectation helper', () => {
  assert.match(
    flutterPublicApiBoundaryVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
  assert.match(
    flutterPackageMetadataVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('Flutter package metadata verifier imports the shared Flutter package metadata helper', () => {
  assert.match(
    flutterPackageMetadataVerifierSource,
    /workspace-flutter-package-metadata-shared\.mjs/,
  );
});

test('Flutter generated-model verifier imports the shared file expectation helper', () => {
  assert.match(
    flutterGeneratedModelsVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('root verify entrypoint delegates common workspace prelude to the shared verify helper', () => {
  assert.match(
    workspaceVerifySharedSource,
    /runWorkspaceVerificationPrelude/,
  );
  assert.match(
    rootVerifySource,
    /runWorkspaceVerificationPrelude/,
  );
});

test('shared workspace prelude executes the automation meta-test', () => {
  assert.match(
    workspaceVerifySharedSource,
    /workspaceRoot,\s*'tests',\s*'verify-sdk-automation\.test\.mjs'/,
  );
});

test('root verify entrypoint imports the shared workspace verify helper', () => {
  assert.match(
    rootVerifySource,
    /workspace-verify-shared\.mjs/,
  );
});

test('automation verifier reads the shared workspace verify helper when guarding root verify entrypoints', () => {
  assert.match(
    automationVerifierSource,
    /workspace-verify-shared\.mjs/,
  );
});

test('shared workspace prelude executes the assembly regression test', () => {
  assert.match(
    workspaceVerifySharedSource,
    /workspaceRoot,\s*'tests',\s*'assemble-sdk\.test\.mjs'/,
  );
});

test('root verify entrypoint delegates workspace assembly to the shared verify helper', () => {
  assert.match(
    workspaceVerifySharedSource,
    /runWorkspaceAssemblyStep/,
  );
  assert.match(
    workspaceVerifySharedSource,
    /assemble-sdk\.mjs/,
  );
  assert.match(
    rootVerifySource,
    /runWorkspaceAssemblyStep/,
  );
});

test('automation verifier guards root README for verification meta-tests and detailed workspace checks', () => {
  assert.match(automationPolicySharedSource, /automation meta-test/i);
  assert.match(automationPolicySharedSource, /assembly regression/i);
  assert.match(automationPolicySharedSource, /usage-surface/i);
  assert.match(automationPolicySharedSource, /media-upload surface|media upload surface/i);
  assert.match(automationVerifierSource, /appendVerificationFlowDocumentationFailures/);
  assert.match(automationVerifierSource, /appendAssemblyMetadataDocumentationFailures/);
});

test('automation verifier delegates .gitignore pattern checks to the shared policy helper', () => {
  assert.match(
    automationPolicySharedSource,
    /appendGitignorePatternFailures/,
  );
  assert.match(
    automationVerifierSource,
    /appendGitignorePatternFailures/,
  );
});

test('automation verifier delegates script invocation policy to the shared helper', () => {
  assert.match(
    automationPolicySharedSource,
    /appendScriptInvocationFailures/,
  );
  assert.match(
    automationVerifierSource,
    /appendScriptInvocationFailures/,
  );
  assert.match(
    automationVerifierSource,
    /normalize-generated-auth-surface\.mjs/,
  );
  assert.match(
    automationVerifierSource,
    /verify-typescript-generated-build-determinism\.mjs/,
  );
  assert.match(
    automationVerifierSource,
    /verify-powershell-wrapper-args\.mjs/,
  );
});

test('automation verifier guards language README files for detailed verification terminology', () => {
  assert.match(automationPolicySharedSource, /must document usage-surface verification terminology/i);
  assert.match(automationPolicySharedSource, /must document media-upload surface verification terminology/i);
  assert.match(automationPolicySharedSource, /must document package metadata verification/i);
});

test('automation verifier guards root and language README files for sdk-assemble wrapper documentation', () => {
  assert.match(
    automationVerifierSource,
    /Workspace README must document assemble-sdk/i,
  );
  assert.match(
    automationVerifierSource,
    /TypeScript workspace README must document sdk-assemble/i,
  );
  assert.match(
    automationVerifierSource,
    /Flutter workspace README must document sdk-assemble/i,
  );
});

test('automation verifier delegates the shared verify-sdk automation entrypoint guard', () => {
  assert.match(
    automationVerifierSource,
    /appendVerifySdkAutomationEntrypointFailures/,
  );
});
