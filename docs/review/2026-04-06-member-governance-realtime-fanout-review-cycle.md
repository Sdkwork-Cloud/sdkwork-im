# 2026-04-06 Member Governance Realtime Fanout Review Cycle

## 1. Finding

### 1.1 High: member governance mutations updated durable truth but did not reach subscribed realtime devices

- Root cause:
  - `conversation-runtime` already appends:
    - `conversation.member_joined`
    - `conversation.member_role_changed`
    - `conversation.member_removed`
    - `conversation.member_left`
  - `local-minimal-node` handlers for add/remove/change-role/leave only recorded audit and returned JSON.
  - No realtime business event was published into `/im/v3/api/realtime/events` or websocket push.
- Impact:
  - roster UIs could not react in real time to member joins, role changes, removals, or leaves
  - multi-device clients of the same principal would miss their own membership changes while online

### 1.2 High: `removed / left` transitions would silently drop the affected principal if fanout only used current active members

- Root cause:
  - after `remove_member` or `leave_conversation`, the affected principal is no longer active
  - a naive fanout based only on `conversation_runtime.list_members(...)` would exclude the removed or leaving principal
- Impact:
  - the affected principal's other registered devices would not receive the event that explains why the conversation disappeared or became inaccessible

## 2. Scope Freeze

This wave fixes only realtime propagation for member governance events in `local-minimal-node`:

- HTTP realtime event window
- websocket push

It does not expand `device sync feed` schema in this wave.

## 3. Design Decision

Member governance mutations are now treated as first-class business realtime events.

- event scope:
  - `scopeType = conversation`
  - `scopeId = conversationId`
- event types:
  - `conversation.member_joined`
  - `conversation.member_role_changed`
  - `conversation.member_removed`
  - `conversation.member_left`
- recipient rule:
  - all current conversation members' registered devices
  - plus the affected principal for `removed / left`

## 4. Implementation

- `services/local-minimal-node/src/lib.rs`
  - `add_member(...)` now publishes `conversation.member_joined`
  - `change_conversation_member_role(...)` now publishes `conversation.member_role_changed`
  - `remove_member(...)` now publishes `conversation.member_removed`
  - `leave_conversation(...)` now publishes `conversation.member_left`
  - added `publish_realtime_membership_event(...)`

### Payload shape

- join/remove/left:
  - `conversationId`
  - `member`
  - `actor`
- role change:
  - `conversationId`
  - `changedAt`
  - `previousMember`
  - `updatedMember`
  - `actor`

## 5. Tests Added

- `services/local-minimal-node/tests/http_e2e_test.rs`
  - `test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device`
- `services/local-minimal-node/tests/websocket_e2e_test.rs`
  - `test_local_minimal_profile_pushes_member_joined_events_over_websocket`

## 6. Verification

### Red

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device -- --exact`
  - failed with `0 != 5`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_pushes_member_joined_events_over_websocket -- --exact`
  - failed on websocket timeout because no push arrived

### Green

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device -- --exact`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_pushes_member_joined_events_over_websocket -- --exact`

## 7. Remaining Risks

- `projection-service` still does not emit member governance entries into `device sync feed`, so offline multi-device compensation for roster changes still depends on higher-level refetch.
- create-time special conversation topology changes such as `agent_dialog` / `system_channel` dedicated create still rely on read-model visibility rather than a dedicated sync delta contract.

## 8. Next Wave

1. Review whether member governance changes need a typed `device sync feed` contract instead of realtime-only propagation.
2. Audit special conversation creation flows for create-time multi-device compensation gaps.
3. Continue freezing dedicated lifecycle commands for `agent_dialog` and dedicated publish orchestration for `system_channel`.
