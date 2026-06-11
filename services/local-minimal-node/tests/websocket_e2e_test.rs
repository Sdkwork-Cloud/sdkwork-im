use im_app_context::DualTokenRequestBuilderExt;
use std::time::Duration;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use futures_util::{SinkExt, StreamExt};
use http_body_util::BodyExt;
use serde_json::json;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tower::ServiceExt;

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("127.0.0.1:{}", address.port()), handle)
}

async fn next_text_json(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> serde_json::Value {
    let message = next_message(socket).await;
    match message {
        Message::Text(text) => serde_json::from_str(text.as_str())
            .expect("websocket text frame should contain valid json"),
        other => panic!("expected text frame, got {other:?}"),
    }
}

async fn next_message(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Message {
    timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should stay open")
        .expect("websocket frame should decode")
}

#[tokio::test]
async fn test_local_minimal_profile_pushes_business_realtime_events_over_websocket() {
    let app = local_minimal_node::build_default_app();
    let (address, handle) = spawn_server(app.clone()).await;

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_ws_realtime",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_ws_realtime/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let mut request = format!("ws://{address}/im/v3/api/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    request.headers_mut().insert(
        "x-sdkwork-tenant-id",
        "t_demo".parse().expect("tenant header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-user-id",
        "u_other_demo".parse().expect("user header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-actor-kind",
        "user".parse().expect("actor kind header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-session-id",
        "s_other".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-device-id",
        "d_other".parse().expect("device header should parse"),
    );
    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], "d_other");

    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_sync_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_ws_realtime",
                        "eventTypes":["message.posted"]
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = next_text_json(&mut socket).await;
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["snapshot"]["deviceId"], "d_other");

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_ws_realtime/messages")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_ws_realtime_1",
                        "summary":"hello websocket",
                        "text":"hello websocket"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let pushed_window = next_text_json(&mut socket).await;
    assert_eq!(pushed_window["type"], "event.window");
    assert_eq!(pushed_window["reason"], "push");
    assert_eq!(pushed_window["window"]["deviceId"], "d_other");
    assert_eq!(
        pushed_window["window"]["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["eventType"],
        "message.posted"
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["scopeId"],
        "c_ws_realtime"
    );

    let payload: serde_json::Value = serde_json::from_str(
        pushed_window["window"]["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_ws_realtime");
    assert_eq!(payload["summary"], "hello websocket");
    assert_eq!(payload["body"]["summary"], "hello websocket");
    assert_eq!(payload["body"]["parts"][0]["kind"], "text");
    assert_eq!(payload["body"]["parts"][0]["text"], "hello websocket");

    socket
        .send(Message::Text(
            json!({
                "type":"events.ack",
                "requestId":"req_ack_1",
                "ackedSeq":1
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("ack frame should send");

    let acked = next_text_json(&mut socket).await;
    assert_eq!(acked["type"], "events.acked");
    assert_eq!(acked["ack"]["ackedThroughSeq"], 1);
    assert_eq!(acked["ack"]["trimmedThroughSeq"], 1);
    assert_eq!(acked["ack"]["retainedEventCount"], 0);

    let realtime_events = app
        .oneshot(
            Request::builder()
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=10")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_other_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_other")
                .with_dual_token_session("s_other")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("realtime events should succeed");
    assert_eq!(realtime_events.status(), StatusCode::OK);
    let realtime_events_body = realtime_events
        .into_body()
        .collect()
        .await
        .expect("realtime events body should collect")
        .to_bytes();
    let realtime_events_json: serde_json::Value = serde_json::from_slice(&realtime_events_body)
        .expect("realtime events should be valid json");
    assert_eq!(realtime_events_json["items"].as_array().unwrap().len(), 0);
    assert_eq!(realtime_events_json["ackedThroughSeq"], 1);
    assert_eq!(realtime_events_json["trimmedThroughSeq"], 1);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_local_minimal_profile_pushes_agent_handoff_lifecycle_events_over_websocket() {
    let app = local_minimal_node::build_default_app();
    let (address, handle) = spawn_server(app.clone()).await;

    let create_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/agent_handoffs")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("ag_source")
                .with_dual_token_actor_kind("agent")
                .with_dual_token_device("d_agent")
                .with_dual_token_session("s_agent")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_ws_handoff",
                        "targetId":"u_demo",
                        "targetKind":"user",
                        "handoffSessionId":"hs_ws",
                        "handoffReason":"manual_escalation"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create agent handoff should succeed");
    assert_eq!(create_handoff.status(), StatusCode::OK);

    let mut request = format!("ws://{address}/im/v3/api/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    request.headers_mut().insert(
        "x-sdkwork-tenant-id",
        "t_demo".parse().expect("tenant header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-user-id",
        "u_demo".parse().expect("user header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-actor-kind",
        "user".parse().expect("actor kind header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-session-id",
        "s_pad".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-device-id",
        "d_pad".parse().expect("device header should parse"),
    );
    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], "d_pad");

    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_handoff_sync_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_ws_handoff",
                        "eventTypes":["conversation.agent_handoff_status_changed"]
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = next_text_json(&mut socket).await;
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["snapshot"]["deviceId"], "d_pad");

    let accept_handoff = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_ws_handoff/agent_handoff/accept")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_phone")
                .with_dual_token_session("s_phone")
                .header("content-type", "application/json")
                .body(Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("accept handoff should succeed");
    assert_eq!(accept_handoff.status(), StatusCode::OK);

    let pushed_window = next_text_json(&mut socket).await;
    assert_eq!(pushed_window["type"], "event.window");
    assert_eq!(pushed_window["reason"], "push");
    assert_eq!(pushed_window["window"]["deviceId"], "d_pad");
    assert_eq!(
        pushed_window["window"]["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["eventType"],
        "conversation.agent_handoff_status_changed"
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["scopeId"],
        "c_ws_handoff"
    );

    let payload: serde_json::Value = serde_json::from_str(
        pushed_window["window"]["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_ws_handoff");
    assert_eq!(payload["currentStatus"], "accepted");
    assert_eq!(payload["changedBy"]["id"], "u_demo");
    assert_eq!(payload["state"]["status"], "accepted");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_local_minimal_profile_pushes_member_joined_events_over_websocket() {
    let app = local_minimal_node::build_default_app();
    let (address, handle) = spawn_server(app.clone()).await;

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_ws_member_events",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let mut request = format!("ws://{address}/im/v3/api/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    request.headers_mut().insert(
        "x-sdkwork-tenant-id",
        "t_demo".parse().expect("tenant header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-user-id",
        "u_demo".parse().expect("user header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-actor-kind",
        "user".parse().expect("actor kind header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-session-id",
        "s_pad".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-sdkwork-device-id",
        "d_pad".parse().expect("device header should parse"),
    );
    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], "d_pad");

    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_member_sync_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_ws_member_events",
                        "eventTypes":["conversation.member_joined"]
                    }
                ]
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = next_text_json(&mut socket).await;
    assert_eq!(synced["type"], "subscriptions.synced");
    assert_eq!(synced["snapshot"]["deviceId"], "d_pad");

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_ws_member_events/members/add")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_device("d_owner")
                .with_dual_token_session("s_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_other_demo",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);

    let pushed_window = next_text_json(&mut socket).await;
    assert_eq!(pushed_window["type"], "event.window");
    assert_eq!(pushed_window["reason"], "push");
    assert_eq!(pushed_window["window"]["deviceId"], "d_pad");
    assert_eq!(
        pushed_window["window"]["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["eventType"],
        "conversation.member_joined"
    );
    assert_eq!(
        pushed_window["window"]["items"][0]["scopeId"],
        "c_ws_member_events"
    );

    let payload: serde_json::Value = serde_json::from_str(
        pushed_window["window"]["items"][0]["payload"]
            .as_str()
            .expect("payload should be string"),
    )
    .expect("payload should be valid json");
    assert_eq!(payload["conversationId"], "c_ws_member_events");
    assert_eq!(payload["member"]["principalId"], "u_other_demo");
    assert_eq!(payload["member"]["state"], "joined");
    assert_eq!(payload["actor"]["id"], "u_demo");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}
