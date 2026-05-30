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
const MANAGED_RUNTIME_STATE_FILES: [&str; 13] = [
    "commit-journal.json",
    "realtime-disconnect-fences.json",
    "realtime-checkpoints.json",
    "realtime-event-windows.json",
    "realtime-subscriptions.json",
    "presence-state.json",
    "device-twin-state.json",
    "stream-state.json",
    "rtc-state.json",
    "notification-tasks.json",
    "automation-executions.json",
    "projection-metadata.json",
    "projection-timeline.json",
];

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
    let mut payload = String::new();
    for event in events {
        payload.push_str(
            serde_json::to_string(event)
                .expect("commit journal event should serialize")
                .as_str(),
        );
        payload.push('\n');
    }
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
    let content = match file_name {
        "commit-journal.json" => "",
        "presence-state.json" => {
            "{\"by_device\":{},\"presence_by_principal\":{},\"online_by_seen_at\":{}}"
        }
        "notification-tasks.json" => "{\"by_notification\":{},\"tasks_by_recipient\":{}}",
        _ => "{}",
    };
    write_runtime_state_file(runtime_dir, file_name, content);
}

#[tokio::test]
async fn test_managed_runtime_dir_inspection_reports_all_expected_files_when_parseable() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    for file_name in MANAGED_RUNTIME_STATE_FILES {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/runtime_dir")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ops_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime_dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime_dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime_dir inspection body should be valid json");

    assert_eq!(json["status"], "ok");
    assert_eq!(json["healthyFileCount"], MANAGED_RUNTIME_STATE_FILES.len());
    assert_eq!(json["missingFileCount"], 0);
    assert_eq!(json["corruptFileCount"], 0);

    let files = json["files"].as_array().expect("files should be an array");
    assert_eq!(files.len(), MANAGED_RUNTIME_STATE_FILES.len());
    assert!(files.iter().any(|file| {
        file["fileName"] == "presence-state.json"
            && file["status"] == "ok"
            && file["recommendedAction"] == "none"
            && file["parseable"] == true
    }));
    assert!(files.iter().any(|file| {
        file["fileName"] == "device-twin-state.json"
            && file["status"] == "ok"
            && file["recommendedAction"] == "none"
            && file["parseable"] == true
    }));
    assert!(files.iter().any(|file| {
        file["fileName"] == "projection-metadata.json"
            && file["status"] == "ok"
            && file["recommendedAction"] == "none"
            && file["parseable"] == true
    }));
    assert!(files.iter().any(|file| {
        file["fileName"] == "projection-timeline.json"
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

    for file_name in MANAGED_RUNTIME_STATE_FILES
        .iter()
        .copied()
        .filter(|file_name| *file_name != "presence-state.json")
    {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }
    write_runtime_state_file(runtime_dir.as_path(), "rtc-state.json", "{not-valid-json");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/runtime_dir")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ops_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime_dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime_dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime_dir inspection body should be valid json");

    assert_eq!(json["status"], "degraded");
    assert_eq!(
        json["healthyFileCount"],
        MANAGED_RUNTIME_STATE_FILES.len() - 2
    );
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

    for file_name in MANAGED_RUNTIME_STATE_FILES {
        seed_runtime_state_file(runtime_dir.as_path(), file_name);
    }
    write_runtime_state_file(runtime_dir.as_path(), "realtime-checkpoints.json", "[]");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/runtime_dir")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ops_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime_dir inspection should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("runtime_dir inspection body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("runtime_dir inspection body should be valid json");

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

    for file_name in MANAGED_RUNTIME_STATE_FILES
        .iter()
        .copied()
        .filter(|file_name| *file_name != "commit-journal.json")
    {
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
