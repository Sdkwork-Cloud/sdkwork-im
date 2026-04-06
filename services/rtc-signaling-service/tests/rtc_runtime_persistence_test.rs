use std::sync::Arc;

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_adapters_local_memory::MemoryRtcStateStore;
use tower::ServiceExt;

#[tokio::test]
async fn test_runtime_restores_rtc_state_on_rebuild_with_shared_store() {
    let rtc_store = Arc::new(MemoryRtcStateStore::default());
    let app_before = rtc_signaling_service::build_app(Arc::new(
        rtc_signaling_service::RtcRuntime::with_store(rtc_store.clone()),
    ));

    let create_response = app_before
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
                        "rtcSessionId":"rtc_rebuild",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let invite_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_rtc_rebuild"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("invite should succeed");
    assert_eq!(invite_response.status(), StatusCode::OK);

    let offer_response = app_before
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalType":"rtc.offer",
                        "schemaRef":"webrtc.offer.v1",
                        "payload":"{\"sdp\":\"offer\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("offer should succeed");
    assert_eq!(offer_response.status(), StatusCode::OK);

    let app_after = rtc_signaling_service::build_app(Arc::new(
        rtc_signaling_service::RtcRuntime::with_store(rtc_store),
    ));

    let accept_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("x-device-id", "d_peer")
                .header("x-session-id", "s_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_rebuild_accept"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept after rebuild should return response");
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
    assert_eq!(accept_json["signalingStreamId"], "st_rtc_rebuild");
    assert_eq!(accept_json["artifactMessageId"], "msg_rtc_rebuild_accept");

    let answer_response = app_after
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/signals")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("x-device-id", "d_peer")
                .header("x-session-id", "s_peer")
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
        .expect("answer after rebuild should return response");
    assert_eq!(answer_response.status(), StatusCode::OK);
    let answer_body = answer_response
        .into_body()
        .collect()
        .await
        .expect("answer body should collect")
        .to_bytes();
    let answer_json: serde_json::Value =
        serde_json::from_slice(&answer_body).expect("answer should be valid json");
    assert_eq!(answer_json["signalType"], "rtc.answer");
    assert_eq!(answer_json["signalingStreamId"], "st_rtc_rebuild");

    let end_response = app_after
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_rebuild/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-device-id", "d_demo")
                .header("x-session-id", "s_demo_new")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_rtc_rebuild_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("end after rebuild should return response");
    assert_eq!(end_response.status(), StatusCode::OK);
}
