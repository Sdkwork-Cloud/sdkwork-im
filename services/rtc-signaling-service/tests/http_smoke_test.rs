use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_create_rtc_session_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["rtcSessionId"], "rtc_demo");
    assert_eq!(value["state"], "started");
}

#[tokio::test]
async fn test_standalone_rtc_service_rejects_conversation_binding_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_conversation_binding_rejected",
                        "conversationId":"c_demo",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session request should return response");

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response should be valid json");

    assert_eq!(value["code"], "conversation_gateway_required");
}

#[tokio::test]
async fn test_duplicate_rtc_session_create_is_idempotent_and_conflicting_retry_is_rejected() {
    let app = rtc_signaling_service::build_default_app();

    let first_create = app
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
                        "rtcSessionId":"rtc_idempotent",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create session request should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_idempotent/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_idempotent"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept request should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let idempotent_create = app
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
                        "rtcSessionId":"rtc_idempotent",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent create request should return response");
    assert_eq!(idempotent_create.status(), StatusCode::OK);
    let idempotent_body = idempotent_create
        .into_body()
        .collect()
        .await
        .expect("idempotent create body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent create should be valid json");
    assert_eq!(idempotent_json["state"], "accepted");
    assert_eq!(
        idempotent_json["artifactMessageId"],
        "msg_accept_idempotent"
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_idempotent",
                        "rtcMode":"video"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting create request should return response");
    assert_eq!(conflicting_create.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_create
        .into_body()
        .collect()
        .await
        .expect("conflicting create body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting create should be valid json");
    assert_eq!(conflicting_json["code"], "rtc_session_conflict");
}

#[tokio::test]
async fn test_rtc_session_updates_are_idempotent_and_conflicting_state_transitions_are_rejected() {
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
                        "rtcSessionId":"rtc_state_machine",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let first_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first accept should succeed");
    assert_eq!(first_accept.status(), StatusCode::OK);

    let duplicate_accept = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate accept should return response");
    assert_eq!(duplicate_accept.status(), StatusCode::OK);
    let duplicate_accept_body = duplicate_accept
        .into_body()
        .collect()
        .await
        .expect("duplicate accept body should collect")
        .to_bytes();
    let duplicate_accept_json: serde_json::Value = serde_json::from_slice(&duplicate_accept_body)
        .expect("duplicate accept should be valid json");
    assert_eq!(duplicate_accept_json["state"], "accepted");
    assert_eq!(
        duplicate_accept_json["artifactMessageId"],
        "msg_accept_once"
    );

    let conflicting_reject = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/reject")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_reject_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting reject should return response");
    assert_eq!(conflicting_reject.status(), StatusCode::CONFLICT);
    let conflicting_reject_body = conflicting_reject
        .into_body()
        .collect()
        .await
        .expect("conflicting reject body should collect")
        .to_bytes();
    let conflicting_reject_json: serde_json::Value =
        serde_json::from_slice(&conflicting_reject_body)
            .expect("conflicting reject should be valid json");
    assert_eq!(
        conflicting_reject_json["code"],
        "rtc_session_state_conflict"
    );

    let conflicting_accept_retry = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting accept retry should return response");
    assert_eq!(conflicting_accept_retry.status(), StatusCode::CONFLICT);
    let conflicting_accept_retry_body = conflicting_accept_retry
        .into_body()
        .collect()
        .await
        .expect("conflicting accept retry body should collect")
        .to_bytes();
    let conflicting_accept_retry_json: serde_json::Value =
        serde_json::from_slice(&conflicting_accept_retry_body)
            .expect("conflicting accept retry should be valid json");
    assert_eq!(
        conflicting_accept_retry_json["code"],
        "rtc_session_state_conflict"
    );

    let first_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_end_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first end should succeed");
    assert_eq!(first_end.status(), StatusCode::OK);

    let duplicate_end = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/end")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_end_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("duplicate end should return response");
    assert_eq!(duplicate_end.status(), StatusCode::OK);
    let duplicate_end_body = duplicate_end
        .into_body()
        .collect()
        .await
        .expect("duplicate end body should collect")
        .to_bytes();
    let duplicate_end_json: serde_json::Value =
        serde_json::from_slice(&duplicate_end_body).expect("duplicate end should be valid json");
    assert_eq!(duplicate_end_json["state"], "ended");
    assert_eq!(duplicate_end_json["artifactMessageId"], "msg_end_once");

    let accept_after_end = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_after_end"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept after end should return response");
    assert_eq!(accept_after_end.status(), StatusCode::CONFLICT);
    let accept_after_end_body = accept_after_end
        .into_body()
        .collect()
        .await
        .expect("accept after end body should collect")
        .to_bytes();
    let accept_after_end_json: serde_json::Value = serde_json::from_slice(&accept_after_end_body)
        .expect("accept after end should be valid json");
    assert_eq!(accept_after_end_json["code"], "rtc_session_state_conflict");
}

#[tokio::test]
async fn test_invite_after_accept_with_different_signaling_stream_conflicts() {
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
                        "rtcSessionId":"rtc_invite_after_accept",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let first_invite = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_after_accept/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_initial"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first invite should succeed");
    assert_eq!(first_invite.status(), StatusCode::OK);

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_after_accept/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "artifactMessageId":"msg_accept_once"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("accept should succeed");
    assert_eq!(accept_response.status(), StatusCode::OK);

    let conflicting_invite = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_invite_after_accept/invite")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "signalingStreamId":"st_invite_conflict"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting invite should return response");
    assert_eq!(conflicting_invite.status(), StatusCode::CONFLICT);
    let conflicting_invite_body = conflicting_invite
        .into_body()
        .collect()
        .await
        .expect("conflicting invite body should collect")
        .to_bytes();
    let conflicting_invite_json: serde_json::Value =
        serde_json::from_slice(&conflicting_invite_body)
            .expect("conflicting invite should be valid json");
    assert_eq!(
        conflicting_invite_json["code"],
        "rtc_session_state_conflict"
    );
}
