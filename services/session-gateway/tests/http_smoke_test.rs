use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryRealtimeDisconnectFenceStore;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthz_returns_ok_and_service_metadata() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["status"], "ok");
    assert_eq!(value["service"], "session-gateway");
}

#[tokio::test]
async fn test_session_resume_returns_presence_snapshot_for_current_device() {
    let app = session_gateway::build_app();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "deviceId":"d_demo",
                        "lastSeenSyncSeq":0
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["deviceId"], "d_demo");
    assert_eq!(value["resumeRequired"], false);
    assert_eq!(value["presence"]["currentDeviceId"], "d_demo");
    assert_eq!(value["presence"]["devices"][0]["deviceId"], "d_demo");
    assert_eq!(value["presence"]["devices"][0]["status"], "online");

    let snapshot = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");

    assert_eq!(snapshot.status(), StatusCode::OK);
    let snapshot_body = snapshot
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_value: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot should be valid json");

    assert_eq!(snapshot_value["currentDeviceId"], "d_demo");
    assert_eq!(snapshot_value["devices"][0]["status"], "online");
}

#[tokio::test]
async fn test_session_resume_rejects_mismatched_bound_device_id() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "deviceId":"d_other",
                        "lastSeenSyncSeq":0
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("resume request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["code"], "device_id_mismatch");
}

