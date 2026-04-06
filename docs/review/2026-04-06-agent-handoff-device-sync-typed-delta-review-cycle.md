# 2026-04-06 Agent Handoff Device Sync Typed Delta Review Cycle

## 1. Finding

### 1.1 High: `agent_handoff` sync-feed delivery existed, but remained a weakly typed hint-only contract

- Root cause:
  - the previous propagation wave already ensured `conversation.agent_handoff_status_changed` reached:
    - commit journal
    - summary / inbox projection
    - device sync feed
    - realtime HTTP event window
    - websocket push
  - but `projection-service::fan_out_agent_handoff_status_to_device_sync_feeds(...)` still created `DeviceSyncFeedEntry` with:
    - `summary = Some(current_status)`
    - `payload_schema = None`
    - `payload = None`
  - therefore multi-device resume clients could only infer lifecycle state from a generic status hint instead of consuming the canonical runtime payload.
- Impact:
  - offline resume clients could not safely reconstruct the full handoff transition from sync-feed alone
  - future client evolution would require out-of-band field guessing or extra summary lookups
  - `agent_handoff` was behind the typed business delta baseline already established for member governance

## 2. Scope Freeze

This wave fixes only the typed sync-feed contract for `conversation.agent_handoff_status_changed`.

It does not:

- change realtime payload behavior
- introduce a new handoff-specific feed
- expand into scheduled publish, close/archive, or other special-conversation lifecycle work

## 3. Design Decision

`agent_handoff` lifecycle sync entries now follow the same additive typed-delta direction used by member governance:

- keep the generic `summary` hint for lightweight clients
- additionally copy the runtime event contract into sync-feed:
  - `payloadSchema = conversation.agent_handoff_status_changed.v1`
  - `payload = original runtime payload json`

This preserves backward compatibility while giving commercial clients a stable typed offline compensation path.

## 4. Implementation

- `services/projection-service/src/lib.rs`
  - `fan_out_agent_handoff_status_to_device_sync_feeds(...)`
    - changed from:
      - `payload_schema: None`
      - `payload: None`
    - to:
      - `payload_schema: event.payload_schema.clone()`
      - `payload: Some(event.payload.clone())`
- `services/projection-service/tests/timeline_projection_test.rs`
  - strengthened `test_agent_handoff_status_change_projects_device_sync_entries_for_active_members`
  - now asserts:
    - `payloadSchema`
    - payload json shape for both source and target device feeds
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - strengthened `test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device`
  - after verifying realtime fanout, now also queries:
    - `/api/v1/devices/d_pad/sync-feed?afterSeq=0`
  - asserts:
    - `originEventType`
    - `payloadSchema`
    - payload json shape

## 5. Verification

### Red

- `cargo test -p projection-service --offline test_agent_handoff_status_change_projects_device_sync_entries_for_active_members -- --exact`
  - failed with:
    - `left: None`
    - `right: Some("conversation.agent_handoff_status_changed.v1")`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device -- --exact`
  - failed with:
    - `left: Null`
    - `right: "conversation.agent_handoff_status_changed.v1"`

### Green

- `cargo test -p projection-service --offline test_agent_handoff_status_change_projects_device_sync_entries_for_active_members -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_agent_handoff_lifecycle_realtime_events_to_other_device -- --exact`

## 6. Remaining Risks

- `agent_handoff` sync-feed currently copies the canonical runtime payload, but there is still no cross-event rule that all business deltas must always expose typed payloads.
- `agent_dialog` and `system_channel` lifecycle-related sync entries should be reviewed with the same question:
  - is the sync-feed entry self-sufficient for offline resume?
- some older generic sync entries still intentionally rely on flat fields only; those boundaries should be audited wave by wave instead of widened blindly.

## 7. Next Wave

1. Review whether `agent_dialog` or `system_channel` lifecycle/state transitions have the same typed sync-gap.
2. Audit other business events already reaching realtime but still using weak sync-feed contracts.
3. Decide whether the platform standard should explicitly separate:
   - required typed payload events
   - allowed hint-only events
