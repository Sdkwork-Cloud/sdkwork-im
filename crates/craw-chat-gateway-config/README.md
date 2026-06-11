# craw-chat-gateway-config

Domain: communication
Capability: chat
Package type: rust-crate
Status: standardizing

This README is the SDKWork module entrypoint for `craw-chat-gateway-config`. The machine-readable component contract is `specs/component.spec.json`; canonical standards are under `../../../sdkwork-specs/`.

## Public API

- `.`

## Required SDK Surface

- None declared in `specs/component.spec.json`.

## Configuration

Configuration keys, runtime entrypoints, and integration contracts are declared in `specs/component.spec.json`. Shared modules must receive configuration through typed bootstrap or service boundaries rather than reading host-local environment state directly.

Split mode defaults Appbase, Drive, and Notary foundation upstreams to the shared
`sdkwork-api-gateway` root. `CRAW_CHAT_FOUNDATION_API_GATEWAY_BASE_URL` and
`SDKWORK_API_GATEWAY_BASE_URL` override that root; `SDKWORK_API_GATEWAY_BIND` derives a local
`http://<bind>` root when no base URL is set. Per-surface upstream keys such as
`CRAW_CHAT_APPBASE_APP_API_UPSTREAM`, `CRAW_CHAT_DRIVE_APP_API_UPSTREAM`, and
`CRAW_CHAT_NOTARY_APP_API_UPSTREAM` are explicit split-deployment overrides.

## SaaS/Private/Local Behavior

This component follows the deployment and runtime rules referenced by its `canonicalSpecs` entries. SaaS, private, and local behavior must stay compatible with the relevant SDKWork specs before implementation changes are made.

## Security

Do not add secrets, live tokens, manual auth headers, or app-local credential handling to this module. Protected API and SDK access must use the generated SDK or approved service boundary declared in the component contract.

## Extension Points

Extension points are limited to public exports, runtime entrypoints, SDK clients, events, and config keys declared in `specs/component.spec.json`.

## Verification

- `cargo test --manifest-path apps/craw-chat/crates/craw-chat-gateway-config/Cargo.toml`

## Owner And Status

Owner and lifecycle status are tracked in `specs/component.spec.json`. Update that contract before changing public integration behavior.
