# 2026-04-06 Local Profile Governance Actor Kind Review Cycle

## 1. Finding

### 1.1 High: `local-minimal-node` governance side effects still trusted `auth.actor_kind`

- Root cause:
  - `conversation_runtime` governance mutations already derive actor identity from durable member truth:
    - `principal_id`
    - `principal_kind`
  - but `local-minimal-node` still built member-governance realtime payloads and audit anchors directly from request auth context:
    - `auth.actor_kind`
  - this affected side effects for:
    - `conversation.member_joined`
    - `conversation.member_removed`
    - `conversation.member_role_changed`
    - `conversation.member_left`
    - `conversation.owner_transferred`
- Impact:
  - a trusted-header request could set:
    - real member id = `u_demo`
    - spoofed auth kind = `agent`
  - the runtime write would still succeed and durable conversation truth would keep the real member kind
  - but realtime fanout and audit export from `local-minimal-node` would publish the spoofed kind instead
  - this split durable truth and local-profile side-effect truth for the same governance mutation

## 2. Reproduction

Two regression tests were added before the fix:

- `test_local_minimal_profile_member_governance_side_effects_preserve_runtime_actor_kind_when_auth_kind_is_mismatched`
- `test_local_minimal_profile_owner_transfer_audit_preserves_runtime_actor_kind_when_auth_kind_is_mismatched`

Red verification proved the defect:

- governance realtime payload returned:
  - actual `actor.kind = "agent"`
  - expected `actor.kind = "user"`
- owner transfer audit export returned:
  - actual `actorKind = "agent"`
  - expected `actorKind = "user"`

## 3. Fix Design

The side-effect layer must not invent or trust actor kind when runtime can already resolve the real member identity.

For local profile governance mutations, the correct sequence is:

1. resolve auth context
2. resolve active conversation member from `conversation_runtime`
3. normalize side-effect actor identity to the resolved member `principal_kind`
4. apply runtime mutation
5. emit realtime and audit side effects using the normalized actor identity

Special note for `leave`:

- the leaving actor is no longer active after the mutation
- so the real member kind must be captured before executing `leave_conversation(...)`

## 4. Implementation

- `services/local-minimal-node/src/lib.rs`
  - added `resolve_conversation_actor_auth_context(...)`
  - this helper clones `AuthContext` and rewrites `actor_kind` from:
    - `conversation_runtime.require_active_member(...).principal_kind`
  - wired normalized actor auth into local side effects for:
    - `add_member(...)`
    - `remove_member(...)`
    - `transfer_conversation_owner(...)`
    - `change_conversation_member_role(...)`
    - `leave_conversation(...)`
- `services/local-minimal-node/tests/http_e2e_test.rs`
  - added the two regression tests above

The repair intentionally leaves runtime governance authorization rules unchanged.

This wave fixes only side-effect identity drift in the local profile.

## 5. Verification

### Red

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_member_governance_side_effects_preserve_runtime_actor_kind_when_auth_kind_is_mismatched`
  - failed with realtime payload `actor.kind = agent`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_owner_transfer_audit_preserves_runtime_actor_kind_when_auth_kind_is_mismatched`
  - failed with audit export `actorKind = agent`

### Green

- `cargo test -p local-minimal-node --offline test_local_minimal_profile_member_governance_side_effects_preserve_runtime_actor_kind_when_auth_kind_is_mismatched`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_owner_transfer_audit_preserves_runtime_actor_kind_when_auth_kind_is_mismatched`
- `cargo test -p local-minimal-node --offline test_local_minimal_profile_fanouts_member_governance_realtime_events_to_registered_owner_device`

## 6. Remaining Risks

- other local-profile side effects outside conversation governance may still rely on raw auth context even when runtime truth is available.
- `conversation_runtime` governance entry points themselves still do not explicitly reject actor-kind mismatch; this wave only fixes local-profile drift, not runtime acceptance semantics.
- websocket-facing governance observation still depends on shared side-effect helpers remaining aligned with this standard.

## 7. Next Wave

1. Audit whether governance runtime entry points should also reject actor-kind mismatch instead of only normalizing side effects.
2. Review other side-effect surfaces such as notifications and device-sync projections for similar trust-on-auth patterns.
3. Extend the same review to stream and RTC conversation-bound mutation side effects where actor metadata is included in payloads.
