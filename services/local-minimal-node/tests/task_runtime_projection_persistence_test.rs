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
    std::env::temp_dir().join(format!("craw_chat_task_runtime_recovery_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_restores_task_runtime_projections_after_rebuild() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let notification_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_restart_demo",
                        "sourceEventId":"evt_restart_demo",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "recipientKind":"user",
                        "title":"hello",
                        "body":"world",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("notification request should return response");
    assert_eq!(notification_response.status(), StatusCode::OK);

    let automation_response = app_before
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
                        "executionId":"ae_restart_demo",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_restart_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("automation request should return response");
    assert_eq!(automation_response.status(), StatusCode::OK);

    assert!(
        runtime_dir
            .join("state")
            .join("notification-tasks.json")
            .exists(),
        "managed local-minimal should persist notification task projections"
    );
    assert!(
        runtime_dir
            .join("state")
            .join("automation-executions.json")
            .exists(),
        "managed local-minimal should persist automation execution projections"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let notifications_after_restart = app_after
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
        .expect("notifications query after restart should return response");
    assert_eq!(notifications_after_restart.status(), StatusCode::OK);
    let notifications_body = notifications_after_restart
        .into_body()
        .collect()
        .await
        .expect("notifications body should collect")
        .to_bytes();
    let notifications_json: serde_json::Value = serde_json::from_slice(&notifications_body)
        .expect("notifications body should be valid json");
    let items = notifications_json["items"]
        .as_array()
        .expect("items should be array");
    assert!(
        items
            .iter()
            .any(|item| item["notificationId"] == "ntf_restart_demo")
    );
    assert!(
        items
            .iter()
            .any(|item| item["notificationId"] == "ntf_automation_user_ae_restart_demo")
    );

    let automation_after_restart = app_after
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/executions/ae_restart_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("x-permissions", "automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("automation query after restart should return response");
    assert_eq!(automation_after_restart.status(), StatusCode::OK);
    let automation_body = automation_after_restart
        .into_body()
        .collect()
        .await
        .expect("automation body should collect")
        .to_bytes();
    let automation_json: serde_json::Value =
        serde_json::from_slice(&automation_body).expect("automation body should be valid json");
    assert_eq!(automation_json["executionId"], "ae_restart_demo");
    assert_eq!(automation_json["state"], "succeeded");

    let _ = fs::remove_dir_all(runtime_dir);
}
