use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8iLCJhY3Rvcl9raW5kIjoidXNlciJ9.";
const AUTOMATION_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8iLCJhY3Rvcl9raW5kIjoidXNlciIsInBlcm1pc3Npb25zIjpbImF1dG9tYXRpb24uZXhlY3V0ZSIsImF1dG9tYXRpb24ucmVhZCJdfQ.";
const PRIVILEGED_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X29wc19hdWRpdF9kZW1vIiwic2lkIjoic19vcHNfYXVkaXRfZGVtbyIsImFjdG9yX2tpbmQiOiJ1c2VyIiwicGVybWlzc2lvbnMiOlsiYXVkaXQucmVhZCIsIm9wcy5yZWFkIl19.";

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
                .uri("/api/v1/conversations")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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
                .uri("/api/v1/devices/register")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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
                .uri("/api/v1/conversations/c_notification_outbox_retry/members/add")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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
                .uri("/api/v1/conversations/c_notification_outbox_retry/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("x-permissions", "ops.read")
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
                .uri("/api/v1/conversations/c_notification_outbox_retry/messages")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
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

    let committed_event_types = notification_journal.committed_event_types();
    assert_eq!(
        committed_event_types,
        vec![
            "notification.requested",
            "notification.dispatched",
            "notification.requested",
            "notification.dispatched",
        ],
        "recovered notification runtime should receive retried and current message notifications"
    );

    let member_notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_member")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("member notifications should return response");
    assert_eq!(member_notifications.status(), StatusCode::OK);
    let member_notifications_json = json_body(member_notifications).await;
    let items = member_notifications_json["items"]
        .as_array()
        .expect("notification items should be array");
    assert_eq!(items.len(), 2);
    let titles = items
        .iter()
        .map(|item| item["title"].as_str().expect("title should be string"))
        .collect::<Vec<_>>();
    assert!(
        titles.contains(&"notification outbox first"),
        "retried message notification should be visible to member"
    );
    assert!(
        titles.contains(&"notification outbox trigger"),
        "current message notification should be visible to member"
    );

    let diagnostics_after_recovery = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_owner")
                .header("x-actor-kind", "user")
                .header("x-device-id", "d_owner")
                .header("x-session-id", "s_owner")
                .header("x-permissions", "ops.read")
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
async fn test_local_minimal_profile_exposes_notification_automation_audit_and_ops_capabilities() {
    let app = local_minimal_node::build_default_app();

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_task10_demo",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/conversations/c_task10_demo/messages")
                .header("authorization", DEMO_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_task10_demo",
                        "summary":"task10 hello",
                        "text":"task10 hello"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("authorization", DEMO_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("notifications query should return response");
    assert_eq!(notifications.status(), StatusCode::OK);
    let notifications_body = notifications
        .into_body()
        .collect()
        .await
        .expect("notifications body should collect")
        .to_bytes();
    let notifications_json: serde_json::Value = serde_json::from_slice(&notifications_body)
        .expect("notifications body should be valid json");
    assert_eq!(
        notifications_json["items"]
            .as_array()
            .expect("items should be array")
            .len(),
        0
    );

    let automation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_task10_demo",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(automation.status(), StatusCode::OK);
    let automation_body = automation
        .into_body()
        .collect()
        .await
        .expect("automation body should collect")
        .to_bytes();
    let automation_json: serde_json::Value =
        serde_json::from_slice(&automation_body).expect("automation body should be valid json");
    assert_eq!(automation_json["state"], "succeeded");
    assert_eq!(automation_json["deliveryStatus"], "applied");
    assert!(
        !automation_json["requestKey"]
            .as_str()
            .expect("automation request key should be present")
            .is_empty()
    );

    let notifications_after_automation = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("authorization", AUTOMATION_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("notifications query after automation should return response");
    assert_eq!(notifications_after_automation.status(), StatusCode::OK);
    let notifications_after_automation_body = notifications_after_automation
        .into_body()
        .collect()
        .await
        .expect("notifications after automation body should collect")
        .to_bytes();
    let notifications_after_automation_json: serde_json::Value =
        serde_json::from_slice(&notifications_after_automation_body)
            .expect("notifications after automation should be valid json");
    assert_eq!(
        notifications_after_automation_json["items"][0]["sourceEventType"],
        "automation.execution_completed"
    );
    assert_eq!(
        notifications_after_automation_json["items"][0]["category"],
        "automation.result"
    );
    assert_eq!(
        notifications_after_automation_json["items"][0]["status"],
        "dispatched"
    );

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_export_body = audit_export
        .into_body()
        .collect()
        .await
        .expect("audit export body should collect")
        .to_bytes();
    let audit_export_json: serde_json::Value =
        serde_json::from_slice(&audit_export_body).expect("audit export body should be valid json");
    assert!(audit_export_json["total"].as_u64().unwrap() >= 2);

    let ops_cluster = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/cluster")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops cluster should return response");
    assert_eq!(ops_cluster.status(), StatusCode::OK);
    let ops_cluster_body = ops_cluster
        .into_body()
        .collect()
        .await
        .expect("ops cluster body should collect")
        .to_bytes();
    let ops_cluster_json: serde_json::Value =
        serde_json::from_slice(&ops_cluster_body).expect("ops cluster body should be valid json");
    assert_eq!(ops_cluster_json["nodes"][0]["profile"], "local-minimal");

    let diagnostics = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/ops/diagnostics")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should return response");
    assert_eq!(diagnostics.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_automation_request_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first automation request should return response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first automation body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first automation body should be valid json");
    assert_eq!(first_json["deliveryStatus"], "applied");

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent automation request should return response");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .expect("idempotent automation body should collect")
        .to_bytes();
    let second_json: serde_json::Value = serde_json::from_slice(&second_body)
        .expect("idempotent automation body should be valid json");
    assert_eq!(second_json["deliveryStatus"], "replayed");
    assert_eq!(second_json["requestKey"], first_json["requestKey"]);

    let notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("notifications query should return response");
    assert_eq!(notifications.status(), StatusCode::OK);
    let notifications_body = notifications
        .into_body()
        .collect()
        .await
        .expect("notifications body should collect")
        .to_bytes();
    let notifications_json: serde_json::Value = serde_json::from_slice(&notifications_body)
        .expect("notifications body should be valid json");
    assert_eq!(
        notifications_json["items"]
            .as_array()
            .expect("items should be array")
            .len(),
        1
    );
    assert_eq!(
        notifications_json["items"][0]["notificationId"],
        "ntf_automation_user_ae_local_idempotent"
    );

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
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
        .expect("audit body should collect")
        .to_bytes();
    let audit_json: serde_json::Value =
        serde_json::from_slice(&audit_body).expect("audit body should be valid json");
    assert_eq!(audit_json["total"], 1);
    assert_eq!(audit_json["items"][0]["aggregateId"], "ae_local_idempotent");

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_conflict_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting automation request should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "automation_execution_conflict");
}

#[tokio::test]
async fn test_local_minimal_profile_isolates_automation_notifications_by_actor_kind() {
    let app = local_minimal_node::build_default_app();

    let user_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-permissions", "automation.execute automation.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_actor_kind",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user automation request should return response");
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_body = user_response
        .into_body()
        .collect()
        .await
        .expect("user automation body should collect")
        .to_bytes();
    let user_json: serde_json::Value =
        serde_json::from_slice(&user_body).expect("user automation body should be valid json");
    assert_eq!(user_json["deliveryStatus"], "applied");

    let system_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-permissions", "automation.execute automation.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_actor_kind",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system automation request should return response");
    assert_eq!(system_response.status(), StatusCode::OK);
    let system_body = system_response
        .into_body()
        .collect()
        .await
        .expect("system automation body should collect")
        .to_bytes();
    let system_json: serde_json::Value =
        serde_json::from_slice(&system_body).expect("system automation body should be valid json");
    assert_eq!(system_json["deliveryStatus"], "applied");

    let user_notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("user notifications query should return response");
    assert_eq!(user_notifications.status(), StatusCode::OK);
    let user_notifications_body = user_notifications
        .into_body()
        .collect()
        .await
        .expect("user notifications body should collect")
        .to_bytes();
    let user_notifications_json: serde_json::Value =
        serde_json::from_slice(&user_notifications_body)
            .expect("user notifications body should be valid json");
    assert_eq!(
        user_notifications_json["items"]
            .as_array()
            .expect("user items should be array")
            .len(),
        1
    );
    assert_eq!(
        user_notifications_json["items"][0]["notificationId"],
        "ntf_automation_user_ae_local_actor_kind"
    );

    let system_notifications = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "system")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("system notifications query should return response");
    assert_eq!(system_notifications.status(), StatusCode::OK);
    let system_notifications_body = system_notifications
        .into_body()
        .collect()
        .await
        .expect("system notifications body should collect")
        .to_bytes();
    let system_notifications_json: serde_json::Value =
        serde_json::from_slice(&system_notifications_body)
            .expect("system notifications body should be valid json");
    assert_eq!(
        system_notifications_json["items"]
            .as_array()
            .expect("system items should be array")
            .len(),
        1
    );
    assert_eq!(
        system_notifications_json["items"][0]["notificationId"],
        "ntf_automation_system_ae_local_actor_kind"
    );
}

