use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;

trait AppContextRequestBuilderExt {
    fn demo_app_context(self) -> Self;
    fn automation_read_context(self) -> Self;
    fn ops_context(self) -> Self;
    fn audit_context(self) -> Self;
}

impl AppContextRequestBuilderExt for axum::http::request::Builder {
    fn demo_app_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "sdkwork_iam_session_demo")
    }

    fn automation_read_context(self) -> Self {
        self.demo_app_context()
            .header("x-sdkwork-permission-scope", "automation.read")
    }

    fn ops_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_ops_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "sdkwork_iam_session_ops")
            .header("x-sdkwork-permission-scope", "ops.read")
    }

    fn audit_context(self) -> Self {
        self.header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_audit_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header("x-sdkwork-session-id", "sdkwork_iam_session_audit")
            .header("x-sdkwork-permission-scope", "audit.write audit.read")
    }
}

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    serde_json::from_slice(&body).expect("response body should be valid json")
}

#[derive(Clone)]
struct ToggleNotificationJournal {
    fail_appends: Arc<AtomicBool>,
    committed: Arc<Mutex<Vec<im_domain_events::CommitEnvelope>>>,
}

impl ToggleNotificationJournal {
    fn new(fail_appends: bool) -> Self {
        Self {
            fail_appends: Arc::new(AtomicBool::new(fail_appends)),
            committed: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn fail_appends(&self) {
        self.fail_appends.store(true, Ordering::SeqCst);
    }

    fn allow_appends(&self) {
        self.fail_appends.store(false, Ordering::SeqCst);
    }

    fn committed_event_types(&self) -> Vec<String> {
        self.committed
            .lock()
            .expect("notification journal commit log should lock")
            .iter()
            .map(|event| event.event_type.clone())
            .collect()
    }
}

impl CommitJournal for ToggleNotificationJournal {
    fn append(
        &self,
        envelope: im_domain_events::CommitEnvelope,
    ) -> Result<CommitPosition, ContractError> {
        if self.fail_appends.load(Ordering::SeqCst) {
            return Err(ContractError::Unavailable(
                "notification journal is temporarily unavailable".into(),
            ));
        }
        let mut committed = self
            .committed
            .lock()
            .expect("notification journal commit log should lock");
        committed.push(envelope);
        Ok(CommitPosition::new(
            "notification-test",
            committed.len() as u64,
        ))
    }
}

#[tokio::test]
async fn test_local_minimal_profile_retries_pending_message_notification_outbox_after_notification_runtime_recovers()
 {
    let projection_service = Arc::new(projection_service::TimelineProjectionService::default());
    let realtime_cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let notification_journal = Arc::new(ToggleNotificationJournal::new(false));
    let notification_runtime = Arc::new(
        notification_service::NotificationRuntime::with_journal_and_projection(
            notification_journal.clone(),
            projection_service.clone(),
        ),
    );
    let app = local_minimal_node::build_app_with_dependencies_realtime_and_notification_runtime(
        "node_a",
        "127.0.0.1:18210",
        projection_service,
        realtime_cluster,
        Arc::new(session_gateway::RealtimeDeliveryRuntime::default()),
        notification_runtime,
    );

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_notification_outbox_retry",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let register_owner = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/devices/register")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("owner register should return response");
    assert_eq!(register_owner.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_notification_outbox_retry/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_member",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should return response");
    assert_eq!(add_member.status(), StatusCode::OK);

    notification_journal.fail_appends();
    let first_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_notification_outbox_retry/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_notification_outbox_retry_1",
                        "summary":"notification outbox first",
                        "text":"notification outbox first"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first post should return response");
    assert_eq!(first_post.status(), StatusCode::OK);
    assert!(
        notification_journal.committed_event_types().is_empty(),
        "failed notification runtime should not commit notification events immediately"
    );

    let diagnostics_after_failure = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after notification failure should return response");
    assert_eq!(diagnostics_after_failure.status(), StatusCode::OK);
    let diagnostics_after_failure_json = json_body(diagnostics_after_failure).await;
    let notification_outbox_after_failure = diagnostics_after_failure_json["sideEffectOutboxes"]
        .as_array()
        .expect("side-effect outboxes should be array")
        .iter()
        .find(|item| item["name"] == "message_notification_delivery")
        .expect("message notification outbox diagnostics should be present");
    assert_eq!(notification_outbox_after_failure["status"], "degraded");
    assert_eq!(notification_outbox_after_failure["pendingCount"], 1);
    assert_eq!(notification_outbox_after_failure["failedAttemptCount"], 1);

    notification_journal.allow_appends();
    let second_post = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_notification_outbox_retry/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_notification_outbox_retry_2",
                        "summary":"notification outbox trigger",
                        "text":"notification outbox trigger"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second post should return response");
    assert_eq!(second_post.status(), StatusCode::OK);

    assert_eq!(
        notification_journal.committed_event_types(),
        vec![
            "notification.requested",
            "notification.dispatched",
            "notification.requested",
            "notification.dispatched",
        ],
        "recovered notification runtime should receive retried and current message notifications"
    );

    let local_app_notification_route = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/notifications")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_member")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("local app notification route should return response");
    assert_eq!(local_app_notification_route.status(), StatusCode::NOT_FOUND);

