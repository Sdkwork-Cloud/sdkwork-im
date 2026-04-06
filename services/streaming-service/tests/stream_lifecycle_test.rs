use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryStreamStateStore;
use std::sync::Arc;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_stream_checkpoint_and_complete_over_http() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_lifecycle",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let checkpoint_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_lifecycle/checkpoint")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("checkpoint request should succeed");
    assert_eq!(checkpoint_response.status(), StatusCode::OK);
    let checkpoint_body = checkpoint_response
        .into_body()
        .collect()
        .await
        .expect("checkpoint body should collect")
        .to_bytes();
    let checkpoint_json: serde_json::Value =
        serde_json::from_slice(&checkpoint_body).expect("checkpoint should be valid json");
    assert_eq!(checkpoint_json["state"], "checkpointed");
    assert_eq!(checkpoint_json["lastCheckpointSeq"], 3);

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_lifecycle/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 5,
                        "resultMessageId": "msg_demo_5"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete should be valid json");
    assert_eq!(complete_json["state"], "completed");
    assert_eq!(complete_json["lastFrameSeq"], 5);
    assert_eq!(complete_json["resultMessageId"], "msg_demo_5");
}

#[tokio::test]
async fn test_stream_abort_over_http_closes_stream_without_result_message() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_abort",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let abort_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_abort/abort")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "reason": "client_cancelled"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("abort request should succeed");
    assert_eq!(abort_response.status(), StatusCode::OK);
    let abort_body = abort_response
        .into_body()
        .collect()
        .await
        .expect("abort body should collect")
        .to_bytes();
    let abort_json: serde_json::Value =
        serde_json::from_slice(&abort_body).expect("abort should be valid json");
    assert_eq!(abort_json["state"], "aborted");
    assert_eq!(abort_json["lastFrameSeq"], 2);
    assert_eq!(abort_json["resultMessageId"], serde_json::Value::Null);
    assert!(abort_json["closedAt"].is_string());

    let complete_after_abort = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_abort/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3,
                        "resultMessageId": "msg_demo_3"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete after abort request should succeed");
    assert_eq!(complete_after_abort.status(), StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_stream_append_and_list_frames_over_http() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_frames",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_frames/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hel\"}",
                        "attributes": {
                            "topic": "llm"
                        }
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);
    let append_body = append_response
        .into_body()
        .collect()
        .await
        .expect("append body should collect")
        .to_bytes();
    let append_json: serde_json::Value =
        serde_json::from_slice(&append_body).expect("append response should be valid json");
    assert_eq!(append_json["frameSeq"], 1);
    assert_eq!(append_json["frameType"], "delta");
    assert_eq!(append_json["sender"]["id"], "u_demo");
    assert_eq!(append_json["attributes"]["topic"], "llm");

    let second_append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_frames/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"lo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append request should succeed");
    assert_eq!(second_append_response.status(), StatusCode::OK);

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_frames/frames?afterFrameSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    assert_eq!(list_json["items"].as_array().unwrap().len(), 2);
    assert_eq!(list_json["items"][0]["frameSeq"], 1);
    assert_eq!(list_json["items"][1]["frameSeq"], 2);
    assert_eq!(list_json["nextAfterFrameSeq"], 2);
    assert_eq!(list_json["hasMore"], false);
}

#[tokio::test]
async fn test_stream_runtime_timestamps_advance_between_distinct_mutations() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_timestamps",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);
    let open_body = open_response
        .into_body()
        .collect()
        .await
        .expect("open body should collect")
        .to_bytes();
    let open_json: serde_json::Value =
        serde_json::from_slice(&open_body).expect("open response should be valid json");
    let opened_at = open_json["openedAt"]
        .as_str()
        .expect("openedAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let first_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_timestamps/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first append request should succeed");
    assert_eq!(first_append.status(), StatusCode::OK);
    let first_append_body = first_append
        .into_body()
        .collect()
        .await
        .expect("first append body should collect")
        .to_bytes();
    let first_append_json: serde_json::Value =
        serde_json::from_slice(&first_append_body).expect("first append should be valid json");
    let first_occurred_at = first_append_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let second_append = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_timestamps/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"world\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second append request should succeed");
    assert_eq!(second_append.status(), StatusCode::OK);
    let second_append_body = second_append
        .into_body()
        .collect()
        .await
        .expect("second append body should collect")
        .to_bytes();
    let second_append_json: serde_json::Value =
        serde_json::from_slice(&second_append_body).expect("second append should be valid json");
    let second_occurred_at = second_append_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let complete_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_timestamps/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "resultMessageId": "msg_complete_timestamps"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
    let complete_body = complete_response
        .into_body()
        .collect()
        .await
        .expect("complete body should collect")
        .to_bytes();
    let complete_json: serde_json::Value =
        serde_json::from_slice(&complete_body).expect("complete should be valid json");
    let closed_at = complete_json["closedAt"]
        .as_str()
        .expect("closedAt should be present")
        .to_owned();

    assert!(opened_at < first_occurred_at);
    assert!(first_occurred_at < second_occurred_at);
    assert!(second_occurred_at < closed_at);
}

