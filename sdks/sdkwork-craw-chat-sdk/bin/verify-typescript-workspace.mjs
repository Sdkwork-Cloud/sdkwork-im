#!/usr/bin/env node
import { spawnSync } from 'node:child_process';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { resolveSdkworkGeneratorRoot } from './sdk-paths.mjs';
import { verifyLanguageWorkspace } from './verify-language-workspace-shared.mjs';

function fail(message) {
  console.error(`[sdkwork-craw-chat-sdk] ${message}`);
  process.exit(1);
}

function parseArgs(argv) {
  if (argv.length > 0) {
    fail(`Unknown argument: ${argv[0]}`);
  }
}

function run(command, args, options = {}) {
  const result = spawnSync(command, args, {
    cwd: options.cwd,
    stdio: 'inherit',
    shell: false,
    timeout: options.timeoutMs,
  });

  if (result.error) {
    fail(`${options.step || command} failed to start: ${result.error.message}`);
  }
  if (typeof result.status === 'number' && result.status !== 0) {
    fail(`${options.step || command} failed with exit code ${result.status}`);
  }
  if (result.signal) {
    fail(`${options.step || command} terminated with signal ${result.signal}`);
  }
}

parseArgs(process.argv.slice(2));

verifyLanguageWorkspace({
  language: 'typescript',
  workspace: 'sdkwork-craw-chat-sdk-typescript',
  primaryClient: 'CrawChatSdkClient',
  maturityTier: 'tier-a',
  requiredPackageLayers: ['generated', 'composed', 'root'],
  readmeRequiredTerms: [
    'sdk.createTextMessage(...)',
    'sdk.send(...)',
    'sdk.uploadAndSendMessage(...)',
    'sdk.decodeMessage(...)',
    'createAiImageGenerationMessage',
    'createAiVideoGenerationMessage',
    'createAgentMessage',
    'sdk.sync.ack(...)',
    'context.ack()',
    'sdk.conversations.createAgentDialog(...)',
    'sdk.conversations.postText(...)',
    'sdk.generated.inbox.getInbox()',
    'sdk.media.uploadAndComplete(...)',
    'sdk.media.attachText(...)',
    'sdk.rtc.postJsonSignal(...)',
    'live.messages.on(...)',
    'live.messages.onConversation(...)',
    'live.data.on(...)',
    'live.signals.on(...)',
    'live.signals.onRtcSession(...)',
    'live.events.on(...)',
    'live.lifecycle.onStateChange(...)',
    'live.lifecycle.onError(...)',
    'live.lifecycle.getState()',
    'rtcMode',
    'signalingStreamId',
    'sdk.rtc.issueParticipantCredential(...)',
    'sdk.rtc.getRecordingArtifact(...)',
    'browser and Node.js',
    'SdkworkBackendClient',
    'createGeneratedBackendClient',
    'verify-typescript-workspace-concurrency.mjs',
    'verify-typescript-live-contract.mjs',
    'runtime root exports',
    'dead auth scaffolding',
  ],
  readmeForbiddenTerms: [
    'live.onMessage(',
    'live.onConversationMessage(',
    'live.onData(',
    'live.onSignal(',
    'live.onRawEvent(',
    'live.onStateChange(',
    'live.onError(',
    'createAiImage(',
    'createAiVideo(',
    'participantIds',
    '`connecting`, `connected`, `error`, and `closed`',
  ],
  generatedReadmeRequiredTerms: [
    'Generator-owned TypeScript transport SDK',
    '@sdkwork/craw-chat-sdk',
    'CrawChatSdkClient',
    'standalone generated transport package',
  ],
  composedReadmeRequiredTerms: [
    'manual-owned authoring source',
    'assemble-single-package.mjs',
    'live.messages.on(...)',
    'live.events.on(...)',
    'live.lifecycle.onStateChange(...)',
    'live.lifecycle.onError(...)',
  ],
  composedReadmeForbiddenTerms: [
    'live.onMessage(',
    'live.onRawEvent(',
    'live.onStateChange(',
    'live.onError(',
  ],
  consumerPackage: {
    name: '@sdkwork/craw-chat-sdk',
    packagePath: 'sdkwork-craw-chat-sdk-typescript',
    manifestPath: 'sdkwork-craw-chat-sdk-typescript/package.json',
  },
});

