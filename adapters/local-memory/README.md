# local-memory

`local-memory` is the default in-memory adapter package used by the current `local-minimal`
profile.

## What It Provides

- `MemoryCommitJournal`
- `MemoryMetadataStore`
- `MemoryTimelineProjectionStore`
- `MemoryRealtimeCheckpointStore`
- `MemoryRealtimeDisconnectFenceStore`
- `MemoryRealtimeSubscriptionStore`
- `MemoryStreamStateStore`
- `MemoryRtcStateStore`
- `MemoryNotificationTaskStore`
- `MemoryAutomationExecutionStore`
- `MemoryPresenceStateStore`
- `MemoryDeviceTwinStore`
- `MemoryStorageDomainSnapshotStore`

## Storage Snapshot Store

`MemoryStorageDomainSnapshotStore` is the in-memory reference adapter for the generic storage
management module.

It stores `StorageDomainSnapshot` values keyed by `snapshot.catalog.domain` and is intended for:

- local testing of storage runtime hydration and save flows
- sandbox and profile baselines before a persistent adapter is wired in
- validating adapter-facing behavior without duplicating provider-specific logic

The semantics are intentionally simple:

- unknown domains return `None`
- saving the same domain overwrites the previous snapshot
- different storage domains remain isolated from each other

## What It Is Not

This crate is not durable storage. It is a fast local adapter for tests, local runtime assembly,
and reference behavior. Production backends still need persistent storage adapters built on the same
`StorageDomainSnapshotStore` contract.
