use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use audit_service::AuditRuntime;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_disk::FileCommitJournal;
use im_auth_context::AuthContext;
use im_domain_core::social::direct_chat_pair_hash;
use im_domain_events::social::{DirectChatBoundPayload, SocialEventType, social_commit_envelope};
use im_domain_events::{AggregateType, EventActor};
use im_platform_contracts::CommitJournal;
use ops_service::OpsRuntime;
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

static NEXT_RUNTIME_DIR_ID: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let sequence = NEXT_RUNTIME_DIR_ID.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_control_plane_social_runtime_{unique}_{sequence}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

fn social_failpoint_file(runtime_dir: &Path) -> PathBuf {
    state_file(runtime_dir, "social-failpoints.json")
}

fn social_tx_marker_file(runtime_dir: &Path) -> PathBuf {
    state_file(runtime_dir, "social-transaction-marker.json")
}

#[tokio::test]
async fn test_control_plane_social_friend_request_write_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let submit_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_001",
                        "eventId":"evt_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestMessage":"hello",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request submit should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    let submit_body = submit_response
        .into_body()
        .collect()
        .await
        .expect("submit body should collect")
        .to_bytes();
    let submit_json: serde_json::Value =
        serde_json::from_slice(&submit_body).expect("submit body should be valid json");

    assert_eq!(submit_json["status"], "submitted");
    assert_eq!(submit_json["friendRequest"]["tenantId"], "t_demo");
    assert_eq!(submit_json["friendRequest"]["requestId"], "fr_001");
    assert_eq!(submit_json["friendRequest"]["requesterUserId"], "u_alice");
    assert_eq!(submit_json["friendRequest"]["targetUserId"], "u_bob");
    assert_eq!(submit_json["friendRequest"]["status"], "pending");
    assert_eq!(
        submit_json["latestCommit"]["eventType"],
        "friend_request.submitted"
    );
    assert_eq!(submit_json["latestCommit"]["scopeType"], "friend_request");
    assert_eq!(
        submit_json["latestCommit"]["payloadSchema"],
        "social.friend_request.submitted.v1"
    );
    assert_eq!(submit_json["latestCommit"]["orderingSeq"], 1);

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["friendRequest"]["requestId"], "fr_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "friend_request.submitted"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(
        audit_export.items[0].action,
        "control.friend_request_submitted"
    );
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("friend request audit should include payload")
            .contains("\"requestId\":\"fr_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_identical_user_pair() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_invalid",
                        "eventId":"evt_invalid",
                        "requesterUserId":"u_same",
                        "targetUserId":"u_same",
                        "requestedAt":"2026-04-10T10:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invalid friend request should return response");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("invalid body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("invalid body should be valid json");
    assert_eq!(json["status"], "invalid");
    assert_eq!(json["code"], "invalid_friend_request");
}

#[tokio::test]
async fn test_control_plane_social_friendship_activation_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let activate_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friendships")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_001",
                        "eventId":"evt_fs_001",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "directChatId":"dc_001",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friendship activation should return response");
    assert_eq!(activate_response.status(), StatusCode::OK);

    let activate_body = activate_response
        .into_body()
        .collect()
        .await
        .expect("activation body should collect")
        .to_bytes();
    let activate_json: serde_json::Value =
        serde_json::from_slice(&activate_body).expect("activation body should be valid json");

    assert_eq!(activate_json["status"], "activated");
    assert_eq!(activate_json["friendship"]["tenantId"], "t_demo");
    assert_eq!(activate_json["friendship"]["friendshipId"], "fs_001");
    assert_eq!(activate_json["friendship"]["userLowId"], "u_alice");
    assert_eq!(activate_json["friendship"]["userHighId"], "u_bob");
    assert_eq!(activate_json["friendship"]["initiatorUserId"], "u_alice");
    assert_eq!(activate_json["friendship"]["status"], "active");
    assert_eq!(
        activate_json["latestCommit"]["eventType"],
        "friendship.activated"
    );
    assert_eq!(activate_json["latestCommit"]["scopeType"], "friendship");
    assert_eq!(
        activate_json["latestCommit"]["payloadSchema"],
        "social.friendship.activated.v1"
    );
    assert_eq!(activate_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        activate_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"directChatId\":\"dc_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friendships/fs_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friendship snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["friendship"]["friendshipId"], "fs_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "friendship.activated"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(audit_export.items[0].action, "control.friendship_activated");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("friendship audit should include payload")
            .contains("\"friendshipId\":\"fs_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_friendship_rejects_duplicate_active_pair() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friendships")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_001",
                        "eventId":"evt_fs_001",
                        "initiatorUserId":"u_alice",
                        "peerUserId":"u_bob",
                        "establishedAt":"2026-04-10T11:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first friendship activation should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friendships")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "friendshipId":"fs_002",
                        "eventId":"evt_fs_002",
                        "initiatorUserId":"u_bob",
                        "peerUserId":"u_alice",
                        "establishedAt":"2026-04-10T11:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate friendship activation should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate body should be valid json");
    assert_eq!(duplicate_json["status"], "conflict");
    assert_eq!(duplicate_json["code"], "friendship_pair_conflict");
}