const scriptDir = path.dirname(fileURLToPath(import.meta.url));
const workspaceRoot = path.resolve(scriptDir, '..');
const generatorRoot = resolveSdkworkGeneratorRoot(workspaceRoot);
const typescriptPackageRoot = path.join(
  workspaceRoot,
  'sdkwork-craw-chat-sdk-typescript',
);
const composedTypeScriptPackageRoot = path.join(
  typescriptPackageRoot,
  'composed',
);
const typescriptCompilerCliPath = path.join(
  generatorRoot,
  'node_modules',
  'typescript',
  'bin',
  'tsc',
);
const rootPackageAssemblePath = path.join(
  typescriptPackageRoot,
  'bin',
  'assemble-single-package.mjs',
);
const composedPackageVerifyPath = path.join(
  scriptDir,
  'verify-typescript-composed-package-layout.mjs',
);
const composedPackageCleanPath = path.join(
  composedTypeScriptPackageRoot,
  'bin',
  'clean-dist.mjs',
);
const composedPackageSmokePath = path.join(
  composedTypeScriptPackageRoot,
  'test',
  'craw-chat-client.test.mjs',
);
const typeScriptLiveContractVerifyPath = path.join(
  scriptDir,
  'verify-typescript-live-contract.mjs',
);
const rootPackageVerifyPath = path.join(
  scriptDir,
  'verify-typescript-single-package-layout.mjs',
);
const rootPackageCleanPath = path.join(
  typescriptPackageRoot,
  'bin',
  'clean-dist.mjs',
);
const rootPackageSmokePath = path.join(
  typescriptPackageRoot,
  'test',
  'craw-chat-client.test.mjs',
);

run('node', [path.join(scriptDir, 'build-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-build',
});
run('node', [path.join(scriptDir, 'verify-typescript-generated-package-install-safety.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-package-install-safety',
});
run('node', [path.join(scriptDir, 'verify-typescript-generated-package.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-package',
});
run('node', [path.join(scriptDir, 'verify-typescript-generated-package-temp-cleanup.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:generated-package-temp-cleanup',
});
run('node', [path.join(scriptDir, 'verify-auth-surface-alignment.mjs'), '--language', 'typescript'], {
  cwd: workspaceRoot,
  step: 'typescript:auth-surface',
});
run('node', [path.join(scriptDir, 'verify-typescript-usage-surface.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:usage-surface',
});
run('node', [path.join(scriptDir, 'verify-typescript-public-api-boundary.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:public-api-boundary',
});
run('node', [composedPackageVerifyPath], {
  cwd: workspaceRoot,
  step: 'typescript:composed-package-layout',
});
run(process.execPath, [typescriptCompilerCliPath, '-p', 'tsconfig.build.json', '--noEmit'], {
  cwd: composedTypeScriptPackageRoot,
  step: 'typescript:composed-typecheck',
});
run(process.execPath, [typescriptCompilerCliPath, '-p', 'tsconfig.build.json'], {
  cwd: composedTypeScriptPackageRoot,
  step: 'typescript:composed-build',
});
run('node', [composedPackageCleanPath], {
  cwd: composedTypeScriptPackageRoot,
  step: 'typescript:composed-clean',
});
run('node', [typeScriptLiveContractVerifyPath, '--package', 'composed'], {
  cwd: workspaceRoot,
  step: 'typescript:composed-live-contract',
});
run('node', [composedPackageSmokePath], {
  cwd: composedTypeScriptPackageRoot,
  step: 'typescript:composed-test',
});
run('node', [rootPackageAssemblePath], {
  cwd: workspaceRoot,
  step: 'typescript:single-package-assemble',
});
run('node', [rootPackageVerifyPath], {
  cwd: workspaceRoot,
  step: 'typescript:single-package-layout',
});
run(process.execPath, [typescriptCompilerCliPath, '-p', 'tsconfig.build.json', '--noEmit'], {
  cwd: typescriptPackageRoot,
  step: 'typescript:typecheck',
});
run('node', [rootPackageCleanPath], {
  cwd: typescriptPackageRoot,
  step: 'typescript:clean',
});
run(process.execPath, [typescriptCompilerCliPath, '-p', 'tsconfig.build.json'], {
  cwd: typescriptPackageRoot,
  step: 'typescript:build',
});
run('node', [typeScriptLiveContractVerifyPath, '--package', 'root'], {
  cwd: workspaceRoot,
  step: 'typescript:live-contract',
});
run('node', [rootPackageSmokePath], {
  cwd: typescriptPackageRoot,
  step: 'typescript:test',
});
run('node', [path.join(scriptDir, 'verify-typescript-single-package-publishability.mjs')], {
  cwd: workspaceRoot,
  step: 'typescript:single-package-publishability',
});
