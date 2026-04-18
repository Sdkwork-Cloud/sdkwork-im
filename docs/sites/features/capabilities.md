# Capability Matrix

## App Collaboration

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Conversations | Standard conversations, agent dialogs, agent handoffs, and system channels | `services/local-minimal-node/src/node/build.rs`, `services/local-minimal-node/tests/http_e2e_test.rs` |
| Membership | List, add, remove, transfer owner, change role, and leave | `services/local-minimal-node/src/node/build.rs` |
| Messages | Send, edit, recall, timeline reads, and system-channel publish | `services/local-minimal-node/tests/http_e2e_test.rs` |
| Read models | Inbox, conversation summary, and read cursor | `services/local-minimal-node/src/node/build.rs` |
| Agent handoff state | Read, accept, resolve, and close handoff state | `services/local-minimal-node/src/node/build.rs` |

## Session, Presence, and Realtime

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Session | Resume and disconnect | `services/local-minimal-node/src/node/build.rs` |
| Presence | Heartbeat and current presence | `services/local-minimal-node/src/node/build.rs` |
| Realtime delivery | Subscription sync, event polling, ack flow, and websocket upgrade | `services/local-minimal-node/src/node/build.rs`, `services/session-gateway/tests/websocket_smoke_test.rs` |
| Device routing | Device register and sync-feed reads | `services/local-minimal-node/src/node/build.rs` |
| Disconnect fences | Reconnect-required recovery behavior survives persistence | `services/local-minimal-node/tests/*persistence_test.rs`, `services/session-gateway/tests/http_smoke_test.rs` |

## Media, Streams, and RTC

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Media | Upload, complete, query, signed download URL, attach, and provider health | `services/local-minimal-node/tests/http_e2e_test.rs`, `services/local-minimal-node/tests/media_provider_http_test.rs` |
| Streams | Open, append frame, list frames, checkpoint, complete, and abort | `services/local-minimal-node/tests/http_e2e_test.rs` |
| RTC | Create, invite, accept, reject, end, signal, credential, recording artifact, provider callback, and provider health | `services/local-minimal-node/tests/http_e2e_test.rs`, `services/local-minimal-node/tests/rtc_runtime_persistence_test.rs` |

## Platform and Operations

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Notifications | Request, list, and get | `services/local-minimal-node/src/node/platform.rs` |
| Automation | Execution request and get | `services/local-minimal-node/src/node/platform.rs` |
| Audit | Record, list, and export | `services/local-minimal-node/src/node/platform.rs` |
| Ops | Health, cluster, lag, replay status, runtime dir, provider bindings, drift, and diagnostics | `services/local-minimal-node/src/node/platform.rs`, `services/ops-service/tests/ops_runtime_test.rs` |

## Extension and IoT

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| User-module provider | Provider health and local or external mode wiring | `services/local-minimal-node/tests/user_module_provider_http_test.rs` |
| Object storage provider | Provider health and provider-generated download URLs | `services/local-minimal-node/tests/media_provider_http_test.rs` |
| IoT access provider | Provider health | `services/local-minimal-node/tests/iot_provider_http_test.rs` |
| IoT protocol adapter | Provider health, uplink decode, and downlink encode path | `services/local-minimal-node/src/node/build.rs`, `services/local-minimal-node/src/node/iot.rs` |

## Control-plane Governance

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Protocol registry | Protocol version, bindings, codecs, schemas, and compatibility matrix | `services/control-plane-api/tests/protocol_registry_test.rs` |
| Protocol governance | Capability profile, quota profile, rollout policy, kill switch, and effective snapshot | `services/control-plane-api/tests/protocol_governance_test.rs` |
| Provider registry | Plugin inventory, effective bindings, and precedence | `services/control-plane-api/tests/provider_registry_test.rs` |
| Provider policies | Preview, commit, history, diff, rollback, conflict, noop, and unavailable cases | `services/control-plane-api/tests/provider_registry_test.rs` |
| Node lifecycle | Drain, activate, and route migration | `services/control-plane-api/tests/drain_routes_test.rs` |

## Important Non-Delivery Boundaries

| Surface | Current status |
| --- | --- |
| App TypeScript and Flutter packages | Workspace layout, OpenAPI authority, and generation wrappers exist; release catalog still says `template_only_pending_generation` and `not_published` |
| Admin TypeScript package | Checked-in admin OpenAPI authority exists, generated and composed workspace is implemented, and `node sdks/sdkwork-craw-chat-sdk-admin/bin/verify-sdk.mjs --language typescript` passes; release publication is still not claimed |
| Admin Flutter package | Checked-in admin OpenAPI authority exists, generated and composed workspace is implemented, and `node sdks/sdkwork-craw-chat-sdk-admin/bin/verify-sdk.mjs --language flutter` passes; release publication is still not claimed |
| Admin OpenAPI source | Checked-in authority and derived files now exist under `sdks/sdkwork-craw-chat-sdk-admin/openapi/` and are refreshed from `services/control-plane-api` runtime OpenAPI endpoints |
| Frontend apps | `apps/craw-chat-portal` and `apps/craw-chat-admin` are present as directories, not documented here as mature deliverables |

## What To Read Next

- Continue with [SDK Overview](/sdk/index) for package-surface and publication-state detail.
