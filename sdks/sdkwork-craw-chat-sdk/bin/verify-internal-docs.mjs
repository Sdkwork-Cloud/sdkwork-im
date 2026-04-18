#!/usr/bin/env node
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const issues = [];

function read(relativePath) {
  return readFileSync(path.join(workspaceRoot, relativePath), 'utf8');
}

function readRequired(relativePath) {
  const absolutePath = path.join(workspaceRoot, relativePath);
  try {
    return readFileSync(absolutePath, 'utf8');
  } catch {
    issues.push(`${relativePath}: file is required`);
    return '';
  }
}

function requireIncludes(source, relativePath, needle, message) {
  if (!source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

function requireExcludes(source, relativePath, needle, message) {
  if (source.includes(needle)) {
    issues.push(`${relativePath}: ${message}`);
  }
}

const workspaceReadmePath = 'README.md';
const workspaceReadmeSource = read(workspaceReadmePath);
for (const marker of [
  '## Verification',
  '## Documentation Standards',
  'docs/README.md',
  'docs/sites/README.md',
  'live service schema',
  'verify-sdk',
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
  'single-package',
  'src/generated/**',
  'CrawChatSdkClient',
  '/openapi/craw-chat-app.openapi.yaml',
  'bin/start-local.ps1',
  'bin/start-local.sh',
  'verify-typescript-live-contract.mjs',
  'runtime root-export validation',
  'dead-auth/dead-residue cleanup',
  'npm pack --dry-run',
  '--with-dart',
  '-WithDart',
  '.sdkwork/dart/pub-cache',
  'verify-flutter-dart-analysis.dart',
]) {
  requireIncludes(workspaceReadmeSource, workspaceReadmePath, marker, `must include ${marker}`);
}
requireExcludes(
  workspaceReadmeSource,
  workspaceReadmePath,
  'TypeScript composed package:',
  'must not describe the TypeScript consumer contract as a composed package anymore',
);

const docsReadmePath = 'docs/README.md';
const docsReadmeSource = read(docsReadmePath);
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'package-standards.md',
  'must link to package-standards.md',
);
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'generation-pipeline.md',
  'must link to generation-pipeline.md',
);
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'verification-matrix.md',
  'must link to verification-matrix.md',
);
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'realtime-extension-boundary.md',
  'must link to realtime-extension-boundary.md',
);
for (const marker of [
  'multilanguage-generator-standard.md',
  'multilanguage-capability-matrix.md',
  'multilanguage-audit-report.md',
]) {
  requireIncludes(docsReadmeSource, docsReadmePath, marker, `must link to ${marker}`);
}
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'sites/README.md',
  'must link to sites/README.md',
);
requireIncludes(
  docsReadmeSource,
  docsReadmePath,
  'bin/verify-docs-contract-tests.mjs',
  'must document bin/verify-docs-contract-tests.mjs',
);
for (const marker of [
  'workspace-internal documentation map',
  '@sdkwork/craw-chat-sdk',
  'craw_chat_sdk',
  '/openapi/craw-chat-app.openapi.yaml',
  'verify-sdk.mjs',
  'verify-sdk-automation.mjs',
  'verify-typescript-live-contract.mjs',
]) {
  requireIncludes(docsReadmeSource, docsReadmePath, marker, `must include ${marker}`);
}

const packageStandardsPath = 'docs/package-standards.md';
const packageStandardsSource = read(packageStandardsPath);
for (const heading of [
  '## TypeScript Standard',
  '### TypeScript Ownership Boundary',
  '### TypeScript Public Contract',
  '## Flutter Standard',
  '### Flutter Ownership Boundary',
  '### Flutter Public Contract',
  '## Shared Naming Rules',
  '## Source Of Truth Links',
]) {
  requireIncludes(packageStandardsSource, packageStandardsPath, heading, `must include heading ${heading}`);
}
for (const marker of [
  '@sdkwork/craw-chat-sdk',
  'CrawChatSdkClient',
  'CrawChatClient',
  'SdkworkBackendClient',
  'createGeneratedBackendClient',
  'generated/server-openapi',
  'src/generated/**',
  'craw_chat_sdk',
  'backend_sdk',
  'sites/README.md',
  'live.messages',
  'connected`, `error`, and `closed`',
]) {
  requireIncludes(packageStandardsSource, packageStandardsPath, marker, `must include ${marker}`);
}

