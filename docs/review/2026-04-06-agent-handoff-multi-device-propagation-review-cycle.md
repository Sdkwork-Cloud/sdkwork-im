# 2026-04-06 Agent Handoff Multi-Device Propagation Review Cycle

## 1. Finding

### 1.1 High: `conversation.agent_handoff_status_changed` reached the journal, but did not reach multi-device consumers

- Root cause:
  - `conversation-runtime` already appends `conversation.agent_handoff_status_changed`.
  - `projection-service` projected the event into `summary` and `inbox`, but did not append any `DeviceSyncFeedEntry`.
  - `local-minimal-node` handlers for `accept / resolve / close` only mutated runtime state and returned JSON. They did not publish any realtime business event.
- Impact:
  - another device of the same principal could not resume the handoff lifecycle from `/api/v1/devices/{deviceId}/sync-feed`
  - subscribed devices could not observe lifecycle changes from `/api/v1/realtime/events`
  - websocket push clients could not receive lifecycle transitions in near real time

## 2. Scope Freeze

This wave fixes only the missing propagation path for `agent_handoff` lifecycle events:

- `projection-service` device sync fanout
- `local-minimal-node` HTTP realtime poll fanout
- `local-minimal-node` websocket push fanout

It does not expand into new admin views or new conversation types.

## 3. Design Decision

`conversation.agent_handoff_status_changed` is now treated as a first-class multi-device event.

- Read model:
  - still projects into `summary` and `inbox`
- Device sync:
  - every active member device must receive a `DeviceSyncFeedEntry`
- Realtime:
  - every subscribed device must receive a business event with the same canonical event type
- Payload rule:
  - realtime payload follows the runtime event shape:
    - `tenantId`
    - `conversationId`
    - `previousStatus`
    - `currentStatus`
    - `changedBy`
    - `changedAt`
    - `state`

## 4. Implementation

- `services/projection-service/src/lib.rs`
  - `AgentHandoffStatusChangedProjectionPayload` now reads `changedBy` and `changedAt`
  - `apply_agent_handoff_status_changed(...)` now fans out device sync entries after updating summary
  - added `fan_out_agent_handoff_status_to_device_sync_feeds(...)`
- `services/local-minimal-node/src/lib.rs`
  - `accept_agent_handoff`
  - `resolve_agent_handoff`
  - `close_agent_handoff`
  now load the previous state, mutate runtime, and publish realtime lifecycle events only when the state actually changed
  - added `publish_realtime_agent_handoff_status_changed_event(...)`
  - added `handoff_lifecycle_changed_at(...)`

## 5. Tests Added

- `services/projection-service/tests/timeline_projection_test.rs`
  - `test_agent_handoff_status_change_projects_device_sync_entries_for_active_members`
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device`
- `services/local-minimal-node/tests/websocket_e2e_test.rs`
  - `test_local_minimal_profile_pushes_agent_handoff_lifecycle_events_over_websocket`

## 6. Verification

### Red

- `cargo test -p projection-service --offline test_agent_handoff_status_change_projects_device_sync_entries_for_active_members -- --exact`
  - failed with `0 != 1`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device -- --exact`
  - failed with `0 != 1`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_pushes_agent_handoff_lifecycle_events_over_websocket -- --exact`
  - failed on websocket timeout because no push arrived

### Green

- `cargo test -p projection-service --offline test_agent_handoff_status_change_projects_device_sync_entries_for_active_members -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_pushes_agent_handoff_lifecycle_events_over_websocket -- --exact`

## 7. Remaining Risks

- `device_sync_feed` still uses the generic `summary` string field for lifecycle hinting. If clients need a fully typed handoff delta entry, the sync contract should be expanded in a later wave.
- realtime fanout now exists in `local-minimal-node`, but the same invariant should be checked in any future gateway or edge profile.

## 8. Next Wave

1. Audit whether `agent_dialog` or `system_channel` lifecycle transitions have the same multi-device propagation gap.
2. Decide whether `DeviceSyncFeedEntry` needs a typed business payload envelope instead of a generic `summary` field.
3. Re-run a broader review for other journal-backed business events to ensure every event has the full chain: journal -> projection -> sync feed -> realtime poll -> websocket push.
