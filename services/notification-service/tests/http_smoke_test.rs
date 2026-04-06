use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tokio::time::{Duration, sleep};
use tower::ServiceExt;

#[tokio::test]
async fn test_request_and_query_notifications_over_http() {
    let app = notification_service::build_default_app();

    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_demo",
                        "sourceEventId":"evt_http_demo",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_target",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("request notification should succeed");
    assert_eq!(create_response.status(), StatusCode::OK);
    let create_body = create_response
        .into_body()
        .collect()
        .await
        .expect("create body should collect")
        .to_bytes();
    let create_json: serde_json::Value =
        serde_json::from_slice(&create_body).expect("create body should be valid json");
    assert_eq!(create_json["notificationId"], "ntf_http_demo");
    assert_eq!(create_json["status"], "dispatched");

    let list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_target")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list notifications should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    assert_eq!(list_json["items"][0]["notificationId"], "ntf_http_demo");

    let get_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications/ntf_http_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_target")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("get notification should succeed");
    assert_eq!(get_response.status(), StatusCode::OK);
    let get_body = get_response
        .into_body()
        .collect()
        .await
        .expect("get body should collect")
        .to_bytes();
    let get_json: serde_json::Value =
        serde_json::from_slice(&get_body).expect("get body should be valid json");
    assert_eq!(get_json["sourceEventType"], "message.posted");
    assert_eq!(get_json["recipientId"], "u_target");

    let non_recipient_list_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("non-recipient list notifications should succeed");
    assert_eq!(non_recipient_list_response.status(), StatusCode::OK);
    let non_recipient_list_body = non_recipient_list_response
        .into_body()
        .collect()
        .await
        .expect("non-recipient list body should collect")
        .to_bytes();
    let non_recipient_list_json: serde_json::Value =
        serde_json::from_slice(&non_recipient_list_body)
            .expect("non-recipient list body should be valid json");
    assert_eq!(
        non_recipient_list_json["items"]
            .as_array()
            .expect("items should be an array")
            .len(),
        0
    );

    let non_recipient_get_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications/ntf_http_demo")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("non-recipient get notification should succeed");
    assert_eq!(non_recipient_get_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_duplicate_notification_id_is_idempotent_and_conflicting_retry_is_rejected_over_http()
{
    let app = notification_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first notification request should return response");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");

    let idempotent_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_idempotent",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"New message",
                        "body":"hello",
                        "payload":"{\"conversationId\":\"c_demo\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("idempotent notification request should return response");
    assert_eq!(idempotent_response.status(), StatusCode::OK);
    let idempotent_body = idempotent_response
        .into_body()
        .collect()
        .await
        .expect("idempotent body should collect")
        .to_bytes();
    let idempotent_json: serde_json::Value =
        serde_json::from_slice(&idempotent_body).expect("idempotent body should be valid json");
    assert_eq!(idempotent_json, first_json);

    let conflicting_response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_http_idempotent",
                        "sourceEventId":"evt_http_conflict",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_other",
                        "title":"Changed message",
                        "body":"different",
                        "payload":"{\"conversationId\":\"c_other\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("conflicting notification request should return response");
    assert_eq!(conflicting_response.status(), StatusCode::CONFLICT);
    let conflicting_body = conflicting_response
        .into_body()
        .collect()
        .await
        .expect("conflicting body should collect")
        .to_bytes();
    let conflicting_json: serde_json::Value =
        serde_json::from_slice(&conflicting_body).expect("conflicting body should be valid json");
    assert_eq!(conflicting_json["code"], "notification_conflict");
}

#[tokio::test]
async fn test_list_notifications_returns_newest_first_with_distinct_timestamps() {
    let app = notification_service::build_default_app();

    let first_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_order_first",
                        "sourceEventId":"evt_order_first",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"First message",
                        "body":"first",
                        "payload":"{\"conversationId\":\"c_first\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("first notification request should succeed");
    assert_eq!(first_response.status(), StatusCode::OK);
    let first_body = first_response
        .into_body()
        .collect()
        .await
        .expect("first body should collect")
        .to_bytes();
    let first_json: serde_json::Value =
        serde_json::from_slice(&first_body).expect("first body should be valid json");

    sleep(Duration::from_millis(5)).await;

    let second_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/notifications/requests")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "notificationId":"ntf_order_second",
                        "sourceEventId":"evt_order_second",
                        "sourceEventType":"message.posted",
                        "category":"message.new",
                        "channel":"inapp",
                        "recipientId":"u_demo",
                        "title":"Second message",
                        "body":"second",
                        "payload":"{\"conversationId\":\"c_second\"}"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("second notification request should succeed");
    assert_eq!(second_response.status(), StatusCode::OK);
    let second_body = second_response
        .into_body()
        .collect()
        .await
        .expect("second body should collect")
        .to_bytes();
    let second_json: serde_json::Value =
        serde_json::from_slice(&second_body).expect("second body should be valid json");

    assert_ne!(
        first_json["requestedAt"], second_json["requestedAt"],
        "separate notification requests must not reuse a fixed requestedAt timestamp"
    );
    assert_ne!(
        first_json["dispatchedAt"], second_json["dispatchedAt"],
        "separate notification requests must not reuse a fixed dispatchedAt timestamp"
    );

    let list_response = app
        .oneshot(
            Request::builder()
                .uri("/api/v1/notifications")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("list notifications should succeed");
    assert_eq!(list_response.status(), StatusCode::OK);
    let list_body = list_response
        .into_body()
        .collect()
        .await
        .expect("list body should collect")
        .to_bytes();
    let list_json: serde_json::Value =
        serde_json::from_slice(&list_body).expect("list body should be valid json");
    let items = list_json["items"]
        .as_array()
        .expect("items should be an array");
    assert_eq!(items.len(), 2);
    assert_eq!(items[0]["notificationId"], "ntf_order_second");
    assert_eq!(items[1]["notificationId"], "ntf_order_first");
}
