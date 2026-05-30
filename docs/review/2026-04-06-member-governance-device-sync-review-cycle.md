# 2026-04-06 Member Governance Device Sync Review Cycle

## 1. Finding

### 1.1 High: member governance reached durable truth and realtime, but still did not reach offline multi-device compensation

- Root cause:
  - `conversation-runtime` already appends:
    - `conversation.member_joined`
    - `conversation.member_role_changed`
    - `conversation.member_removed`
    - `conversation.member_left`
  - `projection-service` only updated:
    - member snapshot
    - read cursor seed
  - it did not append any `DeviceSyncFeedEntry` for those member governance events
- Impact:
  - offline devices could not learn roster deltas from `/im/v3/api/devices/{deviceId}/sync-feed`
  - clients had to refetch the whole member list to explain why a conversation appeared, changed, or disappeared
  - the removed or leaving principal's other devices had no durable explanation for the loss of access

### 1.2 High: the old sync contract could not carry a typed roster delta even if fanout were added

- Root cause:
  - `DeviceSyncFeedEntry` only exposed message/read-cursor oriented flat fields
  - it had no typed business payload fields for:
    - durable payload schema
    - durable payload body
    - actor kind
- Impact:
  - a roster delta consumer could not reconstruct:
    - which member changed
    - the resulting membership state
    - the full role-change before/after payload
    - the actor kind
  - adding fanout without contract expansion would still leave clients dependent on refetch heuristics

## 2. Scope Freeze

This wave fixes only member-governance sync-feed propagation:

- `projection-service` device sync fanout
- additive `DeviceSyncFeedEntry` contract expansion
- `local-minimal-node` sync-feed e2e validation

This wave does not:

- redesign realtime contracts
- introduce a second sync-feed model
- retrofit every existing sync event type to typed payload immediately

## 3. Design Decision

Member governance stays on the unified `DeviceSyncFeedEntry` model.

- We do **not** create a parallel roster-only feed.
- We extend the existing sync envelope additively with optional fields:
  - `actorKind`
  - `payloadSchema`
  - `payload`
- For member governance events, `payloadSchema` and `payload` must carry the durable event contract verbatim.
- Actor identity comes from the commit envelope actor:
  - `actorId`
  - `actorKind`
- Recipient rule:
  - `joined / role_changed`: all current active member devices
  - `removed / left`: all current active member devices plus the affected principal's registered devices

This keeps the system lego-like:

- one sync channel
- one additive envelope
- typed business deltas only where required

## 4. Implementation

- `crates/im-domain-core/src/conversation.rs`
  - added optional sync fields:
    - `actor_kind`
    - `payload_schema`
    - `payload`
- `services/projection-service/src/lib.rs`
  - `apply_member_joined(...)` now appends sync-feed entries after updating member/read-cursor state
  - `apply_member_role_changed(...)` now appends sync-feed entries after updating the member snapshot
  - `apply_member_removed(...)` now appends sync-feed entries and explicitly keeps delivery to the affected principal
  - `apply_member_left(...)` now appends sync-feed entries and explicitly keeps delivery to the affected principal
  - added `fan_out_member_governance_to_device_sync_feeds(...)`
  - existing message/read-cursor/handoff sync entries now also fill `actorKind` when available
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - added end-to-end contract validation for sync-feed roster deltas

## 5. Tests Added

- `crates/im-domain-core/tests/model_contract_test.rs`
  - extended `test_device_sync_feed_entry_serializes_sync_shape`
- `services/projection-service/tests/timeline_projection_test.rs`
  - added `test_member_governance_events_project_typed_sync_feed_deltas`
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - added `test_local_minimal_profile_projects_member_governance_sync_feed_deltas`

## 6. Verification

### Red

- `cargo test -p projection-service --offline test_member_governance_events_project_typed_sync_feed_deltas`
  - failed with `0 != 5`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_projects_member_governance_sync_feed_deltas`
  - failed with `0 != 5`

### Green

- `cargo test -p projection-service --offline test_member_governance_events_project_typed_sync_feed_deltas`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_projects_member_governance_sync_feed_deltas`

## 7. Remaining Risks

- `conversation.agent_handoff_status_changed` and other business sync entries still mostly rely on generic flat hints rather than typed payload propagation.
- create-time topology deltas for `agent_dialog` and `system_channel` are still not frozen as dedicated sync-feed contracts.
- the sync envelope is now richer, but client-side delta consumers still need an event-type dispatch layer if they want to consume typed business payloads generically.

## 8. Next Wave

1. Review whether `agent_handoff` lifecycle sync entries should also adopt typed `payloadSchema + payload`.
2. Audit create-time special-conversation flows for missing sync-feed discoverability deltas.
3. Decide whether the broader sync-feed standard should require typed payload for every business event, or only for events that cannot be safely reconstructed from flat hints.