const generationPipelinePath = 'docs/generation-pipeline.md';
const generationPipelineSource = read(generationPipelinePath);
for (const heading of [
  '## Authority Source',
  '## Required Generation Sequence',
  '## TypeScript Pipeline',
  '## Flutter Pipeline',
  '## Generator Boundary Rules',
  '## Primary Commands',
  '## Related Docs',
]) {
  requireIncludes(
    generationPipelineSource,
    generationPipelinePath,
    heading,
    `must include heading ${heading}`,
  );
}
for (const marker of [
  '/openapi/craw-chat-app.openapi.yaml',
  'bin/start-local.ps1',
  'bin/start-local.sh',
  'src/generated/**',
  'backend_sdk',
  'verify-sdk.mjs',
  'sites/README.md',
]) {
  requireIncludes(generationPipelineSource, generationPipelinePath, marker, `must include ${marker}`);
}

const verificationMatrixPath = 'docs/verification-matrix.md';
const verificationMatrixSource = read(verificationMatrixPath);
for (const heading of [
  '## Root Entry Point',
  '## Shared Automation Checks',
  '## TypeScript Checks',
  '## Flutter Checks',
  '## Docs Checks',
  '## Release Gate Expectation',
  '## Related Docs',
]) {
  requireIncludes(
    verificationMatrixSource,
    verificationMatrixPath,
    heading,
    `must include heading ${heading}`,
  );
}
for (const marker of [
  'bin/verify-sdk.mjs',
  'bin/verify-internal-docs.mjs',
  'bin/verify-docs-contract-tests.mjs',
  'bin/verify-sdk-automation.mjs',
  'bin/verify-typescript-generated-build-determinism.mjs',
  'bin/verify-flutter-dart-analysis.dart',
  'bin/verify-typescript-live-contract.mjs',
  'payload-first',
  'verify-api-docs.mjs',
  'verify-sdk-docs.mjs',
  'sites/README.md',
]) {
  requireIncludes(verificationMatrixSource, verificationMatrixPath, marker, `must include ${marker}`);
}

const realtimeBoundaryPath = 'docs/realtime-extension-boundary.md';
const realtimeBoundarySource = read(realtimeBoundaryPath);
for (const heading of [
  '## What Generation Owns',
  '## What Generation Does Not Own',
  '## TypeScript Rule',
  '## Flutter Rule',
  '## Documentation Rule',
  '## Related Docs',
]) {
  requireIncludes(
    realtimeBoundarySource,
    realtimeBoundaryPath,
    heading,
    `must include heading ${heading}`,
  );
}
for (const marker of [
  '/api/v1/realtime/ws',
  'src/generated/**',
  'craw_chat_sdk',
  'backend_sdk',
  'sites/README.md',
  'manual-owned realtime extension boundary',
]) {
  requireIncludes(realtimeBoundarySource, realtimeBoundaryPath, marker, `must include ${marker}`);
}

const publicDocsSyncPath = 'docs/sites/README.md';
const publicDocsSyncSource = read(publicDocsSyncPath);
for (const heading of [
  '## Public Docs Ownership Map',
  '## Contract Markers That Must Stay Visible',
  '## Wording Rules',
  '## Sync Workflow',
  '## Change Triggers That Require Public Docs Review',
  '## Related Internal Docs',
]) {
  requireIncludes(
    publicDocsSyncSource,
    publicDocsSyncPath,
    heading,
    `must include heading ${heading}`,
  );
}
for (const marker of [
  'docs/sites/sdk/typescript-sdk.md',
  'docs/sites/sdk/flutter-sdk.md',
  'docs/sites/sdk/rust-sdk.md',
  'docs/sites/sdk/java-sdk.md',
  'docs/sites/sdk/csharp-sdk.md',
  'docs/sites/sdk/swift-sdk.md',
  'docs/sites/sdk/kotlin-sdk.md',
  'docs/sites/sdk/go-sdk.md',
  'docs/sites/sdk/python-sdk.md',
  'docs/sites/sdk/generator-boundary.md',
  'docs/sites/api-reference/app/portal-and-auth.md',
  '@sdkwork/craw-chat-sdk',
  'CrawChatSdkClient',
  'craw_chat_sdk',
  'backend_sdk',
  'Tier A',
  'Tier B',
  '/openapi/craw-chat-app.openapi.yaml',
  'verify-api-docs.mjs',
  'verify-sdk-docs.mjs',
  'docs-verify.mjs',
  'two-package consumer model',
  'generated-versus-composed',
]) {
  requireIncludes(publicDocsSyncSource, publicDocsSyncPath, marker, `must include ${marker}`);
}