#[tokio::test]
async fn test_local_minimal_profile_records_automation_audit_per_actor_kind() {
    let app = local_minimal_node::build_default_app();

    let user_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_audit_actor_kind",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("user automation request should return response");
    assert_eq!(user_response.status(), StatusCode::OK);
    let user_json = json_body(user_response).await;
    assert_eq!(user_json["deliveryStatus"], "applied");

    let system_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_audit_actor_kind",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_task10_demo",
                        "inputPayload":"{\"conversationId\":\"c_task10_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("system automation request should return response");
    assert_eq!(system_response.status(), StatusCode::OK);
    let system_json = json_body(system_response).await;
    assert_eq!(system_json["deliveryStatus"], "applied");

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_json = json_body(audit_export).await;

    let automation_items = audit_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .filter(|item| {
            item["aggregateId"] == "ae_local_audit_actor_kind"
                && item["action"] == "automation.execution_requested"
        })
        .cloned()
        .collect::<Vec<_>>();

    assert_eq!(automation_items.len(), 2);
    let actor_kinds = automation_items
        .iter()
        .map(|item| {
            item["actorKind"]
                .as_str()
                .expect("actorKind should be string")
        })
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        actor_kinds,
        std::collections::BTreeSet::from(["system", "user"])
    );
}

#[tokio::test]
async fn test_local_minimal_profile_preserves_automation_audit_for_max_length_execution_ids() {
    let app = local_minimal_node::build_default_app();
    let execution_id = format!("ae_{}", "a".repeat(253));

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header(
                    "x-permissions",
                    "automation.execute automation.read audit.read",
                )
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": execution_id,
                        "triggerType": "webhook.manual",
                        "targetKind": "workflow",
                        "targetRef": "wf_task10_demo",
                        "inputPayload": "{\"conversationId\":\"c_task10_demo\"}",
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(response.status(), StatusCode::OK);
    let execution_json = json_body(response).await;
    assert_eq!(execution_json["deliveryStatus"], "applied");
    assert_eq!(execution_json["executionId"], execution_id);

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_json = json_body(audit_export).await;

    let automation_item = audit_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| {
            item["aggregateId"] == execution_id
                && item["action"] == "automation.execution_requested"
        })
        .expect("automation execution audit should be recorded for legal max-length execution ids");
    assert_eq!(automation_item["actorKind"], "user");
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_agent_response_and_tool_call_lifecycle_over_http() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent",
                        "streamId":"st_local_agent",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_task10_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);
    let start_json = json_body(start_response).await;
    assert_eq!(start_json["streamId"], "st_local_agent");
    assert_eq!(start_json["state"], "opened");

    let delta_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses/st_local_agent/frames")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "frameType":"delta.text",
                        "schemaRef":"schema://agent/response.delta#chunk",
                        "encoding":"json",
                        "payload":"{\"delta\":\"hello\"}",
                        "attributes":{"chunk":"1"}
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response delta should return response");
    assert_eq!(delta_response.status(), StatusCode::OK);
    let delta_json = json_body(delta_response).await;
    assert_eq!(delta_json["sender"]["kind"], "agent");
    assert_eq!(delta_json["sender"]["id"], "ag_demo");

    let tool_request_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent",
                        "toolCallId":"tc_local_lookup",
                        "toolName":"knowledge.search",
                        "argumentsPayload":"{\"query\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool request should return response");
    assert_eq!(tool_request_response.status(), StatusCode::OK);
    let tool_request_json = json_body(tool_request_response).await;
    assert_eq!(tool_request_json["state"], "requested");

    let tool_complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions/ae_local_agent/agent-tool-calls/tc_local_lookup/complete")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "resultPayload":"{\"hits\":[{\"id\":\"doc_1\"}]}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("tool completion should return response");
    assert_eq!(tool_complete_response.status(), StatusCode::OK);
    let tool_complete_json = json_body(tool_complete_response).await;
    assert_eq!(tool_complete_json["state"], "completed");

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses/st_local_agent/complete")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq":1,
                        "resultMessageId":"m_local_agent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response complete should return response");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_json = json_body(complete_response).await;
    assert_eq!(complete_json["state"], "completed");
    assert_eq!(complete_json["resultMessageId"], "m_local_agent");

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_json = json_body(audit_export).await;
    let items = audit_json["items"]
        .as_array()
        .expect("audit items should be array");
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_response_started")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_response_delta")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_tool_call_requested")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_tool_call_completed")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_response_completed")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_preserves_agent_response_audit_for_max_length_stream_ids() {
    let app = local_minimal_node::build_default_app();
    let stream_id = format!("st_{}", "s".repeat(253));

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent_long_stream",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_agent_long_stream",
                        "streamId": stream_id,
                        "streamType": "agent.response.delta",
                        "conversationId": "c_task10_demo",
                        "schemaRef": "schema://agent/response.delta",
                        "memberId": "cm_agent",
                        "agent": {
                            "agent_id": "ag_demo",
                            "session_id": "s_agent",
                            "metadata": {
                                "agentMode": "assistant",
                                "capabilityProfileId": "stable-agent"
                            }
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);
    let start_json = json_body(start_response).await;
    assert_eq!(start_json["streamId"], stream_id);
    assert_eq!(start_json["state"], "opened");

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_json = json_body(audit_export).await;
    let started_audit = audit_json["items"]
        .as_array()
        .expect("audit items should be array")
        .iter()
        .find(|item| {
            item["action"] == "automation.agent_response_started"
                && item["payload"].as_str().is_some_and(|payload| {
                    serde_json::from_str::<serde_json::Value>(payload)
                        .ok()
                        .and_then(|value| value["streamId"].as_str().map(|id| id == stream_id))
                        .unwrap_or(false)
                })
        })
        .expect("agent response audit should be recorded for max-length stream ids");
    let started_payload: serde_json::Value = serde_json::from_str(
        started_audit["payload"]
            .as_str()
            .expect("payload should be present"),
    )
    .expect("started audit payload should be valid json");
    assert_eq!(started_payload["streamId"], stream_id);
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_get_execution_path_id() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri(format!("/api/v1/automation/executions/{}", "e".repeat(257)))
                .header("authorization", AUTOMATION_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("oversized execution lookup should return response");
    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("executionId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_response_stream_id() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent_oversized_stream",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_agent_oversized_stream",
                        "streamId": "s".repeat(257),
                        "streamType": "agent.response.delta",
                        "conversationId": "c_task10_demo",
                        "schemaRef": "schema://agent/response.delta",
                        "memberId": "cm_agent",
                        "agent": {
                            "agent_id": "ag_demo",
                            "session_id": "s_agent",
                            "metadata": {
                                "agentMode": "assistant",
                                "capabilityProfileId": "stable-agent"
                            }
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(start_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("streamId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_response_member_id() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_agent_oversized_member",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_agent_oversized_member",
                        "streamId": "st_local_agent_oversized_member",
                        "streamType": "agent.response.delta",
                        "conversationId": "c_task10_demo",
                        "schemaRef": "schema://agent/response.delta",
                        "memberId": "m".repeat(257),
                        "agent": {
                            "agent_id": "ag_demo",
                            "session_id": "s_agent",
                            "metadata": {
                                "agentMode": "assistant",
                                "capabilityProfileId": "stable-agent"
                            }
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized member id start should return response");
    assert_eq!(start_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(start_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("memberId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_response_execution_id() {
    let app = local_minimal_node::build_default_app();

    let start_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "e".repeat(257),
                        "streamId": "st_local_oversized_start_execution_id",
                        "streamType": "agent.response.delta",
                        "conversationId": "c_task10_demo",
                        "schemaRef": "schema://agent/response.delta",
                        "memberId": "cm_agent",
                        "agent": {
                            "agent_id": "ag_demo",
                            "session_id": "s_agent",
                            "metadata": {
                                "agentMode": "assistant",
                                "capabilityProfileId": "stable-agent"
                            }
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized execution id start should return response");
    assert_eq!(start_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(start_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("executionId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_response_stream_path_ids() {
    let app = local_minimal_node::build_default_app();

    let append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/automation/agent-responses/{}/frames",
                    "s".repeat(257)
                ))
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "frameType": "delta.text",
                        "schemaRef": "schema://agent/response.delta#chunk",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}",
                        "attributes": {}
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized stream path append should return response");
    assert_eq!(append_response.status(), StatusCode::PAYLOAD_TOO_LARGE);

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(format!(
                    "/api/v1/automation/agent-responses/{}/complete",
                    "s".repeat(257)
                ))
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "resultMessageId": "m_done"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized stream path complete should return response");
    assert_eq!(complete_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_metadata() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_agent_metadata",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_oversized_agent_metadata",
                        "streamId": "st_local_oversized_agent_metadata",
                        "streamType": "agent.response.delta",
                        "conversationId": "c_task10_demo",
                        "schemaRef": "schema://agent/response.delta",
                        "memberId": "cm_agent",
                        "agent": {
                            "agent_id": "ag_demo",
                            "session_id": "s_agent",
                            "metadata": {
                                "trace": "x".repeat(65_537)
                            }
                        }
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized agent metadata start should return response");
    assert_eq!(start_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(start_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("agent.metadata")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_result_message_id() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_result_message_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_result_message_id",
                        "streamId":"st_local_oversized_result_message_id",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_task10_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses/st_local_oversized_result_message_id/complete")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "frameSeq": 1,
                        "resultMessageId": "m".repeat(257)
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized result message id request should return response");
    assert_eq!(complete_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(complete_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("resultMessageId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_identity_fields() {
    for (field, agent_id, session_id) in [
        (
            "agent.agent_id",
            serde_json::Value::String("a".repeat(257)),
            serde_json::Value::String("s_agent".into()),
        ),
        (
            "agent.session_id",
            serde_json::Value::String("ag_demo".into()),
            serde_json::Value::String("s".repeat(257)),
        ),
    ] {
        let app = local_minimal_node::build_default_app();

        let create_execution = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/automation/executions")
                    .header("authorization", AUTOMATION_BEARER)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "executionId": format!("ae_local_{}", field.replace('.', "_")),
                            "triggerType":"agent.manual",
                            "targetKind":"conversation",
                            "targetRef":"c_task10_demo",
                            "inputPayload":"{\"prompt\":\"hello\"}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("automation request should return response");
        assert_eq!(create_execution.status(), StatusCode::OK);

        let start_response = app
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri("/api/v1/automation/agent-responses")
                    .header("authorization", AUTOMATION_BEARER)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "executionId": format!("ae_local_{}", field.replace('.', "_")),
                            "streamId": format!("st_local_{}", field.replace('.', "_")),
                            "streamType": "agent.response.delta",
                            "conversationId": "c_task10_demo",
                            "schemaRef": "schema://agent/response.delta",
                            "memberId": "cm_agent",
                            "agent": {
                                "agent_id": agent_id,
                                "session_id": session_id,
                                "metadata": {
                                    "agentMode": "assistant",
                                    "capabilityProfileId": "stable-agent"
                                }
                            }
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("oversized agent identity start should return response");
        assert_eq!(
            start_response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_tool_call_id() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_tool_call_id",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_tool_call_id",
                        "streamId":"st_local_oversized_tool_call_id",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_task10_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let tool_call_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_oversized_tool_call_id",
                        "toolCallId": "t".repeat(257),
                        "toolName": "knowledge.search",
                        "argumentsPayload": "{\"query\":\"hello\"}"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized tool call request should return response");
    assert_eq!(tool_call_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(tool_call_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("toolCallId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_tool_call_execution_id() {
    let app = local_minimal_node::build_default_app();

    let tool_call_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "e".repeat(257),
                        "toolCallId": "tc_local_oversized_execution_id",
                        "toolName": "knowledge.search",
                        "argumentsPayload": "{\"query\":\"hello\"}"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized execution id request should return response");
    assert_eq!(tool_call_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(tool_call_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("executionId")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_complete_agent_tool_call_path_ids() {
    let app = local_minimal_node::build_default_app();

    for (field, execution_id, tool_call_id) in [
        ("executionId", "e".repeat(257), "tc_local_demo".to_string()),
        ("toolCallId", "ae_local_demo".to_string(), "t".repeat(257)),
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method("POST")
                    .uri(format!(
                        "/api/v1/automation/executions/{}/agent-tool-calls/{}/complete",
                        execution_id, tool_call_id
                    ))
                    .header("authorization", AUTOMATION_BEARER)
                    .header("content-type", "application/json")
                    .body(Body::from(
                        serde_json::json!({
                            "resultPayload": "{\"hits\":[{\"id\":\"doc_1\"}]}"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .expect("oversized path id request should return response");
        assert_eq!(
            response.status(),
            StatusCode::PAYLOAD_TOO_LARGE,
            "{field} should be rejected with payload_too_large"
        );
    }
}

#[tokio::test]
async fn test_local_minimal_profile_rejects_oversized_agent_tool_name() {
    let app = local_minimal_node::build_default_app();

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_tool_name",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_oversized_tool_name",
                        "streamId":"st_local_oversized_tool_name",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_task10_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let tool_call_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("authorization", AUTOMATION_BEARER)
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "executionId": "ae_local_oversized_tool_name",
                        "toolCallId": "tc_local_oversized_name",
                        "toolName": "t".repeat(257),
                        "argumentsPayload": "{\"query\":\"hello\"}"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("oversized tool name request should return response");
    assert_eq!(tool_call_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let error_json = json_body(tool_call_response).await;
    assert_eq!(error_json["code"], "payload_too_large");
    assert!(
        error_json["message"]
            .as_str()
            .expect("message should be present")
            .contains("toolName")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_exposes_automation_governance_and_override_audit() {
    let app = local_minimal_node::build_default_app();

    let governance_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/governance")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("governance request should return response");
    assert_eq!(governance_response.status(), StatusCode::OK);
    let governance_json = json_body(governance_response).await;
    assert_eq!(governance_json["capabilityProfileId"], "stable-agent");
    assert_eq!(governance_json["operatorOverrideActive"], false);

    let create_execution = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_guardrail",
                        "triggerType":"agent.manual",
                        "targetKind":"conversation",
                        "targetRef":"c_task10_demo",
                        "inputPayload":"{\"prompt\":\"shutdown\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(create_execution.status(), StatusCode::OK);

    let start_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-responses")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_guardrail",
                        "streamId":"st_local_guardrail",
                        "streamType":"agent.response.delta",
                        "conversationId":"c_task10_demo",
                        "schemaRef":"schema://agent/response.delta",
                        "memberId":"cm_agent",
                        "agent":{
                            "agent_id":"ag_demo",
                            "session_id":"s_agent",
                            "metadata":{
                                "agentMode":"assistant",
                                "capabilityProfileId":"stable-agent"
                            }
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("agent response start should return response");
    assert_eq!(start_response.status(), StatusCode::OK);

    let denied_tool_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_guardrail",
                        "toolCallId":"tc_local_guardrail_denied",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("guardrail tool request should return response");
    assert_eq!(denied_tool_response.status(), StatusCode::FORBIDDEN);
    let denied_tool_json = json_body(denied_tool_response).await;
    assert_eq!(denied_tool_json["code"], "automation_guardrail_denied");

    let allowed_tool_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/agent-tool-calls")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header(
                    "x-permissions",
                    "automation.execute automation.read automation.operator_override",
                )
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_local_guardrail",
                        "toolCallId":"tc_local_guardrail_allowed",
                        "toolName":"ops.shutdown",
                        "argumentsPayload":"{\"scope\":\"tenant\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("override tool request should return response");
    assert_eq!(allowed_tool_response.status(), StatusCode::OK);
    let allowed_tool_json = json_body(allowed_tool_response).await;
    assert_eq!(allowed_tool_json["state"], "requested");

    let audit_export = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("authorization", PRIVILEGED_BEARER)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("audit export should return response");
    assert_eq!(audit_export.status(), StatusCode::OK);
    let audit_json = json_body(audit_export).await;
    let items = audit_json["items"]
        .as_array()
        .expect("audit items should be array");
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.guardrail_denied")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.operator_override_applied")
    );
    assert!(
        items
            .iter()
            .any(|item| item["action"] == "automation.agent_tool_call_requested")
    );
}

#[tokio::test]
async fn test_local_minimal_profile_treats_duplicate_notification_request_as_idempotent() {
    let app = local_minimal_node::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first notification request should return response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first notification body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first notification body should be valid json");
    assert_eq!(first_json["deliveryStatus"], "applied");
    assert!(
        !first_json["requestKey"]
            .as_str()
            .expect("requestKey should be string")
            .is_empty()
    );
    assert_eq!(
        first_json["proofVersion"],
        "notification.request.delivery-proof.v1"
    );

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "recipientKind":"user",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent notification request should return response");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .expect("second notification body should collect")
        .to_bytes();
    let second_json: serde_json::Value = serde_json::from_slice(&second_body)
        .expect("second notification body should be valid json");
    assert_eq!(second_json["deliveryStatus"], "replayed");
    assert_eq!(second_json["requestKey"], first_json["requestKey"]);
    assert_eq!(second_json["proofVersion"], first_json["proofVersion"]);

    let notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("notifications query should return response");
    assert_eq!(notifications.status(), StatusCode::OK);
    let notifications_body = notifications
        .into_body()
        .collect()
        .await
        .expect("notifications body should collect")
        .to_bytes();
    let notifications_json: serde_json::Value = serde_json::from_slice(&notifications_body)
        .expect("notifications body should be valid json");
    assert_eq!(
        notifications_json["items"]
            .as_array()
            .expect("items should be array")
            .len(),
        1
    );
    assert_eq!(
        notifications_json["items"][0]["notificationId"],
        "ntf_local_idempotent"
    );

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
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
        .expect("audit body should collect")
        .to_bytes();
    let audit_json: serde_json::Value =
        serde_json::from_slice(&audit_body).expect("audit body should be valid json");
    assert_eq!(audit_json["total"], 1);
    assert_eq!(
        audit_json["items"][0]["aggregateId"],
        "ntf_local_idempotent"
    );

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_conflict",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_other",
                        "recipientKind":"user",
                        "title":"Changed message",
                        "body":"different",
                        "payload":"{\"conversationId\":\"c_other\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting notification request should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "notification_conflict");
}
