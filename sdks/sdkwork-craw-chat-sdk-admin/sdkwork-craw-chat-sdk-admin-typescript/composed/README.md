# @sdkwork/craw-chat-sdk-admin

Composed Craw Chat admin TypeScript SDK built on top of the generated control-plane backend SDK.

## Package Role

- transport package: `@sdkwork/craw-chat-admin-backend-sdk`
- composed package: `@sdkwork/craw-chat-sdk-admin`

Use this package for admin and control-plane consumers that need the stable client surface
`CrawChatSdkAdminClient`.

## Client Surface

`CrawChatSdkAdminClient` exposes:

- `protocol`
- `providers`
- `cluster`
- `social`
- `system`

## Usage

```ts
import { CrawChatSdkAdminClient } from '@sdkwork/craw-chat-sdk-admin';

const sdk = await CrawChatSdkAdminClient.create({
  backendConfig: {
    baseUrl: 'http://127.0.0.1:18081',
    authToken: 'admin-token',
  },
});

const bindings = await sdk.providers.getProviderBindings();
console.log(bindings);
```
