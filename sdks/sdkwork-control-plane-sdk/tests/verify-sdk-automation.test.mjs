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
const fetchOpenApiSourceScript = readFileSync(
  path.join(workspaceRoot, 'bin', 'fetch-openapi-source.mjs'),
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
const typescriptReadmeSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'README.md'),
  'utf8',
);
const flutterReadmeSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'README.md'),
  'utf8',
);
const typescriptGeneratedBuildSource = readFileSync(
  path.join(workspaceRoot, 'bin', 'build-typescript-generated-package.mjs'),
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
const typescriptSdkVerifyShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-verify.sh'),
  'utf8',
);
const flutterSdkVerifyShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'bin', 'sdk-verify.sh'),
  'utf8',
);
const typescriptSdkVerifyPowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-verify.ps1'),
  'utf8',
);
const typescriptSdkGenShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-gen.sh'),
  'utf8',
);
const flutterSdkGenShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'bin', 'sdk-gen.sh'),
  'utf8',
);
const typescriptSdkGenPowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-gen.ps1'),
  'utf8',
);
const flutterSdkGenPowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'bin', 'sdk-gen.ps1'),
  'utf8',
);
const typescriptSdkAssembleShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-assemble.sh'),
  'utf8',
);
const flutterSdkAssembleShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'bin', 'sdk-assemble.sh'),
  'utf8',
);
const typescriptSdkAssemblePowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-typescript', 'bin', 'sdk-assemble.ps1'),
  'utf8',
);
const flutterSdkAssemblePowerShellSource = readFileSync(
  path.join(workspaceRoot, 'sdkwork-control-plane-sdk-flutter', 'bin', 'sdk-assemble.ps1'),
  'utf8',
);