#[tokio::test]
async fn test_control_plane_social_direct_chat_binding_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let bind_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_001",
                        "eventId":"evt_dc_001",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_direct_001",
                        "boundAt":"2026-04-10T12:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind should return response");
    assert_eq!(bind_response.status(), StatusCode::OK);

    let bind_body = bind_response
        .into_body()
        .collect()
        .await
        .expect("bind body should collect")
        .to_bytes();
    let bind_json: serde_json::Value =
        serde_json::from_slice(&bind_body).expect("bind body should be valid json");

    assert_eq!(bind_json["status"], "bound");
    assert_eq!(bind_json["directChat"]["tenantId"], "t_demo");
    assert_eq!(bind_json["directChat"]["directChatId"], "dc_001");
    assert_eq!(bind_json["directChat"]["leftActorId"], "actor_alice");
    assert_eq!(bind_json["directChat"]["rightActorId"], "actor_bob");
    assert_eq!(bind_json["directChat"]["pairHash"], "actor_alice:actor_bob");
    assert_eq!(bind_json["directChat"]["status"], "active");
    assert_eq!(bind_json["directChat"]["conversationId"], "c_direct_001");
    assert_eq!(bind_json["latestCommit"]["eventType"], "direct_chat.bound");
    assert_eq!(bind_json["latestCommit"]["scopeType"], "direct_chat");
    assert_eq!(
        bind_json["latestCommit"]["payloadSchema"],
        "social.direct_chat.bound.v1"
    );
    assert_eq!(bind_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        bind_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"conversationId\":\"c_direct_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/direct-chats/dc_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["directChat"]["directChatId"], "dc_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "direct_chat.bound"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(audit_export.items[0].action, "control.direct_chat_bound");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("direct chat audit should include payload")
            .contains("\"directChatId\":\"dc_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_direct_chat_rejects_duplicate_active_pair() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_001",
                        "eventId":"evt_dc_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_direct_001",
                        "boundAt":"2026-04-10T12:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_002",
                        "eventId":"evt_dc_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_direct_002",
                        "boundAt":"2026-04-10T12:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate body should be valid json");
    assert_eq!(duplicate_json["status"], "conflict");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");
}

