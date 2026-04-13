use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde_json::json;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

type DisconnectFenceSnapshotWithOptionsEntry<'a> = (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    Option<&'a str>,
    &'a str,
    &'a str,
);
type RealtimeCheckpointSnapshotEntry<'a> =
    (&'a str, &'a str, &'a str, &'a str, u64, u64, u64, &'a str);
type RealtimeSubscriptionItemSnapshotEntry<'a> = (&'a str, &'a str, &'a [&'a str], &'a str);
type RealtimeSubscriptionSnapshotEntry<'a> = (
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a str,
    &'a [RealtimeSubscriptionItemSnapshotEntry<'a>],
);

fn unique_path(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!("craw_chat_{prefix}_{unique}_{sequence}"))
}

fn state_file(root: &Path, file_name: &str) -> PathBuf {
    root.join("state").join(file_name)
}

fn write_state_file(root: &Path, file_name: &str, content: &str) {
    let state_dir = root.join("state");
    fs::create_dir_all(&state_dir).expect("state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("state file should be written");
}

fn write_full_backup_snapshot(root: &Path, owner_node_id: &str) {
    write_state_file(root, "commit-journal.json", "[]\n");
    for file_name in [
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "presence-state.json",
        "device-twin-state.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
        "projection-metadata.json",
        "projection-timeline.json",
    ] {
        write_state_file(root, file_name, "{}\n");
    }
    write_state_file(
        root,
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": owner_node_id,
                "disconnected_at": "2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("disconnect fence snapshot should serialize")
        .as_str(),
    );
}

fn disconnect_fence_snapshot(entries: &[(&str, &str, &str, &str, &str, &str)]) -> String {
    let mut snapshot = serde_json::Map::new();
    for (key, tenant_id, principal_id, device_id, session_id, owner_node_id) in entries {
        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "principal_id": principal_id,
                "device_id": device_id,
                "session_id": session_id,
                "owner_node_id": owner_node_id,
                "disconnected_at": "2026-04-06T00:00:00.000Z"
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("disconnect fence snapshot should serialize")
}

fn disconnect_fence_snapshot_with_options(
    entries: &[DisconnectFenceSnapshotWithOptionsEntry<'_>],
) -> String {
    let mut snapshot = serde_json::Map::new();
    for (key, tenant_id, principal_id, device_id, session_id, owner_node_id, disconnected_at) in
        entries
    {
        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "principal_id": principal_id,
                "device_id": device_id,
                "session_id": session_id,
                "owner_node_id": owner_node_id,
                "disconnected_at": disconnected_at
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("disconnect fence snapshot should serialize")
}

fn realtime_checkpoint_snapshot_with_options(
    entries: &[RealtimeCheckpointSnapshotEntry<'_>],
) -> String {
    let mut snapshot = serde_json::Map::new();
    for (
        key,
        tenant_id,
        principal_id,
        device_id,
        latest_realtime_seq,
        acked_through_seq,
        trimmed_through_seq,
        updated_at,
    ) in entries
    {
        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "principal_id": principal_id,
                "device_id": device_id,
                "latest_realtime_seq": latest_realtime_seq,
                "acked_through_seq": acked_through_seq,
                "trimmed_through_seq": trimmed_through_seq,
                "updated_at": updated_at
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("checkpoint snapshot should serialize")
}

fn realtime_subscription_snapshot_with_options(
    entries: &[RealtimeSubscriptionSnapshotEntry<'_>],
) -> String {
    let mut snapshot = serde_json::Map::new();
    for (key, tenant_id, principal_id, device_id, synced_at, items) in entries {
        let serialized_items = items
            .iter()
            .map(|(scope_type, scope_id, event_types, subscribed_at)| {
                json!({
                    "scopeType": scope_type,
                    "scopeId": scope_id,
                    "eventTypes": event_types.to_vec(),
                    "subscribedAt": subscribed_at
                })
            })
            .collect::<Vec<_>>();
        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "principal_id": principal_id,
                "device_id": device_id,
                "items": serialized_items,
                "synced_at": synced_at
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("subscription snapshot should serialize")
}

#[derive(Clone, Copy)]
struct StreamSessionFixture<'a> {
    owner_principal_id: Option<&'a str>,
    owner_principal_kind: Option<&'a str>,
    stream_type: &'a str,
    scope_kind: &'a str,
    scope_id: &'a str,
    durability_class: &'a str,
    ordering_scope: &'a str,
    schema_ref: Option<&'a str>,
    state: &'a str,
    last_frame_seq: u64,
    last_checkpoint_seq: Option<u64>,
    result_message_id: Option<&'a str>,
    opened_at: &'a str,
    closed_at: Option<&'a str>,
    expires_at: Option<&'a str>,
}

#[derive(Clone, Copy)]
struct StreamFrameFixture<'a> {
    frame_seq: u64,
    frame_type: &'a str,
    schema_ref: Option<&'a str>,
    encoding: &'a str,
    payload: &'a str,
    sender_id: &'a str,
    sender_kind: &'a str,
    sender_device_id: Option<&'a str>,
    sender_session_id: Option<&'a str>,
    occurred_at: &'a str,
}

type StreamStateSnapshotEntry<'a> = (
    &'a str,
    &'a str,
    &'a str,
    StreamSessionFixture<'a>,
    &'a [StreamFrameFixture<'a>],
    &'a str,
);

fn stream_state_snapshot_with_options(entries: &[StreamStateSnapshotEntry<'_>]) -> String {
    let mut snapshot = serde_json::Map::new();
    for (key, tenant_id, stream_id, session, frames, updated_at) in entries {
        let serialized_frames = frames
            .iter()
            .map(|frame| {
                json!({
                    "tenantId": tenant_id,
                    "streamId": stream_id,
                    "streamType": session.stream_type,
                    "scopeKind": session.scope_kind,
                    "scopeId": session.scope_id,
                    "frameSeq": frame.frame_seq,
                    "frameType": frame.frame_type,
                    "schemaRef": frame.schema_ref,
                    "encoding": frame.encoding,
                    "payload": frame.payload,
                    "sender": {
                        "id": frame.sender_id,
                        "kind": frame.sender_kind,
                        "memberId": serde_json::Value::Null,
                        "deviceId": frame.sender_device_id,
                        "sessionId": frame.sender_session_id,
                        "metadata": {}
                    },
                    "attributes": {},
                    "occurredAt": frame.occurred_at
                })
            })
            .collect::<Vec<_>>();

        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "stream_id": stream_id,
                "session": {
                    "tenantId": tenant_id,
                    "streamId": stream_id,
                    "ownerPrincipalId": session.owner_principal_id,
                    "ownerPrincipalKind": session.owner_principal_kind,
                    "streamType": session.stream_type,
                    "scopeKind": session.scope_kind,
                    "scopeId": session.scope_id,
                    "durabilityClass": session.durability_class,
                    "orderingScope": session.ordering_scope,
                    "schemaRef": session.schema_ref,
                    "state": session.state,
                    "lastFrameSeq": session.last_frame_seq,
                    "lastCheckpointSeq": session.last_checkpoint_seq,
                    "resultMessageId": session.result_message_id,
                    "openedAt": session.opened_at,
                    "closedAt": session.closed_at,
                    "expiresAt": session.expires_at
                },
                "frames": serialized_frames,
                "updated_at": updated_at
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("stream state snapshot should serialize")
}

#[derive(Clone, Copy)]
struct RtcSessionFixture<'a> {
    conversation_id: Option<&'a str>,
    rtc_mode: &'a str,
    initiator_id: &'a str,
    initiator_kind: Option<&'a str>,
    state: &'a str,
    signaling_stream_id: Option<&'a str>,
    artifact_message_id: Option<&'a str>,
    started_at: &'a str,
    ended_at: Option<&'a str>,
}

#[derive(Clone, Copy)]
struct RtcSignalFixture<'a> {
    signal_type: &'a str,
    schema_ref: Option<&'a str>,
    payload: &'a str,
    sender_id: &'a str,
    sender_kind: &'a str,
    sender_device_id: Option<&'a str>,
    sender_session_id: Option<&'a str>,
    signaling_stream_id: Option<&'a str>,
    occurred_at: &'a str,
}

type RtcStateSnapshotEntry<'a> = (
    &'a str,
    &'a str,
    &'a str,
    RtcSessionFixture<'a>,
    &'a [RtcSignalFixture<'a>],
    &'a str,
);

fn rtc_state_snapshot_with_options(entries: &[RtcStateSnapshotEntry<'_>]) -> String {
    let mut snapshot = serde_json::Map::new();
    for (key, tenant_id, rtc_session_id, session, signals, updated_at) in entries {
        let serialized_signals = signals
            .iter()
            .map(|signal| {
                json!({
                    "tenantId": tenant_id,
                    "rtcSessionId": rtc_session_id,
                    "conversationId": session.conversation_id,
                    "rtcMode": session.rtc_mode,
                    "signalType": signal.signal_type,
                    "schemaRef": signal.schema_ref,
                    "payload": signal.payload,
                    "sender": {
                        "id": signal.sender_id,
                        "kind": signal.sender_kind,
                        "memberId": serde_json::Value::Null,
                        "deviceId": signal.sender_device_id,
                        "sessionId": signal.sender_session_id,
                        "metadata": {}
                    },
                    "signalingStreamId": signal.signaling_stream_id,
                    "occurredAt": signal.occurred_at
                })
            })
            .collect::<Vec<_>>();

        snapshot.insert(
            (*key).into(),
            json!({
                "tenant_id": tenant_id,
                "rtc_session_id": rtc_session_id,
                "session": {
                    "tenantId": tenant_id,
                    "rtcSessionId": rtc_session_id,
                    "conversationId": session.conversation_id,
                    "rtcMode": session.rtc_mode,
                    "initiatorId": session.initiator_id,
                    "initiatorKind": session.initiator_kind,
                    "state": session.state,
                    "signalingStreamId": session.signaling_stream_id,
                    "artifactMessageId": session.artifact_message_id,
                    "startedAt": session.started_at,
                    "endedAt": session.ended_at
                },
                "signals": serialized_signals,
                "updated_at": updated_at
            }),
        );
    }
    serde_json::to_string_pretty(&serde_json::Value::Object(snapshot))
        .expect("rtc state snapshot should serialize")
}

#[test]
fn test_preview_restore_runtime_dir_reports_ready_without_mutation_for_full_snapshot() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_ready");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": "node_current",
                "disconnected_at": "2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("current fence snapshot should serialize")
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_ready_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    assert_eq!(preview.status, "ready");
    assert_eq!(preview.source_snapshot_quality, "full_snapshot");
    assert_eq!(preview.source_managed_file_count, 12);
    assert_eq!(preview.source_missing_file_count, 0);
    assert_eq!(preview.would_restore_file_count, 1);
    assert_eq!(preview.unchanged_file_count, 11);
    assert_eq!(preview.skipped_file_count, 0);
    assert_eq!(preview.before.status, "ok");
    assert_eq!(preview.source_report_type, None);
    assert_eq!(preview.source_report_status, None);
    assert!(
        !preview.preview_fingerprint.is_empty(),
        "preview fingerprint should be exposed"
    );

    let preview_again = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("repeated restore preview should succeed");
    assert_eq!(
        preview.preview_fingerprint, preview_again.preview_fingerprint,
        "preview fingerprint should remain stable across repeated reads"
    );

    let fence_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "realtime-disconnect-fences.json")
        .expect("disconnect fence action should exist");
    assert_eq!(fence_action.action, "would_restore");
    assert_eq!(fence_action.detail, "content_differs");

    let journal_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "commit-journal.json")
        .expect("commit journal action should exist");
    assert_eq!(journal_action.action, "noop");
    assert_eq!(journal_action.detail, "source_matches_target");

    assert!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "realtime-disconnect-fences.json"
        ))
        .expect("preview must not mutate runtime state")
        .contains("node_current")
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_partial_for_sparse_snapshot() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_partial");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());
    fs::remove_file(state_file(runtime_dir.as_path(), "notification-tasks.json"))
        .expect("notification task file should be removed");

    let backup_dir = unique_path("runtime_dir_preview_restore_partial_backup");
    write_state_file(backup_dir.as_path(), "commit-journal.json", "[]\n");
    write_state_file(backup_dir.as_path(), "notification-tasks.json", "{}\n");
    fs::write(
        backup_dir.join("restore-report.json"),
        serde_json::to_string_pretty(&json!({
            "status": "partial"
        }))
        .expect("restore report should serialize")
        .as_bytes(),
    )
    .expect("restore report should be written");

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("sparse restore preview should succeed");

    assert_eq!(preview.status, "partial");
    assert_eq!(preview.source_snapshot_quality, "partial_snapshot");
    assert_eq!(preview.source_managed_file_count, 2);
    assert_eq!(preview.source_missing_file_count, 10);
    assert_eq!(preview.would_restore_file_count, 1);
    assert_eq!(preview.unchanged_file_count, 1);
    assert_eq!(preview.skipped_file_count, 10);
    assert_eq!(preview.before.status, "degraded");
    assert_eq!(preview.source_report_type.as_deref(), Some("restore"));
    assert_eq!(preview.source_report_status.as_deref(), Some("partial"));

    let notification_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "notification-tasks.json")
        .expect("notification task action should exist");
    assert_eq!(notification_action.action, "would_restore");
    assert_eq!(notification_action.detail, "target_missing");

    let skipped_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "rtc-state.json")
        .expect("rtc action should exist");
    assert_eq!(skipped_action.action, "skip");
    assert_eq!(skipped_action.detail, "missing_in_source_backup_snapshot");

    assert!(
        !state_file(runtime_dir.as_path(), "notification-tasks.json").exists(),
        "preview must not recreate missing runtime files"
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_tracks_projection_snapshot_files() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_projection_snapshot");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        serde_json::to_string_pretty(&json!({
            "t_demo:u_demo:d_demo": {
                "tenant_id": "t_demo",
                "principal_id": "u_demo",
                "device_id": "d_demo",
                "session_id": "s_demo",
                "owner_node_id": "node_backup",
                "disconnected_at": "2026-04-06T00:00:00.000Z"
            }
        }))
        .expect("current fence snapshot should serialize")
        .as_str(),
    );
    write_state_file(
        runtime_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"current\"}\n",
    );
    fs::remove_file(state_file(
        runtime_dir.as_path(),
        "projection-timeline.json",
    ))
    .expect("projection timeline file should be removed");

    let backup_dir = unique_path("runtime_dir_preview_restore_projection_snapshot_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "projection-metadata.json",
        "{\"t_demo:c_demo:conversation-summary\":\"backup\"}\n",
    );
    write_state_file(
        backup_dir.as_path(),
        "projection-timeline.json",
        "{\"t_demo:c_demo\":{\"1\":\"payload_backup\"}}\n",
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    assert_eq!(preview.status, "ready");
    assert_eq!(preview.source_managed_file_count, 12);
    assert_eq!(preview.source_missing_file_count, 0);
    assert_eq!(preview.would_restore_file_count, 2);
    assert_eq!(preview.unchanged_file_count, 10);
    assert_eq!(preview.skipped_file_count, 0);

    let metadata_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "projection-metadata.json")
        .expect("projection metadata action should exist");
    assert_eq!(metadata_action.action, "would_restore");
    assert_eq!(metadata_action.detail, "content_differs");
    let metadata_change_summary = metadata_action
        .change_summary
        .as_ref()
        .expect("projection metadata change summary should exist");
    assert_eq!(metadata_change_summary.summary_kind, "json_object_keys");
    assert_eq!(
        metadata_change_summary.modified_keys,
        vec!["t_demo:c_demo:conversation-summary".to_string()]
    );

    let timeline_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "projection-timeline.json")
        .expect("projection timeline action should exist");
    assert_eq!(timeline_action.action, "would_restore");
    assert_eq!(timeline_action.detail, "target_missing");
    assert!(timeline_action.change_summary.is_none());
    assert!(timeline_action.domain_summary.is_none());

    assert_eq!(
        fs::read_to_string(state_file(
            runtime_dir.as_path(),
            "projection-metadata.json"
        ))
        .expect("preview must not mutate projection metadata state"),
        "{\"t_demo:c_demo:conversation-summary\":\"current\"}\n"
    );
    assert!(
        !state_file(runtime_dir.as_path(), "projection-timeline.json").exists(),
        "preview must not recreate missing projection timeline state"
    );

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_json_object_key_change_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_diff_summary");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        disconnect_fence_snapshot(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                "s_same",
                "node_same",
            ),
            (
                "t_demo:u_demo:d_modified",
                "t_demo",
                "u_demo",
                "d_modified",
                "s_modified",
                "node_current",
            ),
            (
                "t_demo:u_demo:d_removed",
                "t_demo",
                "u_demo",
                "d_removed",
                "s_removed",
                "node_removed",
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_diff_summary_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "realtime-disconnect-fences.json",
        disconnect_fence_snapshot(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                "s_same",
                "node_same",
            ),
            (
                "t_demo:u_demo:d_modified",
                "t_demo",
                "u_demo",
                "d_modified",
                "s_modified",
                "node_backup",
            ),
            (
                "t_demo:u_demo:d_added",
                "t_demo",
                "u_demo",
                "d_added",
                "s_added",
                "node_added",
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let fence_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "realtime-disconnect-fences.json")
        .expect("disconnect fence action should exist");
    let change_summary = fence_action
        .change_summary
        .as_ref()
        .expect("json object diff summary should exist");
    assert_eq!(change_summary.summary_kind, "json_object_keys");
    assert_eq!(
        change_summary.added_keys,
        vec!["t_demo:u_demo:d_added".to_string()]
    );
    assert_eq!(
        change_summary.removed_keys,
        vec!["t_demo:u_demo:d_removed".to_string()]
    );
    assert_eq!(
        change_summary.modified_keys,
        vec!["t_demo:u_demo:d_modified".to_string()]
    );
    assert_eq!(change_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("json-object-diff"));
    assert!(rendered.contains("t_demo:u_demo:d_added"));
    assert!(rendered.contains("t_demo:u_demo:d_removed"));
    assert!(rendered.contains("t_demo:u_demo:d_modified"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_disconnect_fence_typed_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_disconnect_fence_typed");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-disconnect-fences.json",
        disconnect_fence_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                Some("s_same"),
                "node_same",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_owner_changed",
                "t_demo",
                "u_demo",
                "d_owner_changed",
                Some("s_owner_changed"),
                "node_current",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_session_changed",
                "t_demo",
                "u_demo",
                "d_session_changed",
                Some("s_current"),
                "node_same",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_other_changed",
                "t_demo",
                "u_demo",
                "d_other_changed",
                Some("s_other"),
                "node_same",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_removed",
                "t_demo",
                "u_demo",
                "d_removed",
                Some("s_removed"),
                "node_removed",
                "2026-04-06T00:00:00.000Z",
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_disconnect_fence_typed_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "realtime-disconnect-fences.json",
        disconnect_fence_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                Some("s_same"),
                "node_same",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_owner_changed",
                "t_demo",
                "u_demo",
                "d_owner_changed",
                Some("s_owner_changed"),
                "node_backup",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_session_changed",
                "t_demo",
                "u_demo",
                "d_session_changed",
                None,
                "node_same",
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_other_changed",
                "t_demo",
                "u_demo",
                "d_other_changed",
                Some("s_other"),
                "node_same",
                "2026-04-06T00:01:00.000Z",
            ),
            (
                "t_demo:u_demo:d_added",
                "t_demo",
                "u_demo",
                "d_added",
                Some("s_added"),
                "node_added",
                "2026-04-06T00:00:00.000Z",
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let fence_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "realtime-disconnect-fences.json")
        .expect("disconnect fence action should exist");
    let domain_summary = fence_action
        .domain_summary
        .as_ref()
        .expect("disconnect fence typed summary should exist");
    assert_eq!(domain_summary.summary_kind, "disconnect_fences");
    assert_eq!(
        domain_summary.added_keys,
        vec!["t_demo:u_demo:d_added".to_string()]
    );
    assert_eq!(
        domain_summary.removed_keys,
        vec!["t_demo:u_demo:d_removed".to_string()]
    );
    assert_eq!(
        domain_summary.owner_node_changed_keys,
        vec!["t_demo:u_demo:d_owner_changed".to_string()]
    );
    assert_eq!(
        domain_summary.session_changed_keys,
        vec!["t_demo:u_demo:d_session_changed".to_string()]
    );
    assert_eq!(
        domain_summary.other_modified_keys,
        vec!["t_demo:u_demo:d_other_changed".to_string()]
    );
    assert_eq!(domain_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("disconnect-fence-diff"));
    assert!(rendered.contains("d_added"));
    assert!(rendered.contains("d_removed"));
    assert!(rendered.contains("d_owner_changed"));
    assert!(rendered.contains("d_session_changed"));
    assert!(rendered.contains("d_other_changed"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_checkpoint_typed_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_checkpoint_typed");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-checkpoints.json",
        realtime_checkpoint_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_latest_advanced",
                "t_demo",
                "u_demo",
                "d_latest_advanced",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_latest_rewound",
                "t_demo",
                "u_demo",
                "d_latest_rewound",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_acked_advanced",
                "t_demo",
                "u_demo",
                "d_acked_advanced",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_acked_rewound",
                "t_demo",
                "u_demo",
                "d_acked_rewound",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_trimmed_advanced",
                "t_demo",
                "u_demo",
                "d_trimmed_advanced",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_trimmed_rewound",
                "t_demo",
                "u_demo",
                "d_trimmed_rewound",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_timestamp_only",
                "t_demo",
                "u_demo",
                "d_timestamp_only",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_other_modified",
                "t_demo",
                "u_demo",
                "d_other_modified",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_removed",
                "t_demo",
                "u_demo",
                "d_removed",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_checkpoint_typed_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "realtime-checkpoints.json",
        realtime_checkpoint_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_latest_advanced",
                "t_demo",
                "u_demo",
                "d_latest_advanced",
                12,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_latest_rewound",
                "t_demo",
                "u_demo",
                "d_latest_rewound",
                9,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_acked_advanced",
                "t_demo",
                "u_demo",
                "d_acked_advanced",
                10,
                9,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_acked_rewound",
                "t_demo",
                "u_demo",
                "d_acked_rewound",
                10,
                7,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_trimmed_advanced",
                "t_demo",
                "u_demo",
                "d_trimmed_advanced",
                10,
                8,
                7,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_trimmed_rewound",
                "t_demo",
                "u_demo",
                "d_trimmed_rewound",
                10,
                8,
                5,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_timestamp_only",
                "t_demo",
                "u_demo",
                "d_timestamp_only",
                10,
                8,
                6,
                "2026-04-06T00:01:00.000Z",
            ),
            (
                "t_demo:u_demo:d_other_modified",
                "t_demo",
                "u_other",
                "d_other_modified",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
            (
                "t_demo:u_demo:d_added",
                "t_demo",
                "u_demo",
                "d_added",
                10,
                8,
                6,
                "2026-04-06T00:00:00.000Z",
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let checkpoint_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "realtime-checkpoints.json")
        .expect("checkpoint action should exist");
    let domain_summary = checkpoint_action
        .domain_summary
        .as_ref()
        .expect("checkpoint typed summary should exist");
    assert_eq!(domain_summary.summary_kind, "realtime_checkpoints");
    assert_eq!(
        domain_summary.added_keys,
        vec!["t_demo:u_demo:d_added".to_string()]
    );
    assert_eq!(
        domain_summary.removed_keys,
        vec!["t_demo:u_demo:d_removed".to_string()]
    );
    assert_eq!(
        domain_summary.latest_advanced_keys.clone(),
        Some(vec!["t_demo:u_demo:d_latest_advanced".to_string()])
    );
    assert_eq!(
        domain_summary.latest_rewound_keys.clone(),
        Some(vec!["t_demo:u_demo:d_latest_rewound".to_string()])
    );
    assert_eq!(
        domain_summary.acked_advanced_keys.clone(),
        Some(vec!["t_demo:u_demo:d_acked_advanced".to_string()])
    );
    assert_eq!(
        domain_summary.acked_rewound_keys.clone(),
        Some(vec!["t_demo:u_demo:d_acked_rewound".to_string()])
    );
    assert_eq!(
        domain_summary.trimmed_advanced_keys.clone(),
        Some(vec!["t_demo:u_demo:d_trimmed_advanced".to_string()])
    );
    assert_eq!(
        domain_summary.trimmed_rewound_keys.clone(),
        Some(vec!["t_demo:u_demo:d_trimmed_rewound".to_string()])
    );
    assert_eq!(
        domain_summary.timestamp_only_changed_keys.clone(),
        Some(vec!["t_demo:u_demo:d_timestamp_only".to_string()])
    );
    assert_eq!(
        domain_summary.other_modified_keys,
        vec!["t_demo:u_demo:d_other_modified".to_string()]
    );
    assert_eq!(domain_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("checkpoint-diff"));
    assert!(rendered.contains("d_latest_advanced"));
    assert!(rendered.contains("d_latest_rewound"));
    assert!(rendered.contains("d_acked_advanced"));
    assert!(rendered.contains("d_acked_rewound"));
    assert!(rendered.contains("d_trimmed_advanced"));
    assert!(rendered.contains("d_trimmed_rewound"));
    assert!(rendered.contains("d_timestamp_only"));
    assert!(rendered.contains("d_other_modified"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_subscription_typed_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_subscription_typed");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    write_state_file(
        runtime_dir.as_path(),
        "realtime-subscriptions.json",
        realtime_subscription_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_same",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_scope_changed",
                "t_demo",
                "u_demo",
                "d_scope_changed",
                "2026-04-06T00:00:00.000Z",
                &[
                    (
                        "conversation",
                        "c_same",
                        &["message.posted"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "conversation",
                        "c_removed",
                        &["message.posted"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "stream",
                        "s_event_added",
                        &["stream.frame"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "stream",
                        "s_event_removed",
                        &["stream.closed", "stream.frame"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "bot",
                        "b_time",
                        &["agent.delta"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                ],
            ),
            (
                "t_demo:u_demo:d_synced_only",
                "t_demo",
                "u_demo",
                "d_synced_only",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_sync",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_other_modified",
                "t_demo",
                "u_demo",
                "d_other_modified",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_other",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_removed",
                "t_demo",
                "u_demo",
                "d_removed",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_removed",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_subscription_typed_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "realtime-subscriptions.json",
        realtime_subscription_snapshot_with_options(&[
            (
                "t_demo:u_demo:d_same",
                "t_demo",
                "u_demo",
                "d_same",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_same",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_scope_changed",
                "t_demo",
                "u_demo",
                "d_scope_changed",
                "2026-04-06T00:01:00.000Z",
                &[
                    (
                        "conversation",
                        "c_same",
                        &["message.posted"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "conversation",
                        "c_added",
                        &["message.posted"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "stream",
                        "s_event_added",
                        &["stream.closed", "stream.frame"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "stream",
                        "s_event_removed",
                        &["stream.frame"],
                        "2026-04-06T00:00:00.000Z",
                    ),
                    (
                        "bot",
                        "b_time",
                        &["agent.delta"],
                        "2026-04-06T00:01:00.000Z",
                    ),
                ],
            ),
            (
                "t_demo:u_demo:d_synced_only",
                "t_demo",
                "u_demo",
                "d_synced_only",
                "2026-04-06T00:01:00.000Z",
                &[(
                    "conversation",
                    "c_sync",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_other_modified",
                "t_demo",
                "u_other",
                "d_other_modified",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_other",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
            (
                "t_demo:u_demo:d_added",
                "t_demo",
                "u_demo",
                "d_added",
                "2026-04-06T00:00:00.000Z",
                &[(
                    "conversation",
                    "c_added",
                    &["message.posted"],
                    "2026-04-06T00:00:00.000Z",
                )],
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let subscription_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "realtime-subscriptions.json")
        .expect("subscription action should exist");
    let domain_summary = subscription_action
        .domain_summary
        .as_ref()
        .expect("subscription typed summary should exist");
    assert_eq!(domain_summary.summary_kind, "realtime_subscriptions");
    assert_eq!(
        domain_summary.added_keys,
        vec!["t_demo:u_demo:d_added".to_string()]
    );
    assert_eq!(
        domain_summary.removed_keys,
        vec!["t_demo:u_demo:d_removed".to_string()]
    );
    assert_eq!(
        domain_summary.other_modified_keys,
        vec!["t_demo:u_demo:d_other_modified".to_string()]
    );
    assert_eq!(
        domain_summary.timestamp_only_changed_keys.clone(),
        Some(vec!["t_demo:u_demo:d_synced_only".to_string()])
    );

    let domain_summary_json =
        serde_json::to_value(domain_summary).expect("domain summary should serialize");
    assert_eq!(
        domain_summary_json["addedScopeKeys"],
        json!(["t_demo:u_demo:d_scope_changed#conversation:c_added"])
    );
    assert_eq!(
        domain_summary_json["removedScopeKeys"],
        json!(["t_demo:u_demo:d_scope_changed#conversation:c_removed"])
    );
    assert_eq!(
        domain_summary_json["eventTypesAddedScopeKeys"],
        json!(["t_demo:u_demo:d_scope_changed#stream:s_event_added"])
    );
    assert_eq!(
        domain_summary_json["eventTypesRemovedScopeKeys"],
        json!(["t_demo:u_demo:d_scope_changed#stream:s_event_removed"])
    );
    assert_eq!(
        domain_summary_json["subscribedAtOnlyChangedScopeKeys"],
        json!(["t_demo:u_demo:d_scope_changed#bot:b_time"])
    );
    assert_eq!(domain_summary_json["unchangedScopeCount"], json!(2));
    assert_eq!(domain_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("subscription-diff"));
    assert!(rendered.contains("scope_added"));
    assert!(rendered.contains("scope_removed"));
    assert!(rendered.contains("event_types_added"));
    assert!(rendered.contains("event_types_removed"));
    assert!(rendered.contains("subscribed_at_only"));
    assert!(rendered.contains("synced_timestamp_only"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_stream_typed_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_stream_typed");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    let base_session = StreamSessionFixture {
        owner_principal_id: Some("u_demo"),
        owner_principal_kind: Some("user"),
        stream_type: "custom.delta.text",
        scope_kind: "conversation",
        scope_id: "c_stream",
        durability_class: "durableSession",
        ordering_scope: "stream",
        schema_ref: Some("custom.delta.text.v1"),
        state: "active",
        last_frame_seq: 1,
        last_checkpoint_seq: Some(1),
        result_message_id: None,
        opened_at: "2026-04-06T00:00:00.000Z",
        closed_at: None,
        expires_at: None,
    };
    let frame_one = StreamFrameFixture {
        frame_seq: 1,
        frame_type: "delta",
        schema_ref: Some("custom.delta.text.v1"),
        encoding: "json",
        payload: "{\"delta\":\"a\"}",
        sender_id: "u_demo",
        sender_kind: "user",
        sender_device_id: Some("d_demo"),
        sender_session_id: Some("s_demo"),
        occurred_at: "2026-04-06T00:00:01.000Z",
    };
    let frame_two = StreamFrameFixture {
        frame_seq: 2,
        frame_type: "delta",
        schema_ref: Some("custom.delta.text.v1"),
        encoding: "json",
        payload: "{\"delta\":\"b\"}",
        sender_id: "u_demo",
        sender_kind: "user",
        sender_device_id: Some("d_demo"),
        sender_session_id: Some("s_demo"),
        occurred_at: "2026-04-06T00:00:02.000Z",
    };
    let frame_two_modified = StreamFrameFixture {
        payload: "{\"delta\":\"changed\"}",
        ..frame_two
    };

    write_state_file(
        runtime_dir.as_path(),
        "stream-state.json",
        stream_state_snapshot_with_options(&[
            (
                "t_demo:st_same",
                "t_demo",
                "st_same",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_state_changed",
                "t_demo",
                "st_state_changed",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_last_frame_advanced",
                "t_demo",
                "st_last_frame_advanced",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_last_frame_rewound",
                "t_demo",
                "st_last_frame_rewound",
                StreamSessionFixture {
                    last_frame_seq: 4,
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_checkpoint_advanced",
                "t_demo",
                "st_checkpoint_advanced",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_checkpoint_rewound",
                "t_demo",
                "st_checkpoint_rewound",
                StreamSessionFixture {
                    last_checkpoint_seq: Some(5),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_result_changed",
                "t_demo",
                "st_result_changed",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_added",
                "t_demo",
                "st_frame_added",
                StreamSessionFixture {
                    last_frame_seq: 1,
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_removed",
                "t_demo",
                "st_frame_removed",
                StreamSessionFixture {
                    last_frame_seq: 2,
                    ..base_session
                },
                &[frame_one, frame_two],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_modified",
                "t_demo",
                "st_frame_modified",
                StreamSessionFixture {
                    last_frame_seq: 2,
                    ..base_session
                },
                &[frame_one, frame_two],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_timestamp_only",
                "t_demo",
                "st_timestamp_only",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_other_modified",
                "t_demo",
                "st_other_modified",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_owner_changed",
                "t_demo",
                "st_owner_changed",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_removed",
                "t_demo",
                "st_removed",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_stream_typed_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "stream-state.json",
        stream_state_snapshot_with_options(&[
            (
                "t_demo:st_same",
                "t_demo",
                "st_same",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_state_changed",
                "t_demo",
                "st_state_changed",
                StreamSessionFixture {
                    state: "completed",
                    result_message_id: Some("msg_state_changed"),
                    closed_at: Some("2026-04-06T00:00:30.000Z"),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:30.000Z",
            ),
            (
                "t_demo:st_last_frame_advanced",
                "t_demo",
                "st_last_frame_advanced",
                StreamSessionFixture {
                    last_frame_seq: 3,
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_last_frame_rewound",
                "t_demo",
                "st_last_frame_rewound",
                StreamSessionFixture {
                    last_frame_seq: 2,
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_checkpoint_advanced",
                "t_demo",
                "st_checkpoint_advanced",
                StreamSessionFixture {
                    last_checkpoint_seq: Some(3),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_checkpoint_rewound",
                "t_demo",
                "st_checkpoint_rewound",
                StreamSessionFixture {
                    last_checkpoint_seq: Some(2),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_result_changed",
                "t_demo",
                "st_result_changed",
                StreamSessionFixture {
                    result_message_id: Some("msg_stream_result"),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_added",
                "t_demo",
                "st_frame_added",
                StreamSessionFixture {
                    last_frame_seq: 2,
                    ..base_session
                },
                &[frame_one, frame_two],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_removed",
                "t_demo",
                "st_frame_removed",
                StreamSessionFixture {
                    last_frame_seq: 1,
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_frame_modified",
                "t_demo",
                "st_frame_modified",
                StreamSessionFixture {
                    last_frame_seq: 2,
                    ..base_session
                },
                &[frame_one, frame_two_modified],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_timestamp_only",
                "t_demo",
                "st_timestamp_only",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:20.000Z",
            ),
            (
                "t_demo:st_other_modified",
                "t_demo",
                "st_other_modified",
                StreamSessionFixture {
                    ordering_scope: "conversation",
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_owner_changed",
                "t_demo",
                "st_owner_changed",
                StreamSessionFixture {
                    owner_principal_id: Some("u_other_demo"),
                    ..base_session
                },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:st_added",
                "t_demo",
                "st_added",
                StreamSessionFixture { ..base_session },
                &[frame_one],
                "2026-04-06T00:00:10.000Z",
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let stream_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "stream-state.json")
        .expect("stream action should exist");
    let domain_summary = stream_action
        .domain_summary
        .as_ref()
        .expect("stream typed summary should exist");
    assert_eq!(domain_summary.summary_kind, "stream_state");
    assert_eq!(
        domain_summary.added_keys,
        vec!["t_demo:st_added".to_string()]
    );
    assert_eq!(
        domain_summary.removed_keys,
        vec!["t_demo:st_removed".to_string()]
    );
    assert_eq!(
        domain_summary.other_modified_keys,
        vec![
            "t_demo:st_other_modified".to_string(),
            "t_demo:st_owner_changed".to_string()
        ]
    );
    assert_eq!(
        domain_summary.timestamp_only_changed_keys.clone(),
        Some(vec!["t_demo:st_timestamp_only".to_string()])
    );

    let domain_summary_json =
        serde_json::to_value(domain_summary).expect("domain summary should serialize");
    assert_eq!(
        domain_summary_json["streamStateChangedKeys"],
        json!(["t_demo:st_state_changed"])
    );
    assert_eq!(
        domain_summary_json["streamLastFrameAdvancedKeys"],
        json!(["t_demo:st_frame_added", "t_demo:st_last_frame_advanced"])
    );
    assert_eq!(
        domain_summary_json["streamLastFrameRewoundKeys"],
        json!(["t_demo:st_frame_removed", "t_demo:st_last_frame_rewound"])
    );
    assert_eq!(
        domain_summary_json["streamCheckpointAdvancedKeys"],
        json!(["t_demo:st_checkpoint_advanced"])
    );
    assert_eq!(
        domain_summary_json["streamCheckpointRewoundKeys"],
        json!(["t_demo:st_checkpoint_rewound"])
    );
    assert_eq!(
        domain_summary_json["streamResultMessageChangedKeys"],
        json!(["t_demo:st_result_changed", "t_demo:st_state_changed"])
    );
    assert_eq!(
        domain_summary_json["addedFrameKeys"],
        json!(["t_demo:st_frame_added#frame:2"])
    );
    assert_eq!(
        domain_summary_json["removedFrameKeys"],
        json!(["t_demo:st_frame_removed#frame:2"])
    );
    assert_eq!(
        domain_summary_json["modifiedFrameKeys"],
        json!(["t_demo:st_frame_modified#frame:2"])
    );
    assert_eq!(domain_summary_json["unchangedFrameCount"], json!(12));
    assert_eq!(domain_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("stream-diff"));
    assert!(rendered.contains("state_changed"));
    assert!(rendered.contains("last_frame_advanced"));
    assert!(rendered.contains("last_frame_rewound"));
    assert!(rendered.contains("checkpoint_advanced"));
    assert!(rendered.contains("checkpoint_rewound"));
    assert!(rendered.contains("result_message_changed"));
    assert!(rendered.contains("frame_added"));
    assert!(rendered.contains("frame_removed"));
    assert!(rendered.contains("frame_modified"));
    assert!(rendered.contains("updated_at_only"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_reports_rtc_typed_summary() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_rtc_typed");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");
    let _ = local_minimal_node::repair_runtime_dir(runtime_dir.as_path());

    let base_rtc_session = RtcSessionFixture {
        conversation_id: Some("c_rtc"),
        rtc_mode: "voice",
        initiator_id: "u_demo",
        initiator_kind: Some("user"),
        state: "started",
        signaling_stream_id: None,
        artifact_message_id: None,
        started_at: "2026-04-06T00:00:00.000Z",
        ended_at: None,
    };
    let signal_offer = RtcSignalFixture {
        signal_type: "rtc.offer",
        schema_ref: Some("webrtc.offer.v1"),
        payload: "{\"sdp\":\"offer\"}",
        sender_id: "u_demo",
        sender_kind: "user",
        sender_device_id: Some("d_demo"),
        sender_session_id: Some("s_demo"),
        signaling_stream_id: None,
        occurred_at: "2026-04-06T00:00:01.000Z",
    };
    let signal_answer = RtcSignalFixture {
        signal_type: "rtc.answer",
        schema_ref: Some("webrtc.answer.v1"),
        payload: "{\"sdp\":\"answer\"}",
        sender_id: "u_peer",
        sender_kind: "user",
        sender_device_id: Some("d_peer"),
        sender_session_id: Some("s_peer"),
        signaling_stream_id: Some("st_rtc"),
        occurred_at: "2026-04-06T00:00:02.000Z",
    };
    let signal_answer_modified = RtcSignalFixture {
        payload: "{\"sdp\":\"answer-changed\"}",
        ..signal_answer
    };

    write_state_file(
        runtime_dir.as_path(),
        "rtc-state.json",
        rtc_state_snapshot_with_options(&[
            (
                "t_demo:rtc_same",
                "t_demo",
                "rtc_same",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_state_changed",
                "t_demo",
                "rtc_state_changed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signaling_stream_changed",
                "t_demo",
                "rtc_signaling_stream_changed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_artifact_changed",
                "t_demo",
                "rtc_artifact_changed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_added",
                "t_demo",
                "rtc_signal_added",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_removed",
                "t_demo",
                "rtc_signal_removed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer, signal_answer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_modified",
                "t_demo",
                "rtc_signal_modified",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer, signal_answer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_timestamp_only",
                "t_demo",
                "rtc_timestamp_only",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_other_modified",
                "t_demo",
                "rtc_other_modified",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_removed",
                "t_demo",
                "rtc_removed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
        ])
        .as_str(),
    );

    let backup_dir = unique_path("runtime_dir_preview_restore_rtc_typed_backup");
    write_full_backup_snapshot(backup_dir.as_path(), "node_backup");
    write_state_file(
        backup_dir.as_path(),
        "rtc-state.json",
        rtc_state_snapshot_with_options(&[
            (
                "t_demo:rtc_same",
                "t_demo",
                "rtc_same",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_state_changed",
                "t_demo",
                "rtc_state_changed",
                RtcSessionFixture {
                    state: "accepted",
                    ..base_rtc_session
                },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signaling_stream_changed",
                "t_demo",
                "rtc_signaling_stream_changed",
                RtcSessionFixture {
                    signaling_stream_id: Some("st_rtc"),
                    ..base_rtc_session
                },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_artifact_changed",
                "t_demo",
                "rtc_artifact_changed",
                RtcSessionFixture {
                    artifact_message_id: Some("msg_rtc_artifact"),
                    ..base_rtc_session
                },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_added",
                "t_demo",
                "rtc_signal_added",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer, signal_answer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_removed",
                "t_demo",
                "rtc_signal_removed",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_signal_modified",
                "t_demo",
                "rtc_signal_modified",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer, signal_answer_modified],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_timestamp_only",
                "t_demo",
                "rtc_timestamp_only",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:20.000Z",
            ),
            (
                "t_demo:rtc_other_modified",
                "t_demo",
                "rtc_other_modified",
                RtcSessionFixture {
                    rtc_mode: "video",
                    ..base_rtc_session
                },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
            (
                "t_demo:rtc_added",
                "t_demo",
                "rtc_added",
                RtcSessionFixture { ..base_rtc_session },
                &[signal_offer],
                "2026-04-06T00:00:10.000Z",
            ),
        ])
        .as_str(),
    );

    let preview = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        backup_dir.as_path(),
    )
    .expect("restore preview should succeed");

    let rtc_action = preview
        .actions
        .iter()
        .find(|action| action.file_name == "rtc-state.json")
        .expect("rtc action should exist");
    let domain_summary = rtc_action
        .domain_summary
        .as_ref()
        .expect("rtc typed summary should exist");
    assert_eq!(domain_summary.summary_kind, "rtc_state");
    assert_eq!(
        domain_summary.added_keys,
        vec!["t_demo:rtc_added".to_string()]
    );
    assert_eq!(
        domain_summary.removed_keys,
        vec!["t_demo:rtc_removed".to_string()]
    );
    assert_eq!(
        domain_summary.other_modified_keys,
        vec!["t_demo:rtc_other_modified".to_string()]
    );
    assert_eq!(
        domain_summary.timestamp_only_changed_keys.clone(),
        Some(vec!["t_demo:rtc_timestamp_only".to_string()])
    );

    let domain_summary_json =
        serde_json::to_value(domain_summary).expect("domain summary should serialize");
    assert_eq!(
        domain_summary_json["rtcStateChangedKeys"],
        json!(["t_demo:rtc_state_changed"])
    );
    assert_eq!(
        domain_summary_json["rtcSignalingStreamChangedKeys"],
        json!(["t_demo:rtc_signaling_stream_changed"])
    );
    assert_eq!(
        domain_summary_json["rtcArtifactMessageChangedKeys"],
        json!(["t_demo:rtc_artifact_changed"])
    );
    assert_eq!(
        domain_summary_json["addedSignalKeys"],
        json!(["t_demo:rtc_signal_added#signal:1"])
    );
    assert_eq!(
        domain_summary_json["removedSignalKeys"],
        json!(["t_demo:rtc_signal_removed#signal:1"])
    );
    assert_eq!(
        domain_summary_json["modifiedSignalKeys"],
        json!([
            "t_demo:rtc_other_modified#signal:0",
            "t_demo:rtc_signal_modified#signal:1"
        ])
    );
    assert_eq!(domain_summary_json["unchangedSignalCount"], json!(7));
    assert_eq!(domain_summary.unchanged_key_count, 1);

    let rendered = local_minimal_node::format_runtime_dir_restore_preview(&preview);
    assert!(rendered.contains("rtc-diff"));
    assert!(rendered.contains("state_changed"));
    assert!(rendered.contains("signaling_stream_changed"));
    assert!(rendered.contains("artifact_message_changed"));
    assert!(rendered.contains("signal_added"));
    assert!(rendered.contains("signal_removed"));
    assert!(rendered.contains("signal_modified"));
    assert!(rendered.contains("updated_at_only"));

    let _ = fs::remove_dir_all(runtime_dir);
    let _ = fs::remove_dir_all(backup_dir);
}

#[test]
fn test_preview_restore_runtime_dir_rejects_missing_backup_dir_without_mutation() {
    let runtime_dir = unique_path("runtime_dir_preview_restore_missing_backup");
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let missing_backup_dir = unique_path("runtime_dir_preview_missing_backup_input");
    let error = local_minimal_node::preview_restore_runtime_dir(
        runtime_dir.as_path(),
        missing_backup_dir.as_path(),
    )
    .expect_err("missing backup dir should fail");

    assert!(error.contains("backup dir does not exist"));
    assert!(!runtime_dir.join("state").exists());

    let _ = fs::remove_dir_all(runtime_dir);
}
