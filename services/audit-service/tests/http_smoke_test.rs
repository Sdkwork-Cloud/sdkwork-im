use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_record_list_and_export_audit_over_http() {
    let app = audit_service::build_default_app();

    let record_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "recordId":"audit_http_demo",
                        "aggregateType":"notification",
                        "aggregateId":"ntf_demo",
                        "action":"notification.requested",
                        "payload":"{\"recipientId\":\"u_target\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("record audit should succeed");
    assert_eq!(record_response.status(), StatusCode::OK);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/records")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list records should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    assert_eq!(list_json["items"][0]["recordId"], "audit_http_demo");

    let export_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/export")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("export should succeed");
    assert_eq!(export_response.status(), StatusCode::OK);
    let export_body = export_response
        .into_body()
        .collect()
        .await
        .expect("export body should collect")
        .to_bytes();
    let export_json: serde_json::Value =
        serde_json::from_slice(&export_body).expect("export body should be valid json");
    assert_eq!(export_json["total"], 1);
    assert_eq!(export_json["items"][0]["action"], "notification.requested");

    let verify_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/audit/verify")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-permissions", "audit.write,audit.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("verify should succeed");
    assert_eq!(verify_response.status(), StatusCode::OK);
    let verify_body = verify_response
        .into_body()
        .collect()
        .await
        .expect("verify body should collect")
        .to_bytes();
    let verify_json: serde_json::Value =
        serde_json::from_slice(&verify_body).expect("verify body should be valid json");
    assert_eq!(verify_json["tenantId"], "t_demo");
    assert_eq!(verify_json["total"], 1);
    assert_eq!(verify_json["chainValid"], true);
    assert!(
        verify_json["chainHeadHash"].as_str().is_some(),
        "verify response should include chain head hash when records exist"
    );
}