#[tokio::test]
async fn test_control_plane_social_user_block_persists_snapshot_commit_and_audit() {
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks(
        cluster,
        ops_runtime,
        audit_runtime.clone(),
    );

    let block_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/user-blocks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_001",
                        "eventId":"evt_ub_001",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"direct_chat",
                        "directChatId":"dc_001",
                        "effectiveAt":"2026-04-10T12:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user block should return response");
    assert_eq!(block_response.status(), StatusCode::OK);

    let block_body = block_response
        .into_body()
        .collect()
        .await
        .expect("user block body should collect")
        .to_bytes();
    let block_json: serde_json::Value =
        serde_json::from_slice(&block_body).expect("user block body should be valid json");

    assert_eq!(block_json["status"], "blocked");
    assert_eq!(block_json["userBlock"]["tenantId"], "t_demo");
    assert_eq!(block_json["userBlock"]["blockId"], "ub_001");
    assert_eq!(block_json["userBlock"]["blockerUserId"], "u_alice");
    assert_eq!(block_json["userBlock"]["blockedUserId"], "u_bob");
    assert_eq!(block_json["userBlock"]["scope"], "direct_chat");
    assert_eq!(block_json["userBlock"]["directChatId"], "dc_001");
    assert_eq!(block_json["userBlock"]["status"], "active");
    assert_eq!(
        block_json["latestCommit"]["eventType"],
        "user_block.blocked"
    );
    assert_eq!(block_json["latestCommit"]["scopeType"], "user_block");
    assert_eq!(
        block_json["latestCommit"]["payloadSchema"],
        "social.user_block.blocked.v1"
    );
    assert_eq!(block_json["latestCommit"]["orderingSeq"], 1);
    assert!(
        block_json["latestCommit"]["payload"]
            .as_str()
            .expect("payload should be string")
            .contains("\"directChatId\":\"dc_001\"")
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/user-blocks/ub_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user block snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("user block snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("user block snapshot should be valid json");

    assert_eq!(snapshot_json["status"], "snapshot");
    assert_eq!(snapshot_json["userBlock"]["blockId"], "ub_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);
    assert_eq!(
        snapshot_json["commits"][0]["eventType"],
        "user_block.blocked"
    );

    let audit_auth = AuthContext {
        tenant_id: "t_demo".into(),
        actor_id: "u_admin".into(),
        actor_kind: "admin".into(),
        session_id: None,
        device_id: None,
        permissions: BTreeSet::new(),
    };
    let audit_export = audit_runtime.export_bundle(&audit_auth);
    assert_eq!(audit_export.total, 1);
    assert_eq!(audit_export.items[0].action, "control.user_block_blocked");
    assert!(
        audit_export.items[0]
            .payload
            .as_deref()
            .expect("user block audit should include payload")
            .contains("\"blockId\":\"ub_001\"")
    );
}

#[tokio::test]
async fn test_control_plane_social_user_block_rejects_duplicate_active_scope() {
    let app = control_plane_api::build_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/user-blocks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_001",
                        "eventId":"evt_ub_001",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"all",
                        "effectiveAt":"2026-04-10T12:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first user block should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/user-blocks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "blockId":"ub_002",
                        "eventId":"evt_ub_002",
                        "blockerUserId":"u_alice",
                        "blockedUserId":"u_bob",
                        "scope":"all",
                        "effectiveAt":"2026-04-10T12:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate user block should return response");

    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate user block body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate user block body should be valid json");
    assert_eq!(duplicate_json["status"], "conflict");
    assert_eq!(duplicate_json["code"], "user_block_scope_conflict");
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_restores_friend_request_snapshot_and_outbox() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let audit_runtime = Arc::new(AuditRuntime::default());
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        audit_runtime,
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_persist_001",
                        "eventId":"evt_persist_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T13:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("durable friend request should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    assert!(
        state_file(runtime_dir.as_path(), "social-state.json").exists(),
        "durable social state file should be materialized"
    );
    assert!(
        state_file(runtime_dir.as_path(), "social-commit-journal.json").exists(),
        "durable social commit journal should be materialized"
    );

    let journal_body = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("social journal should be readable");
    let journal_json: serde_json::Value =
        serde_json::from_str(&journal_body).expect("social journal should be valid json");
    let journal_items = journal_json
        .as_array()
        .expect("social journal should serialize as an array");
    assert_eq!(journal_items.len(), 1);
    assert_eq!(journal_items[0]["event_type"], "friend_request.submitted");
    assert_eq!(journal_items[0]["scope_type"], "friend_request");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_persist_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after rebuild should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);
    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot body after rebuild should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value =
        serde_json::from_slice(&snapshot_body).expect("snapshot body after rebuild should be json");
    assert_eq!(
        snapshot_json["friendRequest"]["requestId"],
        "fr_persist_001"
    );
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_restores_direct_chat_pair_uniqueness_after_rebuild()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let first_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_persist_001",
                        "eventId":"evt_dc_persist_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_persist_001",
                        "boundAt":"2026-04-10T13:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial durable direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let duplicate_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_persist_002",
                        "eventId":"evt_dc_persist_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_persist_002",
                        "boundAt":"2026-04-10T13:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate durable direct chat bind should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate response body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate response should be valid json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_friend_request_when_snapshot_is_missing() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_replay_001",
                        "eventId":"evt_replay_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:00:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request write should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    fs::remove_file(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be removed");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_replay_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after journal replay should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request replay snapshot body should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request replay snapshot should be json");
    assert_eq!(snapshot_json["friendRequest"]["requestId"], "fr_replay_001");
    assert_eq!(snapshot_json["commits"].as_array().unwrap().len(), 1);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_falls_back_to_snapshot_when_journal_replay_fails() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let submit_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_corrupt_journal_001",
                        "eventId":"evt_corrupt_journal_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:05:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("friend request write before journal corruption should return response");
    assert_eq!(submit_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        "{invalid-journal",
    )
    .expect("corrupted social journal should be writable");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_corrupt_journal_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("friend request snapshot after journal corruption should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("friend request snapshot after journal corruption should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("friend request snapshot after journal corruption should be json");
    assert_eq!(
        snapshot_json["friendRequest"]["requestId"],
        "fr_corrupt_journal_001"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_starts_with_default_when_snapshot_is_invalid_and_journal_missing()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(runtime_dir.join("state")).expect("runtime state dir should be created");
    fs::write(
        state_file(runtime_dir.as_path(), "social-state.json"),
        "{invalid-snapshot",
    )
    .expect("invalid social snapshot fixture should be writable");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let snapshot_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_missing_invalid_snapshot")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot read on invalid startup snapshot should return response");
    assert_eq!(snapshot_response.status(), StatusCode::NOT_FOUND);

    let snapshot_body = snapshot_response
        .into_body()
        .collect()
        .await
        .expect("snapshot read on invalid startup snapshot should collect")
        .to_bytes();
    let snapshot_json: serde_json::Value = serde_json::from_slice(&snapshot_body)
        .expect("snapshot read on invalid startup snapshot should be json");
    assert_eq!(snapshot_json["code"], "friend_request_not_found");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_direct_chat_pair_guard_when_snapshot_is_missing()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let first_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replay_001",
                        "eventId":"evt_dc_replay_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_replay_001",
                        "boundAt":"2026-04-10T14:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("initial direct chat bind should return response");
    assert_eq!(first_response.status(), StatusCode::OK);

    fs::remove_file(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be removed");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let duplicate_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replay_002",
                        "eventId":"evt_dc_replay_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_replay_002",
                        "boundAt":"2026-04-10T14:11:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind after journal replay should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);
    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate direct chat replay response should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value = serde_json::from_slice(&duplicate_body)
        .expect("duplicate direct chat replay response should be json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_discards_friend_request_snapshot_ahead_of_journal()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let original_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_stable_001",
                        "eventId":"evt_stable_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_bob",
                        "requestedAt":"2026-04-10T14:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("original friend request write should return response");
    assert_eq!(original_response.status(), StatusCode::OK);

    let original_journal = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("original journal should be readable");

    let phantom_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_phantom_001",
                        "eventId":"evt_phantom_001",
                        "requesterUserId":"u_alice",
                        "targetUserId":"u_cindy",
                        "requestedAt":"2026-04-10T14:21:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("phantom friend request write should return response");
    assert_eq!(phantom_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        original_journal,
    )
    .expect("journal should be rolled back to original version");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let phantom_snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/friend-requests/fr_phantom_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("phantom snapshot lookup should return response");
    assert_eq!(phantom_snapshot_response.status(), StatusCode::NOT_FOUND);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_discards_direct_chat_snapshot_ahead_of_journal() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_before = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let original_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_stable_001",
                        "eventId":"evt_dc_stable_001",
                        "leftActorId":"actor_carla",
                        "rightActorId":"actor_david",
                        "conversationId":"c_stable_001",
                        "boundAt":"2026-04-10T14:30:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("original direct chat bind should return response");
    assert_eq!(original_response.status(), StatusCode::OK);

    let original_journal = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("original direct chat journal should be readable");

    let phantom_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_phantom_001",
                        "eventId":"evt_dc_phantom_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_phantom_001",
                        "boundAt":"2026-04-10T14:31:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("phantom direct chat bind should return response");
    assert_eq!(phantom_response.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "social-commit-journal.json"),
        original_journal,
    )
    .expect("direct chat journal should be rolled back to original version");

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let replacement_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_replacement_001",
                        "eventId":"evt_dc_replacement_001",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_replacement_001",
                        "boundAt":"2026-04-10T14:32:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("replacement direct chat bind should return response");
    assert_eq!(replacement_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_keeps_direct_chat_pair_guard_after_snapshot_save_failure()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let seed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_seed_001",
                        "eventId":"evt_seed_001",
                        "requesterUserId":"u_seed_alice",
                        "targetUserId":"u_seed_bob",
                        "requestedAt":"2026-04-10T14:40:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed friend request should return response");
    assert_eq!(seed_response.status(), StatusCode::OK);

    let snapshot_path = state_file(runtime_dir.as_path(), "social-state.json");
    let mut readonly_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should exist")
        .permissions();
    readonly_permissions.set_readonly(true);
    fs::set_permissions(&snapshot_path, readonly_permissions)
        .expect("snapshot should become readonly");

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_fail_save_001",
                        "eventId":"evt_dc_fail_save_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_fail_save_001",
                        "boundAt":"2026-04-10T14:41:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with readonly snapshot should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["directChat"]["directChatId"],
        "dc_fail_save_001"
    );
    assert_eq!(
        committed_json["latestCommit"]["eventId"],
        "evt_dc_fail_save_001"
    );
    assert_eq!(committed_json["persistence"]["journalAuthority"], true);
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let mut writable_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should still exist")
        .permissions();
    writable_permissions.set_readonly(false);
    fs::set_permissions(&snapshot_path, writable_permissions)
        .expect("snapshot should become writable again");

    let duplicate_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_fail_save_002",
                        "eventId":"evt_dc_fail_save_002",
                        "leftActorId":"actor_bob",
                        "rightActorId":"actor_alice",
                        "conversationId":"c_fail_save_002",
                        "boundAt":"2026-04-10T14:42:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate direct chat bind after snapshot save failure should return response");
    assert_eq!(duplicate_response.status(), StatusCode::CONFLICT);

    let duplicate_body = duplicate_response
        .into_body()
        .collect()
        .await
        .expect("duplicate response body should collect")
        .to_bytes();
    let duplicate_json: serde_json::Value =
        serde_json::from_slice(&duplicate_body).expect("duplicate response should be valid json");
    assert_eq!(duplicate_json["code"], "direct_chat_pair_conflict");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_replays_same_event_id_after_snapshot_save_failure()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let seed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/friend-requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "requestId":"fr_seed_retry_001",
                        "eventId":"evt_seed_retry_001",
                        "requesterUserId":"u_seed_alice",
                        "targetUserId":"u_seed_bob",
                        "requestedAt":"2026-04-10T14:50:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed friend request should return response");
    assert_eq!(seed_response.status(), StatusCode::OK);

    let snapshot_path = state_file(runtime_dir.as_path(), "social-state.json");
    let mut readonly_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should exist")
        .permissions();
    readonly_permissions.set_readonly(true);
    fs::set_permissions(&snapshot_path, readonly_permissions)
        .expect("snapshot should become readonly");

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_retry_same_event_001",
                        "eventId":"evt_dc_retry_same_event_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_retry_same_event_001",
                        "boundAt":"2026-04-10T14:51:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with readonly snapshot should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let mut writable_permissions = fs::metadata(&snapshot_path)
        .expect("snapshot should still exist")
        .permissions();
    writable_permissions.set_readonly(false);
    fs::set_permissions(&snapshot_path, writable_permissions)
        .expect("snapshot should become writable again");

    let retry_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_retry_same_event_001",
                        "eventId":"evt_dc_retry_same_event_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_retry_same_event_001",
                        "boundAt":"2026-04-10T14:51:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("same event retry should return response");
    assert_eq!(retry_response.status(), StatusCode::OK);

    let retry_body = retry_response
        .into_body()
        .collect()
        .await
        .expect("retry response body should collect")
        .to_bytes();
    let retry_json: serde_json::Value =
        serde_json::from_slice(&retry_body).expect("retry response should be valid json");
    assert_eq!(retry_json["status"], "bound");
    assert_eq!(
        retry_json["directChat"]["directChatId"],
        "dc_retry_same_event_001"
    );
    assert_eq!(
        retry_json["latestCommit"]["eventId"],
        "evt_dc_retry_same_event_001"
    );
    assert_eq!(retry_json["persistence"]["journalAuthority"], true);
    assert_eq!(retry_json["persistence"]["snapshotStatus"], "current");

    let journal_body = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "social-commit-journal.json",
    ))
    .expect("social journal should be readable");
    let journal_json: serde_json::Value =
        serde_json::from_str(&journal_body).expect("social journal should be valid json");
    let journal_items = journal_json
        .as_array()
        .expect("social journal should serialize as an array");
    assert_eq!(journal_items.len(), 2);
    assert_eq!(
        journal_items
            .iter()
            .filter(|item| item["event_id"] == "evt_dc_retry_same_event_001")
            .count(),
        1
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_failpoint_forces_next_snapshot_save_failure_once() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_failpoint_once_001",
                        "eventId":"evt_failpoint_once_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_failpoint_once_001",
                        "boundAt":"2026-04-10T15:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["directChat"]["directChatId"],
        "dc_failpoint_once_001"
    );
    assert_eq!(
        committed_json["latestCommit"]["eventId"],
        "evt_failpoint_once_001"
    );
    assert_eq!(committed_json["persistence"]["journalAuthority"], true);
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let failpoint_file = social_failpoint_file(runtime_dir.as_path());
    if failpoint_file.exists() {
        let failpoint_json: serde_json::Value = serde_json::from_str(
            &fs::read_to_string(&failpoint_file).expect("failpoint file should be readable"),
        )
        .expect("failpoint file should be valid json");
        assert_eq!(failpoint_json["failNextSnapshotSave"], false);
    }

    let retry_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_failpoint_once_001",
                        "eventId":"evt_failpoint_once_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_failpoint_once_001",
                        "boundAt":"2026-04-10T15:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("same-event retry after failpoint should return response");
    assert_eq!(retry_response.status(), StatusCode::OK);

    let retry_body = retry_response
        .into_body()
        .collect()
        .await
        .expect("retry response body should collect")
        .to_bytes();
    let retry_json: serde_json::Value =
        serde_json::from_slice(&retry_body).expect("retry response should be valid json");
    assert_eq!(retry_json["status"], "bound");
    assert_eq!(
        retry_json["directChat"]["directChatId"],
        "dc_failpoint_once_001"
    );
    assert_eq!(retry_json["persistence"]["journalAuthority"], true);
    assert_eq!(retry_json["persistence"]["snapshotStatus"], "current");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_operator_repair_rebuilds_snapshot_after_failpoint()
{
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster.clone(),
        ops_runtime.clone(),
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_operator_repair_001",
                        "eventId":"evt_operator_repair_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_operator_repair_001",
                        "boundAt":"2026-04-10T15:20:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-derived-snapshot")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social operator repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);

    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("repair response body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("repair response should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["journalAuthority"], true);
    assert_eq!(repair_json["snapshotUpdated"], true);
    assert_eq!(repair_json["transactionMarkerCleared"], true);
    assert_eq!(repair_json["aggregateCounts"]["directChats"], 1);

    let snapshot_body = fs::read_to_string(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be readable after repair");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social state snapshot should be valid json");
    assert_eq!(
        snapshot_json["direct_chats"]["dc_operator_repair_001"]["direct_chat"]["conversationId"],
        "c_operator_repair_001"
    );

    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );
    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/direct-chats/dc_operator_repair_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot after repair should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_leaves_pending_tx_marker_after_snapshot_failure_and_clears_it_after_restart_replay()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(state_file(runtime_dir.as_path(), "").as_path())
        .expect("state dir should be created");
    fs::write(
        social_failpoint_file(runtime_dir.as_path()),
        r#"{"failNextSnapshotSave":true}"#,
    )
    .expect("social failpoint file should be written");

    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let committed_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/direct-chats/bindings")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "directChatId":"dc_tx_marker_001",
                        "eventId":"evt_tx_marker_001",
                        "leftActorId":"actor_alice",
                        "rightActorId":"actor_bob",
                        "conversationId":"c_tx_marker_001",
                        "boundAt":"2026-04-11T09:10:00Z"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("direct chat bind with failpoint should return response");
    assert_eq!(committed_response.status(), StatusCode::OK);

    let committed_body = committed_response
        .into_body()
        .collect()
        .await
        .expect("committed response body should collect")
        .to_bytes();
    let committed_json: serde_json::Value =
        serde_json::from_slice(&committed_body).expect("committed response should be valid json");
    assert_eq!(committed_json["status"], "bound");
    assert_eq!(
        committed_json["persistence"]["snapshotStatus"],
        "repair_required"
    );

    let marker_path = social_tx_marker_file(runtime_dir.as_path());
    assert!(
        marker_path.exists(),
        "pending social tx marker should exist after snapshot save failure"
    );
    let marker_body =
        fs::read_to_string(&marker_path).expect("pending social tx marker should be readable");
    let marker_json: serde_json::Value =
        serde_json::from_str(&marker_body).expect("pending social tx marker should be valid json");
    assert_eq!(marker_json["status"], "pending_snapshot_repair");
    assert_eq!(marker_json["eventId"], "evt_tx_marker_001");

    let cluster_after = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime_after = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app_after = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster_after,
        ops_runtime_after,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    assert!(
        !marker_path.exists(),
        "pending social tx marker should be cleared after restart replay repairs snapshot"
    );

    let snapshot_response = app_after
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/direct-chats/dc_tx_marker_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("snapshot after restart replay should return response");
    assert_eq!(snapshot_response.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_control_plane_social_file_runtime_operator_repair_replays_external_journal_append_into_live_state()
 {
    let runtime_dir = unique_runtime_dir();
    let cluster = Arc::new(RealtimeClusterBridge::default());
    let ops_runtime = Arc::new(OpsRuntime::new(
        "node_a",
        "local-minimal",
        "127.0.0.1:18090",
        vec!["session-gateway".into(), "control-plane-api".into()],
        vec![],
    ));
    let app = control_plane_api::build_app_with_cluster_and_governance_sinks_and_runtime_dir(
        cluster,
        ops_runtime,
        Arc::new(AuditRuntime::default()),
        runtime_dir.as_path(),
    );

    let journal_path = state_file(runtime_dir.as_path(), "social-commit-journal.json");
    let journal = FileCommitJournal::new("social", journal_path);
    let payload = DirectChatBoundPayload {
        direct_chat_id: "dc_operator_repair_journal_001".into(),
        conversation_id: "c_operator_repair_journal_001".into(),
        left_actor_id: "actor_alice".into(),
        right_actor_id: "actor_carol".into(),
        pair_hash: direct_chat_pair_hash("actor_alice", "actor_carol")
            .expect("direct chat pair hash should normalize"),
        bound_at: "2026-04-11T01:00:00Z".into(),
    };
    let payload_json =
        serde_json::to_string(&payload).expect("external journal payload should serialize");
    journal
        .append(social_commit_envelope(
            "evt_operator_repair_journal_001",
            "t_demo",
            AggregateType::DirectChat,
            payload.direct_chat_id.as_str(),
            SocialEventType::DirectChatBound,
            1,
            EventActor {
                actor_id: "operator_repair".into(),
                actor_kind: "operator".into(),
                actor_session_id: None,
            },
            payload.bound_at.as_str(),
            payload.bound_at.as_str(),
            payload_json.as_str(),
        ))
        .expect("external journal append should succeed");

    let repair_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/control/social/runtime/repair-derived-snapshot")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.write")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social operator repair should return response");
    assert_eq!(repair_response.status(), StatusCode::OK);

    let repair_body = repair_response
        .into_body()
        .collect()
        .await
        .expect("repair response body should collect")
        .to_bytes();
    let repair_json: serde_json::Value =
        serde_json::from_slice(&repair_body).expect("repair response should be valid json");
    assert_eq!(repair_json["status"], "repaired");
    assert_eq!(repair_json["journalAuthority"], true);
    assert_eq!(repair_json["snapshotUpdated"], true);
    assert_eq!(repair_json["transactionMarkerCleared"], false);
    assert_eq!(repair_json["aggregateCounts"]["directChats"], 1);

    let read_response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/control/social/direct-chats/dc_operator_repair_journal_001")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_admin")
                .header("x-permissions", "control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("direct chat read after repair should return response");
    assert_eq!(read_response.status(), StatusCode::OK);

    let snapshot_body = fs::read_to_string(state_file(runtime_dir.as_path(), "social-state.json"))
        .expect("social state snapshot should be readable after repair");
    let snapshot_json: serde_json::Value =
        serde_json::from_str(&snapshot_body).expect("social state snapshot should be valid json");
    assert_eq!(
        snapshot_json["direct_chats"]["dc_operator_repair_journal_001"]["direct_chat"]["conversationId"],
        "c_operator_repair_journal_001"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