#[tokio::test]
async fn test_stream_append_enforces_ordering_and_idempotent_retry_rules() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_rules",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rules/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append first frame request should succeed");
    assert_eq!(append_first.status(), StatusCode::OK);

    let idempotent_retry = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rules/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent retry request should succeed");
    assert_eq!(idempotent_retry.status(), StatusCode::OK);

    let out_of_order = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rules/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 3,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"skip\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("out of order request should return response");
    assert_eq!(out_of_order.status(), StatusCode::BAD_REQUEST);
    let out_of_order_body = out_of_order
        .into_body()
        .collect()
        .await
        .expect("out of order body should collect")
        .to_bytes();
    let out_of_order_json: serde_json::Value =
        serde_json::from_slice(&out_of_order_body).expect("out of order should be valid json");
    assert_eq!(out_of_order_json["code"], "stream_frame_out_of_order");

    let conflicting_retry = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rules/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"changed\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting retry request should return response");
    assert_eq!(conflicting_retry.status(), StatusCode::CONFLICT);
    let conflicting_retry_body = conflicting_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting retry body should collect")
        .to_bytes();
    let conflicting_retry_json: serde_json::Value = serde_json::from_slice(&conflicting_retry_body)
        .expect("conflicting retry should be valid json");
    assert_eq!(conflicting_retry_json["code"], "stream_frame_conflict");
}

#[tokio::test]
async fn test_stream_append_rejects_closed_stream() {
    let app = streaming_service::build_default_app();

    let open_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_closed",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let complete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_closed/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "resultMessageId": "msg_closed_1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream request should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);

    let append_after_complete = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_closed/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"late\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append after complete request should return response");
    assert_eq!(append_after_complete.status(), StatusCode::BAD_REQUEST);
    let append_after_complete_body = append_after_complete
        .into_body()
        .collect()
        .await
        .expect("append after complete body should collect")
        .to_bytes();
    let append_after_complete_json: serde_json::Value =
        serde_json::from_slice(&append_after_complete_body)
            .expect("append after complete should be valid json");
    assert_eq!(append_after_complete_json["code"], "stream_state_invalid");
}

#[tokio::test]
async fn test_duplicate_open_stream_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = streaming_service::build_default_app();

    let first_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first open stream request should succeed");
    assert_eq!(first_open.status(), StatusCode::OK);

    let append_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_open_idempotent/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);

    let idempotent_open = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent open stream request should succeed");
    assert_eq!(idempotent_open.status(), StatusCode::OK);
    let idempotent_open_body = idempotent_open
        .into_body()
        .collect()
        .await
        .expect("idempotent open body should collect")
        .to_bytes();
    let idempotent_open_json: serde_json::Value = serde_json::from_slice(&idempotent_open_body)
        .expect("idempotent open should be valid json");
    assert_eq!(idempotent_open_json["state"], "active");
    assert_eq!(idempotent_open_json["lastFrameSeq"], 1);

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_open_idempotent/frames?afterFrameSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    assert_eq!(list_json["items"].as_array().unwrap().len(), 1);
    assert_eq!(list_json["items"][0]["frameSeq"], 1);

    let conflicting_open = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_open_idempotent",
                        "streamType":"custom.delta.binary",
                        "scopeKind":"request",
                        "scopeId":"req_other",
                        "durabilityClass":"eventLog",
                        "schemaRef":"custom.delta.binary.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting open stream request should return response");
    assert_eq!(conflicting_open.status(), StatusCode::CONFLICT);
    let conflicting_open_body = conflicting_open
        .into_body()
        .collect()
        .await
        .expect("conflicting open body should collect")
        .to_bytes();
    let conflicting_open_json: serde_json::Value = serde_json::from_slice(&conflicting_open_body)
        .expect("conflicting open should be valid json");
    assert_eq!(conflicting_open_json["code"], "stream_conflict");
}

#[tokio::test]
async fn test_runtime_restores_stream_state_on_rebuild_with_shared_store() {
    let stream_store = Arc::new(MemoryStreamStateStore::default());
    let app_before = streaming_service::build_app(Arc::new(
        streaming_service::StreamingRuntime::with_store(stream_store.clone()),
    ));

    let open_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "streamId":"st_rebuild",
                        "streamType":"custom.delta.text",
                        "scopeKind":"request",
                        "scopeId":"req_demo",
                        "durabilityClass":"durableSession",
                        "schemaRef":"custom.delta.text.v1"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("open stream request should succeed");
    assert_eq!(open_response.status(), StatusCode::OK);

    let append_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rebuild/frames")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 1,
                        "frameType": "delta",
                        "schemaRef": "custom.delta.text.v1",
                        "encoding": "json",
                        "payload": "{\"delta\":\"hello\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("append frame request should succeed");
    assert_eq!(append_response.status(), StatusCode::OK);

    let app_after = streaming_service::build_app(Arc::new(
        streaming_service::StreamingRuntime::with_store(stream_store),
    ));

    let list_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/streams/st_rebuild/frames?afterFrameSeq=0&limit=10")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list frames request after rebuild should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list response should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0]["frameSeq"], 1);

    let complete_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/streams/st_rebuild/complete")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "frameSeq": 2,
                        "resultMessageId": "msg_rebuild_result"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("complete stream request after rebuild should succeed");
    assert_eq!(complete_response.status(), StatusCode::OK);
}
