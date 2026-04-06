use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_request_and_get_execution_over_http() {
    let app = automation_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_demo",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request execution should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create body should be valid json");
    assert_eq!(create_json["executionId"], "ae_http_demo");
    assert_eq!(create_json["state"], "succeeded");

    let get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/automation/executions/ae_http_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get execution should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get body should be valid json");
    assert_eq!(get_json["targetRef"], "wf_http_demo");
    assert_eq!(get_json["triggerType"], "webhook.manual");
}

#[tokio::test]
async fn test_duplicate_execution_id_is_idempotent_and_conflicting_retry_is_rejected_over_http() {
    let app = automation_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first execution request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");

    let idempotent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_demo",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent retry should return response");
    assert_eq!(idempotent_response.status(), StatusCode::OK);
    let idempotent_body = idempotent_response
        .into_body()
        .collect()
        .await
        .expect("idempotent body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent body should be valid json");
    assert_eq!(idempotent_json, first_json);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/automation/executions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "automation.execute automation.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "executionId":"ae_http_idempotent",
                        "triggerType":"webhook.manual",
                        "targetKind":"workflow",
                        "targetRef":"wf_http_other",
                        "inputPayload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry should return response");
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
