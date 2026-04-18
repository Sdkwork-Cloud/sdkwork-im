# Realtime Extension Boundary

This document defines what belongs inside OpenAPI-generated transport and what must remain manual.

It exists because the Craw Chat app contract includes realtime HTTP coordination and a WebSocket
handshake route, but the SDKs do not treat the full realtime transport protocol as generator-owned.

The handshake route documented in the app contract is `GET /api/v1/realtime/ws`.

## What Generation Owns

OpenAPI generation owns the HTTP surface that is directly modeled in the schema:

- session resume
- session disconnect
- presence heartbeat
- realtime subscription sync
- realtime event pull
- realtime ACK flow
- the WebSocket handshake route as an HTTP contract reference

## What Generation Does Not Own

OpenAPI generation does not own:

- a full WebSocket adapter
- frame lifecycle management after upgrade
- transport reconnect orchestration
- protocol negotiation helpers beyond the documented HTTP handshake
- provider-specific realtime runtime glue

Those concerns must remain in the manual-owned realtime extension boundary outside generated output.

## TypeScript Rule

For TypeScript, handwritten realtime extensions belong in manual root-package modules, not in
`generated/server-openapi` and not in `src/generated/**`.

The single-package standard is preserved by keeping:

- generated transport under `src/generated/**`
- manual realtime composition in root `src/**`

## Flutter Rule

For Flutter, handwritten realtime extensions belong in the manual `composed` package or future
manual modules outside `generated/server-openapi`.

The official consumer package remains:

- `craw_chat_sdk`

The generator-owned transport boundary remains:

- `backend_sdk`

Do not hide manual realtime protocol code inside generated package internals.

## Documentation Rule

When documenting realtime in public or internal docs:

- describe HTTP resume, sync, pull, and ACK flow as delivered SDK surface
- describe WebSocket transport as a manual-owned realtime extension boundary unless a real shipped
  adapter is added later
- keep package names and real client names explicit

## Related Docs

- package standards: `package-standards.md`
- generation pipeline: `generation-pipeline.md`
- verification matrix: `verification-matrix.md`
- public docs sync standard: `sites/README.md`