#[tokio::test]
async fn test_presence_snapshot_isolated_by_actor_kind_over_http() {
    let app = session_gateway::build_app();

    let user_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_user")
                .header("x-device-id", "d_user")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("user resume should succeed");
    assert_eq!(user_resume.status(), StatusCode::OK);

    let agent_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-session-id", "s_agent")
                .header("x-device-id", "d_agent")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("agent resume should succeed");
    assert_eq!(agent_resume.status(), StatusCode::OK);

    let user_presence = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_user")
                .header("x-device-id", "d_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user presence request should succeed");
    assert_eq!(user_presence.status(), StatusCode::OK);
    let user_presence_body = user_presence
        .into_body()
        .collect()
        .await
        .expect("user presence body should collect")
        .to_bytes();
    let user_presence_json: serde_json::Value =
        serde_json::from_slice(&user_presence_body).expect("user presence should be valid json");
    let user_devices = user_presence_json["devices"]
        .as_array()
        .expect("user devices should be an array");
    assert_eq!(
        user_devices.len(),
        1,
        "user presence should not include devices from another actor kind sharing the same actor id"
    );
    assert_eq!(user_devices[0]["deviceId"], "d_user");

    let agent_presence = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-session-id", "s_agent")
                .header("x-device-id", "d_agent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("agent presence request should succeed");
    assert_eq!(agent_presence.status(), StatusCode::OK);
    let agent_presence_body = agent_presence
        .into_body()
        .collect()
        .await
        .expect("agent presence body should collect")
        .to_bytes();
    let agent_presence_json: serde_json::Value =
        serde_json::from_slice(&agent_presence_body).expect("agent presence should be valid json");
    let agent_devices = agent_presence_json["devices"]
        .as_array()
        .expect("agent devices should be an array");
    assert_eq!(
        agent_devices.len(),
        1,
        "agent presence should not include devices from another actor kind sharing the same actor id"
    );
    assert_eq!(agent_devices[0]["deviceId"], "d_agent");
}

#[tokio::test]
async fn test_session_resume_rejects_same_device_id_with_different_actor_kind_over_http() {
    let app = session_gateway::build_app();

    let user_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_user")
                .header("x-device-id", "d_shared")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("user resume should succeed");
    assert_eq!(user_resume.status(), StatusCode::OK);

    let agent_resume = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_dual")
                .header("x-actor-kind", "agent")
                .header("x-session-id", "s_agent")
                .header("x-device-id", "d_shared")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("agent resume should return response");

    assert_eq!(agent_resume.status(), StatusCode::CONFLICT);
    let agent_resume_body = agent_resume
        .into_body()
        .collect()
        .await
        .expect("agent resume body should collect")
        .to_bytes();
    let agent_resume_json: serde_json::Value =
        serde_json::from_slice(&agent_resume_body).expect("agent resume should be valid json");
    assert_eq!(agent_resume_json["code"], "device_scope_conflict");
}

#[tokio::test]
async fn test_session_resume_rejects_same_device_id_with_different_principal_over_http() {
    let app = session_gateway::build_app();

    let first_resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner_a")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_owner_a")
                .header("x-device-id", "d_shared_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("first owner resume should succeed");
    assert_eq!(first_resume.status(), StatusCode::OK);

    let conflicting_resume = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner_b")
                .header("x-actor-kind", "user")
                .header("x-session-id", "s_owner_b")
                .header("x-device-id", "d_shared_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("conflicting owner resume should return response");

    assert_eq!(conflicting_resume.status(), StatusCode::CONFLICT);
    let body = conflicting_resume
        .into_body()
        .collect()
        .await
        .expect("conflicting owner resume body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("conflicting owner resume should be valid json");
    assert_eq!(json["code"], "device_scope_conflict");
}

#[tokio::test]
async fn test_presence_heartbeat_and_disconnect_drive_device_offline_transition() {
    let app = session_gateway::build_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("heartbeat request should succeed");
    assert_eq!(heartbeat.status(), StatusCode::OK);

    let disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect request should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);
    let disconnect_body = disconnect
        .into_body()
        .collect()
        .await
        .expect("disconnect body should collect")
        .to_bytes();
    let disconnect_value: serde_json::Value =
        serde_json::from_slice(&disconnect_body).expect("disconnect should be valid json");
    assert_eq!(disconnect_value["devices"][0]["status"], "offline");

    let snapshot = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/presence/me")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("presence request should succeed");
    assert_eq!(snapshot.status(), StatusCode::OK);
    let snapshot_body = snapshot
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_value: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot should be valid json");
    assert_eq!(snapshot_value["devices"][0]["status"], "offline");
}

#[tokio::test]
async fn test_session_gateway_requires_fresh_resume_after_disconnect() {
    let app = session_gateway::build_app();

    let resume_old = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume_old.status(), StatusCode::OK);

    let disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect request should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let stale_heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat should return response");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value = serde_json::from_slice(&stale_heartbeat_body)
        .expect("stale heartbeat should be valid json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let resume_new = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume request should succeed");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let fresh_heartbeat = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("fresh heartbeat should succeed");
    assert_eq!(fresh_heartbeat.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_session_gateway_treats_duplicate_disconnect_as_idempotent_for_same_session() {
    let app = session_gateway::build_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let first_disconnect = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("first disconnect should succeed");
    assert_eq!(first_disconnect.status(), StatusCode::OK);

    let duplicate_disconnect = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("duplicate disconnect should return response");
    assert_eq!(duplicate_disconnect.status(), StatusCode::OK);
    let duplicate_disconnect_body = duplicate_disconnect
        .into_body()
        .collect()
        .await
        .expect("duplicate disconnect body should collect")
        .to_bytes();
    let duplicate_disconnect_json: serde_json::Value =
        serde_json::from_slice(&duplicate_disconnect_body)
            .expect("duplicate disconnect should be valid json");
    assert_eq!(duplicate_disconnect_json["devices"][0]["status"], "offline");
}

#[tokio::test]
async fn test_session_gateway_rebuild_preserves_reconnect_required_fence_until_fresh_resume() {
    let shared_store = Arc::new(MemoryRealtimeDisconnectFenceStore::default());
    let app_before = session_gateway::build_app_with_cluster(Arc::new(
        session_gateway::RealtimeClusterBridge::with_disconnect_fence_store(shared_store.clone()),
    ));

    let resume_old = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
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
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect should succeed before restart");
    assert_eq!(disconnect.status(), StatusCode::OK);

    let app_after = session_gateway::build_app_with_cluster(Arc::new(
        session_gateway::RealtimeClusterBridge::with_disconnect_fence_store(shared_store),
    ));

    let stale_heartbeat = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_old")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("stale heartbeat should return response after restart");
    assert_eq!(stale_heartbeat.status(), StatusCode::CONFLICT);
    let stale_heartbeat_body = stale_heartbeat
        .into_body()
        .collect()
        .await
        .expect("stale heartbeat body should collect")
        .to_bytes();
    let stale_heartbeat_json: serde_json::Value = serde_json::from_slice(&stale_heartbeat_body)
        .expect("stale heartbeat should be valid json");
    assert_eq!(stale_heartbeat_json["code"], "reconnect_required");

    let resume_new = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should clear restored fence");
    assert_eq!(resume_new.status(), StatusCode::OK);

    let fresh_heartbeat = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("fresh heartbeat should succeed after restored fence clears");
    assert_eq!(fresh_heartbeat.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_session_gateway_rejects_sessionless_device_rebind_after_session_resume() {
    let app = session_gateway::build_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume.status(), StatusCode::OK);

    let heartbeat = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("sessionless heartbeat should return response");
    assert_eq!(heartbeat.status(), StatusCode::CONFLICT);
    let heartbeat_body = heartbeat
        .into_body()
        .collect()
        .await
        .expect("heartbeat body should collect")
        .to_bytes();
    let heartbeat_json: serde_json::Value =
        serde_json::from_slice(&heartbeat_body).expect("heartbeat should be valid json");
    assert_eq!(heartbeat_json["code"], "session_id_required");
}

#[tokio::test]
async fn test_presence_runtime_timestamps_advance_between_resume_heartbeat_and_disconnect() {
    let app = session_gateway::build_app();

    let resume = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("resume request should succeed");
    assert_eq!(resume.status(), StatusCode::OK);
    let resume_body = resume
        .into_body()
        .collect()
        .await
        .expect("resume body should collect")
        .to_bytes();
    let resume_json: serde_json::Value =
        serde_json::from_slice(&resume_body).expect("resume should be valid json");
    let resumed_at = resume_json["resumedAt"]
        .as_str()
        .expect("resumedAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let heartbeat = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/presence/heartbeat")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("heartbeat request should succeed");
    assert_eq!(heartbeat.status(), StatusCode::OK);
    let heartbeat_body = heartbeat
        .into_body()
        .collect()
        .await
        .expect("heartbeat body should collect")
        .to_bytes();
    let heartbeat_json: serde_json::Value =
        serde_json::from_slice(&heartbeat_body).expect("heartbeat should be valid json");
    let heartbeat_seen_at = heartbeat_json["devices"][0]["lastSeenAt"]
        .as_str()
        .expect("lastSeenAt should be present after heartbeat")
        .to_owned();

    sleep(Duration::from_millis(20));

    let disconnect = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect request should succeed");
    assert_eq!(disconnect.status(), StatusCode::OK);
    let disconnect_body = disconnect
        .into_body()
        .collect()
        .await
        .expect("disconnect body should collect")
        .to_bytes();
    let disconnect_json: serde_json::Value =
        serde_json::from_slice(&disconnect_body).expect("disconnect should be valid json");
    let disconnect_seen_at = disconnect_json["devices"][0]["lastSeenAt"]
        .as_str()
        .expect("lastSeenAt should be present after disconnect")
        .to_owned();

    assert!(resumed_at < heartbeat_seen_at);
    assert!(heartbeat_seen_at < disconnect_seen_at);
}

#[tokio::test]
async fn test_realtime_subscription_sync_and_empty_event_window_over_http() {
    let app = session_gateway::build_app();

    let sync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync request should succeed");
    assert_eq!(sync_response.status(), StatusCode::OK);
    let sync_body = sync_response
        .into_body()
        .collect()
        .await
        .expect("subscription sync body should collect")
        .to_bytes();
    let sync_json: serde_json::Value =
        serde_json::from_slice(&sync_body).expect("subscription sync should be valid json");
    assert_eq!(sync_json["deviceId"], "d_pad");
    assert_eq!(sync_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(sync_json["items"][0]["scopeType"], "conversation");
    assert_eq!(sync_json["items"][0]["scopeId"], "c_demo");

    let event_window = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime event window request should succeed");
    assert_eq!(event_window.status(), StatusCode::OK);
    let event_window_body = event_window
        .into_body()
        .collect()
        .await
        .expect("realtime event window body should collect")
        .to_bytes();
    let event_window_json: serde_json::Value =
        serde_json::from_slice(&event_window_body).expect("event window should be valid json");
    assert_eq!(event_window_json["deviceId"], "d_pad");
    assert_eq!(event_window_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(event_window_json["ackedThroughSeq"], 0);
    assert_eq!(event_window_json["trimmedThroughSeq"], 0);
    assert_eq!(event_window_json["hasMore"], false);
}

#[tokio::test]
async fn test_realtime_ack_endpoint_accepts_empty_window_over_http() {
    let app = session_gateway::build_app();

    let sync_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_demo",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("subscription sync request should succeed");
    assert_eq!(sync_response.status(), StatusCode::OK);

    let ack_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/events/ack")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"ackedSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("ack request should succeed");
    assert_eq!(ack_response.status(), StatusCode::OK);
    let ack_body = ack_response
        .into_body()
        .collect()
        .await
        .expect("ack body should collect")
        .to_bytes();
    let ack_json: serde_json::Value =
        serde_json::from_slice(&ack_body).expect("ack response should be valid json");
    assert_eq!(ack_json["deviceId"], "d_pad");
    assert_eq!(ack_json["ackedThroughSeq"], 0);
    assert_eq!(ack_json["trimmedThroughSeq"], 0);
    assert_eq!(ack_json["retainedEventCount"], 0);
}

#[tokio::test]
async fn test_session_resume_rejects_oversized_device_id_over_http() {
    let app = session_gateway::build_app();
    let oversized_device_id = "d".repeat(1024);
    let request_body = serde_json::json!({
        "deviceId": oversized_device_id,
        "lastSeenSyncSeq": 0
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("resume request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("deviceId")
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_oversized_scope_id_over_http() {
    let app = session_gateway::build_app();
    let oversized_scope_id = "c".repeat(2048);
    let request_body = serde_json::json!({
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": oversized_scope_id,
                "eventTypes": ["message.posted"]
            }
        ]
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("scopeId")
    );
}

#[tokio::test]
async fn test_realtime_event_window_rejects_limit_above_guardrail_over_http() {
    let app = session_gateway::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/realtime/events?afterSeq=0&limit=5000")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("event window request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "limit_invalid");
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_oversized_event_types_payload_over_http() {
    let app = session_gateway::build_app();
    let oversized_event_types = (0..300)
        .map(|index| format!("evt_{index:03}_{}", "x".repeat(64)))
        .collect::<Vec<_>>();
    let request_body = serde_json::json!({
        "items": [
            {
                "scopeType": "conversation",
                "scopeId": "c_demo",
                "eventTypes": oversized_event_types
            }
        ]
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("eventTypes")
    );
}

#[tokio::test]
async fn test_realtime_subscription_sync_rejects_too_many_items_over_http() {
    let app = session_gateway::build_app();
    let oversized_items = (0..300)
        .map(|index| {
            serde_json::json!({
                "scopeType": "conversation",
                "scopeId": format!("c_{index:03}"),
                "eventTypes": []
            })
        })
        .collect::<Vec<_>>();
    let request_body = serde_json::json!({
        "items": oversized_items
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/realtime/subscriptions/sync")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("subscription sync should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("items")
    );
}
