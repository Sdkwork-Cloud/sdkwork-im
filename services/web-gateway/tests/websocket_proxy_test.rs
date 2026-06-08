use axum::{
    Json, Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    http::{HeaderMap, StatusCode, header},
    response::IntoResponse,
    routing::get,
};
use craw_chat_api_registry::{
    HttpMethod, RouteDescriptor, RouteProtocol, RouteVisibility, SdkTarget, build_registry,
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use craw_chat_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
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

async fn websocket_context_echo(headers: HeaderMap, ws: WebSocketUpgrade) -> impl IntoResponse {
    let tenant_id = header_value(&headers, "x-sdkwork-tenant-id").unwrap_or_default();
    let user_id = header_value(&headers, "x-sdkwork-user-id").unwrap_or_default();
    let session_id = header_value(&headers, "x-sdkwork-session-id").unwrap_or_default();
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let _ = socket
                .send(Message::Text(
                    json!({
                        "tenantId": tenant_id,
                        "userId": user_id,
                        "sessionId": session_id,
                    })
                    .to_string()
                    .into(),
                ))
                .await;
            let _ = socket.close().await;
        })
}

async fn websocket_signed_context_echo(
    headers: HeaderMap,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let result = im_app_context::resolve_app_context_with_signature_config(
        &headers,
        im_app_context::AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("gateway-signing-secret".to_owned()),
        },
    );
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let payload = match result {
                Ok(context) => json!({
                    "tenantId": context.tenant_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "signatureValid": true,
                }),
                Err(error) => json!({
                    "code": error.code(),
                    "message": error.message(),
                    "signatureValid": false,
                }),
            };
            let _ = socket.send(Message::Text(payload.to_string().into())).await;
            let _ = socket.close().await;
        })
}

async fn appbase_current_session(headers: HeaderMap) -> impl IntoResponse {
    let auth_token = header_value(&headers, header::AUTHORIZATION.as_str());
    let access_token = header_value(&headers, "access-token");
    if auth_token.as_deref() != Some("Bearer real-auth-token")
        || access_token.as_deref() != Some("real-access-token")
    {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "type": "about:blank",
                "title": "Unauthorized",
                "status": 401,
                "detail": "dual token session is required"
            })),
        )
            .into_response();
    }

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": "tenant_real",
                    "userId": "user_real",
                    "sessionId": "session_real",
                    "appId": "sdkwork-chat-pc",
                    "actorKind": "user"
                }
            }
        })),
    )
        .into_response()
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
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
async fn gateway_derives_realtime_websocket_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-appbase-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", "Bearer real-auth-token")
    .with_header("access-token", "real-access-token")
    .with_header("x-sdkwork-tenant-id", "t_demo")
    .with_header("x-sdkwork-user-id", "user_test006_a_com")
    .with_header("x-sdkwork-session-id", "spoofed_session");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    let received = socket
        .next()
        .await
        .expect("upstream context frame should arrive")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = received else {
        panic!("expected text context frame, got {received:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");

    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_signs_realtime_websocket_context_projection_when_signature_secret_is_configured() {
    let _signature_secret = ScopedEnvVar::set(
        "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET",
        "gateway-signing-secret",
    );
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app =
        Router::new().route("/im/v3/api/realtime/ws", get(websocket_signed_context_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-appbase-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", "Bearer real-auth-token")
    .with_header("access-token", "real-access-token")
    .with_header("x-sdkwork-tenant-id", "t_demo")
    .with_header("x-sdkwork-user-id", "user_test006_a_com")
    .with_header("x-sdkwork-context-signature", "spoofed-signature");

    let (mut socket, _) = connect_async(request)
        .await
        .expect("gateway websocket connection should succeed");
    let received = socket
        .next()
        .await
        .expect("upstream context frame should arrive")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = received else {
        panic!("expected text context frame, got {received:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");

    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["signatureValid"], true);

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_proxies_realtime_websocket_upgrade_and_frames() {
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_echo));
    let (upstream_address, upstream_handle) = spawn_server(upstream_app).await;
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;

    let gateway_app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(
            "session-gateway",
            format!("http://{upstream_address}").as_str(),
        ),
        service_upstream(
            "sdkwork-appbase-app-api",
            format!("http://{appbase_address}").as_str(),
        ),
    ]));
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws")
            .parse()
            .unwrap(),
    )
    .with_sub_protocol(LINK_WEBSOCKET_SUBPROTOCOL)
    .with_header("Authorization", "Bearer real-auth-token")
    .with_header("access-token", "real-access-token");

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
    appbase_handle.abort();
}

struct ScopedEnvVar {
    name: &'static str,
    previous: Option<String>,
}

impl ScopedEnvVar {
    fn set(name: &'static str, value: &str) -> Self {
        let previous = std::env::var(name).ok();
        unsafe {
            std::env::set_var(name, value);
        }
        Self { name, previous }
    }
}

impl Drop for ScopedEnvVar {
    fn drop(&mut self) {
        if let Some(previous) = &self.previous {
            unsafe {
                std::env::set_var(self.name, previous);
            }
            return;
        }

        unsafe {
            std::env::remove_var(self.name);
        }
    }
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
