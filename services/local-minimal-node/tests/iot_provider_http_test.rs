use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_profile_gets_iot_access_provider_health_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/iot/access/provider_health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("iot provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("iot provider health body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("iot provider health response should be valid json");

    assert_eq!(json["pluginId"], "iot-access-local");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "local");
    assert_eq!(json["details"]["assignedProtocols"], "mqtt,xiaozhi");
}

#[tokio::test]
async fn test_local_minimal_profile_gets_iot_protocol_provider_health_over_http() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/iot/protocol/provider_health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("iot protocol provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("iot protocol provider health body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("iot protocol provider health response should be valid json");

    assert_eq!(json["pluginId"], "iot-mqtt");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "mqtt");
    assert_eq!(json["details"]["protocolKey"], "mqtt");
}
