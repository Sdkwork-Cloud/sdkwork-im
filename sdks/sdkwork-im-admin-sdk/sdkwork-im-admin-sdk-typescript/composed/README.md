# @sdkwork/im-admin-sdk

Composed IM admin TypeScript SDK built on the generated admin backend transport package.

## Client Surface

The preferred consumer entrypoint is `ImAdminSdkClient`, which exposes the admin domains
published by the generated backend transport package.

## Package Boundary

- Consume generated transport symbols only through `@sdkwork/im-admin-backend-sdk`.
- Do not import `generated/server-openapi/src/*` private source paths from manual or downstream code.