const multilanguageGeneratorStandardPath = 'docs/multilanguage-generator-standard.md';
const multilanguageGeneratorStandardSource = readRequired(multilanguageGeneratorStandardPath);
for (const marker of [
  '# Multilanguage Generator Standard',
  '## Official Language Set',
  '## Live Schema Refresh',
  '## Generated Versus Semantic Boundary',
  '## Verification Contract',
  '## Workspace-Owned Fixes Versus External Generator Gaps',
  'typescript',
  'flutter',
  'rust',
  'java',
  'csharp',
  'swift',
  'kotlin',
  'go',
  'python',
  '/openapi/craw-chat-app.openapi.yaml',
  'generate-sdk.ps1',
  'generate-sdk.sh',
  'verify-sdk.mjs',
  'normalize-generated-auth-surface.mjs',
  'assemble-sdk.mjs',
]) {
  requireIncludes(
    multilanguageGeneratorStandardSource,
    multilanguageGeneratorStandardPath,
    marker,
    `must include ${marker}`,
  );
}

const multilanguageCapabilityMatrixPath = 'docs/multilanguage-capability-matrix.md';
const multilanguageCapabilityMatrixSource = readRequired(multilanguageCapabilityMatrixPath);
for (const marker of [
  '# Multilanguage Capability Matrix',
  'Tier A',
  'Tier B',
  'TypeScript',
  'Flutter',
  'Rust',
  'Java',
  'C#',
  'Swift',
  'Kotlin',
  'Go',
  'Python',
  '@sdkwork/craw-chat-sdk',
  'craw_chat_sdk',
  'sdkwork-craw-chat-backend-sdk',
  'com.sdkwork:craw-chat-backend-sdk',
  'Sdkwork.CrawChat.BackendSdk',
  'CrawChatBackendSdk',
  'github.com/sdkwork/craw-chat-backend-sdk',
]) {
  requireIncludes(
    multilanguageCapabilityMatrixSource,
    multilanguageCapabilityMatrixPath,
    marker,
    `must include ${marker}`,
  );
}

const multilanguageAuditReportPath = 'docs/multilanguage-audit-report.md';
const multilanguageAuditReportSource = readRequired(multilanguageAuditReportPath);
for (const marker of [
  '# Multilanguage Audit Report',
  '## Audit Scope',
  '## TypeScript',
  '## Flutter',
  '## Rust',
  '## Java',
  '## C#',
  '## Swift',
  '## Kotlin',
  '## Go',
  '## Python',
  'generation status',
  'verification status',
  'action required in workspace',
  'action required in external generator',
]) {
  requireIncludes(
    multilanguageAuditReportSource,
    multilanguageAuditReportPath,
    marker,
    `must include ${marker}`,
  );
}

const openApiReadmePath = 'openapi/README.md';
const openApiReadmeSource = read(openApiReadmePath);
for (const marker of [
  '# SDKWork Craw Chat SDK OpenAPI Sources',
  '## Files',
  '## Rules',
  '## Related Docs',
  '/openapi/craw-chat-app.openapi.yaml',
  'CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH',
  'last successful live schema snapshot',
  'generation-pipeline.md',
]) {
  requireIncludes(openApiReadmeSource, openApiReadmePath, marker, `must include ${marker}`);
}

if (issues.length > 0) {
  console.error(issues.join('\n'));
  process.exit(1);
}

console.log('[sdkwork-craw-chat-sdk] Internal docs verification passed.');
