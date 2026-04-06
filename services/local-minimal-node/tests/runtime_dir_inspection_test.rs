use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_domain_core::message::{ContentPart, Message, MessageBody, MessageType, Sender};
use im_domain_events::CommitEnvelope;
use tower::ServiceExt;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_runtime_dir_inspection_{unique}_{sequence}"
    ))
}

fn write_runtime_state_file(runtime_dir: &Path, file_name: &str, content: &str) {
    let state_dir = runtime_dir.join("state");
    fs::create_dir_all(&state_dir).expect("state dir should be created");
    fs::write(state_dir.join(file_name), content).expect("state file should be written");
}

fn write_commit_journal(runtime_dir: &Path, events: &[CommitEnvelope]) {
    let payload = serde_json::to_string_pretty(events).expect("commit journal should serialize");
    write_runtime_state_file(runtime_dir, "commit-journal.json", payload.as_str());
}

fn invalid_replay_message_envelope() -> CommitEnvelope {
    let payload = serde_json::to_string(&Message {
        tenant_id: "t_demo".into(),
        conversation_id: "c_demo".into(),
        message_id: "msg_demo_1".into(),
        message_seq: 1,
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        message_type: MessageType::Standard,
        delivery_mode: "realtime".into(),
        client_msg_id: Some("client_demo_1".into()),
        stream_session_id: None,
        rtc_session_id: None,
        body: MessageBody {
            summary: Some("first".into()),
            parts: vec![ContentPart::text("first")],
            render_hints: Default::default(),
        },
        attributes: Default::default(),
        metadata: Default::default(),
        occurred_at: "1970-01-01T00:00:00Z".into(),
        committed_at: Some("1970-01-01T00:00:00Z".into()),
    })
    .expect("message payload should serialize");

    CommitEnvelope::minimal(
        "evt_invalid_journal",
        "t_demo",
        "message.posted",
        "conversation",
        "c_demo",
        1,
    )
    .with_payload("message.posted.v1", payload.as_str())
}

fn seed_runtime_state_file(runtime_dir: &Path, file_name: &str) {
    let content = if file_name == "commit-journal.json" {
        "[]"
    } else {
        "{}"
    };
    write_runtime_state_file(runtime_dir, file_name, content);
}

#[tokio::test]
async fn test_managed_runtime_dir_inspection_reports_all_expected_files_when_parseable() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    for file_name in [
        "commit-journal.json",
        "realtime-disconnect-fences.json",
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "presence-state.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
    ] {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/runtime-dir")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ops_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime-dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime-dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime-dir inspection body should be valid json");

    assert_eq!(json["status"], "ok");
    assert_eq!(json["healthyFileCount"], 9);
    assert_eq!(json["missingFileCount"], 0);
    assert_eq!(json["corruptFileCount"], 0);

    let files = json["files"].as_array().expect("files should be an array");
    assert_eq!(files.len(), 9);
    assert!(files.iter().any(|file| {
        file["fileName"] == "presence-state.json"
            && file["status"] == "ok"
            && file["recommendedAction"] == "none"
            && file["parseable"] == true
    }));

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_managed_runtime_dir_inspection_reports_missing_and_corrupt_files_as_degraded() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    for file_name in [
        "commit-journal.json",
        "realtime-disconnect-fences.json",
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
    ] {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }
    write_runtime_state_file(runtime_dir.as_path(), "rtc-state.json", "{not-valid-json");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/runtime-dir")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ops_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime-dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime-dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime-dir inspection body should be valid json");

    assert_eq!(json["status"], "degraded");
    assert_eq!(json["healthyFileCount"], 7);
    assert_eq!(json["missingFileCount"], 1);
    assert_eq!(json["corruptFileCount"], 1);

    let files = json["files"].as_array().expect("files should be an array");
    assert!(files.iter().any(|file| {
        file["fileName"] == "presence-state.json"
            && file["status"] == "missing"
            && file["recommendedAction"] == "recreate_on_next_managed_start_or_write"
            && file["exists"] == false
    }));
    assert!(files.iter().any(|file| {
        file["fileName"] == "rtc-state.json"
            && file["status"] == "corrupt"
            && file["recommendedAction"] == "manual_json_repair_or_restore"
            && file["parseable"] == false
            && file["parseError"].is_string()
    }));

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_managed_runtime_dir_inspection_reports_typed_store_shape_violation_as_corrupt() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    for file_name in [
        "commit-journal.json",
        "realtime-disconnect-fences.json",
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "presence-state.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
    ] {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }
    write_runtime_state_file(runtime_dir.as_path(), "realtime-checkpoints.json", "[]");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/runtime-dir")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_ops_demo")
                .header("x-permissions", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime-dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime-dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime-dir inspection body should be valid json");

    assert_eq!(json["status"], "degraded");
    let files = json["files"].as_array().expect("files should be an array");
    assert!(files.iter().any(|file| {
        file["fileName"] == "realtime-checkpoints.json"
            && file["status"] == "corrupt"
            && file["parseable"] == false
            && file["recommendedAction"] == "manual_json_repair_or_restore"
    }));

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_managed_runtime_dir_inspection_reports_journal_replay_violation_as_corrupt() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    for file_name in [
        "realtime-disconnect-fences.json",
        "realtime-checkpoints.json",
        "realtime-subscriptions.json",
        "presence-state.json",
        "stream-state.json",
        "rtc-state.json",
        "notification-tasks.json",
        "automation-executions.json",
    ] {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }
    write_commit_journal(runtime_dir.as_path(), &[invalid_replay_message_envelope()]);

    let view = local_minimal_node::inspect_runtime_dir(runtime_dir.as_path());

    assert_eq!(view.status, "degraded");
    assert!(view.files.iter().any(|file| {
        file.file_name == "commit-journal.json"
            && file.status == "corrupt"
            && file.parseable
            && file.recommended_action == "manual_json_repair_or_restore"
    }));

    let _ = fs::remove_dir_all(runtime_dir);
}
