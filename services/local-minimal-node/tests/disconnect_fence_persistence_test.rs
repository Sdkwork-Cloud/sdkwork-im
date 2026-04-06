use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_disconnect_fence_runtime_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_persists_disconnect_fence_across_rebuild_via_runtime_dir()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let resume_old = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad","lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("old resume should succeed before restart");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let disconnect = app_before
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad"}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed before restart");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let disconnect_fence_file = runtime_dir
        .join("state")
        .join("realtime-disconnect-fences.json");
    assert!(
        disconnect_fence_file.exists(),
        "default local-minimal runtime should persist disconnect fences under the runtime state dir"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let stale_heartbeat = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad"}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat should return a response after restart");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value =
        serde_json::from_slice(&stale_heartbeat_body).expect("stale heartbeat should be json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let fresh_resume = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad","lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should return a response");
    assert_eq!(fresh_resume.status(), StatusCode::OK);

    let fresh_heartbeat = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad"}"#))
                .unwrap(),
        )
        .await
        .expect("fresh heartbeat should return a response");
    assert_eq!(fresh_heartbeat.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}
