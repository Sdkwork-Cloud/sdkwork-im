use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

use im_adapters_local_disk::{
    FileCommitJournal, FileMetadataStore, FileTimelineProjectionStore, read_commit_journal_file,
    validate_metadata_store_file, validate_timeline_projection_store_file,
};
use im_platform_contracts::{
    CommitEnvelope, CommitJournal, ContractError, MetadataSnapshotRecord, MetadataStore,
    TimelineProjectionBatch, TimelineProjectionRecord, TimelineProjectionStore,
};

fn unique_store_file(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}.json"))
}

fn commit_envelope(thread_id: usize, seq: usize) -> CommitEnvelope {
    CommitEnvelope::minimal(
        &format!("evt_thread_{thread_id}_{seq}"),
        "t_demo",
        "test.appended",
        "test",
        &format!("agg_{thread_id}"),
        seq as u64,
    )
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
fn test_file_commit_journal_preserves_cross_instance_appends() {
    let file_path = unique_store_file("commit_journal_concurrent");
    let thread_count = 4;
    let appends_per_thread = 64;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let journal = FileCommitJournal::new("local-disk-test", &file_path);
            for seq in 0..appends_per_thread {
                barrier.wait();
                journal
                    .append(commit_envelope(thread_id, seq))
                    .expect("cross-instance append should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("writer thread should join");
    }

    let events = read_commit_journal_file(&file_path)
        .expect("commit journal should remain readable after concurrent appends");
    assert_eq!(
        events.len(),
        thread_count * appends_per_thread,
        "concurrent appends from distinct journal instances should not lose events"
    );

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
}

#[test]
fn test_file_metadata_store_preserves_cross_instance_snapshot_updates() {
    let file_path = unique_store_file("metadata_store_concurrent");
    let thread_count = 4;
    let writes_per_thread = 32;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let store = FileMetadataStore::new(&file_path);
            for seq in 0..writes_per_thread {
                let key = format!("conversation:c_{thread_id}_{seq}");
                let value = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
                barrier.wait();
                store
                    .put_snapshot("tenant:t_demo", key.as_str(), value.as_str())
                    .expect("cross-instance metadata snapshot should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("metadata writer thread should join");
    }

    let reopened = FileMetadataStore::new(&file_path);
    for thread_id in 0..thread_count {
        for seq in 0..writes_per_thread {
            let key = format!("conversation:c_{thread_id}_{seq}");
            let expected = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
            assert_eq!(
                reopened.snapshot("tenant:t_demo", key.as_str()).as_deref(),
                Some(expected.as_str()),
                "cross-instance metadata updates should retain every unique key"
            );
        }
    }

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
}

#[test]
fn test_file_timeline_projection_store_preserves_cross_instance_entries() {
    let file_path = unique_store_file("timeline_projection_store_concurrent");
    let conversation_id = "t_demo:c_concurrent";
    let thread_count = 4;
    let writes_per_thread = 32;
    let barrier = Arc::new(Barrier::new(thread_count));
    let mut handles = Vec::new();

    for thread_id in 0..thread_count {
        let barrier = barrier.clone();
        let file_path = file_path.clone();
        handles.push(thread::spawn(move || {
            let store = FileTimelineProjectionStore::new(&file_path);
            for seq in 0..writes_per_thread {
                let message_seq = (thread_id * writes_per_thread + seq + 1) as u64;
                let payload = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
                barrier.wait();
                store
                    .upsert_timeline_entry(conversation_id, message_seq, payload.as_str())
                    .expect("cross-instance timeline upsert should succeed");
            }
        }));
    }

    for handle in handles {
        handle.join().expect("timeline writer thread should join");
    }

    let reopened = FileTimelineProjectionStore::new(&file_path);
    let entries = reopened.entries(conversation_id);
    assert_eq!(
        entries.len(),
        thread_count * writes_per_thread,
        "cross-instance timeline upserts should retain every unique message sequence"
    );
    for thread_id in 0..thread_count {
        for seq in 0..writes_per_thread {
            let message_seq = (thread_id * writes_per_thread + seq + 1) as u64;
            let payload = format!("{{\"thread\":{thread_id},\"seq\":{seq}}}");
            assert!(
                entries.iter().any(|(stored_seq, stored_payload)| {
                    *stored_seq == message_seq && stored_payload == &payload
                }),
                "missing timeline projection entry for message_seq={message_seq}"
            );
        }
    }

    let _ = fs::remove_file(&file_path);
    let _ = fs::remove_file(file_path.with_extension("json.lock"));
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
