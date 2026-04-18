import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';

function read(relativePath) {
  return readFileSync(new URL(`../${relativePath}`, import.meta.url), 'utf8');
}

const workspaceExpectations = [
  {
    path: 'sdkwork-craw-chat-sdk-rust/README.md',
    required: [
      'sdkwork-craw-chat-backend-sdk',
      'craw_chat_sdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier A',
      'transport crate',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-java/README.md',
    required: [
      'com.sdkwork:craw-chat-backend-sdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport artifact',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-csharp/README.md',
    required: [
      'Sdkwork.CrawChat.BackendSdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport package',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-swift/README.md',
    required: [
      'CrawChatBackendSdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport package',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-kotlin/README.md',
    required: [
      'com.sdkwork:craw-chat-backend-sdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport artifact',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-go/README.md',
    required: [
      'github.com/sdkwork/craw-chat-backend-sdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport module',
    ],
  },
  {
    path: 'sdkwork-craw-chat-sdk-python/README.md',
    required: [
      'sdkwork-craw-chat-backend-sdk',
      'CrawChatSdkClient',
      'generated/server-openapi',
      'composed',
      'Tier B',
      'transport package',
    ],
  },
];

for (const expectation of workspaceExpectations) {
  const source = read(expectation.path);
  for (const needle of expectation.required) {
    assert.match(
      source,
      new RegExp(needle.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')),
      `${expectation.path} must include ${needle}.`,
    );
  }
}

console.log('multilanguage workspace README contract test passed');
