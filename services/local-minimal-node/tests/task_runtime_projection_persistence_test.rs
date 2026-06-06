use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_task_runtime_recovery_{unique}"))
}

#[tokio::test]
async fn test_default_local_minimal_profile_initializes_task_runtime_state_without_local_appbase_routes()
 {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    for state_file in ["notification-tasks.json", "automation-executions.json"] {
        assert!(
            runtime_dir.join("state").join(state_file).exists(),
            "managed local-minimal runtime should still initialize task state file {state_file}"
        );
    }

    for (method, path, body) in [
        (
            "POST",
            "/app/v3/api/notifications/requests",
            Body::from(
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
            ),
        ),
        ("GET", "/app/v3/api/notifications", Body::empty()),
        (
            "POST",
            "/app/v3/api/automation/executions",
            Body::from(
                r#"{
                    "executionId":"ae_restart_demo",
                    "triggerType":"webhook.manual",
                    "targetKind":"workflow",
                    "targetRef":"wf_restart_demo",
                    "inputPayload":"{\"conversationId\":\"c_demo\"}"
                }"#,
            ),
        ),
        (
            "GET",
            "/app/v3/api/automation/executions/ae_restart_demo",
            Body::empty(),
        ),
    ] {
        let mut builder = Request::builder()
            .method(method)
            .uri(path)
            .header("x-sdkwork-tenant-id", "t_demo")
            .header("x-sdkwork-user-id", "u_demo")
            .header("x-sdkwork-actor-kind", "user")
            .header(
                "x-sdkwork-permission-scope",
                "notification.write automation.execute automation.read",
            );
        if method == "POST" {
            builder = builder.header("content-type", "application/json");
        }

        let response = app
            .clone()
            .oneshot(builder.body(body).unwrap())
            .await
            .expect("appbase-owned local app route should return response");
        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{method} {path} must not be mounted by local-minimal-node"
        );
    }

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let diagnostics = app_after
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_ops")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-permission-scope", "ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("backend diagnostics should return response after rebuild");
    assert_eq!(diagnostics.status(), StatusCode::OK);

    let _ = fs::remove_dir_all(runtime_dir);
}
