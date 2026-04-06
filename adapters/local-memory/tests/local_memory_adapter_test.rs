use im_adapters_local_memory::{
    MemoryCommitJournal, MemoryMetadataStore, MemoryRealtimeCheckpointStore,
    MemoryRealtimeDisconnectFenceStore, MemoryTimelineProjectionStore,
};
use im_platform_contracts::{
    CommitJournal, MetadataStore, RealtimeCheckpointRecord, RealtimeCheckpointStore,
    RealtimeDisconnectFenceRecord, RealtimeDisconnectFenceStore, TimelineProjectionStore,
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
