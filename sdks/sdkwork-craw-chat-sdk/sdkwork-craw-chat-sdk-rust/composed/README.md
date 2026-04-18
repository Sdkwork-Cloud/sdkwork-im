# Craw Chat SDK Rust Composed Crate

This crate is the manual-owned Rust composition layer for Craw Chat.

## Boundary

- generated crate: `sdkwork-craw-chat-backend-sdk`
  Generator-owned transport crate in `generated/server-openapi`.
- composed crate: `craw-chat-sdk`
  Manual-owned consumer crate in `composed`.

Do not hand-edit generated Rust output. Regenerate from the root workspace wrappers instead.
Manual Rust code in this crate must consume generated APIs through the generated crate root exports, not through generated private modules.

## Public API

- crate name: `craw-chat-sdk`
- library name: `craw_chat_sdk`
- primary entrypoint: `CrawChatClient`
- main modules:
  - `session`
  - `presence`
  - `realtime`
  - `devices`
  - `inbox`
  - `conversations`
  - `messages`
  - `media`
  - `streams`
  - `rtc`
- convenience builders:
  - text message and text edit builders
  - text stream frame builder
  - JSON RTC signal builder

## Scope

This crate wraps the generated HTTP SDK with semantic client modules and request builders.
The websocket transport is documented at the workspace root but is not implemented in this round.
