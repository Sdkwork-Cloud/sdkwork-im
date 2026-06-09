use axum::{
    Json, Router,
    extract::{
        OriginalUri,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
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
use std::time::Duration;
use tokio::net::TcpListener;
use tokio::time::timeout;
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

fn embedded_gateway_config() -> WebGatewayConfig {
    WebGatewayConfig {
        bind_addr: "127.0.0.1:0".to_owned(),
        runtime_mode: GatewayRuntimeMode::Embedded,
        strict_startup: true,
        upstreams: Vec::new(),
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
    let device_id = header_value(&headers, "x-sdkwork-device-id").unwrap_or_default();
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let _ = socket
                .send(Message::Text(
                    json!({
                        "tenantId": tenant_id,
                        "userId": user_id,
                        "sessionId": session_id,
                        "deviceId": device_id,
                    })
                    .to_string()
                    .into(),
                ))
                .await;
            let _ = socket.close().await;
        })
}

async fn websocket_query_echo(
    OriginalUri(uri): OriginalUri,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    let query = uri.query().unwrap_or_default().to_owned();
    ws.protocols([LINK_WEBSOCKET_SUBPROTOCOL])
        .on_upgrade(move |mut socket| async move {
            let _ = socket
                .send(Message::Text(
                    json!({
                        "query": query,
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

async fn next_text_json(
    socket: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> serde_json::Value {
    let message = timeout(Duration::from_secs(12), socket.next())
        .await
        .expect("websocket frame should arrive before timeout")
        .expect("websocket should remain open")
        .expect("websocket frame should decode");
    let TungsteniteMessage::Text(text) = message else {
        panic!("expected text frame, got {message:?}");
    };
    serde_json::from_str(text.as_str()).expect("websocket text frame should be json")
}

#[tokio::test]
async fn gateway_accepts_browser_realtime_websocket_auth_init_before_upstream_connect() {
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
        format!("ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device-frame")
            .parse()
            .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-1",
                "authToken": "real-auth-token",
                "accessToken": "real-access-token",
                "deviceId": "device-frame"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let auth_ok = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    let TungsteniteMessage::Text(text) = auth_ok else {
        panic!("expected auth.ok text frame, got {auth_ok:?}");
    };
    let auth_ok: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth.ok frame should be json");
    assert_eq!(auth_ok["type"], "auth.ok");
    assert_eq!(auth_ok["requestId"], "auth-1");
    assert_eq!(auth_ok["tenantId"], "tenant_real");
    assert_eq!(auth_ok["principalId"], "user_real");
    assert_eq!(auth_ok["sessionId"], "session_real");
    assert_eq!(auth_ok["deviceId"], "device-frame");

    let context_frame = socket
        .next()
        .await
        .expect("upstream context frame should arrive after auth.ok")
        .expect("upstream context frame should decode");
    let TungsteniteMessage::Text(text) = context_frame else {
        panic!("expected context text frame, got {context_frame:?}");
    };
    let context: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("context frame should be json");
    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["deviceId"], "device-frame");

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn embedded_gateway_accepts_browser_realtime_websocket_without_session_gateway_upstream() {
    let gateway_app = web_gateway::build_app_with_registry_and_runtime_routers(
        embedded_gateway_config(),
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(web_gateway::build_embedded_appbase_im_runtime_router()),
        None,
    );
    let (gateway_address, gateway_handle) = spawn_server(gateway_app).await;

    let registration = reqwest::Client::new()
        .post(format!(
            "http://{gateway_address}/app/v3/api/auth/registrations"
        ))
        .json(&json!({
            "channel": "EMAIL",
            "confirmPassword": "dev123456",
            "email": "embedded-ws-user@sdkwork-iam.local",
            "name": "Embedded Websocket User",
            "password": "dev123456",
            "username": "embedded-ws-user"
        }))
        .send()
        .await
        .expect("embedded registration should return response");
    assert_eq!(registration.status(), StatusCode::OK);
    let registration: serde_json::Value = registration
        .json()
        .await
        .expect("embedded registration should return json");
    let auth_token = registration["data"]["authToken"]
        .as_str()
        .expect("registration should return auth token");
    let access_token = registration["data"]["accessToken"]
        .as_str()
        .expect("registration should return access token");
    let tenant_id = registration["data"]["context"]["tenantId"]
        .as_str()
        .expect("registration should return tenant context")
        .to_owned();
    let user_id = registration["data"]["user"]["userId"]
        .as_str()
        .expect("registration should return user id")
        .to_owned();
    assert_ne!(tenant_id, "t_demo");
    assert!(user_id.starts_with("iamu_"));

    let current_session = reqwest::Client::new()
        .get(format!(
            "http://{gateway_address}/app/v3/api/auth/sessions/current"
        ))
        .header("authorization", format!("Bearer {auth_token}"))
        .header("access-token", access_token)
        .send()
        .await
        .expect("embedded current session should return response");
    assert_eq!(current_session.status(), StatusCode::OK);
    let current_session: serde_json::Value = current_session
        .json()
        .await
        .expect("embedded current session should return json");
    assert_eq!(current_session["data"]["context"]["tenantId"], tenant_id);
    assert_eq!(current_session["data"]["context"]["userId"], user_id);

    let websocket_request = ClientRequestBuilder::new(
        format!("ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device-frame")
            .parse()
            .unwrap(),
    );

    let (mut socket, response) = connect_async(websocket_request)
        .await
        .expect("embedded gateway websocket should upgrade before auth.init");
    assert!(
        response
            .headers()
            .get(tokio_tungstenite::tungstenite::http::header::SEC_WEBSOCKET_PROTOCOL)
            .is_none(),
        "auth.init browser websocket flow must not negotiate the IM CCP subprotocol before gateway authentication"
    );

    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "embedded-auth-1",
                "authToken": auth_token,
                "accessToken": access_token,
                "deviceId": "device-frame"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("embedded auth.init frame should send");

    let auth_ok = next_text_json(&mut socket).await;
    assert_eq!(auth_ok["type"], "auth.ok");
    assert_eq!(auth_ok["requestId"], "embedded-auth-1");
    assert_eq!(auth_ok["tenantId"], tenant_id);
    assert_eq!(auth_ok["principalId"], user_id);
    assert_eq!(auth_ok["deviceId"], "device-frame");

    let connected = next_text_json(&mut socket).await;
    assert_eq!(connected["type"], "realtime.connected");
    assert_eq!(connected["tenantId"], tenant_id);
    assert_eq!(connected["principalId"], user_id);
    assert_eq!(connected["deviceId"], "device-frame");

    let _ = socket.close(None).await;
    gateway_handle.abort();
}

#[tokio::test]
async fn gateway_strips_sensitive_realtime_websocket_query_before_upstream_connect() {
    let appbase_app = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let (appbase_address, appbase_handle) = spawn_server(appbase_app).await;
    let upstream_app = Router::new().route("/im/v3/api/realtime/ws", get(websocket_query_echo));
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
        format!(
            "ws://{gateway_address}/im/v3/api/realtime/ws?deviceId=device-frame&conversationId=conversation-leak&authToken=query-auth-token&accessToken=query-access-token&token=query-token&authorization=query-authorization&refreshToken=query-refresh-token"
        )
        .parse()
        .unwrap(),
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "auth.init",
                "requestId": "auth-sensitive-query",
                "authToken": "real-auth-token",
                "accessToken": "real-access-token",
                "deviceId": "device-frame"
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("auth.init frame should send");

    let auth_ok = socket
        .next()
        .await
        .expect("auth.ok frame should arrive")
        .expect("auth.ok frame should decode");
    assert!(matches!(auth_ok, TungsteniteMessage::Text(_)));

    let query_frame = socket
        .next()
        .await
        .expect("upstream query frame should arrive after auth.ok")
        .expect("upstream query frame should decode");
    let TungsteniteMessage::Text(text) = query_frame else {
        panic!("expected query text frame, got {query_frame:?}");
    };
    let query: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("query frame should be json");
    let query = query["query"]
        .as_str()
        .expect("query frame should expose upstream query");
    assert_eq!(query, "deviceId=device-frame");
    assert!(!query.contains("conversationId"));
    assert!(!query.contains("authToken"));
    assert!(!query.contains("accessToken"));
    assert!(!query.contains("authorization"));
    assert!(!query.contains("refreshToken"));
    assert!(!query.contains("query-token"));

    let _ = socket.close(None).await;
    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
}

#[tokio::test]
async fn gateway_rejects_realtime_websocket_business_frame_before_auth_init() {
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
    );

    let (mut socket, _) = connect_async(request)
        .await
        .expect("browser websocket connection should upgrade before auth.init");
    socket
        .send(TungsteniteMessage::Text(
            json!({
                "type": "subscriptions.sync",
                "requestId": "sub-before-auth",
                "items": []
            })
            .to_string()
            .into(),
        ))
        .await
        .expect("pre-auth business frame should send");

    let error_frame = socket
        .next()
        .await
        .expect("auth error frame should arrive")
        .expect("auth error frame should decode");
    let TungsteniteMessage::Text(text) = error_frame else {
        panic!("expected auth error text frame, got {error_frame:?}");
    };
    let error: serde_json::Value =
        serde_json::from_str(text.as_str()).expect("auth error frame should be json");
    assert_eq!(error["type"], "error");
    assert_eq!(error["code"], "websocket_auth_required");
    assert_eq!(error["requestId"], "sub-before-auth");

    let close_frame = socket
        .next()
        .await
        .expect("gateway should close websocket after auth error")
        .expect("close frame should decode");
    assert!(matches!(close_frame, TungsteniteMessage::Close(_)));

    gateway_handle.abort();
    upstream_handle.abort();
    appbase_handle.abort();
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
