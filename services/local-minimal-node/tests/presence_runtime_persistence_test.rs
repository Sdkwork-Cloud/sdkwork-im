use im_app_context::DualTokenRequestBuilderExt;
use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_presence_runtime_recovery_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_presence_runtime_and_requires_fresh_resume_after_restart()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    for device_id in ["d_phone", "d_pad"] {
        let register = app_before
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/presence/heartbeat")
                    .with_dual_token_tenant("t_demo")
                    .with_dual_token_user("u_demo")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_device(device_id)
                    .header("content-type", "application/json")
                    .body(Body::from(format!(r#"{{"deviceId":"{device_id}"}}"#)))
                    .unwrap(),
            )
            .await
            .expect("device register should return response");
        assert_eq!(register.status(), StatusCode::OK);
    }

    let resume_before = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_before")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume before restart should return response");
    assert_eq!(resume_before.status(), StatusCode::OK);

    let heartbeat_before = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_before")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("heartbeat before restart should return response");
    assert_eq!(heartbeat_before.status(), StatusCode::OK);

    assert!(
        runtime_dir
            .join("state")
            .join("presence-state.json")
            .exists(),
        "managed local-minimal should persist presence runtime state"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let presence_after = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/presence/me")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request after restart should return response");
    assert_eq!(presence_after.status(), StatusCode::OK);
    let presence_body = presence_after
        .into_body()
        .collect()
        .await
        .expect("presence body should collect")
        .to_bytes();
    let presence_json: serde_json::Value =
        serde_json::from_slice(&presence_body).expect("presence body should be valid json");
    let devices = presence_json["devices"]
        .as_array()
        .expect("devices should be an array");
    assert_eq!(devices.len(), 2);
    assert_eq!(presence_json["currentDeviceId"], "d_pad");
    assert_eq!(devices[0]["deviceId"], "d_pad");
    assert_eq!(devices[0]["status"], "offline");
    assert!(devices[0]["lastResumeAt"].is_string());
    assert!(devices[0]["lastSeenAt"].is_string());
    assert_eq!(devices[1]["deviceId"], "d_phone");
    assert_eq!(devices[1]["status"], "offline");

    let stale_heartbeat = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_before")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat after restart should return response");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value = serde_json::from_slice(&stale_heartbeat_body)
        .expect("stale heartbeat body should be valid json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let resume_after = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_session("s_after")
                .with_dual_token_device("d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume after restart should return response");
    assert_eq!(resume_after.status(), StatusCode::OK);
    let resume_after_body = resume_after
        .into_body()
        .collect()
        .await
        .expect("resume body should collect")
        .to_bytes();
    let resume_after_json: serde_json::Value =
        serde_json::from_slice(&resume_after_body).expect("resume body should be valid json");
    assert_eq!(
        resume_after_json["presence"]["devices"][0]["status"],
        "online"
    );
    assert_eq!(
        resume_after_json["presence"]["devices"][0]["sessionId"],
        "s_after"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
