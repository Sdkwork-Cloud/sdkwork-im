use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::request::Builder as RequestBuilder;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_device_twin_runtime_recovery_{unique}"))
}

async fn json_body(response: axum::response::Response) -> serde_json::Value {
    let bytes = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    serde_json::from_slice(&bytes).expect("response body should be valid json")
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
async fn test_default_local_minimal_profile_restores_device_twin_state_after_rebuild() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app_before = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());

    let register_response = app_before
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

    let desired_response = app_before
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
                    "desiredStateJson":"{\"targetTemperature\":22}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("desired twin write should return response");
    assert_eq!(desired_response.status(), StatusCode::OK);

    let reported_response = app_before
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
                    "reportedStateJson":"{\"temperature\":21.5}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("reported twin write should return response");
    assert_eq!(reported_response.status(), StatusCode::OK);

    let twin_state_file = runtime_dir.join("state").join("device-twin-state.json");
    assert!(
        twin_state_file.exists(),
        "default local-minimal runtime should persist twin state under the runtime state dir"
    );

    let app_after = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let get_response = app_after
        .oneshot(
            owner_actor(Request::builder().uri("/im/v3/api/devices/d_sensor/twin"))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("twin get after restart should return response");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_json = json_body(get_response).await;
    assert_eq!(get_json["tenantId"], "t_demo");
    assert_eq!(get_json["deviceId"], "d_sensor");
    assert_eq!(get_json["desiredStateJson"], "{\"targetTemperature\":22}");
    assert_eq!(get_json["reportedStateJson"], "{\"temperature\":21.5}");

    let _ = fs::remove_dir_all(runtime_dir);
}
