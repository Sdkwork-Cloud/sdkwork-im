use im_adapters_local_memory::{
    MemoryAutomationExecutionStore, MemoryCommitJournal, MemoryMetadataStore,
    MemoryRealtimeCheckpointStore, MemoryRealtimeDisconnectFenceStore,
    MemoryStorageDomainSnapshotStore,
    MemoryTimelineProjectionStore,
};
use im_domain_core::automation::{AutomationExecution, AutomationExecutionState};
use im_platform_contracts::{
    AutomationExecutionRecord, AutomationExecutionStore, CommitJournal, MetadataStore,
    RealtimeCheckpointRecord, RealtimeCheckpointStore, RealtimeDisconnectFenceRecord,
    RealtimeDisconnectFenceStore, TimelineProjectionStore,
};
use im_storage_contracts::{
    StorageBindingRecord, StorageCatalog, StorageConfigRecord, StorageCredentialMode,
    StorageDomainSnapshot, StorageDomainSnapshotStore, StorageSecretRecord,
};

#[test]
fn test_memory_commit_journal_appends_in_order() {
    let journal = MemoryCommitJournal::with_partition("local-minimal");

    let first = journal
        .append(im_domain_events::CommitEnvelope::minimal(
            "evt_1",
            "t_demo",
            "message.posted",
            "conversation",
            "c_demo",
            1,
        ))
        .expect("first append should succeed");
    let second = journal
        .append(im_domain_events::CommitEnvelope::minimal(
            "evt_2",
            "t_demo",
            "message.posted",
            "conversation",
            "c_demo",
            2,
        ))
        .expect("second append should succeed");

    assert_eq!(first.partition, "local-minimal");
    assert_eq!(first.offset, 1);
    assert_eq!(second.partition, "local-minimal");
    assert_eq!(second.offset, 2);
    assert_eq!(journal.recorded().len(), 2);
}

#[test]
fn test_memory_metadata_store_overwrites_snapshot_for_same_scope_and_key() {
    let metadata = MemoryMetadataStore::default();

    metadata
        .put_snapshot(
            "tenant:t_demo",
            "conversation:c_demo",
            "{\"state\":\"draft\"}",
        )
        .expect("first snapshot should succeed");
    metadata
        .put_snapshot(
            "tenant:t_demo",
            "conversation:c_demo",
            "{\"state\":\"ready\"}",
        )
        .expect("second snapshot should succeed");

    assert_eq!(
        metadata
            .snapshot("tenant:t_demo", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );
}

#[test]
fn test_memory_timeline_projection_store_upserts_by_sequence() {
    let projection = MemoryTimelineProjectionStore::default();

    projection
        .upsert_timeline_entry("t_demo:c_demo", 1, "{\"summary\":\"first\"}")
        .expect("first upsert should succeed");
    projection
        .upsert_timeline_entry("t_demo:c_demo", 2, "{\"summary\":\"second\"}")
        .expect("second upsert should succeed");
    projection
        .upsert_timeline_entry("t_demo:c_demo", 2, "{\"summary\":\"second-v2\"}")
        .expect("idempotent upsert should succeed");

    assert_eq!(
        projection.entries("t_demo:c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );
}

#[test]
fn test_memory_realtime_checkpoint_store_overwrites_same_device_checkpoint() {
    let store = MemoryRealtimeCheckpointStore::default();

    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 3,
            acked_through_seq: 2,
            trimmed_through_seq: 2,
            updated_at: "2026-04-05T12:00:00Z".into(),
        })
        .expect("first checkpoint save should succeed");
    store
        .save_checkpoint(RealtimeCheckpointRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            latest_realtime_seq: 5,
            acked_through_seq: 4,
            trimmed_through_seq: 4,
            updated_at: "2026-04-05T12:01:00Z".into(),
        })
        .expect("second checkpoint save should succeed");

    let checkpoint = store
        .load_checkpoint("t_demo", "u_demo", "d_pad")
        .expect("checkpoint load should succeed")
        .expect("checkpoint should exist");
    assert_eq!(checkpoint.latest_realtime_seq, 5);
    assert_eq!(checkpoint.acked_through_seq, 4);
    assert_eq!(checkpoint.trimmed_through_seq, 4);
}

#[test]
fn test_memory_realtime_disconnect_fence_store_overwrites_and_clears_same_device_fence() {
    let store = MemoryRealtimeDisconnectFenceStore::default();

    store
        .save_fence(RealtimeDisconnectFenceRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_old".into()),
            owner_node_id: "node_a".into(),
            disconnected_at: "2026-04-06T12:00:00Z".into(),
        })
        .expect("first fence save should succeed");
    store
        .save_fence(RealtimeDisconnectFenceRecord {
            tenant_id: "t_demo".into(),
            principal_id: "u_demo".into(),
            device_id: "d_pad".into(),
            session_id: Some("s_new".into()),
            owner_node_id: "node_b".into(),
            disconnected_at: "2026-04-06T12:01:00Z".into(),
        })
        .expect("second fence save should succeed");

    let fence = store
        .load_fence("t_demo", "u_demo", "d_pad")
        .expect("fence load should succeed")
        .expect("fence should exist");
    assert_eq!(fence.session_id.as_deref(), Some("s_new"));
    assert_eq!(fence.owner_node_id, "node_b");

    assert!(
        store
            .clear_fence("t_demo", "u_demo", "d_pad")
            .expect("fence clear should succeed")
    );
    assert!(
        store
            .load_fence("t_demo", "u_demo", "d_pad")
            .expect("fence load should succeed")
            .is_none()
    );
}

