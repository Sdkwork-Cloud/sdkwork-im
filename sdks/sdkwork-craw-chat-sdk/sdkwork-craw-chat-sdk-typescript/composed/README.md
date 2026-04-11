# @sdkwork/craw-chat-sdk

Composed TypeScript SDK for Craw Chat.

This package sits above the generated `@sdkwork/craw-chat-backend-sdk` package and provides:

- a consumer-facing `CrawChatClient`
- business-oriented module names
- convenience builders for common message, stream, and RTC flows

`generated/server-openapi` remains generator-owned. This `composed` package is manual-owned.

Within this workspace, manual code may reference generated type files only through `src/generated-backend-types.ts`.
Do not import from `../generated/server-openapi/src/types/*` anywhere else in the composed package.

## Usage

```ts
import { CrawChatClient } from '@sdkwork/craw-chat-sdk';

const sdk = await CrawChatClient.create({
  backendConfig: {
    baseUrl: 'https://api.example.com',
    authToken: '<token>',
  },
});

await sdk.conversations.postText('conversation-1', 'hello world', {
  clientMsgId: 'msg-1',
});
```

## Verification

Local verification used in this workspace:

- `node ../../bin/verify-typescript-public-api-boundary.mjs`
- `node ../../bin/build-typescript-generated-package.mjs`
- `node ../../bin/verify-typescript-generated-package.mjs`
- `node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json --noEmit`
- `node ../../../../../../sdk/sdkwork-sdk-generator/node_modules/typescript/bin/tsc -p tsconfig.build.json`
- `node ./test/craw-chat-client.test.mjs`
