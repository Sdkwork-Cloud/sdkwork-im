import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const sharedVerifierSource = read('bin/verify-language-workspace-shared.mjs');

assert.match(
  sharedVerifierSource,
  /readmeRequiredTerms/,
  'verify-language-workspace-shared.mjs must support readmeRequiredTerms for per-language README verification.',
);
assert.match(
  sharedVerifierSource,
  /generatedReadmeRequiredTerms/,
  'verify-language-workspace-shared.mjs must support generatedReadmeRequiredTerms for generated README verification.',
);
assert.match(
  sharedVerifierSource,
  /composedReadmeRequiredTerms/,
  'verify-language-workspace-shared.mjs must support composedReadmeRequiredTerms for composed README verification.',
);
assert.match(
  sharedVerifierSource,
  /readmeForbiddenTerms/,
  'verify-language-workspace-shared.mjs must support readmeForbiddenTerms for per-language README verification.',
);
assert.match(
  sharedVerifierSource,
  /generatedReadmeForbiddenTerms/,
  'verify-language-workspace-shared.mjs must support generatedReadmeForbiddenTerms for generated README verification.',
);
assert.match(
  sharedVerifierSource,
  /composedReadmeForbiddenTerms/,
  'verify-language-workspace-shared.mjs must support composedReadmeForbiddenTerms for composed README verification.',
);

const languageVerifierExpectations = [
  {
    path: 'bin/verify-typescript-workspace.mjs',
    markers: [
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
    forbiddenMarkers: [
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
    generatedReadmeMarkers: [
      'Generator-owned TypeScript transport SDK',
      '@sdkwork/craw-chat-sdk',
      'CrawChatSdkClient',
      'standalone generated transport package',
    ],
    composedReadmeMarkers: [
      'manual-owned authoring source',
      'assemble-single-package.mjs',
      'live.messages.on(...)',
      'live.events.on(...)',
      'live.lifecycle.onStateChange(...)',
      'live.lifecycle.onError(...)',
    ],
    composedReadmeForbiddenMarkers: [
      'live.onMessage(',
      'live.onRawEvent(',
      'live.onStateChange(',
      'live.onError(',
    ],
  },
  {
    path: 'bin/verify-flutter-workspace.mjs',
    markers: [
      'sdk.createXxxMessage()',
      'sdk.send()',
      'sdk.decodeMessage()',
      'TypeScript',
      'craw_chat_sdk',
      'backend_sdk',
      'CrawChatClient',
      'AuthApi',
      'PortalApi',
      'sdk.auth',
      'sdk.portal',
      'client.auth',
      'client.portal',
      'WebSocket adapter',
    ],
    generatedReadmeMarkers: [
      'package:backend_sdk/backend_sdk.dart',
      'AuthApi',
      'PortalApi',
      'client.auth',
      'client.portal',
    ],
    composedReadmeMarkers: [
      'manual-owned consumer layer',
      'official Flutter app-consumer package',
      'package:craw_chat_sdk/craw_chat_sdk.dart',
      'AuthApi',
      'PortalApi',
      'sdk.auth',
      'sdk.portal',
      'client.auth',
      'client.portal',
      'WebSocket adapter',
    ],
  },
  {
    path: 'bin/verify-rust-workspace.mjs',
    markers: ['sdkwork-craw-chat-backend-sdk', 'craw_chat_sdk', 'Tier A', 'transport crate'],
  },
  {
    path: 'bin/verify-java-workspace.mjs',
    markers: ['com.sdkwork:craw-chat-backend-sdk', 'Tier B', 'transport artifact'],
  },
  {
    path: 'bin/verify-csharp-workspace.mjs',
    markers: ['Sdkwork.CrawChat.BackendSdk', 'Tier B', 'transport package'],
  },
  {
    path: 'bin/verify-swift-workspace.mjs',
    markers: ['CrawChatBackendSdk', 'Tier B', 'transport package'],
  },
  {
    path: 'bin/verify-kotlin-workspace.mjs',
    markers: ['com.sdkwork:craw-chat-backend-sdk', 'Tier B', 'transport artifact'],
  },
  {
    path: 'bin/verify-go-workspace.mjs',
    markers: ['github.com/sdkwork/craw-chat-backend-sdk', 'Tier B', 'transport module'],
  },
  {
    path: 'bin/verify-python-workspace.mjs',
    markers: ['sdkwork-craw-chat-backend-sdk', 'Tier B', 'transport package'],
  },
];

for (const expectation of languageVerifierExpectations) {
  const source = read(expectation.path);

  if (expectation.markers) {
    assert.match(
      source,
      /readmeRequiredTerms/,
      `${expectation.path} must pass readmeRequiredTerms to the shared workspace verifier.`,
    );
  }

  for (const marker of expectation.markers || []) {
    assert.match(
      source,
      new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
      `${expectation.path} must require README marker ${marker}.`,
    );
  }

  if (expectation.forbiddenMarkers) {
    assert.match(
      source,
      /readmeForbiddenTerms/,
      `${expectation.path} must pass readmeForbiddenTerms to the shared workspace verifier.`,
    );

    for (const marker of expectation.forbiddenMarkers) {
      assert.match(
        source,
        new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
        `${expectation.path} must forbid README marker ${marker}.`,
      );
    }
  }

  if (expectation.generatedReadmeMarkers) {
    assert.match(
      source,
      /generatedReadmeRequiredTerms/,
      `${expectation.path} must pass generatedReadmeRequiredTerms to the shared workspace verifier.`,
    );

    for (const marker of expectation.generatedReadmeMarkers) {
      assert.match(
        source,
        new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
        `${expectation.path} must require generated README marker ${marker}.`,
      );
    }
  }

  if (expectation.composedReadmeMarkers) {
    assert.match(
      source,
      /composedReadmeRequiredTerms/,
      `${expectation.path} must pass composedReadmeRequiredTerms to the shared workspace verifier.`,
    );

    for (const marker of expectation.composedReadmeMarkers) {
      assert.match(
        source,
        new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
        `${expectation.path} must require composed README marker ${marker}.`,
      );
    }
  }

  if (expectation.composedReadmeForbiddenMarkers) {
    assert.match(
      source,
      /composedReadmeForbiddenTerms/,
      `${expectation.path} must pass composedReadmeForbiddenTerms to the shared workspace verifier.`,
    );

    for (const marker of expectation.composedReadmeForbiddenMarkers) {
      assert.match(
        source,
        new RegExp(marker.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
        `${expectation.path} must forbid composed README marker ${marker}.`,
      );
    }
  }
}

console.log('language workspace README term contract test passed');