test('automation verifier guards workspace README against stale control-plane authority wording', () => {
  assert.match(
    automationVerifierSource,
    /checked-in[\s\S]*authority[\s\S]*OpenAPI 3\.x|checked-in[\s\S]*authority[\s\S]*contract/i,
  );
  assert.match(automationVerifierSource, /socialRuntime|social-runtime/);
  assert.match(automationVerifierSource, /control-plane\\\/social/);
  assert.match(automationVerifierSource, /control-plane\\\/social-runtime/);
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

test('PowerShell wrapper verifier validates admin language-local sdk-gen and sdk-verify forwarders', () => {
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

test('automation verifier requires admin language sdk-gen forwarders', () => {
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-gen\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-gen\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-gen\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-gen\.sh/,
  );
});

test('automation verifier requires admin root and language sdk-assemble forwarders', () => {
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
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-assemble\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-assemble\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-assemble\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-assemble\.sh/,
  );
});

test('admin language shell sdk-verify wrappers delegate to the root verify-sdk.sh entrypoint', () => {
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

test('admin language shell sdk-gen wrappers delegate to the root generate-sdk.sh entrypoint', () => {
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

test('admin TypeScript PowerShell sdk-verify wrapper delegates to the root verify-sdk.ps1 entrypoint', () => {
  assert.match(
    typescriptSdkVerifyPowerShellSource,
    /bin\\verify-sdk\.ps1/,
  );
  assert.match(
    typescriptSdkVerifyPowerShellSource,
    /Languages\s*=\s*@\("typescript"\)/,
  );
});

test('admin language PowerShell sdk-gen wrappers delegate to the root generate-sdk.ps1 entrypoint', () => {
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

test('admin language shell sdk-assemble wrappers delegate to the root assemble-sdk.sh entrypoint', () => {
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

test('admin language PowerShell sdk-assemble wrappers delegate to the root assemble-sdk.ps1 entrypoint', () => {
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

test('shell wrapper verifier validates admin sdk-gen and sdk-assemble forwarders', () => {
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

test('admin OpenAPI source scripts import the shared OpenAPI source helper', () => {
  assert.match(
    fetchOpenApiSourceScript,
    /workspace-openapi-source-shared\.mjs/,
  );
  assert.match(
    prepareOpenApiSourceScript,
    /workspace-openapi-source-shared\.mjs/,
  );
});

test('admin OpenAPI source scripts delegate CLI parsing to the shared OpenAPI source helper', () => {
  assert.match(
    openApiSourceSharedSource,
    /\bparseOpenApiSourceArgs\b/,
  );
  assert.match(
    openApiSourceSharedSource,
    /\bparseFetchOpenApiSourceArgs\b/,
  );
  assert.match(
    prepareOpenApiSourceScript,
    /\bparseOpenApiSourceArgs\b/,
  );
  assert.match(
    fetchOpenApiSourceScript,
    /\bparseFetchOpenApiSourceArgs\b/,
  );
});

test('admin fetch-openapi-source delegates runtime fetch and process lifecycle helpers to the shared OpenAPI source helper', () => {
  assert.match(
    openApiSourceSharedSource,
    /\bfetchOpenApiDocument\b/,
  );
  assert.match(
    openApiSourceSharedSource,
    /\bwaitForRuntimeOpenApi\b/,
  );
  assert.match(
    openApiSourceSharedSource,
    /\bstartRuntimeProcess\b/,
  );
  assert.match(
    openApiSourceSharedSource,
    /\bstopRuntimeProcess\b/,
  );
  assert.match(
    fetchOpenApiSourceScript,
    /\bfetchOpenApiDocument\b/,
  );
  assert.match(
    fetchOpenApiSourceScript,
    /\bwaitForRuntimeOpenApi\b/,
  );
  assert.match(
    fetchOpenApiSourceScript,
    /\bstartRuntimeProcess\b/,
  );
  assert.match(
    fetchOpenApiSourceScript,
    /\bstopRuntimeProcess\b/,
  );
});

test('admin sdk-generator-root delegates generator root resolution to the shared helper', () => {
  assert.match(
    sdkGeneratorRootSource,
    /workspace-sdk-generator-root-shared\.mjs/,
  );
});

test('admin fetch-openapi-source launches the runtime with inherited stdio', () => {
  assert.match(
    fetchOpenApiSourceScript,
    /stdio:\s*\[\s*'ignore',\s*'inherit',\s*'inherit'\s*\]/,
  );
});

test('TypeScript public API boundary verifier imports the shared file expectation helper', () => {
  assert.match(
    typescriptPublicApiBoundaryVerifierSource,
    /workspace-file-expectation-shared\.mjs/,
  );
});

test('Flutter usage-surface verifier imports the shared file expectation helper', () => {
  assert.match(
    flutterUsageSurfaceVerifierSource,
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

test('automation verifier guards language README files for flat client creation and verify entrypoints', () => {
  assert.match(automationVerifierSource, /ControlPlaneSdkClient\.create/);
  assert.match(automationVerifierSource, /sdk-verify/);
  assert.match(automationVerifierSource, /verify-typescript-workspace\.mjs/);
  assert.match(automationVerifierSource, /verify-flutter-workspace\.mjs/);
  assert.match(
    automationVerifierSource,
    /verify-sdk\\\.mjs --language flutter --with-dart/,
  );
  assert.match(automationVerifierSource, /appendVerificationFlowDocumentationFailures/);
});

test('admin language README files document sdk-verify wrappers', () => {
  assert.match(
    typescriptReadmeSource,
    /sdk-verify/,
  );
  assert.match(
    flutterReadmeSource,
    /sdk-verify/,
  );
});

test('admin language README files document sdk-gen wrappers', () => {
  assert.match(
    typescriptReadmeSource,
    /sdk-gen/,
  );
  assert.match(
    flutterReadmeSource,
    /sdk-gen/,
  );
});

test('automation verifier requires admin root gitignore and language sdk-verify forwarders', () => {
  assert.match(automationVerifierSource, /\.gitignore/);
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-verify\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-typescript\/bin\/sdk-verify\.sh/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-verify\.ps1/,
  );
  assert.match(
    automationVerifierSource,
    /sdkwork-control-plane-sdk-flutter\/bin\/sdk-verify\.sh/,
  );
});

test('automation verifier guards workspace README for assembly metadata semantics', () => {
  assert.match(automationPolicySharedSource, /\.sdkwork-assembly\.json/);
  assert.match(automationPolicySharedSource, /manifestPath/);
  assert.match(automationPolicySharedSource, /generatedAt/);
  assert.match(automationPolicySharedSource, /generated[\s\S]*composed/i);
  assert.match(automationVerifierSource, /appendAssemblyMetadataDocumentationFailures/);
});

test('automation verifier delegates admin .gitignore pattern checks to the shared policy helper', () => {
  assert.match(
    automationPolicySharedSource,
    /appendGitignorePatternFailures/,
  );
  assert.match(
    automationVerifierSource,
    /appendGitignorePatternFailures/,
  );
});

test('automation verifier delegates admin wrapper command policy to the shared helper', () => {
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
    /fetch-openapi-source\.mjs/,
  );
  assert.match(
    automationVerifierSource,
    /prepare-openapi-source\.mjs/,
  );
  assert.match(
    automationVerifierSource,
    /verify-powershell-wrapper-args\.mjs/,
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

test('automation verifier guards admin verification docs for meta-tests and usage-surface terminology', () => {
  assert.match(automationPolicySharedSource, /automation meta-test/i);
  assert.match(automationPolicySharedSource, /assembly regression/i);
  assert.match(automationPolicySharedSource, /usage-surface/i);
});

test('automation verifier guards admin language README files for detailed verification terminology', () => {
  assert.match(automationPolicySharedSource, /must document usage-surface verification terminology/i);
  assert.match(automationPolicySharedSource, /must document package metadata verification/i);
});

test('automation verifier guards admin root and language README files for sdk-assemble wrapper documentation', () => {
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
