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

fn system_actor(builder: RequestBuilder) -> RequestBuilder {
    builder
        .header("x-sdkwork-tenant-id", "t_demo")
        .header("x-sdkwork-user-id", "u_owner")
        .header("x-sdkwork-actor-kind", "system")
        .header("x-sdkwork-device-id", "d_system_console")
        .header("x-sdkwork-session-id", "s_system_console")
}

#[tokio::test]
async fn test_local_minimal_profile_device_telemetry_uses_device_sender_and_requires_read_capability()
 {
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

    let open_response = app
        .clone()
        .oneshot(
            device_actor(Request::builder().method("POST").uri("/im/v3/api/streams"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                    "streamId":"st_device_telemetry",
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
        .expect("open telemetry stream should return response");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/streams/st_device_telemetry/frames"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "frameSeq":1,
                    "frameType":"telemetry",
                    "schemaRef":"cc.device.telemetry.v1",
                    "encoding":"json",
                    "payload":"{\"temperature\":21.5}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("append telemetry frame should return response");
    assert_eq!(append_response.status(), StatusCode::OK);
    let append_json = json_body(append_response).await;
    assert_eq!(append_json["sender"]["id"], "d_sensor");
    assert_eq!(append_json["sender"]["kind"], "device");
    assert_eq!(append_json["sender"]["deviceId"], "d_sensor");
    assert_eq!(append_json["sender"]["sessionId"], "s_sensor");

    let forbidden_list = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .uri("/im/v3/api/streams/st_device_telemetry/frames?afterFrameSeq=0&limit=10"),
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("forbidden telemetry list should return response");
    assert_eq!(forbidden_list.status(), StatusCode::FORBIDDEN);
    let forbidden_json = json_body(forbidden_list).await;
    assert_eq!(forbidden_json["code"], "device_permission_denied");

    let allowed_list = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .uri("/im/v3/api/streams/st_device_telemetry/frames?afterFrameSeq=0&limit=10"),
            )
            .header("x-sdkwork-permission-scope", "device.telemetry.read")
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("allowed telemetry list should return response");
    assert_eq!(allowed_list.status(), StatusCode::OK);
    let allowed_json = json_body(allowed_list).await;
    let items = allowed_json["items"]
        .as_array()
        .expect("telemetry items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["sender"]["id"], "d_sensor");
    assert_eq!(items[0]["scopeKind"], "device");
    assert_eq!(items[0]["scopeId"], "d_sensor");
}

#[tokio::test]
async fn test_local_minimal_profile_device_stream_rejects_same_actor_id_different_actor_kind() {
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

    let open_response = app
        .clone()
        .oneshot(
            device_actor(Request::builder().method("POST").uri("/im/v3/api/streams"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                    "streamId":"st_device_telemetry_kind_guard",
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
        .expect("open telemetry stream should return response");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/streams/st_device_telemetry_kind_guard/frames"),
            )
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "frameSeq":1,
                    "frameType":"telemetry",
                    "schemaRef":"cc.device.telemetry.v1",
                    "encoding":"json",
                    "payload":"{\"temperature\":22.0}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("append telemetry frame should return response");
    assert_eq!(append_response.status(), StatusCode::OK);

    let forbidden_list = app
        .oneshot(
            system_actor(Request::builder().uri(
                "/im/v3/api/streams/st_device_telemetry_kind_guard/frames?afterFrameSeq=0&limit=10",
            ))
            .header("x-sdkwork-permission-scope", "device.telemetry.read")
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("cross-kind telemetry list should return response");
    assert_eq!(forbidden_list.status(), StatusCode::FORBIDDEN);
    let forbidden_json = json_body(forbidden_list).await;
    assert_eq!(forbidden_json["code"], "device_permission_denied");
}

#[tokio::test]
async fn test_local_minimal_profile_device_command_requires_send_capability_and_is_readable_by_device()
 {
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

    let forbidden_open = app
        .clone()
        .oneshot(
            owner_actor(Request::builder().method("POST").uri("/im/v3/api/streams"))
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                    "streamId":"st_device_command",
                    "streamType":"device.command",
                    "scopeKind":"device",
                    "scopeId":"d_sensor",
                    "durabilityClass":"durableSession",
                    "schemaRef":"cc.device.command.v1"
                }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("forbidden command open should return response");
    assert_eq!(forbidden_open.status(), StatusCode::FORBIDDEN);
    let forbidden_json = json_body(forbidden_open).await;
    assert_eq!(forbidden_json["code"], "device_permission_denied");

    let allowed_open = app
        .clone()
        .oneshot(
            owner_actor(Request::builder().method("POST").uri("/im/v3/api/streams"))
                .header("x-sdkwork-permission-scope", "device.command.send")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                    "streamId":"st_device_command",
                    "streamType":"device.command",
                    "scopeKind":"device",
                    "scopeId":"d_sensor",
                    "durabilityClass":"durableSession",
                    "schemaRef":"cc.device.command.v1"
                }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("allowed command open should return response");
    assert_eq!(allowed_open.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            owner_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/streams/st_device_command/frames"),
            )
            .header("x-sdkwork-permission-scope", "device.command.send")
            .header("content-type", "application/json")
            .body(Body::from(
                r#"{
                    "frameSeq":1,
                    "frameType":"command",
                    "schemaRef":"cc.device.command.v1",
                    "encoding":"json",
                    "payload":"{\"command\":\"lock\"}"
                }"#,
            ))
            .unwrap(),
        )
        .await
        .expect("append command frame should return response");
    assert_eq!(append_response.status(), StatusCode::OK);
    let append_json = json_body(append_response).await;
    assert_eq!(append_json["sender"]["id"], "u_owner");
    assert_eq!(append_json["sender"]["kind"], "user");

    let device_list = app
        .clone()
        .oneshot(
            device_actor(
                Request::builder()
                    .uri("/im/v3/api/streams/st_device_command/frames?afterFrameSeq=0&limit=10"),
            )
            .body(Body::empty())
            .unwrap(),
        )
        .await
        .expect("device command list should return response");
    assert_eq!(device_list.status(), StatusCode::OK);
    let device_list_json = json_body(device_list).await;
    let items = device_list_json["items"]
        .as_array()
        .expect("command items should be array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["frameType"], "command");
    assert_eq!(items[0]["scopeKind"], "device");
    assert_eq!(items[0]["scopeId"], "d_sensor");
    assert_eq!(items[0]["sender"]["id"], "u_owner");
}

#[tokio::test]
async fn test_local_minimal_profile_user_actor_cannot_inherit_system_device_command_access() {
    let app = local_minimal_node::build_default_app();

    let register_response = app
        .clone()
        .oneshot(
            system_actor(
                Request::builder()
                    .method("POST")
                    .uri("/im/v3/api/devices/register"),
            )
            .header("content-type", "application/json")
            .body(Body::from(r#"{}"#))
            .unwrap(),
        )
        .await
        .expect("system device register should return response");
    assert_eq!(register_response.status(), StatusCode::OK);

    let forbidden_open = app
        .oneshot(
            owner_actor(Request::builder().method("POST").uri("/im/v3/api/streams"))
                .header("x-sdkwork-permission-scope", "device.command.send")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                    "streamId":"st_system_device_command_guard",
                    "streamType":"device.command",
                    "scopeKind":"device",
                    "scopeId":"d_system_console",
                    "durabilityClass":"durableSession",
                    "schemaRef":"cc.device.command.v1"
                }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("cross-kind command open should return response");
    assert_eq!(forbidden_open.status(), StatusCode::FORBIDDEN);
    let forbidden_json = json_body(forbidden_open).await;
    assert_eq!(forbidden_json["code"], "device_permission_denied");
}