#[test]
fn test_memory_automation_execution_store_isolates_same_actor_id_across_principal_kind() {
    let store = MemoryAutomationExecutionStore::default();

    for principal_kind in ["user", "system"] {
        store
            .save_execution(AutomationExecutionRecord {
                tenant_id: "t_demo".into(),
                principal_id: "u_demo".into(),
                execution_id: "ae_kind_isolation".into(),
                execution: AutomationExecution {
                    tenant_id: "t_demo".into(),
                    principal_id: "u_demo".into(),
                    principal_kind: principal_kind.into(),
                    execution_id: "ae_kind_isolation".into(),
                    trigger_type: "webhook.manual".into(),
                    target_kind: "workflow".into(),
                    target_ref: "wf_demo".into(),
                    input_payload: Some("{\"conversationId\":\"c_demo\"}".into()),
                    output_payload: Some("{\"accepted\":true}".into()),
                    state: AutomationExecutionState::Succeeded,
                    retry_count: 0,
                    requested_at: "2026-04-06T00:00:00.000Z".into(),
                    completed_at: Some("2026-04-06T00:00:01.000Z".into()),
                    failure_reason: None,
                },
                updated_at: "2026-04-06T00:00:01.000Z".into(),
            })
            .expect("save should succeed");
    }

    let user_execution = store
        .load_execution("t_demo", "user", "u_demo", "ae_kind_isolation")
        .expect("user execution load should succeed")
        .expect("user execution should exist");
    let system_execution = store
        .load_execution("t_demo", "system", "u_demo", "ae_kind_isolation")
        .expect("system execution load should succeed")
        .expect("system execution should exist");
    assert_eq!(user_execution.execution.principal_kind, "user");
    assert_eq!(system_execution.execution.principal_kind, "system");
}

#[test]
fn test_memory_storage_domain_snapshot_store_returns_none_for_unknown_domain() {
    let store = MemoryStorageDomainSnapshotStore::default();

    let snapshot = store
        .load_snapshot("object-storage")
        .expect("snapshot load should succeed");

    assert!(snapshot.is_none());
}

#[test]
fn test_memory_storage_domain_snapshot_store_overwrites_same_domain_snapshot() {
    let store = MemoryStorageDomainSnapshotStore::default();

    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-aws"))
                .with_config(StorageConfigRecord::new_global("object-storage-aws")),
        )
        .expect("first snapshot save should succeed");
    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-google"))
                .with_config(StorageConfigRecord::new_global("object-storage-google"))
                .with_secret(
                    StorageSecretRecord::new_global(
                        "object-storage-google",
                        StorageCredentialMode::ServiceAccountJson,
                        "{\"serviceAccountJson\":{\"client_email\":\"storage@sdkwork.local\"}}",
                    )
                    .with_secret_fingerprint("fp-object-storage-google"),
                ),
        )
        .expect("second snapshot save should succeed");

    let snapshot = store
        .load_snapshot("object-storage")
        .expect("snapshot load should succeed")
        .expect("snapshot should exist");

    assert_eq!(snapshot.bindings.len(), 1);
    assert_eq!(snapshot.bindings[0].provider_plugin_id, "object-storage-google");
    assert_eq!(snapshot.secrets.len(), 1);
    assert_eq!(
        snapshot.secrets[0].secret_fingerprint,
        "fp-object-storage-google"
    );
}

#[test]
fn test_memory_storage_domain_snapshot_store_isolates_domains() {
    let store = MemoryStorageDomainSnapshotStore::default();

    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog::object_storage())
                .with_binding(StorageBindingRecord::new_global("object-storage-aws"))
                .with_config(StorageConfigRecord::new_global("object-storage-aws")),
        )
        .expect("object storage snapshot save should succeed");
    store
        .save_snapshot(
            StorageDomainSnapshot::new(StorageCatalog {
                domain: "chat-archive".into(),
                provider_schemas: Vec::new(),
            })
            .with_binding(StorageBindingRecord::new_global("archive-provider"))
            .with_config(StorageConfigRecord::new_global("archive-provider")),
        )
        .expect("archive storage snapshot save should succeed");

    let object_storage = store
        .load_snapshot("object-storage")
        .expect("object storage snapshot load should succeed")
        .expect("object storage snapshot should exist");
    let chat_archive = store
        .load_snapshot("chat-archive")
        .expect("chat archive snapshot load should succeed")
        .expect("chat archive snapshot should exist");

    assert_eq!(object_storage.catalog.domain, "object-storage");
    assert_eq!(object_storage.bindings[0].provider_plugin_id, "object-storage-aws");
    assert_eq!(chat_archive.catalog.domain, "chat-archive");
    assert_eq!(chat_archive.bindings[0].provider_plugin_id, "archive-provider");
}
