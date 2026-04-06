use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

const DEMO_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8ifQ.";
const AUTOMATION_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X2RlbW8iLCJzaWQiOiJzX2RlbW8iLCJwZXJtaXNzaW9ucyI6WyJhdXRvbWF0aW9uLmV4ZWN1dGUiLCJhdXRvbWF0aW9uLnJlYWQiXX0.";
const PRIVILEGED_BEARER: &str = "Bearer eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0.eyJ0ZW5hbnRfaWQiOiJ0X2RlbW8iLCJzdWIiOiJ1X29wc19hdWRpdF9kZW1vIiwic2lkIjoic19vcHNfYXVkaXRfZGVtbyIsInBlcm1pc3Npb25zIjpbImF1ZGl0LnJlYWQiLCJvcHMucmVhZCJdfQ.";

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

    let notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
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
        "ntf_automation_ae_local_idempotent"
    );

    let audit_export = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
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
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
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

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-permissions", "audit.read")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
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

    let notifications = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
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
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_local_idempotent",
                        "sourceEventId":"evt_local_conflict",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_other",
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
