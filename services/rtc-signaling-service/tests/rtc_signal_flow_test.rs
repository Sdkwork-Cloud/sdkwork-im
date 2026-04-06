use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use std::thread::sleep;
use std::time::Duration;
use tower::ServiceExt;

#[tokio::test]
async fn test_invite_accept_and_end_rtc_session_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_flow",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let invite_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_flow/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_rtc_demo"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite request should succeed");
    assert_eq!(invite_response.status(), StatusCode::OK);
    let invite_body = invite_response
        .into_body()
        .collect()
        .await
        .expect("invite body should collect")
        .to_bytes();
    let invite_json: serde_json::Value =
        serde_json::from_slice(&invite_body).expect("invite should be valid json");
    assert_eq!(invite_json["state"], "started");
    assert_eq!(invite_json["signalingStreamId"], "st_rtc_demo");

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_flow/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept request should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);
    let accept_body = accept_response
        .into_body()
        .collect()
        .await
        .expect("accept body should collect")
        .to_bytes();
    let accept_json: serde_json::Value =
        serde_json::from_slice(&accept_body).expect("accept should be valid json");
    assert_eq!(accept_json["state"], "accepted");
    assert_eq!(accept_json["artifactMessageId"], "msg_accept");

    let end_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_flow/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end request should succeed");
    assert_eq!(end_response.status(), StatusCode::OK);
    let end_body = end_response
        .into_body()
        .collect()
        .await
        .expect("end body should collect")
        .to_bytes();
    let end_json: serde_json::Value =
        serde_json::from_slice(&end_body).expect("end should be valid json");
    assert_eq!(end_json["state"], "ended");
    assert_eq!(end_json["artifactMessageId"], "msg_end");
}

#[tokio::test]
async fn test_reject_rtc_session_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_reject",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let reject_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_reject/reject")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_reject"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("reject request should succeed");
    assert_eq!(reject_response.status(), StatusCode::OK);
    let reject_body = reject_response
        .into_body()
        .collect()
        .await
        .expect("reject body should collect")
        .to_bytes();
    let reject_json: serde_json::Value =
        serde_json::from_slice(&reject_body).expect("reject should be valid json");
    assert_eq!(reject_json["state"], "rejected");
    assert_eq!(reject_json["artifactMessageId"], "msg_reject");
}

#[tokio::test]
async fn test_post_rtc_signal_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_signal_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let invite_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_http/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_signal_http"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite request should succeed");
    assert_eq!(invite_response.status(), StatusCode::OK);

    let signal_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_signal_http/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("signal request should succeed");
    assert_eq!(signal_response.status(), StatusCode::OK);
    let signal_body = signal_response
        .into_body()
        .collect()
        .await
        .expect("signal body should collect")
        .to_bytes();
    let signal_json: serde_json::Value =
        serde_json::from_slice(&signal_body).expect("signal should be valid json");
    assert_eq!(signal_json["rtcSessionId"], "rtc_signal_http");
    assert_eq!(signal_json["signalType"], "rtc.offer");
    assert_eq!(signal_json["schemaRef"], "webrtc.offer.v1");
    assert_eq!(signal_json["sender"]["id"], "u_demo");
    assert_eq!(signal_json["signalingStreamId"], "st_signal_http");
}

#[tokio::test]
async fn test_rtc_runtime_timestamps_advance_between_session_and_signal_mutations() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_timestamps",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create should be valid json");
    let started_at = create_json["startedAt"]
        .as_str()
        .expect("startedAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let signal_first = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_timestamps/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"offer\"}",
                        "signalingStreamId":"st_rtc_timestamps"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first signal should succeed");
    assert_eq!(signal_first.status(), StatusCode::OK);
    let signal_first_body = signal_first
        .into_body()
        .collect()
        .await
        .expect("first signal body should collect")
        .to_bytes();
    let signal_first_json: serde_json::Value =
        serde_json::from_slice(&signal_first_body).expect("first signal should be valid json");
    let first_occurred_at = signal_first_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let signal_second = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_timestamps/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.answer",
                        "schemaRef":"webrtc.answer.v1",
                        "payload":"{\"sdp\":\"answer\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second signal should succeed");
    assert_eq!(signal_second.status(), StatusCode::OK);
    let signal_second_body = signal_second
        .into_body()
        .collect()
        .await
        .expect("second signal body should collect")
        .to_bytes();
    let signal_second_json: serde_json::Value =
        serde_json::from_slice(&signal_second_body).expect("second signal should be valid json");
    let second_occurred_at = signal_second_json["occurredAt"]
        .as_str()
        .expect("occurredAt should be present")
        .to_owned();

    sleep(Duration::from_millis(20));

    let end_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_timestamps/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_end_timestamps"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end request should succeed");
    assert_eq!(end_response.status(), StatusCode::OK);
    let end_body = end_response
        .into_body()
        .collect()
        .await
        .expect("end body should collect")
        .to_bytes();
    let end_json: serde_json::Value =
        serde_json::from_slice(&end_body).expect("end should be valid json");
    let ended_at = end_json["endedAt"]
        .as_str()
        .expect("endedAt should be present")
        .to_owned();

    assert!(started_at < first_occurred_at);
    assert!(first_occurred_at < second_occurred_at);
    assert!(second_occurred_at < ended_at);
}
