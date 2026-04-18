use std::sync::Arc;
use std::time::Duration;

use axum::Router;
use axum::http::Request;
use craw_chat_ccp_binding_ws::{CCP_WS_SUBPROTOCOL, WsBinding, WsBindingMessage, WsOpcode};
use craw_chat_ccp_codec::CcpCodec;
use craw_chat_ccp_codec_json::JsonEnvelopeCodec;
use craw_chat_ccp_control::{AuthBindFrame, ControlFrame, HelloFrame};
use craw_chat_ccp_core::{CapabilitySet, CcpEnvelope, CcpRoute, ProtocolVersion, TransportBinding};
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpListener;
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::ClientRequestBuilder;
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

fn encode_ccp_text_frame(
    schema: &str,
    kind: &str,
    payload: serde_json::Value,
) -> tokio_tungstenite::tungstenite::Message {
    encode_ccp_text_frame_with_route(schema, kind, None, payload)
}

fn encode_ccp_text_frame_with_route(
    schema: &str,
    kind: &str,
    route: Option<CcpRoute>,
    payload: serde_json::Value,
) -> tokio_tungstenite::tungstenite::Message {
    let codec = JsonEnvelopeCodec::new();
    let binding = WsBinding::new();
    let envelope = CcpEnvelope::new(
        ProtocolVersion::new("ccp", 1, 0),
        TransportBinding::Ws1,
        kind,
        schema,
        None,
        route,
        Vec::<String>::new(),
        None,
        payload.to_string(),
    );
    let message = binding
        .encode(&envelope, &codec)
        .expect("ccp envelope should encode");
    match message.opcode {
        WsOpcode::Text => Message::Text(
            String::from_utf8(message.payload)
                .expect("ccp text payload should stay utf8")
                .into(),
        ),
        WsOpcode::Binary => Message::Binary(message.payload.into()),
    }
}

fn decode_ccp_envelope(message: Message) -> CcpEnvelope {
    let codec = JsonEnvelopeCodec::new();
    let binding = WsBinding::new();
    let binding_message = match message {
        Message::Text(text) => WsBindingMessage {
            protocol_id: TransportBinding::Ws1.protocol_id(),
            content_type: codec.content_type(),
            opcode: WsOpcode::Text,
            payload: text.to_string().into_bytes(),
        },
        Message::Binary(bytes) => WsBindingMessage {
            protocol_id: TransportBinding::Ws1.protocol_id(),
            content_type: codec.content_type(),
            opcode: WsOpcode::Binary,
            payload: bytes.to_vec(),
        },
        other => panic!("expected CCP websocket frame, got {other:?}"),
    };
    binding
        .decode(&binding_message, &codec)
        .expect("ccp websocket frame should decode")
}

fn envelope_payload_json(envelope: &CcpEnvelope) -> serde_json::Value {
    serde_json::from_str(envelope.payload.as_str()).expect("ccp payload should be valid json")
}

fn assert_policy_close_with_reason(message: Message, reason: &str) {
    match message {
        Message::Close(Some(frame)) => {
            assert_eq!(
                frame.code,
                tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Policy
            );
            assert_eq!(frame.reason.as_str(), reason);
        }
        other => panic!("expected policy close frame, got {other:?}"),
    }
}

