use im_app_context::DualTokenRequestBuilderExt;
use axum::body::Body;
use axum::http::{Request, StatusCode, header::CONTENT_TYPE};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_healthz_returns_ok_and_service_metadata() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/healthz")
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

    assert_eq!(value["status"], "ok");
    assert_eq!(value["service"], "control-plane-api");
}

#[tokio::test]
async fn test_public_app_exports_live_openapi_json() {
    let app = control_plane_api::build_public_app();

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

    assert_eq!(value["openapi"], "3.1.2");
    assert_eq!(value["info"]["title"], "Control Plane API");
    assert!(value["paths"]["/backend/v3/api/control/protocol_registry"].is_object());
}

#[tokio::test]
async fn test_public_app_serves_docs_page_for_live_openapi() {
    let app = control_plane_api::build_public_app();

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
    assert!(html.contains("Control Plane API"));
    assert!(html.contains("/openapi.json"));
}

#[tokio::test]
async fn test_delivered_shared_channel_sync_inventory_route_returns_snapshot() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/runtime/delivered_shared_channel_sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_control_reader")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
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
    assert_eq!(value["status"], "snapshot");
    assert_eq!(value["deliveredCount"], 0);
    assert_eq!(
        value["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );
}

#[tokio::test]
async fn test_delivery_state_shared_channel_sync_inventory_route_returns_snapshot() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/runtime/delivery_state_shared_channel_sync")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_control_reader")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
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
    assert_eq!(value["status"], "snapshot");
    assert_eq!(value["deliveredCount"], 0);
    assert_eq!(value["pendingCount"], 0);
    assert_eq!(value["deadLetterCount"], 0);
    assert_eq!(value["totalCount"], 0);
    assert_eq!(
        value["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );
}

#[tokio::test]
async fn test_shared_channel_sync_inventory_rejects_invalid_limit() {
    let app = control_plane_api::build_app();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/backend/v3/api/control/social/runtime/pending_shared_channel_sync?limit=0")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_control_reader")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(value["type"], "about:blank");
    assert_eq!(value["title"], "Bad Request");
    assert_eq!(value["status"], 400);
    assert_eq!(value["errorStatus"], "invalid");
    assert_eq!(value["code"], "limit_invalid");
}

#[tokio::test]
async fn test_control_plane_social_friend_request_rejects_oversized_request_id_over_http() {
    let app = control_plane_api::build_app();
    let request_body = serde_json::json!({
        "requestId": "r".repeat(2048),
        "eventId": "evt_oversized_friend_request",
        "requesterUserId": "u_alice",
        "targetUserId": "u_bob",
        "requestedAt": "2026-04-10T10:00:00Z"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/friend_requests")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_admin")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("friend request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["type"], "about:blank");
    assert_eq!(json["title"], "Payload Too Large");
    assert_eq!(json["status"], 413);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("requestId")
    );
}

#[tokio::test]
async fn test_control_plane_external_member_link_rejects_oversized_display_name_over_http() {
    let app = control_plane_api::build_app();
    let request_body = serde_json::json!({
        "linkId": "link_oversized_display_name",
        "eventId": "evt_oversized_display_name",
        "connectionId": "conn_oversized_display_name",
        "localActorId": "u_alice",
        "localActorKind": "user",
        "externalMemberId": "ext_bob",
        "externalDisplayName": "d".repeat(4096),
        "linkedAt": "2026-04-10T10:00:00Z"
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/external_member_links")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_admin")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("external member link request should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["type"], "about:blank");
    assert_eq!(json["title"], "Payload Too Large");
    assert_eq!(json["status"], 413);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("externalDisplayName")
    );
}

#[tokio::test]
async fn test_control_plane_targeted_pending_claim_rejects_oversized_request_key_over_http() {
    let app = control_plane_api::build_app();
    let request_body = serde_json::json!({
        "requestKeys": ["k".repeat(4096)]
    })
    .to_string();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/backend/v3/api/control/social/runtime/claim_pending_shared_channel_sync_targeted")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_admin")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("control.write")
                .header("content-type", "application/json")
                .body(Body::from(request_body))
                .unwrap(),
        )
        .await
        .expect("targeted pending claim should return response");

    assert_eq!(response.status(), StatusCode::PAYLOAD_TOO_LARGE);
    assert_eq!(
        response
            .headers()
            .get(CONTENT_TYPE)
            .and_then(|value| value.to_str().ok()),
        Some("application/problem+json; charset=utf-8")
    );
    let body = response
        .into_body()
        .collect()
        .await
        .expect("body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).expect("body should be valid json");
    assert_eq!(json["type"], "about:blank");
    assert_eq!(json["title"], "Payload Too Large");
    assert_eq!(json["status"], 413);
    assert_eq!(json["errorStatus"], "invalid");
    assert_eq!(json["code"], "payload_too_large");
    assert!(
        json["message"]
            .as_str()
            .expect("message should be present")
            .contains("requestKeys")
    );
}
