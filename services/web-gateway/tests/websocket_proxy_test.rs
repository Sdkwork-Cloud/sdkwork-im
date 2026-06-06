use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::IntoResponse,
    routing::get,
};
use craw_chat_api_registry::{
    HttpMethod, RouteDescriptor, RouteProtocol, RouteVisibility, SdkTarget, build_registry,
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use craw_chat_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{Message as TungsteniteMessage, client::ClientRequestBuilder},
};

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

fn test_gateway_config(
    upstreams: Vec<craw_chat_gateway_config::ServiceUpstreamConfig>,
) -> WebGatewayConfig {
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::Split,
        strict_startup: true,
        upstreams,
    }
}

async fn websocket_echo(ws: WebSocketUpgrade) -> impl IntoResponse {
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(handle_echo_socket)
}

async fn handle_echo_socket(mut socket: WebSocket) {
    while let Some(message) = socket.next().await {
        let Ok(message) = message else {
            break;
        };

        match message {
            Message::Text(text) => {
                if socket.send(Message::Text(text)).await.is_err() {
                    break;
                }
            }
            Message::Binary(bytes) => {
                if socket.send(Message::Binary(bytes)).await.is_err() {
                    break;
                }
            }
            Message::Ping(payload) => {
                if socket.send(Message::Pong(payload)).await.is_err() {
                    break;
                }
            }
            Message::Pong(_) => {}
            Message::Close(frame) => {
                let _ = socket.send(Message::Close(frame)).await;
                break;
            }
        }
    }
}

#[tokio::test]
async fn gateway_proxies_realtime_websocket_upgrade_and_frames() {
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "session-gateway",
        format!("http://{upstream_address}").as_str(),
    )]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL);

    let (mut socket, response) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("gateway should select websocket subprotocol"),
        LINK_WEBSOCKET_SUBPROTOCOL
    );

    socket
        .send(TungsteniteMessage::Text("hello gateway".into()))
        .await
        .expect("client text frame should send");
    let echoed = socket
        .next()
        .await
        .expect("echoed frame should arrive")
        .expect("echoed frame should decode");
    assert_eq!(echoed, TungsteniteMessage::Text("hello gateway".into()));

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
}

#[tokio::test]
async fn gateway_proxies_registry_owned_websocket_routes_beyond_realtime_path() {
    let upstream_app = Router::new().route("/ws/custom/echo", get(websocket_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let registry = build_registry(vec![RouteDescriptor {
        service_id: "session-gateway".to_owned(),
        methods: vec![HttpMethod::Get],
        path_pattern: "/ws/custom/{*path}".to_owned(),
        visibility: RouteVisibility::Public,
        sdk_targets: vec![SdkTarget::SdkworkImSdk],
        operation_group: "realtime".to_owned(),
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols: vec![LINK_WEBSOCKET_SUBPROTOCOL.to_owned()],
    }])
    .expect("custom websocket route registry should build");

    let gateway_app = web_gateway::build_app_with_registry(
        test_gateway_config(vec![service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        )]),
        registry,
    );
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/ws/custom/echo")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL);

    let (mut socket, response) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed for custom route");
    assert_eq!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .expect("gateway should select websocket subprotocol"),
        LINK_WEBSOCKET_SUBPROTOCOL
    );

    socket
        .send(TungsteniteMessage::Text("hello custom route".into()))
        .await
        .expect("client text frame should send");
    let echoed = socket
        .next()
        .await
        .expect("echoed frame should arrive")
        .expect("echoed frame should decode");
    assert_eq!(
        echoed,
        TungsteniteMessage::Text("hello custom route".into())
    );

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
}
