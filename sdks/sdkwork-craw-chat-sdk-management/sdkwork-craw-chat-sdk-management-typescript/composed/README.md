# @sdkwork/craw-chat-sdk-management

Composed management TypeScript SDK built on the generated management backend transport package.

## Client Surface

The preferred consumer entrypoint is `CrawChatSdkManagementClient`, which exposes the management domains published by the generated backend transport package.
Use the composed package by default for operator-console and management integrations.

## Package Boundary

- Consume generated transport symbols only through `@sdkwork/craw-chat-management-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private source paths from manual or downstream code.
