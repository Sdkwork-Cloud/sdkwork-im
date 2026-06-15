# Capability Matrix

## App Collaboration

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Conversations | Standard conversations, agent dialogs, agent handoffs, and system channels | `services/local-minimal-node/src/node/build.rs`, `services/local-minimal-node/tests/http_e2e_test.rs` |
| Membership | List, add, remove, transfer owner, change role, and leave | `services/local-minimal-node/src/node/build.rs` |
| Messages | Send, edit, recall, timeline reads, and system-channel publish | `services/local-minimal-node/tests/http_e2e_test.rs` |
| Read models | Inbox, conversation summary, and read cursor | `services/local-minimal-node/src/node/build.rs` |
| Agent handoff state | Read, accept, resolve, and close handoff state | `services/local-minimal-node/src/node/build.rs` |

## Presence And Realtime

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Client route presence | Heartbeat and current presence | `services/local-minimal-node/src/node/build.rs` |
| Realtime delivery | Subscription sync, event polling, ack flow, and websocket upgrade | `services/local-minimal-node/src/node/build.rs`, `services/session-gateway/tests/websocket_smoke_test.rs` |
| Route ownership | Reconnect-required recovery behavior survives persistence | `services/local-minimal-node/tests/*persistence_test.rs`, `services/session-gateway/tests/http_smoke_test.rs` |

## Media, Streams, and Calls

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Media | Upload, complete, query, signed download URL, attach, and provider health | `services/local-minimal-node/tests/http_e2e_test.rs`, `services/local-minimal-node/tests/media_provider_http_test.rs` |
| Streams | Open, append frame, list frames, checkpoint, complete, and abort | `services/local-minimal-node/tests/http_e2e_test.rs` |
| Calls | IM-owned create, invite, accept, reject, end, signal, and RTC participant credential handoff | `services/local-minimal-node/tests/http_e2e_test.rs`, `services/local-minimal-node/tests/rtc_runtime_persistence_test.rs` |

## Platform and Operations

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Notifications | Request, list, and get | `services/local-minimal-node/src/node/platform.rs` |
| Automation | Execution request and get | `services/local-minimal-node/src/node/platform.rs` |
| Audit | Record, list, and export | `services/local-minimal-node/src/node/platform.rs` |
| Ops | Health, cluster, lag, replay status, runtime dir, provider bindings, drift, and diagnostics | `services/local-minimal-node/src/node/platform.rs`, `services/ops-service/tests/ops_runtime_test.rs` |

## Extension And AIoT

| Capability group | Current implementation | Evidence |
| --- | --- | --- |
| Principal-profile provider | Provider health and upstream-context or external-catalog mode wiring | `services/local-minimal-node/tests/principal_profile_provider_http_test.rs` |
| Object storage provider | Provider health and provider-generated download URLs | `services/local-minimal-node/tests/media_provider_http_test.rs` |
| AIoT dependency surface | App and backend IoT routes are consumed through the shared `sdkwork-api-gateway`; Sdkwork IM local servers do not mount the sibling `sdkwork-aiot` runtime | `specs/component.spec.json`, `scripts/dependency-management-standard.test.mjs` |

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
| IM standard SDK | `sdkwork-im-sdk` owns standardized IM development APIs under `/im/v3/api/*`; TypeScript, Flutter, Rust, and generated transport lanes are checked in and locally verifiable |
| App API SDK | `sdkwork-im-app-sdk` owns app-business and non-management HTTP APIs under `/app/v3/api/*`, including provider health, notifications, automation execution, and app-facing RTC provider routes; AIoT is declared as a dependency SDK instead of copied into IM |
| Backend SDK | `sdkwork-im-backend-sdk` owns `/backend/v3/api/*`, including ops, audit, automation governance, control-plane governance, node operations, and every admin route |
| RTC SDK | `sdkwork-rtc-sdk` owns provider runtime, provider package, native driver, capability negotiation, and provider-selection standards; it is not an OpenAPI-generated HTTP SDK |
| Frontend apps | `apps/sdkwork-im-portal` and `apps/sdkwork-im-admin` are present as directories, not documented here as mature deliverables |

## What To Read Next

- Continue with [SDK Overview](/sdk/index) for package-surface and publication-state detail.
