# Local-Minimal Runtime Checkpoint Persistence And Runtime Dir Standard (2026-04-06)

## 1. Runtime Layout

Managed local-minimal deployment must materialize:

```text
.runtime/local-minimal/
  config/
  logs/
  pids/
  state/
```

The realtime checkpoint file is:

```text
.runtime/local-minimal/state/realtime-checkpoints.json
```

The disconnect fence file remains:

```text
.runtime/local-minimal/state/realtime-disconnect-fences.json
```

## 2. Startup Contract

`local-minimal.env` must contain:

- `CRAW_CHAT_BIND_ADDR`
- `CRAW_CHAT_RUNTIME_DIR`
- `CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET`

Managed startup scripts must export `CRAW_CHAT_RUNTIME_DIR` before launching the binary.

## 3. Operational Meaning

After restart, the same runtime dir gives the node access to:

- restored realtime checkpoint truth
- restored disconnect fences

It does not yet imply automatic recovery of:

- conversation aggregates
- membership state
- live subscription sets

Operators should treat checkpoint durability and conversation-domain durability as separate operational layers until a later standard freezes full cold-restart recovery.

## 4. Verification Checklist

Private deployment validation should confirm:

1. the runtime dir exists before launch
2. `state/realtime-checkpoints.json` is created after realtime ack or trim activity
3. restart with the same runtime dir preserves the checkpoint window
4. store failure surfaces as `503` instead of crashing the node