fn assert_connection_closed_after_oversized_message(
    message: Option<Result<Message, tokio_tungstenite::tungstenite::Error>>,
) {
    match message {
        Some(Ok(Message::Close(_))) | Some(Err(_)) => {}
        Some(Ok(other)) => {
            panic!("expected websocket close after oversized message, got {other:?}")
        }
        None => panic!("expected websocket close after oversized message"),
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
    assert_eq!(connected["actor"]["id"], "u_demo");
    assert_eq!(connected["actor"]["kind"], "user");
    assert_eq!(connected["sender"]["principalId"], "u_demo");
    assert_eq!(connected["sender"]["deviceId"], "d_pad");
    assert_eq!(connected["sender"]["sessionId"], "s_pad");
    assert_eq!(connected["sender"]["senderId"], "u_demo:d_pad");
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
async fn test_realtime_websocket_rejects_oversized_request_id() {
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

    let _connected = next_text_json(&mut socket).await;

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"r".repeat(1024),
                "afterSeq":0,
                "limit":10
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("oversized request id frame should send");

    let error = next_text_json(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert!(error["requestId"].is_null());
    assert_eq!(error["code"], "payload_too_large");
    assert!(
        error["message"]
            .as_str()
            .expect("message should be a string")
            .contains("requestId"),
        "error should point to requestId payload guard, got: {error:?}"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_oversized_frame_type() {
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

    let _connected = next_text_json(&mut socket).await;

    socket
        .send(Message::Text(
            json!({
                "type":"x".repeat(1024),
                "requestId":"req_oversized_type_1"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("oversized frame type should send");

    let error = next_text_json(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert_eq!(error["requestId"], "req_oversized_type_1");
    assert_eq!(error["code"], "payload_too_large");
    assert!(
        error["message"]
            .as_str()
            .expect("message should be a string")
            .contains("type"),
        "error should point to type payload guard, got: {error:?}"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_connection_for_oversized_raw_message() {
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

    let _connected = next_text_json(&mut socket).await;

    socket
        .send(Message::Text("x".repeat(700_000).into()))
        .await
        .expect("oversized websocket message should send");

    let next = timeout(Duration::from_secs(5), socket.next())
        .await
        .expect("oversized websocket message should trigger a close");
    assert_connection_closed_after_oversized_message(next);

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_negotiates_ccp_subprotocol_and_wraps_business_frames() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-1".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.kind, "control");
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");
    let hello_ack_payload = envelope_payload_json(&hello_ack);
    assert_eq!(hello_ack_payload["type"], "hello_ack");
    assert_eq!(hello_ack_payload["data"]["accepted"], true);

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.kind, "control");
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");
    let auth_ok_payload = envelope_payload_json(&auth_ok);
    assert_eq!(auth_ok_payload["type"], "auth_ok");
    assert_eq!(auth_ok_payload["data"]["tenant_id"], "t_demo");
    assert_eq!(auth_ok_payload["data"]["principal_id"], "u_demo");
    assert_eq!(auth_ok_payload["data"]["device_id"], "d_pad");
    assert_eq!(auth_ok_payload["data"]["session_id"], "s_pad");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.kind, "evt");
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");
    assert_eq!(connected_payload["deviceId"], "d_pad");
    assert_eq!(connected_payload["actor"]["id"], "u_demo");
    assert_eq!(connected_payload["sender"]["senderId"], "u_demo:d_pad");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.subscriptions.sync.v1",
            "cmd",
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_sync_ccp_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_demo",
                        "eventTypes":["message.posted"]
                    }
                ]
            }),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(synced.schema, "cc.realtime.subscriptions.synced.v1");
    let synced_payload = envelope_payload_json(&synced);
    assert_eq!(synced_payload["type"], "subscriptions.synced");
    assert_eq!(synced_payload["requestId"], "req_sync_ccp_1");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_ccp_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("event pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_ccp_1");
    assert_eq!(window_payload["reason"], "pull");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.ack.v1",
            "ack",
            json!({
                "type":"events.ack",
                "requestId":"req_ack_ccp_1",
                "ackedSeq":0
            }),
        ))
        .await
        .expect("ack frame should send");

    let acked = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(acked.schema, "cc.realtime.events.acked.v1");
    let acked_payload = envelope_payload_json(&acked);
    assert_eq!(acked_payload["type"], "events.acked");
    assert_eq!(acked_payload["requestId"], "req_ack_ccp_1");
    assert_eq!(acked_payload["ack"]["deviceId"], "d_pad");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_control_kind_after_handshake() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-invalid-kind".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_wrong_kind_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("wrong-kind business frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_wrong_kind_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("kind"),
        "error should explain CCP kind mismatch, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_invalid_kind_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_after_invalid_kind_1");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_wrong_schema_after_handshake() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-invalid-schema".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.ack.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_wrong_schema_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("wrong-schema business frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_wrong_schema_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("schema"),
        "error should explain CCP schema mismatch, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_invalid_schema_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(
        window_payload["requestId"],
        "req_pull_after_invalid_schema_1"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_accepts_ccp_heartbeat_control_frame_after_handshake() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-heartbeat".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.heartbeat.v1",
            "control",
            serde_json::to_value(ControlFrame::Heartbeat(
                craw_chat_ccp_control::HeartbeatFrame { sequence: Some(1) },
            ))
            .expect("heartbeat frame should serialize"),
        ))
        .await
        .expect("heartbeat frame should send");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_heartbeat_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("pull frame after heartbeat should send");

    let first_response = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(first_response.kind, "evt");
    assert_eq!(first_response.schema, "cc.realtime.event.window.v1");
    let first_response_payload = envelope_payload_json(&first_response);
    assert_eq!(first_response_payload["type"], "event.window");
    assert_eq!(
        first_response_payload["requestId"],
        "req_pull_after_heartbeat_1"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_ccp_business_frame_with_client_route_metadata() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-client-route".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");

    socket
        .send(encode_ccp_text_frame_with_route(
            "cc.realtime.events.pull.v1",
            "cmd",
            Some(CcpRoute::new(
                "t_forged",
                Some("u_forged".into()),
                Some("d_forged".into()),
            )),
            json!({
                "type":"events.pull",
                "requestId":"req_pull_client_route_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("client-route pull frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "error");
    assert_eq!(error.schema, "cc.realtime.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["requestId"], "req_pull_client_route_1");
    assert_eq!(error_payload["code"], "invalid_frame");
    assert!(
        error_payload["message"]
            .as_str()
            .expect("message should be a string")
            .contains("route"),
        "error should explain client route metadata is forbidden, got: {error_payload:?}"
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.events.pull.v1",
            "cmd",
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_client_route_1",
                "afterSeq":0,
                "limit":10
            }),
        ))
        .await
        .expect("valid pull frame should send");

    let window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(window.kind, "evt");
    assert_eq!(window.schema, "cc.realtime.event.window.v1");
    let window_payload = envelope_payload_json(&window);
    assert_eq!(window_payload["type"], "event.window");
    assert_eq!(window_payload["requestId"], "req_pull_after_client_route_1");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_handshake_starts_without_hello() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("out-of-order auth bind frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_HELLO_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_after_ccp_handshake_protocol_close() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster_runtime_and_presence(
        cluster.clone(),
        Arc::new(session_gateway::RealtimeDeliveryRuntime::default()),
        Arc::new(session_gateway::SessionPresenceRuntime::default()),
    );
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("out-of-order auth bind frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["data"]["code"], "CCP_HELLO_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released after handshake protocol close");

    assert!(
        cluster
            .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
            .is_none(),
        "ccp handshake failure must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_after_client_close() {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());
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

    socket
        .close(None)
        .await
        .expect("client close should send successfully");

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released after client websocket close");

    assert!(
        cluster
            .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
            .is_none(),
        "closed websocket must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_releases_route_when_upgrade_client_disconnects_before_socket_handoff()
 {
    let cluster = Arc::new(session_gateway::RealtimeClusterBridge::default());
    let app = session_gateway::build_app_with_cluster(cluster.clone());
    let (address, handle) = spawn_server(app).await;

    let mut stream = tokio::net::TcpStream::connect(address.as_str())
        .await
        .expect("raw tcp connection should succeed");
    let upgrade_request = format!(
        "GET /api/v1/realtime/ws HTTP/1.1\r\n\
Host: {address}\r\n\
Connection: Upgrade\r\n\
Upgrade: websocket\r\n\
Sec-WebSocket-Version: 13\r\n\
Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
Sec-WebSocket-Protocol: {CCP_WS_SUBPROTOCOL}\r\n\
x-tenant-id: t_demo\r\n\
x-user-id: u_demo\r\n\
x-session-id: s_pad\r\n\
x-device-id: d_pad\r\n\
\r\n"
    );
    stream
        .write_all(upgrade_request.as_bytes())
        .await
        .expect("upgrade request should write");
    stream
        .shutdown()
        .await
        .expect("client shutdown should succeed");
    drop(stream);

    timeout(Duration::from_secs(5), async {
        loop {
            if cluster
                .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
                .is_none()
            {
                break;
            }
            tokio::task::yield_now().await;
        }
    })
    .await
    .expect("route should be released when upgrade client disconnects before websocket handoff");

    assert!(
        cluster
            .resolve_device_route_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
            .is_none(),
        "aborted websocket upgrade must not leave a ghost route bound to the node"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_rejects_stale_session_frames_after_http_resume_takeover() {
    let app = session_gateway::build_app();
    let http_app = app.clone();
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
        "s_old".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-device-id",
        "d_demo".parse().expect("device header should parse"),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let resume_new = http_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_new.status(), axum::http::StatusCode::OK);

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_stale_after_resume_1",
                "afterSeq":0,
                "limit":10
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("stale websocket frame should send");

    let error = next_text_json(&mut socket).await;
    assert_eq!(error["type"], "error");
    assert_eq!(error["requestId"], "req_stale_after_resume_1");
    assert_eq!(error["code"], "stale_session");

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_stale_session_before_push_after_http_resume_takeover() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
    let http_app = app.clone();
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
        "s_old".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-device-id",
        "d_demo".parse().expect("device header should parse"),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    socket
        .send(Message::Text(
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_sync_stale_push_1",
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
    assert_eq!(synced["requestId"], "req_sync_stale_push_1");

    let resume_new = http_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_new.status(), axum::http::StatusCode::OK);

    runtime
        .publish_scope_event_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "messageId": "msg_stale_push_1",
                "summary": "stale websocket must not receive this push"
            })
            .to_string(),
            vec!["d_demo".into()],
        )
        .expect("publish after resume takeover should succeed");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "stale_session");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_idle_stale_session_after_http_resume_takeover() {
    let app = session_gateway::build_app();
    let http_app = app.clone();
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
        "s_old".parse().expect("session header should parse"),
    );
    request.headers_mut().insert(
        "x-device-id",
        "d_demo".parse().expect("device header should parse"),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");

    let resume_new = http_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_new.status(), axum::http::StatusCode::OK);

    let close = timeout(Duration::from_secs(1), socket.next())
        .await
        .expect("stale idle websocket should be closed promptly after takeover")
        .expect("websocket stream should yield a close frame")
        .expect("websocket close frame should decode");
    assert_policy_close_with_reason(close, "stale_session");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_stale_ccp_handshake_after_http_resume_takeover() {
    let app = session_gateway::build_app();
    let http_app = app.clone();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_old")
    .with_header("x-device-id", "d_demo");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-stale-handshake-takeover".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    let resume_new = http_app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/v1/sessions/resume")
                .header("x-tenant-id", "t_demo")
                .header("x-user-id", "u_demo")
                .header("x-session-id", "s_new")
                .header("x-device-id", "d_demo")
                .header("content-type", "application/json")
                .body(axum::body::Body::from(r#"{"lastSeenSyncSeq":0}"#))
                .unwrap(),
        )
        .await
        .expect("fresh resume should succeed");
    assert_eq!(resume_new.status(), axum::http::StatusCode::OK);

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_demo".into()),
                session_id: Some("s_old".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("stale auth bind frame should send");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "stale_session");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_hello_uses_wrong_schema() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-wrong-schema".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("wrong-schema hello frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_SCHEMA_INCOMPATIBLE");
    assert!(
        error_payload["data"]["message"]
            .as_str()
            .expect("message should be a string")
            .contains("schema"),
        "error should explain CCP control schema mismatch, got: {error_payload:?}"
    );

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_with_policy_after_ccp_handshake_receives_hello_twice() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    let hello_message = encode_ccp_text_frame(
        "cc.control.hello.v1",
        "control",
        serde_json::to_value(ControlFrame::Hello(HelloFrame {
            protocol: ProtocolVersion::new("ccp", 1, 0),
            binding: TransportBinding::Ws1,
            capabilities: CapabilitySet::from_iter(["payload.json"]),
            trace_id: Some("trace-hello-order".into()),
        }))
        .expect("hello frame should serialize"),
    );

    socket
        .send(hello_message.clone())
        .await
        .expect("hello frame should send");
    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(hello_message)
        .await
        .expect("duplicate hello frame should send");

    let error = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(error.kind, "control");
    assert_eq!(error.schema, "cc.control.error.v1");
    let error_payload = envelope_payload_json(&error);
    assert_eq!(error_payload["type"], "error");
    assert_eq!(error_payload["data"]["code"], "CCP_AUTH_BIND_REQUIRED");

    let close = next_message(&mut socket).await;
    assert_policy_close_with_reason(close, "ccp.protocol_error");

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_pushes_live_business_frames_over_ccp_subprotocol() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-live-push-ccp".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");

    socket
        .send(encode_ccp_text_frame(
            "cc.realtime.subscriptions.sync.v1",
            "cmd",
            json!({
                "type":"subscriptions.sync",
                "requestId":"req_live_push_ccp_1",
                "items":[
                    {
                        "scopeType":"conversation",
                        "scopeId":"c_demo",
                        "eventTypes":["message.posted"]
                    }
                ]
            }),
        ))
        .await
        .expect("subscription sync frame should send");

    let synced = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(synced.schema, "cc.realtime.subscriptions.synced.v1");
    let synced_payload = envelope_payload_json(&synced);
    assert_eq!(synced_payload["type"], "subscriptions.synced");

    runtime
        .publish_scope_event_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "messageId": "msg_ccp_push_1",
                "summary": "hello ccp push"
            })
            .to_string(),
            vec!["d_pad".into()],
        )
        .expect("live publish should succeed");

    let pushed_window = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(pushed_window.schema, "cc.realtime.event.window.v1");
    let pushed_payload = envelope_payload_json(&pushed_window);
    assert_eq!(pushed_payload["type"], "event.window");
    assert_eq!(pushed_payload["reason"], "push");
    assert_eq!(pushed_payload["window"]["deviceId"], "d_pad");
    assert_eq!(
        pushed_payload["window"]["items"].as_array().unwrap().len(),
        1
    );
    assert_eq!(
        pushed_payload["window"]["items"][0]["eventType"],
        "message.posted"
    );

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_skips_session_resume_when_capability_not_negotiated() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, response) = connect_async(request)
        .await
        .expect("websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("server should select websocket subprotocol"),
        CCP_WS_SUBPROTOCOL
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-no-resume".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");

    let hello_ack = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(hello_ack.kind, "control");
    assert_eq!(hello_ack.schema, "cc.control.hello_ack.v1");
    let hello_ack_payload = envelope_payload_json(&hello_ack);
    assert_eq!(hello_ack_payload["type"], "hello_ack");
    assert_eq!(hello_ack_payload["data"]["accepted"], true);
    assert_eq!(
        hello_ack_payload["data"]["capabilities"]["items"],
        json!(["payload.json"])
    );

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");

    let auth_ok = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(auth_ok.kind, "control");
    assert_eq!(auth_ok.schema, "cc.control.auth_ok.v1");
    let auth_ok_payload = envelope_payload_json(&auth_ok);
    assert_eq!(auth_ok_payload["type"], "auth_ok");

    let connected = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(connected.kind, "evt");
    assert_eq!(connected.schema, "cc.realtime.connected.v1");
    let connected_payload = envelope_payload_json(&connected);
    assert_eq!(connected_payload["type"], "realtime.connected");
    assert_eq!(connected_payload["deviceId"], "d_pad");

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

#[tokio::test]
async fn test_realtime_websocket_sends_ccp_goaway_before_disconnect_close() {
    let app = session_gateway::build_app();
    let (address, handle) = spawn_server(app.clone()).await;
    let request = ClientRequestBuilder::new(
        format!("ws://{address}/api/v1/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(CCP_WS_SUBPROTOCOL)
    .with_header("x-tenant-id", "t_demo")
    .with_header("x-user-id", "u_demo")
    .with_header("x-session-id", "s_pad")
    .with_header("x-device-id", "d_pad");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("websocket connection should succeed");

    socket
        .send(encode_ccp_text_frame(
            "cc.control.hello.v1",
            "control",
            serde_json::to_value(ControlFrame::Hello(HelloFrame {
                protocol: ProtocolVersion::new("ccp", 1, 0),
                binding: TransportBinding::Ws1,
                capabilities: CapabilitySet::from_iter(["payload.json"]),
                trace_id: Some("trace-hello-goaway".into()),
            }))
            .expect("hello frame should serialize"),
        ))
        .await
        .expect("hello frame should send");
    let _ = decode_ccp_envelope(next_message(&mut socket).await);

    socket
        .send(encode_ccp_text_frame(
            "cc.control.auth_bind.v1",
            "control",
            serde_json::to_value(ControlFrame::AuthBind(AuthBindFrame {
                principal_id: "u_demo".into(),
                device_id: Some("d_pad".into()),
                session_id: Some("s_pad".into()),
                actor_kind: "user".into(),
            }))
            .expect("auth bind frame should serialize"),
        ))
        .await
        .expect("auth bind frame should send");
    let _ = decode_ccp_envelope(next_message(&mut socket).await);
    let _ = decode_ccp_envelope(next_message(&mut socket).await);

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

    let goaway = decode_ccp_envelope(next_message(&mut socket).await);
    assert_eq!(goaway.kind, "control");
    assert_eq!(goaway.schema, "cc.control.goaway.v1");
    let goaway_payload = envelope_payload_json(&goaway);
    assert_eq!(goaway_payload["type"], "go_away");
    assert_eq!(goaway_payload["data"]["code"], "SESSION_DISCONNECT");
    assert_eq!(goaway_payload["data"]["message"], "session.disconnect");

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

#[tokio::test]
async fn test_realtime_websocket_uses_runtime_link_queue_owner_limits_for_catchup_and_pull() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=520 {
        runtime
            .publish_scope_event_for_principal_kind(
                "t_demo",
                "u_demo",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
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

    let catchup = next_text_json(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["hasMore"], true);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_pull_backpressure_1",
                "afterSeq":0,
                "limit":999
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("event pull frame should send");

    let pull = next_text_json(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_backpressure_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["hasMore"], true);
    assert_eq!(pull["window"]["nextAfterSeq"], 512);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_degrades_live_push_to_pull_only_when_runtime_link_detects_overload()
 {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=900 {
        runtime
            .publish_scope_event_for_principal_kind(
                "t_demo",
                "u_demo",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
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

    let catchup = next_text_json(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    runtime
        .publish_scope_event_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "index": 901
            })
            .to_string(),
            vec!["d_pad".into()],
        )
        .expect("overload publish should succeed");

    assert!(
        timeout(Duration::from_millis(250), socket.next())
            .await
            .is_err(),
        "live push should degrade to pull-only under overload backlog"
    );

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_pull_after_overload_1",
                "afterSeq":128,
                "limit":999
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("event pull frame should send");

    let pull = next_text_json(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_after_overload_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_clamps_stale_pull_replay_when_backlog_is_still_over_hard_limit() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=900 {
        runtime
            .publish_scope_event_for_principal_kind(
                "t_demo",
                "u_demo",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
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

    let catchup = next_text_json(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["hasMore"], true);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_pull_stale_replay_overload_1",
                "afterSeq":0,
                "limit":999
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("event pull frame should send");

    let pull = next_text_json(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_stale_replay_overload_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["items"][0]["realtimeSeq"], 129);
    assert_eq!(pull["window"]["hasMore"], true);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_recovers_buffered_push_after_pull_reduces_backlog_under_hard_limit()
 {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=700 {
        runtime
            .publish_scope_event_for_principal_kind(
                "t_demo",
                "u_demo",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime,
    );
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

    let catchup = next_text_json(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    socket
        .send(Message::Text(
            json!({
                "type":"events.pull",
                "requestId":"req_pull_recovery_1",
                "afterSeq":128,
                "limit":999
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("event pull frame should send");

    let pull = next_text_json(&mut socket).await;
    assert_eq!(pull["type"], "event.window");
    assert_eq!(pull["requestId"], "req_pull_recovery_1");
    assert_eq!(pull["reason"], "pull");
    assert_eq!(pull["window"]["items"].as_array().unwrap().len(), 512);
    assert_eq!(pull["window"]["nextAfterSeq"], 640);

    let recovered_push = next_text_json(&mut socket).await;
    assert_eq!(recovered_push["type"], "event.window");
    assert_eq!(recovered_push["reason"], "push");
    assert_eq!(
        recovered_push["window"]["items"].as_array().unwrap().len(),
        60
    );
    assert_eq!(recovered_push["window"]["hasMore"], false);
    assert_eq!(recovered_push["window"]["nextAfterSeq"], 700);

    let _ = socket.close(None).await;
    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_realtime_websocket_closes_when_runtime_link_detects_extreme_overload_backlog() {
    let runtime = Arc::new(session_gateway::RealtimeDeliveryRuntime::default());
    runtime
        .ensure_device_state_for_principal_kind("t_demo", "u_demo", "user", "d_pad")
        .expect("device state should initialize");
    runtime
        .sync_subscriptions_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "d_pad",
            vec![session_gateway::RealtimeSubscriptionItemInput {
                scope_type: "conversation".into(),
                scope_id: "c_demo".into(),
                event_types: vec!["message.posted".into()],
            }],
        )
        .expect("subscription seed should succeed");
    for index in 1..=1200 {
        runtime
            .publish_scope_event_for_principal_kind(
                "t_demo",
                "u_demo",
                "user",
                "conversation",
                "c_demo",
                "message.posted",
                json!({
                    "type": "message.posted",
                    "index": index
                })
                .to_string(),
                vec!["d_pad".into()],
            )
            .expect("seed realtime event should publish");
    }

    let app = session_gateway::build_app_with_cluster_and_runtime(
        Arc::new(session_gateway::RealtimeClusterBridge::default()),
        runtime.clone(),
    );
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

    let catchup = next_text_json(&mut socket).await;
    assert_eq!(catchup["type"], "event.window");
    assert_eq!(catchup["reason"], "catchup");
    assert_eq!(catchup["window"]["items"].as_array().unwrap().len(), 128);
    assert_eq!(catchup["window"]["nextAfterSeq"], 128);

    runtime
        .publish_scope_event_for_principal_kind(
            "t_demo",
            "u_demo",
            "user",
            "conversation",
            "c_demo",
            "message.posted",
            json!({
                "type": "message.posted",
                "index": 1201
            })
            .to_string(),
            vec!["d_pad".into()],
        )
        .expect("extreme overload publish should succeed");

    let close = next_message(&mut socket).await;
    match close {
        Message::Close(Some(frame)) => {
            assert_eq!(
                frame.code,
                tokio_tungstenite::tungstenite::protocol::frame::coding::CloseCode::Library(
                    session_gateway::REALTIME_OVERLOAD_CLOSE_CODE,
                )
            );
            assert_eq!(
                frame.reason.as_str(),
                session_gateway::REALTIME_OVERLOAD_CLOSE_REASON
            );
        }
        other => panic!("expected close frame, got {other:?}"),
    }

    handle.abort();
    let _ = handle.await;
}
