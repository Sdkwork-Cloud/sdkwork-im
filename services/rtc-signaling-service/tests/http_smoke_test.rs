use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = rtc_signaling_service::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(
        value["info"]["title"],
        "Craw Chat RTC Signaling Service API"
    );
    assert!(value["paths"]["/api/v1/rtc/sessions"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = rtc_signaling_service::build_public_app();

    let response = app
        .oneshot(Request::builder().uri("/docs").body(Body::empty()).unwrap())
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let html = String::from_utf8(body.to_vec()).expect("docs should be valid utf-8");

    assert!(html.contains("OpenAPI 3.1"));
    assert!(html.contains("Craw Chat RTC Signaling Service API"));
    assert!(html.contains("/openapi.json"));
}

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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert!(
        !first_create_json["requestKey"]
            .as_str()
            .expect("first create requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        first_create_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

    let accept_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_idempotent/accept")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_peer")
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
    assert_eq!(idempotent_json["deliveryStatus"], "replayed");
    assert_eq!(
        idempotent_json["requestKey"],
        first_create_json["requestKey"]
    );
    assert_eq!(
        idempotent_json["proofVersion"],
        first_create_json["proofVersion"]
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
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
async fn test_duplicate_rtc_session_create_with_same_actor_id_but_different_actor_kind_is_conflict()
{
    let app = rtc_signaling_service::build_default_app();

    let first_create = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "shared_actor")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_kind_scope",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first create session request should succeed");
    assert_eq!(first_create.status(), StatusCode::OK);
    let first_create_body = first_create
        .into_body()
        .collect()
        .await
        .expect("first create body should collect")
        .to_bytes();
    let first_create_json: serde_json::Value =
        serde_json::from_slice(&first_create_body).expect("first create should be valid json");
    assert_eq!(first_create_json["deliveryStatus"], "applied");
    assert_eq!(
        first_create_json["requestKey"],
        "6#t_demo4#user12#shared_actor6#create14#rtc_kind_scope"
    );

    let conflicting_create = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "shared_actor")
                .header("x-actor-kind", "system")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_kind_scope",
                        "rtcMode":"voice"
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
    assert_eq!(duplicate_accept_json["deliveryStatus"], "replayed");
    assert!(
        !duplicate_accept_json["requestKey"]
            .as_str()
            .expect("duplicate accept requestKey should be present")
            .is_empty()
    );
    assert_eq!(
        duplicate_accept_json["proofVersion"],
        "rtc.session.delivery-proof.v1"
    );

    let conflicting_reject = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_state_machine/reject")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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
                .header("x-actor-kind", "user")
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

#[tokio::test]
async fn test_issue_rtc_participant_credential_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_external_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let credential_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions/rtc_external_http/credentials")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "participantId":"u_peer"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("issue rtc credential request should return response");

    assert_eq!(credential_response.status(), StatusCode::OK);
    let credential_body = credential_response
        .into_body()
        .collect()
        .await
        .expect("credential body should collect")
        .to_bytes();
    let credential_json: serde_json::Value =
        serde_json::from_slice(&credential_body).expect("credential response should be valid json");

    assert_eq!(credential_json["tenantId"], "t_demo");
    assert_eq!(credential_json["rtcSessionId"], "rtc_external_http");
    assert_eq!(credential_json["participantId"], "u_peer");
    assert_eq!(
        credential_json["credential"],
        "volcengine-token:t_demo:rtc_external_http:u_peer"
    );
    assert!(credential_json["expiresAt"].as_str().is_some());
}

#[tokio::test]
async fn test_get_rtc_provider_health_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rtc/provider-health")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider health body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider health response should be valid json");

    assert_eq!(json["pluginId"], "rtc-volcengine");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "volcengine");
    assert_eq!(
        json["details"]["accessEndpoint"],
        "wss://rtc.volcengine.local/session"
    );
    assert!(json["checkedAt"].as_str().is_some());
}

#[tokio::test]
async fn test_map_rtc_provider_callback_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_callback_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let callback_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/provider-callbacks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_callback_http",
                        "callbackType":"room-ended",
                        "payloadJson":"{\"reason\":\"host_left\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("rtc provider callback request should return response");

    assert_eq!(callback_response.status(), StatusCode::OK);
    let callback_body = callback_response
        .into_body()
        .collect()
        .await
        .expect("callback body should collect")
        .to_bytes();
    let callback_json: serde_json::Value =
        serde_json::from_slice(&callback_body).expect("callback response should be valid json");

    assert_eq!(callback_json["rtcSessionId"], "rtc_callback_http");
    assert_eq!(callback_json["eventType"], "room-ended");
    assert_eq!(callback_json["participantId"], serde_json::Value::Null);
    assert_eq!(callback_json["payloadJson"], "{\"reason\":\"host_left\"}");
}

#[tokio::test]
async fn test_map_rtc_provider_callback_rejects_oversized_payload_json_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_callback_oversized_payload",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let callback_body = serde_json::json!({
        "rtcSessionId":"rtc_callback_oversized_payload",
        "callbackType":"room-ended",
        "payloadJson":"x".repeat(262145)
    })
    .to_string();
    let callback_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/provider-callbacks")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(callback_body))
                .unwrap(),
        )
        .await
        .expect("oversized callback request should return response");

    assert_eq!(callback_response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    let callback_body = callback_response
        .into_body()
        .collect()
        .await
        .expect("callback rejection body should collect")
        .to_bytes();
    let callback_json: serde_json::Value =
        serde_json::from_slice(&callback_body).expect("callback rejection should be valid json");
    assert_eq!(callback_json["code"], "payload_too_large");
    assert!(
        callback_json["message"]
            .as_str()
            .expect("callback rejection message should be a string")
            .contains("payloadJson"),
        "error should point to payloadJson guard, got: {callback_json:?}"
    );
}

#[tokio::test]
async fn test_get_rtc_recording_artifact_over_http() {
    let app = rtc_signaling_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "rtcSessionId":"rtc_recording_http",
                        "rtcMode":"voice"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create rtc session should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);

    let artifact_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/rtc/sessions/rtc_recording_http/artifacts/recording")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("recording artifact request should return response");

    assert_eq!(artifact_response.status(), StatusCode::OK);
    let artifact_body = artifact_response
        .into_body()
        .collect()
        .await
        .expect("recording artifact body should collect")
        .to_bytes();
    let artifact_json: serde_json::Value = serde_json::from_slice(&artifact_body)
        .expect("recording artifact response should be valid json");

    assert_eq!(artifact_json["tenantId"], "t_demo");
    assert_eq!(artifact_json["rtcSessionId"], "rtc_recording_http");
    assert_eq!(artifact_json["bucket"], "rtc-artifacts");
    assert_eq!(
        artifact_json["objectKey"],
        "recordings/t_demo/rtc_recording_http.mp4"
    );
    assert_eq!(
        artifact_json["storageProvider"],
        "object-storage-volcengine"
    );
    assert_eq!(
        artifact_json["playbackUrl"],
        "https://tos.volcengine.local/rtc-artifacts/recordings/t_demo/rtc_recording_http.mp4?provider=object-storage-volcengine&expires=3600"
    );
}

#[tokio::test]
async fn test_create_rtc_session_rejects_oversized_session_id_over_http() {
    let app = rtc_signaling_service::build_default_app();
    let oversized_session_id = "r".repeat(257);
    let request_body = serde_json::json!({
        "rtcSessionId": oversized_session_id,
        "rtcMode":"voice"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/rtc/sessions")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-actor-kind", "user")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("oversized session id create request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
}
