#!/usr/bin/env node
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import {
  ensureRequiredPaths,
  finishVerification,
  makeRead,
  requireMatch,
  requireNotMatch,
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

const requiredPaths = [
  'README.md',
  '.gitignore',
  'openapi/README.md',
  'openapi/control-plane.openapi.yaml',
  'openapi/control-plane.sdkgen.yaml',
  'bin/fetch-openapi-source.mjs',
  'bin/prepare-openapi-source.mjs',
  'bin/sdk-generator-root.mjs',
  'bin/generate-sdk.ps1',
  'bin/generate-sdk.sh',
  'bin/assemble-sdk.ps1',
  'bin/assemble-sdk.sh',
  'bin/verify-sdk.mjs',
  'bin/verify-sdk.ps1',
  'bin/verify-sdk.sh',
  'bin/assemble-sdk.mjs',
  'bin/verify-sdk-automation.mjs',
  'bin/verify-powershell-wrapper-args.mjs',
  'bin/verify-shell-wrapper-args.mjs',
  'bin/npm-runtime.mjs',
  'bin/build-typescript-generated-package.mjs',
  'bin/verify-typescript-generated-package.mjs',
  'bin/verify-typescript-generated-sdk-gen-runtime.mjs',
  'bin/verify-typescript-usage-surface.mjs',
  'bin/verify-typescript-public-api-boundary.mjs',
  'bin/verify-typescript-workspace.mjs',
  'bin/verify-flutter-generated-models.mjs',
  'bin/verify-flutter-public-api-boundary.mjs',
  'bin/verify-flutter-usage-surface.mjs',
  'bin/verify-flutter-package-metadata.mjs',
  'bin/verify-flutter-dart-analysis.dart',
  'bin/verify-flutter-workspace.mjs',
  'tests/assemble-sdk.test.mjs',
  'tests/typescript-publish-core.test.mjs',
  'tests/typescript-generated-sdk-gen-runtime.test.mjs',
  'tests/verify-sdk-automation.test.mjs',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-assemble.ps1',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-assemble.sh',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-gen.ps1',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-gen.sh',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-verify.ps1',
  'sdkwork-control-plane-sdk-typescript/bin/sdk-verify.sh',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-assemble.ps1',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-assemble.sh',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-gen.ps1',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-gen.sh',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-verify.ps1',
  'sdkwork-control-plane-sdk-flutter/bin/sdk-verify.sh',
];

ensureRequiredPaths({
  workspaceRoot,
  requiredPaths,
  prefix: 'sdkwork-control-plane-sdk',
});

const failures = [];

const readmeSource = read('README.md');
const workspaceGitignoreSource = read('.gitignore');
const typescriptReadmeSource = read('sdkwork-control-plane-sdk-typescript/README.md');
const flutterReadmeSource = read('sdkwork-control-plane-sdk-flutter/README.md');
const ps1Source = read('bin/generate-sdk.ps1');
const shSource = read('bin/generate-sdk.sh');
const verifySdkSource = read('bin/verify-sdk.mjs');
const verifyFlutterWorkspaceSource = read('bin/verify-flutter-workspace.mjs');
const npmRuntimeSource = read('bin/npm-runtime.mjs');
const typescriptWorkspaceVerifierSource = read('bin/verify-typescript-workspace.mjs');
const generatedTypeScriptRunTscSource = read(
  'sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/run-tsc.mjs',
);
const composedTypeScriptRunTscSource = read(
  'sdkwork-control-plane-sdk-typescript/composed/bin/run-tsc.mjs',
);
const workspaceVerifySharedSource = read('../workspace-verify-shared.mjs');
const verifySdkInfrastructureSource = [
  verifySdkSource,
  workspaceVerifySharedSource,
].join('\n');
const generatedTypeScriptSdkGenCoreSource = read(
  'sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/sdk-gen-core.mjs',
);
const generatedTypeScriptSdkGenShellSource = read(
  'sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/sdk-gen.sh',
);
const generatedTypeScriptSdkGenBatchSource = read(
  'sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/sdk-gen.bat',
);
const generatedTypeScriptPublishCoreSource = read(
  'sdkwork-control-plane-sdk-typescript/generated/server-openapi/bin/publish-core.mjs',
);

appendScriptInvocationFailures({
  source: ps1Source,
  failures,
  label: 'PowerShell generator wrapper',
  invocations: [
    {
      pattern: /fetch-openapi-source\.mjs/,
      description: 'fetch-openapi-source.mjs',
    },
    {
      pattern: /prepare-openapi-source\.mjs/,
      description: 'prepare-openapi-source.mjs',
    },
    {
      pattern: /verify-sdk\.mjs/,
      description: 'verify-sdk.mjs',
    },
  ],
});
appendScriptInvocationFailures({
  source: shSource,
  failures,
  label: 'Shell generator wrapper',
  invocations: [
    {
      pattern: /fetch-openapi-source\.mjs/,
      description: 'fetch-openapi-source.mjs',
    },
    {
      pattern: /prepare-openapi-source\.mjs/,
      description: 'prepare-openapi-source.mjs',
    },
    {
      pattern: /verify-sdk\.mjs/,
      description: 'verify-sdk.mjs',
    },
  ],
});

requireMatch({
  source: readmeSource,
  pattern: /checked-in[\s\S]*authority[\s\S]*OpenAPI 3\.x|checked-in[\s\S]*authority[\s\S]*contract/i,
  message: 'Workspace README must document the checked-in control-plane authority contract.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /control-plane\.openapi\.yaml/,
  message: 'Workspace README must reference control-plane.openapi.yaml.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /ControlPlaneSdkClient\.create/,
  message: 'Workspace README must document flat ControlPlaneSdkClient.create(...) usage.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /socialRuntime|social-runtime/,
  message: 'Workspace README must document the socialRuntime module family.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /control-plane\/social/,
  message: 'Workspace README must reference the social control-plane API page.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /control-plane\/social-runtime/,
  message: 'Workspace README must reference the social runtime control-plane API page.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /verify-sdk\.mjs --language typescript --language flutter/,
  message: 'Workspace README must document the cross-language verify-sdk command.',
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
});
requireMatch({
  source: readmeSource,
  pattern: /verify-flutter-dart-analysis\.dart/,
  message: 'Workspace README must document the Windows Flutter Dart analysis fallback entrypoint.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /@sdkwork\/control-plane-sdk/,
  message: 'Workspace README must reference the composed TypeScript package name.',
  failures,
});
requireMatch({
  source: readmeSource,
  pattern: /control_plane_sdk/,
  message: 'Workspace README must reference the composed Flutter package name.',
  failures,
});
appendAssemblyMetadataDocumentationFailures({
  source: readmeSource,
  failures,
  label: 'Workspace README',
  explainAssemblyMetadata: true,
  requireGeneratedComposed: true,
});

requireMatch({
  source: typescriptReadmeSource,
  pattern: /sdk-gen/,
  message: 'TypeScript workspace README must document sdk-gen.',
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
  pattern: /ControlPlaneSdkClient\.create/,
  message: 'TypeScript workspace README must document ControlPlaneSdkClient.create(...).',
  failures,
});
requireNotMatch({
  source: typescriptReadmeSource,
  pattern: /backendConfig/,
  message: 'TypeScript workspace README must not document backendConfig as public consumer API.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /socialRuntime|social-runtime/,
  message: 'TypeScript workspace README must document the socialRuntime module.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /control-plane\/social/,
  message: 'TypeScript workspace README must reference the social control-plane API page.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /control-plane\/social-runtime/,
  message: 'TypeScript workspace README must reference the social runtime control-plane API page.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /verify-typescript-workspace\.mjs/,
  message: 'TypeScript workspace README must document verify-typescript-workspace.mjs.',
  failures,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /verify-sdk\.mjs --language typescript/,
  message: 'TypeScript workspace README must document the language-scoped verify-sdk command.',
  failures,
});
appendVerificationFlowDocumentationFailures({
  source: typescriptReadmeSource,
  failures,
  label: 'TypeScript workspace README',
  requireUsageSurface: true,
});
requireMatch({
  source: typescriptReadmeSource,
  pattern: /\/api\/admin\/\*/,
  message: 'TypeScript workspace README must explain the manual-owned /api/admin/* helper boundary.',
  failures,
});

requireMatch({
  source: flutterReadmeSource,
  pattern: /sdk-gen/,
  message: 'Flutter workspace README must document sdk-gen.',
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
  pattern: /ControlPlaneSdkClient\.create/,
  message: 'Flutter workspace README must document ControlPlaneSdkClient.create(...).',
  failures,
});
requireNotMatch({
  source: flutterReadmeSource,
  pattern: /backendConfig/,
  message: 'Flutter workspace README must not document backendConfig as public consumer API.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /socialRuntime|social-runtime/,
  message: 'Flutter workspace README must document the socialRuntime module.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /control-plane\/social/,
  message: 'Flutter workspace README must reference the social control-plane API page.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /control-plane\/social-runtime/,
  message: 'Flutter workspace README must reference the social runtime control-plane API page.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /verify-flutter-workspace\.mjs/,
  message: 'Flutter workspace README must document verify-flutter-workspace.mjs.',
  failures,
});
requireMatch({
  source: flutterReadmeSource,
  pattern: /verify-sdk\.mjs --language flutter --with-dart/,
  message: 'Flutter workspace README must document the native Dart verify-sdk command.',
  failures,
});
appendVerificationFlowDocumentationFailures({
  source: flutterReadmeSource,
  failures,
  label: 'Flutter workspace README',
  requireUsageSurface: true,
  requirePackageMetadata: true,
});

requireMatch({
  source: verifyFlutterWorkspaceSource,
  pattern: /verify-flutter-usage-surface\.mjs/,
  message: 'verify-flutter-workspace.mjs must execute verify-flutter-usage-surface.mjs.',
  failures,
});
requireMatch({
  source: typescriptWorkspaceVerifierSource,
  pattern: /verify-typescript-generated-sdk-gen-runtime\.mjs/,
  message: 'verify-typescript-workspace.mjs must execute verify-typescript-generated-sdk-gen-runtime.mjs.',
  failures,
});
requireMatch({
  source: typescriptWorkspaceVerifierSource,
  pattern: /typescript:generated-sdk-gen-runtime/,
  message: 'verify-typescript-workspace.mjs must label the generated sdk-gen runtime step as typescript:generated-sdk-gen-runtime.',
  failures,
});
requireMatch({
  source: npmRuntimeSource,
  pattern: /resolveNpmInvocation/,
  message: 'bin/npm-runtime.mjs must export resolveNpmInvocation.',
  failures,
});
requireMatch({
  source: npmRuntimeSource,
  pattern: /createNpmCommandArgs/,
  message: 'bin/npm-runtime.mjs must export createNpmCommandArgs.',
  failures,
});
requireMatch({
  source: generatedTypeScriptSdkGenCoreSource,
  pattern: /npm-runtime\.mjs/,
  message: 'TypeScript generated sdk-gen-core.mjs must import bin/npm-runtime.mjs.',
  failures,
});
requireMatch({
  source: generatedTypeScriptSdkGenCoreSource,
  pattern: /createNpmCommandArgs/,
  message: 'TypeScript generated sdk-gen-core.mjs must resolve npm through createNpmCommandArgs.',
  failures,
});
requireMatch({
  source: generatedTypeScriptSdkGenShellSource,
  pattern: /sdk-gen-core\.mjs/,
  message: 'TypeScript generated sdk-gen.sh must delegate to sdk-gen-core.mjs.',
  failures,
});
requireNotMatch({
  source: generatedTypeScriptSdkGenShellSource,
  pattern: /npm install && npm run build/,
  message: 'TypeScript generated sdk-gen.sh must not inline bare npm install/build commands.',
  failures,
});
requireMatch({
  source: generatedTypeScriptSdkGenBatchSource,
  pattern: /sdk-gen-core\.mjs/i,
  message: 'TypeScript generated sdk-gen.bat must delegate to sdk-gen-core.mjs.',
  failures,
});
requireNotMatch({
  source: generatedTypeScriptSdkGenBatchSource,
  pattern: /npm install && npm run build/i,
  message: 'TypeScript generated sdk-gen.bat must not inline bare npm install/build commands.',
  failures,
});
requireMatch({
  source: verifySdkSource,
  pattern: /typescript-publish-core\.test\.mjs/,
  message: 'verify-sdk.mjs must execute tests/typescript-publish-core.test.mjs for the TypeScript language.',
  failures,
});
requireMatch({
  source: verifySdkSource,
  pattern: /typescript:publish-core/,
  message: 'verify-sdk.mjs must label the TypeScript publish-core regression step as typescript:publish-core.',
  failures,
});
requireMatch({
  source: generatedTypeScriptPublishCoreSource,
  pattern: /npm-runtime\.mjs/,
  message: 'TypeScript generated publish-core.mjs must import bin/npm-runtime.mjs.',
  failures,
});
requireMatch({
  source: generatedTypeScriptPublishCoreSource,
  pattern: /createNpmCommandArgs/,
  message: 'TypeScript generated publish-core.mjs must resolve npm through createNpmCommandArgs.',
  failures,
});
requireNotMatch({
  source: generatedTypeScriptPublishCoreSource,
  pattern: /run\('npm'/,
  message: 'TypeScript generated publish-core.mjs must not execute bare npm commands directly.',
  failures,
});
for (const [label, source] of [
  ['generated TypeScript run-tsc.mjs', generatedTypeScriptRunTscSource],
  ['composed TypeScript run-tsc.mjs', composedTypeScriptRunTscSource],
]) {
  requireMatch({
    source,
    pattern: /generator-runtime\.mjs/,
    message: `${label} must import bin/generator-runtime.mjs.`,
    failures,
  });
  requireMatch({
    source,
    pattern: /resolveGeneratorModulePath/,
    message: `${label} must resolve the generator-owned TypeScript compiler through resolveGeneratorModulePath.`,
    failures,
  });
  requireNotMatch({
    source,
    pattern: /sdkwork-sdk-generator/,
    message: `${label} must not hardcode sdkwork-sdk-generator path math locally.`,
    failures,
  });
}
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
appendVerifySdkAutomationEntrypointFailures({
  source: verifySdkInfrastructureSource,
  failures,
  verb: 'execute',
});
appendScriptInvocationFailures({
  source: verifySdkInfrastructureSource,
  failures,
  label: 'verify-sdk.mjs',
  verb: 'execute',
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
      pattern: /verify-flutter-workspace\.mjs/,
      description: 'verify-flutter-workspace.mjs',
    },
  ],
});

finishVerification({ prefix: 'sdkwork-control-plane-sdk', failures });
