use axum::body::Body;
use axum::http::request::Builder as RequestBuilder;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body should be valid json")
}

fn assert_state_json_eq(actual: &serde_json::Value, expected: &str) {
    let actual_json: serde_json::Value = serde_json::from_str(
        actual
            .as_str()
            .expect("device twin state should be serialized as json string"),
    )
    .expect("device twin state should be parseable json");
    let expected_json: serde_json::Value =
        serde_json::from_str(expected).expect("expected twin state should be valid json");
    assert_eq!(actual_json, expected_json);
}

fn device_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "device")
        .header("x-sdkwork-device-id", "d_sensor")
        .header("x-sdkwork-session-id", "s_sensor")
}

fn owner_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "user")
        .header("x-sdkwork-device-id", "d_console")
        .header("x-sdkwork-session-id", "s_console")
}

#[tokio::test]
async fn test_local_minimal_profile_device_twin_mainline_supports_desired_and_reported_state() {
    let app = local_minimal_node::build_default_app();

    let register_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let desired_response = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/d_sensor/twin/desired"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "desiredStateJson":"{\"targetTemperature\":22,\"mode\":\"cool\"}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("desired twin write should return response");
    assert_eq!(desired_response.status(), StatusCode::OK);
    let desired_json = json_body(desired_response).await;
    assert_eq!(desired_json["tenantId"], "t_demo");
    assert_eq!(desired_json["deviceId"], "d_sensor");
    assert_state_json_eq(
        &desired_json["desiredStateJson"],
        "{\"targetTemperature\":22,\"mode\":\"cool\"}",
    );
    assert_eq!(desired_json["reportedStateJson"], "{}");

    let reported_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/d_sensor/twin/reported"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "reportedStateJson":"{\"temperature\":21.5,\"online\":true}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("reported twin write should return response");
    assert_eq!(reported_response.status(), StatusCode::OK);
    let reported_json = json_body(reported_response).await;
    assert_state_json_eq(
        &reported_json["desiredStateJson"],
        "{\"targetTemperature\":22,\"mode\":\"cool\"}",
    );
    assert_state_json_eq(
        &reported_json["reportedStateJson"],
        "{\"temperature\":21.5,\"online\":true}",
    );

    let get_response = app
        .oneshot(
            owner_actor(Request::builder().uri("/im/v3/api/devices/d_sensor/twin"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("twin get should return response");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_json = json_body(get_response).await;
    assert_state_json_eq(
        &get_json["desiredStateJson"],
        "{\"targetTemperature\":22,\"mode\":\"cool\"}",
    );
    assert_state_json_eq(
        &get_json["reportedStateJson"],
        "{\"temperature\":21.5,\"online\":true}",
    );
    assert!(
        get_json["updatedAt"]
            .as_str()
            .expect("updatedAt should be string")
            .contains('T')
    );
}
