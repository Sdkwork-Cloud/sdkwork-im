use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_device_scoped_streams_require_authorizing_gateway() {
    let app = streaming_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/streams")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_owner")
                .header("x-sdkwork-actor-kind", "device")
                .header("x-sdkwork-device-id", "d_sensor")
                .header("x-sdkwork-session-id", "s_sensor")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_device_scope_direct",
                        "streamType":"device.telemetry",
                        "scopeKind":"device",
                        "scopeId":"d_sensor",
                        "durabilityClass":"durableSession",
                        "schemaRef":"cc.device.telemetry.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("device scope open should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");
    assert_eq!(json["code"], "conversation_gateway_required");
}
