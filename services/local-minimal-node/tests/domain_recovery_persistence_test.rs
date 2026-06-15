use im_app_context::DualTokenRequestBuilderExt;
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

static UNIQUE_RUNTIME_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let counter = UNIQUE_RUNTIME_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "sdkwork_im_domain_recovery_runtime_{unique}_{counter}"
    ))
}

fn state_file(runtime_dir: &std::path::Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
}

#[tokio::test]
async fn test_default_local_minimal_profile_rebuild_restores_conversation_domain_state() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_domain_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_first_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_domain_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_domain_restart_1",
                        "summary":"first",
                        "text":"first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(post_first_message.status(), StatusCode::OK);

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let summary_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_domain_restart")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("conversation summary after restart should return a response");
    assert_eq!(summary_after_restart.status(), StatusCode::OK);
    let summary_after_restart_body = summary_after_restart
        .into_body()
        .collect()
        .await
        .expect("conversation summary after restart body should collect")
        .to_bytes();
    let summary_after_restart_json: serde_json::Value =
        serde_json::from_slice(&summary_after_restart_body)
            .expect("conversation summary after restart should be valid json");
    assert_eq!(summary_after_restart_json["messageCount"], 1);
    assert_eq!(
        summary_after_restart_json["lastMessageId"],
        "msg_c_domain_restart_1"
    );

    let members_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_domain_restart/members")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("members after restart should return a response");
    assert_eq!(members_after_restart.status(), StatusCode::OK);

    let post_second_message = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_domain_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_domain_restart_2",
                        "summary":"second",
                        "text":"second"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post after restart should return a response");
    assert_eq!(post_second_message.status(), StatusCode::OK);

    let timeline_after_restart = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_domain_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_new")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after restart should return a response");
    assert_eq!(timeline_after_restart.status(), StatusCode::OK);
    let timeline_after_restart_body = timeline_after_restart
        .into_body()
        .collect()
        .await
        .expect("timeline after restart body should collect")
        .to_bytes();
    let timeline_after_restart_json: serde_json::Value =
        serde_json::from_slice(&timeline_after_restart_body)
            .expect("timeline after restart should be valid json");
    let items = timeline_after_restart_json["items"]
        .as_array()
        .expect("timeline items should be an array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["messageId"], "msg_c_domain_restart_1");
    assert_eq!(items[1]["messageId"], "msg_c_domain_restart_2");

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_profile_bootstrap_survives_corrupted_commit_journal() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let create_before = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_corrupt_journal_bootstrap",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("seed conversation write should return response");
    assert_eq!(create_before.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "commit-journal.json"),
        "{invalid-journal",
    )
    .expect("corrupted commit journal should be writable");

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let health = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthz should return response even when startup journal replay is degraded");
    assert_eq!(health.status(), StatusCode::OK);

    let create_after = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_recovered")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_after_corrupt_journal_bootstrap",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("write after degraded startup should return response");
    assert_eq!(create_after.status(), StatusCode::SERVICE_UNAVAILABLE);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_projection_queries_from_runtime_dir_snapshots_when_commit_journal_is_missing()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_projection_snapshot_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_projection_snapshot_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_projection_snapshot_restart_1",
                        "summary":"snapshot summary",
                        "text":"snapshot summary"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let update_read_cursor = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_projection_snapshot_restart/read_cursor")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "readSeq":1,
                        "lastReadMessageId":"msg_c_projection_snapshot_restart_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("update read cursor should succeed");
    assert_eq!(update_read_cursor.status(), StatusCode::OK);

    assert!(
        state_file(runtime_dir.as_path(), "projection-metadata.json").exists(),
        "managed runtime dir should persist projection metadata snapshots"
    );
    assert!(
        state_file(runtime_dir.as_path(), "projection-timeline.json").exists(),
        "managed runtime dir should persist projection timeline snapshots"
    );

    fs::remove_file(state_file(runtime_dir.as_path(), "commit-journal.json"))
        .expect("commit journal should be removed to force snapshot restore");

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let inbox_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/inbox")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_after")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("inbox after restart should return a response");
    assert_eq!(inbox_after_restart.status(), StatusCode::OK);
    let inbox_after_restart_body = inbox_after_restart
        .into_body()
        .collect()
        .await
        .expect("inbox after restart body should collect")
        .to_bytes();
    let inbox_after_restart_json: serde_json::Value =
        serde_json::from_slice(&inbox_after_restart_body)
            .expect("inbox after restart should be valid json");
    let inbox_items = inbox_after_restart_json["items"]
        .as_array()
        .expect("inbox items should be an array");
    assert_eq!(inbox_items.len(), 1);
    assert_eq!(
        inbox_items[0]["conversationId"],
        "c_projection_snapshot_restart"
    );
    assert_eq!(inbox_items[0]["conversationType"], "group");
    assert_eq!(inbox_items[0]["lastSummary"], "snapshot summary");

    let summary_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_snapshot_restart")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_after")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("summary after restart should return a response");
    assert_eq!(summary_after_restart.status(), StatusCode::OK);
    let summary_after_restart_body = summary_after_restart
        .into_body()
        .collect()
        .await
        .expect("summary after restart body should collect")
        .to_bytes();
    let summary_after_restart_json: serde_json::Value =
        serde_json::from_slice(&summary_after_restart_body)
            .expect("summary after restart should be valid json");
    assert_eq!(
        summary_after_restart_json["lastSummary"],
        "snapshot summary"
    );

    let timeline_after_restart = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_snapshot_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_after")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after restart should return a response");
    assert_eq!(timeline_after_restart.status(), StatusCode::OK);
    let timeline_after_restart_body = timeline_after_restart
        .into_body()
        .collect()
        .await
        .expect("timeline after restart body should collect")
        .to_bytes();
    let timeline_after_restart_json: serde_json::Value =
        serde_json::from_slice(&timeline_after_restart_body)
            .expect("timeline after restart should be valid json");
    let timeline_items = timeline_after_restart_json["items"]
        .as_array()
        .expect("timeline items should be an array");
    assert_eq!(timeline_items.len(), 1);
    assert_eq!(
        timeline_items[0]["messageId"],
        "msg_c_projection_snapshot_restart_1"
    );

    let read_cursor_after_restart = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_snapshot_restart/read_cursor")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_after")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read cursor after restart should return a response");
    assert_eq!(read_cursor_after_restart.status(), StatusCode::OK);
    let read_cursor_after_restart_body = read_cursor_after_restart
        .into_body()
        .collect()
        .await
        .expect("read cursor after restart body should collect")
        .to_bytes();
    let read_cursor_after_restart_json: serde_json::Value =
        serde_json::from_slice(&read_cursor_after_restart_body)
            .expect("read cursor after restart should be valid json");
    assert_eq!(read_cursor_after_restart_json["readSeq"], 1);
    assert_eq!(
        read_cursor_after_restart_json["lastReadMessageId"],
        "msg_c_projection_snapshot_restart_1"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_profile_surfaces_projection_plane_observability_over_ops_health_and_diagnostics()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_projection_observability",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_pad")
                .with_dual_token_session("s_pad")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"deviceId":"d_pad"}"#))
                .unwrap(),
        )
        .await
        .expect("device register should succeed");
    assert_eq!(register.status(), StatusCode::OK);

    let post_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_projection_observability/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_projection_observability_1",
                        "summary":"projection observability",
                        "text":"projection observability"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let ops_health = app_before
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/health")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops health should return response");
    assert_eq!(ops_health.status(), StatusCode::OK);
    let ops_health_body = ops_health
        .into_body()
        .collect()
        .await
        .expect("ops health body should collect")
        .to_bytes();
    let ops_health_json: serde_json::Value =
        serde_json::from_slice(&ops_health_body).expect("ops health should be valid json");
    assert_eq!(ops_health_json["projectionPlane"]["status"], "ok");
    assert!(
        ops_health_json["projectionPlane"]["metrics"]["conversationSnapshotPersist"]["successCount"]
            .as_u64()
            .unwrap() >= 1,
        "ops health should expose conversation snapshot persist metrics"
    );
    assert!(
        ops_health_json["projectionPlane"]["metrics"]["clientRouteSyncSnapshotPersist"]
            ["successCount"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops health should expose client route sync snapshot persist metrics"
    );
    assert!(
        ops_health_json["projectionPlane"]["updateDelay"]["timelineMs"]
            .as_u64()
            .is_some(),
        "ops health should expose projection timeline update delay"
    );
    assert!(
        ops_health_json["projectionPlane"]["updateDelay"]["inboxMs"]
            .as_u64()
            .is_some(),
        "ops health should expose projection inbox update delay"
    );
    assert_eq!(
        ops_health_json["projectionPlane"]["updateDelay"]["sourceEventType"],
        "message.posted"
    );
    assert_eq!(
        ops_health_json["projectionPlane"]["updateDelay"]["scopeId"],
        "6#t_demo26#c_projection_observability"
    );

    let lag_before_restart = app_before
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/lag")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops lag before restart should return response");
    assert_eq!(lag_before_restart.status(), StatusCode::OK);
    let lag_before_restart_body = lag_before_restart
        .into_body()
        .collect()
        .await
        .expect("ops lag before restart body should collect")
        .to_bytes();
    let lag_before_restart_json: serde_json::Value =
        serde_json::from_slice(&lag_before_restart_body)
            .expect("ops lag before restart should be valid json");
    assert!(
        lag_before_restart_json["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| {
                item["component"] == "projection_live"
                    && item["scopeId"] == "6#t_demo26#c_projection_observability"
                    && item["currentOffset"] == item["committedOffset"]
                    && item["lag"] == 0
            }),
        "ops lag should expose the live projection lag item after the real projection apply path runs"
    );

    fs::remove_file(state_file(runtime_dir.as_path(), "commit-journal.json"))
        .expect("commit journal should be removed to force snapshot restore");

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let diagnostics = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should return response");
    assert_eq!(diagnostics.status(), StatusCode::OK);
    let diagnostics_body = diagnostics
        .into_body()
        .collect()
        .await
        .expect("ops diagnostics body should collect")
        .to_bytes();
    let diagnostics_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_body).expect("ops diagnostics should be valid json");
    assert_eq!(diagnostics_json["projectionPlane"]["status"], "ok");
    assert!(
        diagnostics_json["projectionPlane"]["metrics"]["conversationSnapshotRestore"]
            ["successCount"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops diagnostics should expose conversation snapshot restore metrics"
    );
    assert!(
        diagnostics_json["projectionPlane"]["metrics"]["clientRouteSyncSnapshotRestore"]
            ["successCount"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops diagnostics should expose client route sync snapshot restore metrics"
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["backlogSize"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["replayedEventCount"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["replay"]["durationMs"],
        0
    );
    assert!(
        diagnostics_json["projectionPlane"]["rebuildDurationMs"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops diagnostics should expose rebuild duration after snapshot-only recovery"
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["updateDelay"]["timelineMs"],
        0
    );
    assert_eq!(
        diagnostics_json["projectionPlane"]["updateDelay"]["inboxMs"],
        0
    );
    assert!(
        diagnostics_json["projectionPlane"]["traces"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["operation"] == "conversation_snapshot.restore"
                && item["outcome"] == "success"),
        "ops diagnostics should expose projection restore traces"
    );
    assert!(
        diagnostics_json["projectionPlane"]["logs"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["code"] == "projection_snapshot_restore_succeeded"),
        "ops diagnostics should expose projection restore structured logs"
    );

    let lag = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/lag")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops lag should return response");
    assert_eq!(lag.status(), StatusCode::OK);
    let lag_body = lag
        .into_body()
        .collect()
        .await
        .expect("ops lag body should collect")
        .to_bytes();
    let lag_json: serde_json::Value =
        serde_json::from_slice(&lag_body).expect("ops lag should be valid json");
    assert!(
        lag_json["items"]
            .as_array()
            .unwrap()
            .iter()
            .any(|item| item["component"] == "projection_replay" && item["lag"] == 0),
        "ops lag should expose zero projection replay lag after snapshot-only recovery"
    );

    let replay_status = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/replay_status")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops replay_status should return response");
    assert_eq!(replay_status.status(), StatusCode::OK);
    let replay_status_body = replay_status
        .into_body()
        .collect()
        .await
        .expect("ops replay_status body should collect")
        .to_bytes();
    let replay_status_json: serde_json::Value = serde_json::from_slice(&replay_status_body)
        .expect("ops replay_status should be valid json");
    assert_eq!(replay_status_json["status"], "idle");
    assert_eq!(replay_status_json["replay"]["backlogSize"], 0);
    assert_eq!(replay_status_json["replay"]["replayedEventCount"], 0);
    assert_eq!(replay_status_json["replay"]["durationMs"], 0);
    assert_eq!(replay_status_json["replayThroughputPerSecond"], 0);

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_local_minimal_profile_reports_projection_replay_backlog_and_lag_after_stale_snapshot_restart()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let create_conversation = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_projection_replay_lag",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_first_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_projection_replay_lag/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_projection_replay_lag_1",
                        "summary":"first replay checkpoint",
                        "text":"first replay checkpoint"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should succeed");
    assert_eq!(post_first_message.status(), StatusCode::OK);

    let stale_projection_metadata = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "projection-metadata.json",
    ))
    .expect("projection metadata should exist after first message");
    let stale_projection_timeline = fs::read_to_string(state_file(
        runtime_dir.as_path(),
        "projection-timeline.json",
    ))
    .expect("projection timeline should exist after first message");

    let post_second_message = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_projection_replay_lag/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_projection_replay_lag_2",
                        "summary":"second replay checkpoint",
                        "text":"second replay checkpoint"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post should succeed");
    assert_eq!(post_second_message.status(), StatusCode::OK);

    fs::write(
        state_file(runtime_dir.as_path(), "projection-metadata.json"),
        stale_projection_metadata,
    )
    .expect("projection metadata should be rewound to stale snapshot");
    fs::write(
        state_file(runtime_dir.as_path(), "projection-timeline.json"),
        stale_projection_timeline,
    )
    .expect("projection timeline should be rewound to stale snapshot");

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let diagnostics = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should return response");
    assert_eq!(diagnostics.status(), StatusCode::OK);
    let diagnostics_body = diagnostics
        .into_body()
        .collect()
        .await
        .expect("ops diagnostics body should collect")
        .to_bytes();
    let diagnostics_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_body).expect("ops diagnostics should be valid json");
    assert!(
        diagnostics_json["projectionPlane"]["replay"]["backlogSize"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops diagnostics should expose replay backlog after stale snapshot recovery"
    );
    assert!(
        diagnostics_json["projectionPlane"]["replay"]["replayedEventCount"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops diagnostics should expose replayed event count after stale snapshot recovery"
    );
    assert!(
        diagnostics_json["projectionPlane"]["replay"]["durationMs"]
            .as_u64()
            .is_some(),
        "ops diagnostics should expose replay duration after stale snapshot recovery"
    );

    let lag = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/lag")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops lag should return response");
    assert_eq!(lag.status(), StatusCode::OK);
    let lag_body = lag
        .into_body()
        .collect()
        .await
        .expect("ops lag body should collect")
        .to_bytes();
    let lag_json: serde_json::Value =
        serde_json::from_slice(&lag_body).expect("ops lag should be valid json");
    assert!(
        lag_json["items"].as_array().unwrap().iter().any(|item| {
            item["component"] == "projection_replay"
                && item["scopeId"] == "6#t_demo23#c_projection_replay_lag"
                && item["lag"].as_u64().unwrap() >= 1
                && item["currentOffset"].as_u64().unwrap()
                    > item["committedOffset"].as_u64().unwrap()
        }),
        "ops lag should expose the stale projection replay gap"
    );

    let replay_status = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/replay_status")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops replay_status should return response");
    assert_eq!(replay_status.status(), StatusCode::OK);
    let replay_status_body = replay_status
        .into_body()
        .collect()
        .await
        .expect("ops replay_status body should collect")
        .to_bytes();
    let replay_status_json: serde_json::Value = serde_json::from_slice(&replay_status_body)
        .expect("ops replay_status should be valid json");
    assert_eq!(replay_status_json["status"], "replayed");
    assert!(
        replay_status_json["replay"]["backlogSize"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops replay_status should expose replay backlog after stale snapshot recovery"
    );
    assert!(
        replay_status_json["replay"]["replayedEventCount"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops replay_status should expose replayed event count after stale snapshot recovery"
    );
    assert!(
        replay_status_json["replay"]["durationMs"].as_u64().unwrap() >= 1,
        "ops replay_status should expose a positive replay duration after stale snapshot recovery"
    );
    assert!(
        replay_status_json["replayThroughputPerSecond"]
            .as_u64()
            .unwrap()
            >= 1,
        "ops replay_status should expose replay throughput after stale snapshot recovery"
    );

    let timeline = app_after
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/chat/conversations/c_projection_replay_lag/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone_restart")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("timeline after restart should return a response");
    assert_eq!(timeline.status(), StatusCode::OK);
    let timeline_body = timeline
        .into_body()
        .collect()
        .await
        .expect("timeline body should collect")
        .to_bytes();
    let timeline_json: serde_json::Value =
        serde_json::from_slice(&timeline_body).expect("timeline should be valid json");
    assert_eq!(
        timeline_json["items"].as_array().unwrap().len(),
        2,
        "stale snapshot replay should recover the missing message"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
