use std::time::Duration;

use axum::Router;
use futures_util::{SinkExt, StreamExt};
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

#[tokio::test]
async fn test_realtime_websocket_binds_http_control_semantics() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let mut request = format!("ws://{address}/api/v1/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    request.headers_mut().insert(
        "x-tenant-id",
        "t_demo".parse().expect("tenant header should parse"),
    );
    request.headers_mut().insert(
        "x-user-id",
        "u_demo".parse().expect("user header should parse"),
    );
    request.headers_mut().insert(
        "x-session-id",
        "s_pad".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-device-id",
        "d_pad".parse().expect("device header should parse"),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["deviceId"], "d_pad");
    assert_eq!(connected["ackedThroughSeq"], 0);
    assert_eq!(connected["trimmedThroughSeq"], 0);
    assert_eq!(connected["latestRealtimeSeq"], 0);

    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_sync_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_demo",
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
    assert_eq!(synced["requestId"], "req_sync_1");
    assert_eq!(synced["snapshot"]["deviceId"], "d_pad");
    assert_eq!(synced["snapshot"]["items"][0]["scopeType"], "conversation");
    assert_eq!(synced["snapshot"]["items"][0]["scopeId"], "c_demo");

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_pull_1",
                "afterSeq":0,
                "limit":10
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("event pull frame should send");

    let window = next_text_json(&mut socket).await;
    assert_eq!(window["type"], "event.window");
    assert_eq!(window["requestId"], "req_pull_1");
    assert_eq!(window["reason"], "pull");
    assert_eq!(window["window"]["deviceId"], "d_pad");
    assert_eq!(window["window"]["items"].as_array().unwrap().len(), 0);
    assert_eq!(window["window"]["ackedThroughSeq"], 0);
    assert_eq!(window["window"]["trimmedThroughSeq"], 0);

    socket
        .send(Message::Text(
            json!({
                "type":"events.ack",
                "requestId":"req_ack_1",
                "ackedSeq":0
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("ack frame should send");

    let acked = next_text_json(&mut socket).await;
    assert_eq!(acked["type"], "events.acked");
    assert_eq!(acked["requestId"], "req_ack_1");
    assert_eq!(acked["ack"]["deviceId"], "d_pad");
    assert_eq!(acked["ack"]["ackedThroughSeq"], 0);
    assert_eq!(acked["ack"]["trimmedThroughSeq"], 0);
    assert_eq!(acked["ack"]["retainedEventCount"], 0);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_when_session_disconnects() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app.clone()).await;
    let mut request = format!("ws://{address}/api/v1/realtime/ws")
        .into_client_request()
        .expect("websocket request should build");
    request.headers_mut().insert(
        "x-tenant-id",
        "t_demo".parse().expect("tenant header should parse"),
    );
    request.headers_mut().insert(
        "x-user-id",
        "u_demo".parse().expect("user header should parse"),
    );
    request.headers_mut().insert(
        "x-session-id",
        "s_pad".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-device-id",
        "d_pad".parse().expect("device header should parse"),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let disconnect = app
        .oneshot(
            axum::http::Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/disconnect")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_pad")
                .header("x-device-id", "d_pad")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{}"#))
                .unwrap(),
        )
        .await
        .expect("disconnect request should succeed");
    assert_eq!(disconnect.status(), axum::http::StatusCode::OK);

    let close = next_message(&mut socket).await;
    match close {
        Message::Close(Some(frame)) => {
            assert_eq!(
                frame.code,
                tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(
                    session_gateway::SESSION_DISCONNECT_CLOSE_CODE,
                )
            );
            assert_eq!(
                frame.reason.as_str(),
                session_gateway::SESSION_DISCONNECT_CLOSE_REASON
            );
        }
        other => panic!("expected close frame, got {other:?}"),
    }

    handle.abort();
    let _ = handle.await;
}
