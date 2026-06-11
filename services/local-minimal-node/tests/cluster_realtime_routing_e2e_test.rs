use im_app_context::DualTokenRequestBuilderExt;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{ContractError, RealtimeCheckpointRecord, RealtimeCheckpointStore};
use session_gateway::{RealtimeClusterBridge, RealtimeDeliveryRuntime};
use tower::ServiceExt;

#[derive(Clone)]
struct ToggleCheckpointStore {
    fail_saves: Arc<AtomicBool>,
}

impl ToggleCheckpointStore {
    fn new(fail_saves: bool) -> Self {
        Self {
            fail_saves: Arc::new(AtomicBool::new(fail_saves)),
        }
    }

    fn fail_saves(&self) {
        self.fail_saves.store(true, Ordering::SeqCst);
    }

    fn allow_saves(&self) {
        self.fail_saves.store(false, Ordering::SeqCst);
    }
}

impl RealtimeCheckpointStore for ToggleCheckpointStore {
    fn load_checkpoint(
        &self,
        _tenant_id: &str,
        _principal_kind: &str,
        _principal_id: &str,
        _device_id: &str,
    ) -> Result<Option<RealtimeCheckpointRecord>, ContractError> {
        Ok(None)
    }

    fn save_checkpoints(
        &self,
        _records: Vec<RealtimeCheckpointRecord>,
    ) -> Result<(), ContractError> {
        if self.fail_saves.load(Ordering::SeqCst) {
            Err(ContractError::Unavailable(
                "synthetic realtime checkpoint save failure".into(),
            ))
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn test_local_minimal_profile_routes_realtime_events_to_remote_owner_node() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18101",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18102",
        projection_service.clone(),
        realtime_cluster.clone(),
    );

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cluster_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote")
                .with_dual_token_session("s_remote")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should succeed");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote")
                .with_dual_token_session("s_remote")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_realtime",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should succeed");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    let remote_cluster = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/cluster")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote")
                .with_dual_token_session("s_remote")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote ops cluster should succeed");
    assert_eq!(remote_cluster.status(), StatusCode::OK);
    let remote_cluster_body = remote_cluster
        .into_body()
        .collect()
        .await
        .expect("remote cluster body should collect")
        .to_bytes();
    let remote_cluster_json: serde_json::Value =
        serde_json::from_slice(&remote_cluster_body).expect("remote cluster should be valid json");
    assert_eq!(remote_cluster_json["nodes"][0]["nodeId"], "node_b");
    assert_eq!(remote_cluster_json["nodes"][0]["clientRouteCount"], 1);

    let remote_diagnostics = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote")
                .with_dual_token_session("s_remote")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote ops diagnostics should succeed");
    assert_eq!(remote_diagnostics.status(), StatusCode::OK);
    let remote_diagnostics_body = remote_diagnostics
        .into_body()
        .collect()
        .await
        .expect("remote diagnostics body should collect")
        .to_bytes();
    let remote_diagnostics_json: serde_json::Value =
        serde_json::from_slice(&remote_diagnostics_body)
            .expect("remote diagnostics should be valid json");
    assert_eq!(
        remote_diagnostics_json["clientRoutes"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
    assert_eq!(
        remote_diagnostics_json["clientRoutes"][0]["deviceId"],
        "d_remote"
    );
    assert_eq!(
        remote_diagnostics_json["clientRoutes"][0]["ownerNodeId"],
        "node_b"
    );

    let post_message = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_route_1",
                        "summary":"cluster hello",
                        "text":"cluster hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let remote_events = app_b
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote")
                .with_dual_token_session("s_remote")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote realtime events should succeed");
    assert_eq!(remote_events.status(), StatusCode::OK);
    let remote_events_body = remote_events
        .into_body()
        .collect()
        .await
        .expect("remote realtime body should collect")
        .to_bytes();
    let remote_events_json: serde_json::Value = serde_json::from_slice(&remote_events_body)
        .expect("remote realtime events should be valid json");
    assert_eq!(remote_events_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(
        remote_events_json["items"][0]["scopeId"],
        "c_cluster_realtime"
    );
    assert_eq!(
        remote_events_json["items"][0]["eventType"],
        "message.posted"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_records_remote_realtime_delivery_failure_without_failing_committed_message()
 {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    let remote_checkpoint_store = ToggleCheckpointStore::new(false);
    let remote_runtime = Arc::new(
        RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(Arc::new(
            remote_checkpoint_store.clone(),
        )),
    );

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18103",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies_and_runtime(
        "node_b",
        "127.0.0.1:18104",
        projection_service.clone(),
        realtime_cluster.clone(),
        remote_runtime,
    );

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cluster_realtime_failure",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime_failure/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_failure")
                .with_dual_token_session("s_remote_failure")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should return response");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_failure")
                .with_dual_token_session("s_remote_failure")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_realtime_failure",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should return response");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    remote_checkpoint_store.fail_saves();
    let post_message = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime_failure/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_route_failure_1",
                        "summary":"cluster failure",
                        "text":"cluster failure"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_json: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post response should be valid json");
    assert_eq!(post_json["deliveryStatus"], "applied");
    assert_eq!(post_json["messageSeq"], 1);

    let audit_export = app_a
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .with_dual_token_permission_scope("audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_json: serde_json::Value =
        serde_json::from_slice(&audit_body).expect("audit export should be valid json");
    let side_effect_failure = audit_json["items"]
        .as_array()
        .expect("audit items should be an array")
        .iter()
        .find(|item| item["action"] == "message.side_effect_failed")
        .expect("realtime side effect failure must be audited after committed message");
    assert_eq!(side_effect_failure["aggregateType"], "conversation");
    assert_eq!(
        side_effect_failure["aggregateId"],
        "c_cluster_realtime_failure"
    );
    let audit_payload: serde_json::Value = serde_json::from_str(
        side_effect_failure["payload"]
            .as_str()
            .expect("audit payload should be string"),
    )
    .expect("audit payload should be valid json");
    assert_eq!(audit_payload["sideEffect"], "realtime_delivery");
    assert_eq!(audit_payload["errorCode"], "realtime_delivery_failed");
    assert_eq!(audit_payload["messageId"], post_json["messageId"]);
    assert!(
        audit_payload["errorMessage"]
            .as_str()
            .expect("error message should be a string")
            .contains("checkpoint_store_unavailable")
    );
    assert!(
        audit_payload["errorMessage"]
            .as_str()
            .expect("error message should be a string")
            .contains("d_remote_failure")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_retries_pending_realtime_outbox_after_delivery_store_recovers()
{
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    let remote_checkpoint_store = ToggleCheckpointStore::new(false);
    let remote_runtime = Arc::new(
        RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(Arc::new(
            remote_checkpoint_store.clone(),
        )),
    );

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18108",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies_and_runtime(
        "node_b",
        "127.0.0.1:18109",
        projection_service.clone(),
        realtime_cluster.clone(),
        remote_runtime,
    );

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cluster_realtime_outbox_retry",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime_outbox_retry/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_retry")
                .with_dual_token_session("s_remote_retry")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should return response");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_retry")
                .with_dual_token_session("s_remote_retry")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_realtime_outbox_retry",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should return response");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    remote_checkpoint_store.fail_saves();
    let first_post = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime_outbox_retry/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_outbox_retry_1",
                        "summary":"outbox retry first",
                        "text":"outbox retry first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should return response");
    assert_eq!(first_post.status(), StatusCode::OK);

    let diagnostics_after_failed_delivery = app_a
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics after failed delivery should return response");
    assert_eq!(diagnostics_after_failed_delivery.status(), StatusCode::OK);
    let diagnostics_after_failed_delivery_body = diagnostics_after_failed_delivery
        .into_body()
        .collect()
        .await
        .expect("diagnostics after failed delivery body should collect")
        .to_bytes();
    let diagnostics_after_failed_delivery_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_after_failed_delivery_body)
            .expect("diagnostics after failed delivery should be valid json");
    let realtime_outbox_after_failed_delivery =
        diagnostics_after_failed_delivery_json["sideEffectOutboxes"]
            .as_array()
            .expect("side-effect outboxes should be array")
            .iter()
            .find(|item| item["name"] == "message_realtime_delivery")
            .expect("realtime delivery outbox diagnostics should be present");
    assert_eq!(
        realtime_outbox_after_failed_delivery["name"],
        "message_realtime_delivery"
    );
    assert_eq!(realtime_outbox_after_failed_delivery["status"], "degraded");
    assert_eq!(realtime_outbox_after_failed_delivery["pendingCount"], 1);
    assert_eq!(realtime_outbox_after_failed_delivery["deliveredCount"], 0);
    assert_eq!(
        realtime_outbox_after_failed_delivery["failedAttemptCount"],
        1
    );
    assert!(
        realtime_outbox_after_failed_delivery["oldestPendingCreatedAt"].is_string(),
        "ops diagnostics should expose the oldest pending outbox timestamp"
    );

    let remote_events_before_recovery = app_b
        .clone()
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_retry")
                .with_dual_token_session("s_remote_retry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote events before recovery should return response");
    assert_eq!(remote_events_before_recovery.status(), StatusCode::OK);
    let remote_events_before_body = remote_events_before_recovery
        .into_body()
        .collect()
        .await
        .expect("remote events before body should collect")
        .to_bytes();
    let remote_events_before_json: serde_json::Value =
        serde_json::from_slice(&remote_events_before_body)
            .expect("remote events before recovery should be valid json");
    assert_eq!(
        remote_events_before_json["items"].as_array().unwrap().len(),
        0,
        "failed delivery must remain pending instead of being lost"
    );

    remote_checkpoint_store.allow_saves();
    let second_post = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_realtime_outbox_retry/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_outbox_retry_2",
                        "summary":"outbox retry trigger",
                        "text":"outbox retry trigger"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post should return response");
    assert_eq!(second_post.status(), StatusCode::OK);

    let remote_events_after_recovery = app_b
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_retry")
                .with_dual_token_session("s_remote_retry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote events after recovery should return response");
    assert_eq!(remote_events_after_recovery.status(), StatusCode::OK);
    let remote_events_after_body = remote_events_after_recovery
        .into_body()
        .collect()
        .await
        .expect("remote events after body should collect")
        .to_bytes();
    let remote_events_after_json: serde_json::Value =
        serde_json::from_slice(&remote_events_after_body)
            .expect("remote events after recovery should be valid json");
    let items = remote_events_after_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 2);
    let payloads = items
        .iter()
        .map(|item| {
            serde_json::from_str::<serde_json::Value>(
                item["payload"].as_str().expect("payload should be string"),
            )
            .expect("payload should be valid json")
        })
        .collect::<Vec<_>>();
    assert_eq!(payloads[0]["summary"], "outbox retry first");
    assert_eq!(payloads[1]["summary"], "outbox retry trigger");

    let diagnostics_after_recovery = app_a
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics after recovery should return response");
    assert_eq!(diagnostics_after_recovery.status(), StatusCode::OK);
    let diagnostics_after_recovery_body = diagnostics_after_recovery
        .into_body()
        .collect()
        .await
        .expect("diagnostics after recovery body should collect")
        .to_bytes();
    let diagnostics_after_recovery_json: serde_json::Value =
        serde_json::from_slice(&diagnostics_after_recovery_body)
            .expect("diagnostics after recovery should be valid json");
    let realtime_outbox_after_recovery = diagnostics_after_recovery_json["sideEffectOutboxes"]
        .as_array()
        .expect("side-effect outboxes should be array")
        .iter()
        .find(|item| item["name"] == "message_realtime_delivery")
        .expect("realtime delivery outbox diagnostics should be present");
    assert_eq!(realtime_outbox_after_recovery["status"], "ok");
    assert_eq!(realtime_outbox_after_recovery["pendingCount"], 0);
    assert_eq!(realtime_outbox_after_recovery["deliveredCount"], 2);
    assert_eq!(realtime_outbox_after_recovery["failedAttemptCount"], 1);
    assert!(
        realtime_outbox_after_recovery["oldestPendingCreatedAt"].is_null(),
        "ops diagnostics should clear oldest pending timestamp after outbox drains"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_persists_pending_realtime_outbox_across_runtime_restart() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    let runtime_dir = std::env::temp_dir().join(format!(
        "craw_chat_realtime_outbox_restart_{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos()
    ));
    std::fs::create_dir_all(runtime_dir.as_path()).expect("runtime dir should be created");
    let remote_checkpoint_store = ToggleCheckpointStore::new(false);
    let remote_runtime = Arc::new(
        RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(Arc::new(
            remote_checkpoint_store.clone(),
        )),
    );

    let app_a_before = local_minimal_node::build_app_with_dependencies_and_runtime_dir(
        "node_a",
        "127.0.0.1:18108",
        runtime_dir.as_path(),
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies_and_runtime(
        "node_b",
        "127.0.0.1:18110",
        projection_service.clone(),
        realtime_cluster.clone(),
        remote_runtime,
    );

    let create_conversation = app_a_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_outbox_runtime_restart",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app_a_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_outbox_runtime_restart/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    let register_remote_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_restart")
                .with_dual_token_session("s_remote_restart")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register remote device should return response");
    assert_eq!(register_remote_device.status(), StatusCode::OK);

    let sync_remote_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_restart")
                .with_dual_token_session("s_remote_restart")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_outbox_runtime_restart",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync remote subscriptions should return response");
    assert_eq!(sync_remote_subscriptions.status(), StatusCode::OK);

    remote_checkpoint_store.fail_saves();
    let first_post = app_a_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_outbox_runtime_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_outbox_runtime_restart_1",
                        "summary":"restart pending",
                        "text":"restart pending"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should return response");
    assert_eq!(first_post.status(), StatusCode::OK);
    remote_checkpoint_store.allow_saves();

    let outbox_path = runtime_dir
        .join("state")
        .join("message-side-effect-outbox.json");
    let outbox_payload =
        std::fs::read_to_string(outbox_path.as_path()).expect("outbox file should exist");
    assert!(
        outbox_payload.contains("\"status\": \"pending\""),
        "failed realtime delivery should persist as pending outbox"
    );

    let app_a_after = local_minimal_node::build_app_with_dependencies_and_runtime_dir(
        "node_a",
        "127.0.0.1:18108",
        runtime_dir.as_path(),
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let second_post = app_a_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_outbox_runtime_restart/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_outbox_runtime_restart_2",
                        "summary":"restart trigger",
                        "text":"restart trigger"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post should return response");
    let second_post_status = second_post.status();
    let second_post_body = second_post
        .into_body()
        .collect()
        .await
        .expect("second post body should collect")
        .to_bytes();
    assert_eq!(
        second_post_status,
        StatusCode::OK,
        "second post body: {}",
        String::from_utf8_lossy(&second_post_body)
    );

    let remote_events = app_b
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_remote_restart")
                .with_dual_token_session("s_remote_restart")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("remote events should return response");
    assert_eq!(remote_events.status(), StatusCode::OK);
    let remote_events_body = remote_events
        .into_body()
        .collect()
        .await
        .expect("remote events body should collect")
        .to_bytes();
    let remote_events_json: serde_json::Value =
        serde_json::from_slice(&remote_events_body).expect("remote events should be valid json");
    let items = remote_events_json["items"]
        .as_array()
        .expect("items should be array");
    assert_eq!(items.len(), 2);

    let _ = std::fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_local_minimal_profile_continues_realtime_fanout_after_one_target_fails() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());
    let failing_checkpoint_store = ToggleCheckpointStore::new(false);
    let failing_runtime = Arc::new(
        RealtimeDeliveryRuntime::with_checkpoint_store_permissive_for_tests(Arc::new(
            failing_checkpoint_store.clone(),
        )),
    );

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18105",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies_and_runtime(
        "node_b",
        "127.0.0.1:18106",
        projection_service.clone(),
        realtime_cluster.clone(),
        failing_runtime,
    );
    let app_c = local_minimal_node::build_app_with_dependencies(
        "node_c",
        "127.0.0.1:18107",
        projection_service.clone(),
        realtime_cluster.clone(),
    );

    let create_conversation = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_cluster_partial_realtime_failure",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    for principal_id in ["u_a_fail", "u_z_ok"] {
        let add_member = app_a
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/chat/conversations/c_cluster_partial_realtime_failure/members/add")
                    .with_dual_token_tenant("t_demo")
                    .with_dual_token_user("u_demo")
                    .with_dual_token_actor_kind("user")
                    .with_dual_token_device("d_owner")
                    .with_dual_token_session("s_owner")
                    .header("content-type", "application/json")
                    .body(Body::from(format!(
                        r#"{{
                            "principalId":"{principal_id}",
                            "principalKind":"user",
                            "role":"member"
                        }}"#
                    )))
                    .unwrap(),
            )
            .await
            .expect("add member should return response");
        assert_eq!(add_member.status(), StatusCode::OK);
    }

    let register_failing_device = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_a_fail")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_fail")
                .with_dual_token_session("s_fail")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register failing device should return response");
    assert_eq!(register_failing_device.status(), StatusCode::OK);

    let sync_failing_subscriptions = app_b
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_a_fail")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_fail")
                .with_dual_token_session("s_fail")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_partial_realtime_failure",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync failing subscriptions should return response");
    assert_eq!(sync_failing_subscriptions.status(), StatusCode::OK);

    let register_healthy_device = app_c
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_z_ok")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_ok")
                .with_dual_token_session("s_ok")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("register healthy device should return response");
    assert_eq!(register_healthy_device.status(), StatusCode::OK);

    let sync_healthy_subscriptions = app_c
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/realtime/subscriptions/sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_z_ok")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_ok")
                .with_dual_token_session("s_ok")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "items":[
                            {
                                "scopeType":"conversation",
                                "scopeId":"c_cluster_partial_realtime_failure",
                                "eventTypes":["message.posted"]
                            }
                        ]
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("sync healthy subscriptions should return response");
    assert_eq!(sync_healthy_subscriptions.status(), StatusCode::OK);

    failing_checkpoint_store.fail_saves();
    let post_message = app_a
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_cluster_partial_realtime_failure/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_cluster_partial_failure_1",
                        "summary":"cluster partial failure",
                        "text":"cluster partial failure"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);
    let post_body = post_message
        .into_body()
        .collect()
        .await
        .expect("post message body should collect")
        .to_bytes();
    let post_json: serde_json::Value =
        serde_json::from_slice(&post_body).expect("post response should be valid json");
    assert_eq!(post_json["deliveryStatus"], "applied");

    let healthy_events = app_c
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_z_ok")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_ok")
                .with_dual_token_session("s_ok")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthy realtime events should return response");
    assert_eq!(healthy_events.status(), StatusCode::OK);
    let healthy_events_body = healthy_events
        .into_body()
        .collect()
        .await
        .expect("healthy realtime body should collect")
        .to_bytes();
    let healthy_events_json: serde_json::Value = serde_json::from_slice(&healthy_events_body)
        .expect("healthy realtime events should be valid json");
    assert_eq!(
        healthy_events_json["items"].as_array().unwrap().len(),
        1,
        "healthy fanout targets should still receive realtime delivery when another target fails"
    );
    assert_eq!(
        healthy_events_json["items"][0]["scopeId"],
        "c_cluster_partial_realtime_failure"
    );
    assert_eq!(
        healthy_events_json["items"][0]["eventType"],
        "message.posted"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_sessionless_rebind_after_cross_node_resume_takeover() {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(RealtimeClusterBridge::default());

    let app_a = local_minimal_node::build_app_with_dependencies(
        "node_a",
        "127.0.0.1:18113",
        projection_service.clone(),
        realtime_cluster.clone(),
    );
    let app_b = local_minimal_node::build_app_with_dependencies(
        "node_b",
        "127.0.0.1:18114",
        projection_service.clone(),
        realtime_cluster.clone(),
    );

    realtime_cluster
        .bind_client_route_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_resume",
            "node_a",
            Some("s_old"),
            "http",
        )
        .expect("old route bind should seed shared route directory");
    realtime_cluster
        .bind_client_route_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_resume",
            "node_b",
            Some("s_new"),
            "http",
        )
        .expect("new route bind should seed takeover state");

    let sessionless_register = app_a
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/presence/heartbeat")
                .header(
                    "authorization",
                    format!(
                        "Bearer {}",
                        serde_json::json!({
                            "tenant_id": "t_demo",
                            "login_scope": "TENANT",
                            "user_id": "u_demo",
                            "app_id": "craw-chat",
                            "auth_level": "password",
                            "subject_type": "user"
                        })
                    ),
                )
                .header(
                    "Access-Token",
                    serde_json::json!({
                        "tenant_id": "t_demo",
                        "login_scope": "TENANT",
                        "user_id": "u_demo",
                        "app_id": "craw-chat",
                        "environment": "dev",
                        "deployment_mode": "local",
                        "auth_level": "password",
                        "actor_id": "u_demo",
                        "actor_kind": "user",
                        "device_id": "d_resume",
                        "data_scope": ["tenant"],
                        "permission_scope": ["*"],
                        "subject_type": "user"
                    })
                    .to_string(),
                )
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("sessionless register should return response");
    assert_eq!(sessionless_register.status(), StatusCode::CONFLICT);
    let sessionless_register_body = sessionless_register
        .into_body()
        .collect()
        .await
        .expect("sessionless register body should collect")
        .to_bytes();
    let sessionless_register_json: serde_json::Value =
        serde_json::from_slice(&sessionless_register_body)
            .expect("sessionless register body should be valid json");
    assert_eq!(sessionless_register_json["code"], "session_id_required");

    let diagnostics_after = app_b
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_resume")
                .with_dual_token_session("s_new")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after sessionless register should succeed");
    assert_eq!(diagnostics_after.status(), StatusCode::OK);
    let diagnostics_after_body = diagnostics_after
        .into_body()
        .collect()
        .await
        .expect("diagnostics after body should collect")
        .to_bytes();
    let diagnostics_after_json: serde_json::Value = serde_json::from_slice(&diagnostics_after_body)
        .expect("diagnostics after should be valid json");
    assert_eq!(
        diagnostics_after_json["clientRoutes"][0]["ownerNodeId"],
        "node_b"
    );
}
