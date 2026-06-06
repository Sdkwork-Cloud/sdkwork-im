use axum::body::Body;
use axum::http::request::Builder as RequestBuilder;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

fn owner_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "user")
        .header("x-sdkwork-device-id", "d_console")
        .header("x-sdkwork-session-id", "s_console")
}

#[tokio::test]
async fn test_local_minimal_profile_does_not_expose_appbase_device_twin_routes() {
    let app = local_minimal_node::build_default_app();

    for (method, uri, body) in [
        ("GET", "/im/v3/api/devices/d_sensor/twin", ""),
        (
            "POST",
            "/im/v3/api/devices/d_sensor/twin/desired",
            r#"{"desiredStateJson":"{\"targetTemperature\":22}"}"#,
        ),
        (
            "POST",
            "/im/v3/api/devices/d_sensor/twin/reported",
            r#"{"reportedStateJson":"{\"temperature\":21.5}"}"#,
        ),
        ("GET", "/app/v3/api/devices/d_sensor/twin", ""),
        (
            "POST",
            "/app/v3/api/devices/d_sensor/twin/desired",
            r#"{"desiredStateJson":"{\"targetTemperature\":22}"}"#,
        ),
        (
            "POST",
            "/app/v3/api/devices/d_sensor/twin/reported",
            r#"{"reportedStateJson":"{\"temperature\":21.5}"}"#,
        ),
    ] {
        let response = app
            .clone()
            .oneshot(
                owner_actor(Request::builder().method(method).uri(uri))
                    .header("content-type", "application/json")
                    .body(Body::from(body))
                    .unwrap(),
            )
            .await
            .expect("device twin boundary request should return response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "local-minimal-node must not expose appbase-owned device twin route {method} {uri}"
        );
    }
}
