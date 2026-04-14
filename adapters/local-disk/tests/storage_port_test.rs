use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_local_disk::{
    FileMetadataStore, FileTimelineProjectionStore, validate_metadata_store_file,
    validate_timeline_projection_store_file,
};
use im_platform_contracts::{
    ContractError, MetadataSnapshotRecord, MetadataStore, TimelineProjectionBatch,
    TimelineProjectionRecord, TimelineProjectionStore,
};

fn unique_store_file(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}.json"))
}

#[test]
fn test_file_metadata_store_persists_latest_snapshot_across_reopen() {
    let file_path = unique_store_file("metadata_store");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshot(
            "tenant:t_demo",
            "conversation:c_demo",
            "{\"state\":\"draft\"}",
        )
        .expect("first metadata snapshot should succeed");
    store
        .put_snapshot(
            "tenant:t_demo",
            "conversation:c_demo",
            "{\"state\":\"ready\"}",
        )
        .expect("second metadata snapshot should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened
            .snapshot("tenant:t_demo", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_metadata_store_file_rejects_array_shape() {
    let file_path = unique_store_file("metadata_store_invalid");
    fs::write(&file_path, b"[]").expect("metadata store file should be written");

    let error = validate_metadata_store_file(&file_path)
        .expect_err("array-shaped metadata store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse metadata store"));

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_timeline_projection_store_upserts_by_sequence_across_reopen() {
    let file_path = unique_store_file("timeline_projection_store");
    let store = FileTimelineProjectionStore::new(&file_path);

    store
        .upsert_timeline_entry("t_demo:c_demo", 1, "{\"summary\":\"first\"}")
        .expect("first timeline upsert should succeed");
    store
        .upsert_timeline_entry("t_demo:c_demo", 2, "{\"summary\":\"second\"}")
        .expect("second timeline upsert should succeed");
    store
        .upsert_timeline_entry("t_demo:c_demo", 2, "{\"summary\":\"second-v2\"}")
        .expect("idempotent timeline upsert should succeed");

    let reopened = FileTimelineProjectionStore::new(&file_path);
    assert_eq!(
        reopened.entries("t_demo:c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_metadata_store_batches_snapshot_updates_across_reopen() {
    let file_path = unique_store_file("metadata_store_batch");
    let store = FileMetadataStore::new(&file_path);

    store
        .put_snapshots(&[
            MetadataSnapshotRecord {
                scope: "tenant:t_demo".into(),
                key: "conversation:c_demo".into(),
                value: "{\"state\":\"draft\"}".into(),
            },
            MetadataSnapshotRecord {
                scope: "tenant:t_demo".into(),
                key: "profile:u_demo".into(),
                value: "{\"name\":\"demo\"}".into(),
            },
            MetadataSnapshotRecord {
                scope: "tenant:t_demo".into(),
                key: "conversation:c_demo".into(),
                value: "{\"state\":\"ready\"}".into(),
            },
        ])
        .expect("batched metadata snapshots should succeed");

    let reopened = FileMetadataStore::new(&file_path);
    assert_eq!(
        reopened
            .snapshot("tenant:t_demo", "conversation:c_demo")
            .as_deref(),
        Some("{\"state\":\"ready\"}")
    );
    assert_eq!(
        reopened
            .snapshot("tenant:t_demo", "profile:u_demo")
            .as_deref(),
        Some("{\"name\":\"demo\"}")
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_file_timeline_projection_store_batches_multiple_scopes_across_reopen() {
    let file_path = unique_store_file("timeline_projection_store_batch");
    let store = FileTimelineProjectionStore::new(&file_path);

    store
        .upsert_timeline_batches(&[
            TimelineProjectionBatch {
                conversation_id: "t_demo:c_demo".into(),
                records: vec![
                    TimelineProjectionRecord {
                        message_seq: 1,
                        payload: "{\"summary\":\"first\"}".into(),
                    },
                    TimelineProjectionRecord {
                        message_seq: 2,
                        payload: "{\"summary\":\"second\"}".into(),
                    },
                ],
            },
            TimelineProjectionBatch {
                conversation_id: "device-sync:t_demo:u_demo:d_demo".into(),
                records: vec![TimelineProjectionRecord {
                    message_seq: 9,
                    payload: "{\"syncSeq\":9}".into(),
                }],
            },
            TimelineProjectionBatch {
                conversation_id: "t_demo:c_demo".into(),
                records: vec![TimelineProjectionRecord {
                    message_seq: 2,
                    payload: "{\"summary\":\"second-v2\"}".into(),
                }],
            },
        ])
        .expect("batched timeline upserts should succeed");

    let reopened = FileTimelineProjectionStore::new(&file_path);
    assert_eq!(
        reopened.entries("t_demo:c_demo"),
        vec![
            (1, "{\"summary\":\"first\"}".to_string()),
            (2, "{\"summary\":\"second-v2\"}".to_string()),
        ]
    );
    assert_eq!(
        reopened.entries("device-sync:t_demo:u_demo:d_demo"),
        vec![(9, "{\"syncSeq\":9}".to_string())]
    );

    let _ = fs::remove_file(file_path);
}

#[test]
fn test_validate_timeline_projection_store_file_rejects_array_shape() {
    let file_path = unique_store_file("timeline_projection_store_invalid");
    fs::write(&file_path, b"[]").expect("timeline projection store file should be written");

    let error = validate_timeline_projection_store_file(&file_path)
        .expect_err("array-shaped timeline projection store should be rejected");
    assert!(matches!(error, ContractError::Unavailable(_)));
    let message = match error {
        ContractError::Unavailable(message) => message,
        other => panic!("unexpected error variant: {other:?}"),
    };
    assert!(message.contains("failed to parse timeline projection store"));

    let _ = fs::remove_file(file_path);
}