    let diagnostics_after_recovery = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_owner")
                .header("x-sdkwork-session-id", "s_owner")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("diagnostics after notification recovery should return response");
    assert_eq!(diagnostics_after_recovery.status(), StatusCode::OK);
    let diagnostics_after_recovery_json = json_body(diagnostics_after_recovery).await;
    let notification_outbox_after_recovery = diagnostics_after_recovery_json["sideEffectOutboxes"]
        .as_array()
        .expect("side-effect outboxes should be array")
        .iter()
        .find(|item| item["name"] == "message_notification_delivery")
        .expect("message notification outbox diagnostics should be present");
    assert_eq!(notification_outbox_after_recovery["status"], "ok");
    assert_eq!(notification_outbox_after_recovery["pendingCount"], 0);
    assert_eq!(notification_outbox_after_recovery["deliveredCount"], 2);
    assert_eq!(notification_outbox_after_recovery["failedAttemptCount"], 1);
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_im_and_backend_surfaces_without_local_appbase_api_routes()
 {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_task10_standard_surface",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return response");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_task10_standard_surface/messages")
                .demo_app_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_task10_standard_surface",
                        "summary":"standard surface message",
                        "text":"standard surface message"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should return response");
    assert_eq!(post_message.status(), StatusCode::OK);

    for (method, path, body) in [
        ("GET", "/app/v3/api/iam/verification_policy", Body::empty()),
        (
            "GET",
            "/app/v3/api/open_platform/qr_auth/sessions",
            Body::empty(),
        ),
        ("GET", "/app/v3/api/notifications", Body::empty()),
        (
            "POST",
            "/app/v3/api/automation/executions",
            Body::from(
                r#"{
                    "executionId":"ae_task10_local_appbase_boundary",
                    "triggerType":"webhook.manual",
                    "targetKind":"workflow",
                    "targetRef":"wf_task10_boundary",
                    "inputPayload":"{}"
                }"#,
            ),
        ),
    ] {
        let mut builder = Request::builder()
            .method(method)
            .uri(path)
            .demo_app_context();
        if method == "POST" {
            builder = builder.header("content-type", "application/json");
        }
        let response = app
            .clone()
            .oneshot(builder.body(body).unwrap())
            .await
            .expect("local appbase-owned route should return response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must not be mounted by local-minimal-node"
        );
    }

    let governance = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/automation/governance")
                .automation_read_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("automation governance should return response");
    assert_eq!(governance.status(), StatusCode::OK);
    let governance_json = json_body(governance).await;
    assert_eq!(governance_json["capabilityProfileId"], "stable-agent");
    assert_eq!(governance_json["operatorOverrideActive"], false);

    let audit_record = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/audit/records")
                .audit_context()
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_task10_standard_surface",
                        "aggregateType":"conversation",
                        "aggregateId":"c_task10_standard_surface",
                        "action":"conversation.standard_surface_verified",
                        "payload":"{\"surface\":\"backend\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("audit record should return response");
    assert_eq!(audit_record.status(), StatusCode::OK);
    let audit_record_json = json_body(audit_record).await;
    assert_eq!(audit_record_json["deliveryStatus"], "applied");
    assert_eq!(
        audit_record_json["proofVersion"],
        "audit.record.delivery-proof.v1"
    );

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/audit/export")
                .audit_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_json = json_body(audit_export).await;
    let audit_items = audit_export_json["items"]
        .as_array()
        .expect("audit export items should be array");
    assert!(
        audit_items
            .iter()
            .any(|item| item["recordId"] == "audit_task10_standard_surface"),
        "backend audit API should expose the recorded backend audit anchor"
    );

    let ops_cluster = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/cluster")
                .ops_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops cluster should return response");
    assert_eq!(ops_cluster.status(), StatusCode::OK);
    let ops_cluster_json = json_body(ops_cluster).await;
    assert_eq!(ops_cluster_json["nodes"][0]["profile"], "local-minimal");

    let diagnostics = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .ops_context()
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should return response");
    assert_eq!(diagnostics.status(), StatusCode::OK);
}
