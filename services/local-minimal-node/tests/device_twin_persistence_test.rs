use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::request::Builder as RequestBuilder;
use axum::http::{Request, StatusCode};
use tower::ServiceExt;

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("craw_chat_device_twin_boundary_{unique}"))
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
async fn test_default_local_minimal_profile_does_not_persist_appbase_device_twin_state() {
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    let response = app
        .oneshot(
            owner_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/d_sensor/twin/desired"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{"desiredStateJson":"{\"targetTemperature\":22}"}"#,
            ))
            .unwrap(),
        )
        .await
        .expect("device twin boundary request should return response");
    assert_eq!(response.status(), StatusCode::NOT_FOUND);

    let twin_state_file = runtime_dir.join("state").join("device-twin-state.json");
    assert!(
        !twin_state_file.exists(),
        "local-minimal-node must not create appbase-owned device twin state files"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
