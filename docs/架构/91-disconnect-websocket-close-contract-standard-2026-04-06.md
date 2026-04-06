# 91. Disconnect WebSocket Close Contract Standard (2026-04-06)

## 1. Goal

When the platform closes a live realtime websocket because the same device explicitly called `session.disconnect`, the close frame must expose a stable public contract.

## 2. Problem Boundary

This standard applies when:

- a device has an already-open websocket transport on `/api/v1/realtime/ws`
- that same device successfully calls `POST /api/v1/sessions/disconnect`
- the server closes the transport because of that explicit disconnect lifecycle

This standard extends Standard 90. Standard 90 froze the requirement that the socket must close. This standard freezes how that close is represented on the wire.

## 3. Required Contract

The server must send a websocket close frame with:

- close code: `4001`
- close reason: `"session.disconnect"`

The server is not allowed to use `Close(None)` for this lifecycle anymore.

## 4. Semantics

The close contract means:

1. the current device session has crossed an explicit disconnect boundary
2. the current websocket transport is terminal for that session
3. the client must not continue using the previous socket as if it were a transient network interruption
4. any further device-bound access must follow Standard 88 and establish a fresh `session.resume`

This close does not mean:

- server overload
- node restart
- generic network instability
- protocol parse failure

## 5. Code Selection Rule

`4001` is reserved here as a private-use application close code for explicit session disconnect.

Rationale:

- `1000` is too generic for SDK policy
- `1001` implies endpoint departure, not explicit session lifecycle
- `1008` implies policy violation, which is incorrect here
- `4000-4999` is the correct private-use range for product-specific websocket semantics

## 6. Reason String Rule

The reason string must be:

- exact value: `"session.disconnect"`
- ASCII
- stable across gateway and local profile implementations

The platform must not emit alternative spellings for the same lifecycle, such as:

- `"disconnect"`
- `"session_disconnected"`
- `"reconnect_required"`

Those variants weaken telemetry consistency and SDK branching logic.

## 7. Scope Consistency

This close contract applies equally to:

- normal disconnect handling
- duplicate same-session disconnect requests that are treated idempotently under Standard 89
- disconnect detection while a websocket is idle
- disconnect detection when a late inbound websocket frame arrives after the disconnect generation changed

## 8. Verification Standard

Regression coverage must prove:

1. websocket connects successfully
2. `session.disconnect` succeeds for the same device session
3. the websocket receives a close frame
4. the close frame code is exactly `4001`
5. the close frame reason is exactly `"session.disconnect"`

Coverage must exist for:

- `services/session-gateway/tests/websocket_smoke_test.rs`
- `services/local-minimal-node/tests/websocket_e2e_test.rs`

## 9. SDK Interpretation Rule

SDKs and platform adapters should interpret close code `4001` and reason `"session.disconnect"` as:

- explicit session termination for the current device session
- not a silent transparent reconnect to the old session
- reconnect only after a higher-level session flow performs a fresh `session.resume` or equivalent re-establishment action

## 10. Non-Goals

This standard still does not freeze:

- durable reconnect-fence persistence across process restart
- automatic client reconnect policy above the close contract
- crash-recovery session epoch semantics
