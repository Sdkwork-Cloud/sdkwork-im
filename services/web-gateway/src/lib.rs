use axum::{
    Json, Router,
    body::Body,
    extract::{
        Path, Request, State,
        ws::{CloseFrame, Message, Utf8Bytes, WebSocket, WebSocketUpgrade},
    },
    http::{HeaderMap, HeaderValue, Method, StatusCode, header},
    response::Html,
    response::{IntoResponse, Response},
    routing::get,
};
use craw_chat_api_registry::{
    ContractKind, HttpMethod, RouteDescriptor, RouteProtocol, RouteRegistry, RouteVisibility,
    SdkTarget, ServiceSchemaIndexEntry, build_registry, sdk_contract_summaries,
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig};
use craw_chat_gateway_observability::{
    GatewayStartupSummary, build_startup_summary_with_registry, route_summaries,
    surface_group_summaries,
};
use craw_chat_openapi::{OpenApiServiceSpec, render_docs_html};
use craw_chat_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use futures_util::{SinkExt, StreamExt};
use im_app_context::sign_app_context_headers;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::{Map, Value, json};
use std::collections::BTreeSet;
use std::time::Duration;
use tokio_tungstenite::{
    MaybeTlsStream, WebSocketStream, connect_async, tungstenite,
    tungstenite::client::IntoClientRequest,
};
use tower::ServiceExt;
use tower_http::cors::{AllowHeaders, AllowMethods, AllowOrigin, CorsLayer};

const BROWSER_ORIGINS_ENV: &str = "CRAW_CHAT_BROWSER_ORIGINS";
const APPBASE_APP_API_SERVICE_ID: &str = "sdkwork-appbase-app-api";
const APP_CONTEXT_SIGNATURE_SECRET_ENV: &str = "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET";
const IM_REALTIME_WEBSOCKET_PATH: &str = "/im/v3/api/realtime/ws";
const WEBSOCKET_AUTH_INIT_TIMEOUT_SECONDS: u64 = 10;

#[derive(Clone)]
struct GatewayState {
    client: Client,
    config: WebGatewayConfig,
    registry: RouteRegistry,
    embedded_runtime_router: Option<Router>,
    product_runtime_router: Option<Router>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct GatewayHealthResponse {
    status: &'static str,
    service: &'static str,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct WebsocketAuthInitFrame {
    #[serde(rename = "type")]
    frame_type: String,
    request_id: Option<String>,
    auth_token: Option<String>,
    access_token: Option<String>,
    device_id: Option<String>,
}

pub fn build_app(config: WebGatewayConfig) -> Router {
    build_app_with_registry(
        config,
        build_gateway_registry().expect("web-gateway route registry should build"),
    )
}

pub fn build_app_with_registry(config: WebGatewayConfig, registry: RouteRegistry) -> Router {
    build_app_with_registry_and_product_runtime(config, registry, None)
}

pub fn build_app_with_registry_and_product_runtime(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    product_runtime_router: Option<Router>,
) -> Router {
    build_app_with_registry_and_runtime_routers(config, registry, None, product_runtime_router)
}

pub fn build_app_with_registry_and_runtime_routers(
    config: WebGatewayConfig,
    registry: RouteRegistry,
    embedded_runtime_router: Option<Router>,
    product_runtime_router: Option<Router>,
) -> Router {
    Router::new()
        .route("/healthz", get(healthz))
        .route("/readyz", get(readyz))
        .route("/openapi.json", get(openapi_json))
        .route("/openapi/index.json", get(openapi_index_json))
        .route(
            "/openapi/runtime-summary.json",
            get(openapi_runtime_summary_json),
        )
        .route(
            "/openapi/services/{service_schema}",
            get(service_openapi_json),
        )
        .route("/docs", get(docs))
        .route("/docs/services/{service_id}", get(service_docs))
        .route("/", gateway_proxy_routes())
        .route("/{*path}", gateway_proxy_routes())
        .with_state(GatewayState {
            client: Client::new(),
            config,
            registry,
            embedded_runtime_router,
            product_runtime_router,
        })
        .layer(build_browser_cors_layer())
}

async fn healthz() -> Json<GatewayHealthResponse> {
    Json(GatewayHealthResponse {
        status: "ok",
        service: "web-gateway",
    })
}

async fn readyz() -> Json<GatewayHealthResponse> {
    Json(GatewayHealthResponse {
        status: "ok",
        service: "web-gateway",
    })
}

async fn openapi_json(State(state): State<GatewayState>) -> Result<Json<Value>, Response> {
    let documents = fetch_service_openapi_documents(&state).await?;
    Ok(Json(build_aggregate_openapi_document(&documents)))
}

async fn openapi_index_json(State(state): State<GatewayState>) -> Json<Value> {
    Json(json!({
        "sdkContracts": sdk_contract_summaries(""),
        "services": service_schema_index_entries(&state.config, &state.registry),
        "routes": route_summaries(&state.registry),
        "surfaceGroups": surface_group_summaries(&state.registry),
    }))
}

async fn openapi_runtime_summary_json(
    State(state): State<GatewayState>,
    request: Request,
) -> Json<GatewayStartupSummary> {
    Json(build_startup_summary_with_registry(
        &state.config,
        &state.registry,
        request_base_url(&request),
    ))
}

async fn service_openapi_json(
    Path(service_schema): Path<String>,
    State(state): State<GatewayState>,
) -> Result<Json<Value>, Response> {
    let service_id = service_schema
        .strip_suffix(".openapi.json")
        .unwrap_or(service_schema.as_str());
    Ok(Json(
        fetch_service_openapi_document(&state, service_id).await?,
    ))
}

async fn docs() -> Html<String> {
    Html(render_docs_html(&aggregate_gateway_openapi_spec()))
}

async fn service_docs(Path(service_id): Path<String>) -> Html<String> {
    Html(render_docs_html(&service_openapi_spec(service_id.as_str())))
}

async fn proxy_get_request(
    websocket_upgrade: Result<
        WebSocketUpgrade,
        axum::extract::ws::rejection::WebSocketUpgradeRejection,
    >,
    State(state): State<GatewayState>,
    request: Request,
) -> Response {
    let route = state
        .registry
        .resolve(HttpMethod::Get, request.uri().path());
    if let (Some(route), Ok(websocket_upgrade)) = (route, websocket_upgrade)
        && route.protocol == RouteProtocol::Websocket
    {
        return proxy_websocket_request(
            websocket_upgrade,
            request,
            &state,
            route.service_id.as_str(),
            route.websocket_subprotocols.as_slice(),
        )
        .await;
    }

    if route.is_some() {
        return proxy_request(State(state), request).await;
    }

    delegate_to_runtime_router(
        runtime_router_for_path(&state, request.uri().path()),
        request,
    )
    .await
}

async fn proxy_websocket_request(
    ws: WebSocketUpgrade,
    mut request: Request,
    state: &GatewayState,
    service_id: &str,
    websocket_subprotocols: &[String],
) -> Response {
    let Some(upstream_base_url) = state.config.upstream_base_url(service_id) else {
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream target is not configured for {service_id}").as_str(),
        );
    };
    if should_authenticate_realtime_websocket_with_init_frame(
        service_id,
        request.uri().path(),
        request.headers(),
    ) {
        let path_and_query = request
            .uri()
            .path_and_query()
            .map(|value| value.as_str().to_owned())
            .unwrap_or_else(|| "/".to_owned());
        let original_headers = request.headers().clone();
        let state = state.clone();
        let upstream_base_url = upstream_base_url.to_owned();
        return ws
            .protocols(websocket_subprotocols.to_vec())
            .on_upgrade(move |downstream_socket| {
                proxy_realtime_websocket_after_auth_init(
                    downstream_socket,
                    state,
                    upstream_base_url,
                    path_and_query,
                    original_headers,
                )
            })
            .into_response();
    }
    if should_resolve_proxied_context_from_appbase_session(service_id, request.uri().path())
        && let Err(response) =
            inject_appbase_session_context_for_configured_upstream(state, &mut request).await
    {
        return response;
    }
    let Ok(upstream_url) = upstream_websocket_url(
        upstream_base_url,
        request
            .uri()
            .path_and_query()
            .map(|value| value.as_str())
            .unwrap_or("/"),
    ) else {
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!(
                "gateway websocket upstream URL is invalid for {}",
                service_id
            )
            .as_str(),
        );
    };
    let mut upstream_request = match upstream_url.as_str().into_client_request() {
        Ok(request) => request,
        Err(error) => {
            return json_error_response(
                StatusCode::BAD_GATEWAY,
                format!(
                    "gateway failed to prepare websocket upstream request for {}: {error}",
                    service_id
                )
                .as_str(),
            );
        }
    };
    copy_websocket_headers(request.headers(), upstream_request.headers_mut());

    match connect_async(upstream_request).await {
        Ok((upstream_socket, _)) => ws
            .protocols(websocket_subprotocols.to_vec())
            .on_upgrade(move |downstream_socket| {
                proxy_websocket_streams(downstream_socket, upstream_socket)
            })
            .into_response(),
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!(
                "gateway websocket upstream request to {} failed: {error}",
                service_id
            )
            .as_str(),
        ),
    }
}

async fn proxy_request(State(state): State<GatewayState>, mut request: Request) -> Response {
    let Some(registry_method) = map_http_method(request.method()) else {
        return json_error_response(
            StatusCode::METHOD_NOT_ALLOWED,
            "gateway does not support proxying this method",
        );
    };
    let Some(route) = state
        .registry
        .resolve(registry_method, request.uri().path())
    else {
        let (parts, body) = request.into_parts();
        return delegate_to_runtime_router(
            runtime_router_for_path(&state, parts.uri.path()),
            Request::from_parts(parts, body),
        )
        .await;
    };
    let service_id = route.service_id.clone();
    let Some(upstream_base_url) = state.config.upstream_base_url(service_id.as_str()) else {
        if state.config.runtime_mode == GatewayRuntimeMode::Embedded {
            let Some(runtime_router) =
                runtime_router_for_missing_embedded_upstream(&state, service_id.as_str())
            else {
                return json_error_response(
                    StatusCode::BAD_GATEWAY,
                    format!("upstream target is not configured for {service_id}").as_str(),
                );
            };
            return delegate_to_runtime_router(Some(runtime_router), request).await;
        }
        return json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream target is not configured for {service_id}").as_str(),
        );
    };
    if should_resolve_proxied_context_from_appbase_session(
        service_id.as_str(),
        request.uri().path(),
    ) && let Err(response) =
        inject_appbase_session_context_for_configured_upstream(&state, &mut request).await
    {
        return response;
    }
    let (parts, body) = request.into_parts();
    let method = parts.method;
    let headers = parts.headers;
    let uri = parts.uri;
    let upstream_url = format!(
        "{}{}",
        upstream_base_url.trim_end_matches('/'),
        uri.path_and_query()
            .map(|value| value.as_str())
            .unwrap_or("/")
    );
    let mut request_builder = state.client.request(method, upstream_url);

    for (name, value) in headers.iter() {
        if *name == header::HOST || *name == header::CONTENT_LENGTH || *name == header::CONNECTION {
            continue;
        }
        request_builder = request_builder.header(name, value);
    }
    let body = match axum::body::to_bytes(body, usize::MAX).await {
        Ok(body) => body,
        Err(error) => {
            return json_error_response(
                StatusCode::BAD_REQUEST,
                format!("gateway failed to read request body: {error}").as_str(),
            );
        }
    };

    match request_builder.body(body).send().await {
        Ok(upstream_response) => build_proxy_response(service_id.as_str(), upstream_response).await,
        Err(error) => json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway upstream request to {service_id} failed: {error}").as_str(),
        ),
    }
}

async fn delegate_to_runtime_router(
    runtime_router: Option<Router>,
    mut request: Request,
) -> Response {
    let Some(router) = runtime_router else {
        return json_error_response(StatusCode::NOT_FOUND, "gateway route owner not found");
    };

    request.extensions_mut().clear();
    if should_resolve_im_runtime_context_from_appbase_session(request.uri().path())
        && let Err(response) =
            inject_appbase_session_context_for_embedded_runtime(router.clone(), &mut request).await
    {
        return response;
    }

    match router.oneshot(request).await {
        Ok(response) => response,
        Err(error) => match error {},
    }
}

async fn inject_appbase_session_context_for_embedded_runtime(
    router: Router,
    request: &mut Request,
) -> Result<(), Response> {
    let session_response = router
        .oneshot(build_current_session_request(request.headers()))
        .await
        .map_err(|error| match error {})?;
    let status = session_response.status();
    let (parts, body) = session_response.into_parts();
    let body = axum::body::to_bytes(body, usize::MAX)
        .await
        .map_err(|error| {
            json_error_response(
                StatusCode::BAD_GATEWAY,
                format!("gateway failed to read appbase current-session response: {error}")
                    .as_str(),
            )
        })?;

    if !status.is_success() {
        return Err(Response::from_parts(parts, Body::from(body)));
    }

    let session: Value = serde_json::from_slice(&body).map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway failed to decode appbase current-session response: {error}").as_str(),
        )
    })?;
    let context_headers = appbase_session_context_headers(&session).ok_or_else(|| {
        json_error_response(
            StatusCode::UNAUTHORIZED,
            "appbase current-session response is missing tenant/user context",
        )
    })?;
    replace_sdkwork_context_headers(request.headers_mut(), context_headers)?;
    Ok(())
}

async fn inject_appbase_session_context_for_configured_upstream(
    state: &GatewayState,
    request: &mut Request,
) -> Result<(), Response> {
    let auth_header = request.headers().get(header::AUTHORIZATION).cloned();
    let access_token = request.headers().get("access-token").cloned();
    let context_headers =
        fetch_appbase_session_context_headers(state, auth_header, access_token).await?;
    replace_sdkwork_context_headers(request.headers_mut(), context_headers)?;
    Ok(())
}

async fn fetch_appbase_session_context_headers(
    state: &GatewayState,
    auth_header: Option<HeaderValue>,
    access_token: Option<HeaderValue>,
) -> Result<Vec<(&'static str, HeaderValue)>, Response> {
    let Some(appbase_base_url) = state.config.upstream_base_url(APPBASE_APP_API_SERVICE_ID) else {
        return Err(json_error_response(
            StatusCode::BAD_GATEWAY,
            "upstream target is not configured for sdkwork-appbase-app-api",
        ));
    };
    let current_session_url = format!(
        "{}/app/v3/api/auth/sessions/current",
        appbase_base_url.trim_end_matches('/')
    );
    let mut request_builder = state.client.get(current_session_url);
    if let Some(value) = auth_header {
        request_builder = request_builder.header(header::AUTHORIZATION, value);
    }
    if let Some(value) = access_token {
        request_builder = request_builder.header("access-token", value);
    }

    let response = request_builder.send().await.map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway appbase current-session upstream request failed: {error}").as_str(),
        )
    })?;
    let status = response.status();
    let headers = response.headers().clone();
    let body = response.bytes().await.unwrap_or_default();
    if !status.is_success() {
        return Err(build_raw_response(status, &headers, Body::from(body)));
    }
    let session: Value = serde_json::from_slice(&body).map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway failed to decode appbase current-session response: {error}").as_str(),
        )
    })?;
    let context_headers = appbase_session_context_headers(&session).ok_or_else(|| {
        json_error_response(
            StatusCode::UNAUTHORIZED,
            "appbase current-session response is missing tenant/user context",
        )
    })?;
    Ok(context_headers)
}

fn build_current_session_request(headers: &HeaderMap) -> Request {
    let mut request = Request::builder()
        .method(Method::GET)
        .uri("/app/v3/api/auth/sessions/current")
        .body(Body::empty())
        .expect("gateway current-session request should build");
    if let Some(value) = headers.get(header::AUTHORIZATION) {
        request
            .headers_mut()
            .insert(header::AUTHORIZATION, value.clone());
    }
    if let Some(value) = headers.get("access-token") {
        request.headers_mut().insert("access-token", value.clone());
    }
    request
}

fn should_resolve_im_runtime_context_from_appbase_session(path: &str) -> bool {
    path.starts_with("/im/v3/api/")
}

fn should_resolve_proxied_context_from_appbase_session(service_id: &str, path: &str) -> bool {
    match service_id {
        "session-gateway" => {
            path.starts_with("/im/v3/api/realtime/") || path.starts_with("/im/v3/api/presence/")
        }
        "conversation-runtime" => path.starts_with("/im/v3/api/chat/"),
        "projection-service" => path.starts_with("/im/v3/api/chat/"),
        "streaming-service" => path.starts_with("/im/v3/api/streams"),
        "media-service" => path.starts_with("/im/v3/api/media/"),
        "sdkwork-rtc-signaling-service" => path.starts_with("/app/v3/api/rtc/"),
        "sdkwork-drive-app-api" => path.starts_with("/app/v3/api/drive/"),
        "notification-service" => path.starts_with("/app/v3/api/notifications"),
        "automation-service" => path.starts_with("/app/v3/api/automation/"),
        _ => false,
    }
}

fn should_authenticate_realtime_websocket_with_init_frame(
    service_id: &str,
    path: &str,
    headers: &HeaderMap,
) -> bool {
    service_id == "session-gateway"
        && path == IM_REALTIME_WEBSOCKET_PATH
        && (!headers.contains_key(header::AUTHORIZATION) || !headers.contains_key("access-token"))
}

async fn proxy_realtime_websocket_after_auth_init(
    mut downstream_socket: WebSocket,
    state: GatewayState,
    upstream_base_url: String,
    path_and_query: String,
    original_headers: HeaderMap,
) {
    let Some(auth_init) = read_websocket_auth_init_frame(&mut downstream_socket).await else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            None,
            "websocket_auth_required",
            "auth.init frame is required before realtime websocket frames",
        )
        .await;
        return;
    };
    if auth_init.frame_type != "auth.init" {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_auth_required",
            "auth.init frame is required before realtime websocket frames",
        )
        .await;
        return;
    }

    let Some(auth_header) = websocket_auth_init_authorization_header(&auth_init) else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_auth_required",
            "auth.init authToken is required",
        )
        .await;
        return;
    };
    let Some(access_token) = websocket_auth_init_access_token_header(&auth_init) else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_auth_required",
            "auth.init accessToken is required",
        )
        .await;
        return;
    };
    let mut context_headers =
        match fetch_appbase_session_context_headers(&state, Some(auth_header), Some(access_token))
            .await
        {
            Ok(headers) => headers,
            Err(_) => {
                close_websocket_with_auth_error(
                    &mut downstream_socket,
                    auth_init.request_id.as_deref(),
                    "websocket_auth_failed",
                    "websocket auth.init session validation failed",
                )
                .await;
                return;
            }
        };
    merge_websocket_auth_init_device_header(&mut context_headers, auth_init.device_id.as_deref());
    let auth_ok_context = websocket_auth_ok_context(&context_headers);

    let Ok(upstream_url) = upstream_websocket_url(upstream_base_url.as_str(), path_and_query.as_str())
    else {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_upstream_unavailable",
            "gateway websocket upstream URL is invalid",
        )
        .await;
        return;
    };
    let mut upstream_request = match upstream_url.as_str().into_client_request() {
        Ok(request) => request,
        Err(_) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_upstream_unavailable",
                "gateway failed to prepare websocket upstream request",
            )
            .await;
            return;
        }
    };
    copy_websocket_headers(&original_headers, upstream_request.headers_mut());
    if let Err(()) =
        replace_sdkwork_context_headers_for_upstream(upstream_request.headers_mut(), context_headers)
    {
        close_websocket_with_auth_error(
            &mut downstream_socket,
            auth_init.request_id.as_deref(),
            "websocket_auth_failed",
            "gateway failed to prepare websocket auth context",
        )
        .await;
        return;
    }

    match connect_async(upstream_request).await {
        Ok((upstream_socket, _)) => {
            send_websocket_auth_ok(&mut downstream_socket, &auth_init, &auth_ok_context).await;
            proxy_websocket_streams(downstream_socket, upstream_socket).await;
        }
        Err(_) => {
            close_websocket_with_auth_error(
                &mut downstream_socket,
                auth_init.request_id.as_deref(),
                "websocket_upstream_unavailable",
                "gateway websocket upstream request failed after auth.init",
            )
            .await;
        }
    }
}

async fn read_websocket_auth_init_frame(socket: &mut WebSocket) -> Option<WebsocketAuthInitFrame> {
    let next_message = tokio::time::timeout(
        Duration::from_secs(WEBSOCKET_AUTH_INIT_TIMEOUT_SECONDS),
        socket.next(),
    )
    .await
    .ok()??;
    let Ok(message) = next_message else {
        return None;
    };
    let text = match message {
        Message::Text(text) => text.to_string(),
        Message::Binary(bytes) => String::from_utf8(bytes.to_vec()).ok()?,
        Message::Close(_) => return None,
        Message::Ping(payload) => {
            let _ = socket.send(Message::Pong(payload)).await;
            return None;
        }
        Message::Pong(_) => return None,
    };
    serde_json::from_str::<WebsocketAuthInitFrame>(text.as_str()).ok()
}

fn websocket_auth_init_authorization_header(frame: &WebsocketAuthInitFrame) -> Option<HeaderValue> {
    let token = frame.auth_token.as_deref()?.trim();
    if token.is_empty() {
        return None;
    }
    HeaderValue::from_str(normalize_websocket_auth_token(token).as_str()).ok()
}

fn websocket_auth_init_access_token_header(frame: &WebsocketAuthInitFrame) -> Option<HeaderValue> {
    let token = frame.access_token.as_deref()?.trim();
    if token.is_empty() {
        return None;
    }
    HeaderValue::from_str(token).ok()
}

fn normalize_websocket_auth_token(token: &str) -> String {
    if token
        .get(..7)
        .is_some_and(|prefix| prefix.eq_ignore_ascii_case("Bearer "))
    {
        token.to_owned()
    } else {
        format!("Bearer {token}")
    }
}

fn merge_websocket_auth_init_device_header(
    context_headers: &mut Vec<(&'static str, HeaderValue)>,
    device_id: Option<&str>,
) {
    if context_headers
        .iter()
        .any(|(name, _)| *name == "x-sdkwork-device-id")
    {
        return;
    }
    let Some(device_id) = device_id.map(str::trim).filter(|value| !value.is_empty()) else {
        return;
    };
    if let Ok(value) = HeaderValue::from_str(device_id) {
        context_headers.push(("x-sdkwork-device-id", value));
    }
}

fn websocket_auth_ok_context(
    context_headers: &[(&'static str, HeaderValue)],
) -> Map<String, Value> {
    let mut context = Map::new();
    if let Some(value) = context_header_value(context_headers, "x-sdkwork-tenant-id") {
        context.insert("tenantId".to_owned(), Value::String(value));
    }
    if let Some(value) = context_header_value(context_headers, "x-sdkwork-user-id") {
        context.insert("principalId".to_owned(), Value::String(value));
    }
    if let Some(value) = context_header_value(context_headers, "x-sdkwork-session-id") {
        context.insert("sessionId".to_owned(), Value::String(value));
    }
    if let Some(value) = context_header_value(context_headers, "x-sdkwork-device-id") {
        context.insert("deviceId".to_owned(), Value::String(value));
    }
    context
}

fn context_header_value(
    context_headers: &[(&'static str, HeaderValue)],
    target_name: &str,
) -> Option<String> {
    context_headers
        .iter()
        .find(|(name, _)| *name == target_name)
        .and_then(|(_, value)| value.to_str().ok())
        .map(str::to_owned)
}

fn replace_sdkwork_context_headers_for_upstream(
    headers: &mut HeaderMap,
    context_headers: Vec<(&'static str, HeaderValue)>,
) -> Result<(), ()> {
    replace_sdkwork_context_headers(headers, context_headers).map_err(|_| ())
}

async fn send_websocket_auth_ok(
    socket: &mut WebSocket,
    frame: &WebsocketAuthInitFrame,
    context: &Map<String, Value>,
) {
    let mut payload = Map::new();
    payload.insert("type".to_owned(), Value::String("auth.ok".to_owned()));
    payload.insert(
        "requestId".to_owned(),
        frame
            .request_id
            .as_ref()
            .map(|value| Value::String(value.clone()))
            .unwrap_or(Value::Null),
    );
    for (name, value) in context {
        payload.insert(name.clone(), value.clone());
    }
    let _ = socket
        .send(Message::Text(Value::Object(payload).to_string().into()))
        .await;
}

async fn close_websocket_with_auth_error(
    socket: &mut WebSocket,
    request_id: Option<&str>,
    code: &str,
    message: &str,
) {
    let _ = socket
        .send(Message::Text(
            json!({
                "type": "error",
                "requestId": request_id,
                "code": code,
                "message": message,
            })
            .to_string()
            .into(),
        ))
        .await;
    let _ = socket
        .send(Message::Close(Some(CloseFrame {
            code: axum::extract::ws::close_code::POLICY,
            reason: Utf8Bytes::from(code.to_owned()),
        })))
        .await;
}

fn remove_sdkwork_context_headers(headers: &mut HeaderMap) {
    for name in [
        "x-sdkwork-app-id",
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-environment",
        "x-sdkwork-deployment-mode",
        "x-sdkwork-auth-level",
        "x-sdkwork-data-scope",
        "x-sdkwork-permission-scope",
        "x-sdkwork-actor-id",
        "x-sdkwork-actor-kind",
        "x-sdkwork-device-id",
        "x-sdkwork-context-signature",
    ] {
        headers.remove(name);
    }
}

fn replace_sdkwork_context_headers(
    headers: &mut HeaderMap,
    context_headers: Vec<(&'static str, HeaderValue)>,
) -> Result<(), Response> {
    remove_sdkwork_context_headers(headers);
    for (name, value) in context_headers {
        headers.insert(name, value);
    }
    sign_sdkwork_context_headers_if_configured(headers)
}

fn sign_sdkwork_context_headers_if_configured(headers: &mut HeaderMap) -> Result<(), Response> {
    let Some(secret) = std::env::var(APP_CONTEXT_SIGNATURE_SECRET_ENV)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
    else {
        return Ok(());
    };
    let signature = sign_app_context_headers(headers, secret.as_str()).map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway failed to sign app context projection: {error}").as_str(),
        )
    })?;
    let signature = HeaderValue::from_str(signature.as_str()).map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("gateway produced invalid app context signature header: {error}").as_str(),
        )
    })?;
    headers.insert("x-sdkwork-context-signature", signature);
    Ok(())
}

fn appbase_session_context_headers(session: &Value) -> Option<Vec<(&'static str, HeaderValue)>> {
    let context = session
        .get("data")
        .and_then(|data| data.get("context"))
        .or_else(|| session.get("context"))?;
    let tenant_id = json_string(context, &["tenantId", "tenant_id"])?;
    let user_id = json_string(context, &["userId", "user_id"])?;

    let mut headers = Vec::new();
    push_context_header(&mut headers, "x-sdkwork-tenant-id", tenant_id);
    push_context_header(&mut headers, "x-sdkwork-user-id", user_id);
    if let Some(value) = json_string(context, &["appId", "app_id"]) {
        push_context_header(&mut headers, "x-sdkwork-app-id", value);
    }
    if let Some(value) = json_string(context, &["organizationId", "organization_id"]) {
        push_context_header(&mut headers, "x-sdkwork-organization-id", value);
    }
    if let Some(value) = json_string(context, &["sessionId", "session_id"]) {
        push_context_header(&mut headers, "x-sdkwork-session-id", value);
    }
    if let Some(value) = json_string(context, &["environment", "env"]) {
        push_context_header(&mut headers, "x-sdkwork-environment", value);
    }
    if let Some(value) = json_string(context, &["deploymentMode", "deployment_mode"]) {
        push_context_header(&mut headers, "x-sdkwork-deployment-mode", value);
    }
    if let Some(value) = json_string(context, &["authLevel", "auth_level"]) {
        push_context_header(&mut headers, "x-sdkwork-auth-level", value);
    }
    if let Some(value) = json_string(context, &["actorId", "actor_id"]) {
        push_context_header(&mut headers, "x-sdkwork-actor-id", value);
    }
    if let Some(value) = json_string(context, &["actorKind", "actor_kind"]) {
        push_context_header(&mut headers, "x-sdkwork-actor-kind", value);
    }
    if let Some(value) = json_string(context, &["deviceId", "device_id"]) {
        push_context_header(&mut headers, "x-sdkwork-device-id", value);
    }
    if let Some(value) = json_string_array(context, &["dataScope", "data_scope"]) {
        push_context_header(&mut headers, "x-sdkwork-data-scope", value);
    }
    if let Some(value) = json_string_array(context, &["permissionScope", "permission_scope"]) {
        push_context_header(&mut headers, "x-sdkwork-permission-scope", value);
    }

    Some(headers)
}

fn push_context_header(
    headers: &mut Vec<(&'static str, HeaderValue)>,
    name: &'static str,
    value: String,
) {
    if let Ok(value) = HeaderValue::from_str(value.as_str()) {
        headers.push((name, value));
    }
}

fn json_string(value: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        value.get(*name).and_then(|raw| {
            raw.as_str()
                .map(str::trim)
                .filter(|value| !value.is_empty() && *value != "0")
                .map(str::to_owned)
                .or_else(|| {
                    raw.as_i64()
                        .filter(|value| *value != 0)
                        .map(|value| value.to_string())
                })
                .or_else(|| {
                    raw.as_u64()
                        .filter(|value| *value != 0)
                        .map(|value| value.to_string())
                })
        })
    })
}

fn json_string_array(value: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        value.get(*name).and_then(|raw| {
            raw.as_array()
                .map(|items| {
                    items
                        .iter()
                        .filter_map(Value::as_str)
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .collect::<Vec<_>>()
                })
                .filter(|items| !items.is_empty())
                .map(|items| items.join(","))
                .or_else(|| {
                    raw.as_str()
                        .map(str::trim)
                        .filter(|value| !value.is_empty())
                        .map(str::to_owned)
                })
        })
    })
}

fn runtime_router_for_path(state: &GatewayState, path: &str) -> Option<Router> {
    if should_delegate_to_product_runtime(path) {
        return state
            .product_runtime_router
            .clone()
            .or_else(|| state.embedded_runtime_router.clone());
    }

    if should_delegate_to_embedded_runtime(path) {
        return state
            .embedded_runtime_router
            .clone()
            .or_else(|| state.product_runtime_router.clone());
    }

    state.product_runtime_router.clone()
}

fn runtime_router_for_missing_embedded_upstream(
    state: &GatewayState,
    service_id: &str,
) -> Option<Router> {
    if service_id == APPBASE_APP_API_SERVICE_ID {
        return state.embedded_runtime_router.clone();
    }

    state
        .embedded_runtime_router
        .clone()
        .or_else(|| state.product_runtime_router.clone())
}

fn should_delegate_to_embedded_runtime(path: &str) -> bool {
    path == "/im/v3/openapi.json"
        || path == "/app/v3/openapi.json"
        || path == "/backend/v3/openapi.json"
        || path.starts_with("/im/v3/api/")
        || path.starts_with("/app/v3/api/")
        || path.starts_with("/backend/v3/api/")
}

fn should_delegate_to_product_runtime(path: &str) -> bool {
    path.starts_with("/app/v3/api/portal/")
}

async fn proxy_websocket_streams(
    downstream_socket: WebSocket,
    upstream_socket: WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>,
) {
    let (mut downstream_sender, mut downstream_receiver) = downstream_socket.split();
    let (mut upstream_sender, mut upstream_receiver) = upstream_socket.split();

    loop {
        tokio::select! {
            downstream_message = downstream_receiver.next() => {
                match downstream_message {
                    Some(Ok(message)) => {
                        let message = downstream_to_upstream_message(message);
                        let should_stop = matches!(message, tungstenite::Message::Close(_));
                        if upstream_sender.send(message).await.is_err() {
                            break;
                        }
                        if should_stop {
                            break;
                        }
                    }
                    Some(Err(_)) | None => {
                        let _ = upstream_sender.close().await;
                        break;
                    }
                }
            }
            upstream_message = upstream_receiver.next() => {
                match upstream_message {
                    Some(Ok(message)) => {
                        let should_stop = matches!(message, tungstenite::Message::Close(_));
                        let Some(message) = upstream_to_downstream_message(message) else {
                            continue;
                        };
                        if downstream_sender.send(message).await.is_err() {
                            break;
                        }
                        if should_stop {
                            break;
                        }
                    }
                    Some(Err(_)) | None => {
                        let _ = downstream_sender.send(Message::Close(None)).await;
                        break;
                    }
                }
            }
        }
    }
}

async fn build_proxy_response(service_id: &str, upstream_response: reqwest::Response) -> Response {
    let status = upstream_response.status();
    let headers = upstream_response.headers().clone();
    let body = upstream_response.bytes().await.unwrap_or_default();
    let mut response = build_raw_response(status, &headers, Body::from(body));
    response.headers_mut().insert(
        "x-craw-chat-upstream-service",
        HeaderValue::from_str(service_id)
            .expect("static gateway upstream service id should be a valid header value"),
    );
    response
}

fn build_raw_response(status: StatusCode, headers: &HeaderMap, body: Body) -> Response {
    let mut response_builder = Response::builder().status(status);

    for (name, value) in headers.iter() {
        if *name == header::TRANSFER_ENCODING || *name == header::CONNECTION {
            continue;
        }
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(body)
        .expect("proxied gateway response should build")
}

fn map_http_method(method: &Method) -> Option<HttpMethod> {
    match *method {
        Method::DELETE => Some(HttpMethod::Delete),
        Method::GET => Some(HttpMethod::Get),
        Method::HEAD => Some(HttpMethod::Head),
        Method::OPTIONS => Some(HttpMethod::Options),
        Method::PATCH => Some(HttpMethod::Patch),
        Method::POST => Some(HttpMethod::Post),
        Method::PUT => Some(HttpMethod::Put),
        _ => None,
    }
}

fn gateway_proxy_routes() -> axum::routing::MethodRouter<GatewayState> {
    get(proxy_get_request)
        .post(proxy_request)
        .put(proxy_request)
        .patch(proxy_request)
        .delete(proxy_request)
        .options(proxy_request)
}

fn build_browser_cors_layer() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(AllowOrigin::list(resolve_browser_origins()))
        .allow_methods(AllowMethods::list([
            Method::DELETE,
            Method::GET,
            Method::HEAD,
            Method::OPTIONS,
            Method::PATCH,
            Method::POST,
            Method::PUT,
        ]))
        .allow_headers(AllowHeaders::list(resolve_browser_headers()))
}

fn resolve_browser_origins() -> Vec<header::HeaderValue> {
    let configured = std::env::var(BROWSER_ORIGINS_ENV).ok();
    let origins = configured
        .as_deref()
        .map(parse_browser_origin_list)
        .filter(|origins| !origins.is_empty())
        .unwrap_or_else(default_browser_origins);

    origins
        .into_iter()
        .map(|origin| {
            origin
                .parse::<header::HeaderValue>()
                .expect("configured browser origin should be a valid header value")
        })
        .collect()
}

fn parse_browser_origin_list(raw: &str) -> Vec<String> {
    let mut origins = Vec::new();
    for value in raw
        .split(',')
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        let normalized = value.trim_end_matches('/').to_owned();
        if !origins.contains(&normalized) {
            origins.push(normalized);
        }
    }
    origins
}

fn default_browser_origins() -> Vec<String> {
    [
        "http://127.0.0.1:1620",
        "http://localhost:1620",
        "http://127.0.0.1:4176",
        "http://localhost:4176",
        "tauri://localhost",
    ]
    .into_iter()
    .map(str::to_owned)
    .collect()
}

fn resolve_browser_headers() -> Vec<header::HeaderName> {
    let mut headers = Vec::new();
    for header_name in [
        header::AUTHORIZATION.as_str(),
        header::CONTENT_TYPE.as_str(),
        "access-token",
        "x-sdkwork-app-id",
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-environment",
        "x-sdkwork-deployment-mode",
        "x-sdkwork-auth-level",
        "x-sdkwork-data-scope",
        "x-sdkwork-actor-id",
        "x-sdkwork-actor-kind",
        "x-sdkwork-device-id",
        "x-sdkwork-permission-scope",
        "x-sdkwork-context-signature",
    ] {
        if let Ok(parsed) = header_name.parse::<header::HeaderName>()
            && !headers.contains(&parsed)
        {
            headers.push(parsed);
        }
    }
    headers
}

pub fn build_gateway_registry() -> Result<RouteRegistry, String> {
    build_registry(gateway_route_descriptors()).map_err(|error| error.message)
}

fn gateway_route_descriptors() -> Vec<RouteDescriptor> {
    let mut entries = Vec::new();

    entries.extend(prefix_routes(
        "session-gateway",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/presence/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "presence",
    ));
    entries.extend(prefix_routes(
        "session-gateway",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/realtime/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "realtime",
    ));
    entries.push(websocket_route(
        "session-gateway",
        "/im/v3/api/realtime/ws",
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "realtime",
        &[LINK_WEBSOCKET_SUBPROTOCOL],
    ));
    entries.extend(prefix_routes(
        "control-plane-api",
        all_http_methods(),
        &["/backend/v3/api/control/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "control",
    ));
    entries.extend(exact_routes(
        "conversation-runtime",
        vec![HttpMethod::Post],
        &["/im/v3/api/chat/conversations"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(prefix_routes(
        "conversation-runtime",
        vec![HttpMethod::Post],
        &[
            "/im/v3/api/chat/conversations/{*path}",
            "/im/v3/api/chat/messages/{*path}",
        ],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(exact_routes(
        "projection-service",
        vec![HttpMethod::Get],
        &["/im/v3/api/chat/contacts", "/im/v3/api/chat/inbox"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(prefix_routes(
        "projection-service",
        vec![HttpMethod::Get],
        &["/im/v3/api/chat/conversations/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "conversations",
    ));
    entries.extend(exact_routes(
        "streaming-service",
        vec![HttpMethod::Post],
        &["/im/v3/api/streams"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "streams",
    ));
    entries.extend(prefix_routes(
        "streaming-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/streams/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "streams",
    ));
    entries.extend(prefix_routes(
        "sdkwork-rtc-signaling-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/app/v3/api/rtc/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkRtcAppSdk],
        "rtc",
    ));
    entries.extend(prefix_routes(
        "sdkwork-drive-app-api",
        all_http_methods(),
        &["/app/v3/api/drive/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkDriveAppSdk],
        "drive",
    ));
    entries.extend(prefix_routes(
        "media-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/im/v3/api/media/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImSdk],
        "media",
    ));
    entries.extend(prefix_routes(
        "sdkwork-appbase-app-api",
        all_http_methods(),
        &[
            "/app/v3/api/auth/{*path}",
            "/app/v3/api/iam/{*path}",
            "/app/v3/api/open_platform/{*path}",
            "/app/v3/api/system/iam/{*path}",
        ],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "iam",
    ));
    entries.extend(exact_routes(
        "notification-service",
        vec![HttpMethod::Get],
        &["/app/v3/api/notifications"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "notifications",
    ));
    entries.extend(prefix_routes(
        "notification-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/app/v3/api/notifications/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "notifications",
    ));
    entries.extend(prefix_routes(
        "automation-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/app/v3/api/automation/{*path}"],
        RouteVisibility::Public,
        vec![SdkTarget::SdkworkImAppSdk],
        "automation",
    ));
    entries.extend(prefix_routes(
        "audit-service",
        vec![HttpMethod::Get, HttpMethod::Post],
        &["/backend/v3/api/audit/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "audit",
    ));
    entries.extend(prefix_routes(
        "ops-service",
        vec![HttpMethod::Get],
        &["/backend/v3/api/ops/{*path}"],
        RouteVisibility::Internal,
        vec![SdkTarget::SdkworkImBackendSdk],
        "ops",
    ));

    entries
}

fn prefix_routes(
    service_id: &str,
    methods: Vec<HttpMethod>,
    paths: &[&str],
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> Vec<RouteDescriptor> {
    paths
        .iter()
        .map(|path| {
            route_descriptor(
                service_id,
                methods.clone(),
                path,
                visibility,
                sdk_targets.clone(),
                operation_group,
            )
        })
        .collect()
}

fn exact_routes(
    service_id: &str,
    methods: Vec<HttpMethod>,
    paths: &[&str],
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> Vec<RouteDescriptor> {
    prefix_routes(
        service_id,
        methods,
        paths,
        visibility,
        sdk_targets,
        operation_group,
    )
}

fn route_descriptor(
    service_id: &str,
    methods: Vec<HttpMethod>,
    path_pattern: &str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
) -> RouteDescriptor {
    route_descriptor_with_protocol(RouteDescriptorSpec {
        service_id,
        methods,
        path_pattern,
        visibility,
        sdk_targets,
        operation_group,
        protocol: RouteProtocol::Http,
        websocket_subprotocols: &[],
    })
}

fn websocket_route(
    service_id: &str,
    path_pattern: &str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &str,
    websocket_subprotocols: &[&str],
) -> RouteDescriptor {
    route_descriptor_with_protocol(RouteDescriptorSpec {
        service_id,
        methods: vec![HttpMethod::Get],
        path_pattern,
        visibility,
        sdk_targets,
        operation_group,
        protocol: RouteProtocol::Websocket,
        websocket_subprotocols,
    })
}

struct RouteDescriptorSpec<'a> {
    service_id: &'a str,
    methods: Vec<HttpMethod>,
    path_pattern: &'a str,
    visibility: RouteVisibility,
    sdk_targets: Vec<SdkTarget>,
    operation_group: &'a str,
    protocol: RouteProtocol,
    websocket_subprotocols: &'a [&'a str],
}

fn route_descriptor_with_protocol(spec: RouteDescriptorSpec<'_>) -> RouteDescriptor {
    RouteDescriptor {
        service_id: spec.service_id.to_owned(),
        methods: spec.methods,
        path_pattern: spec.path_pattern.to_owned(),
        visibility: spec.visibility,
        sdk_targets: spec.sdk_targets,
        operation_group: spec.operation_group.to_owned(),
        protocol: spec.protocol,
        websocket_subprotocols: spec
            .websocket_subprotocols
            .iter()
            .map(|value| (*value).to_owned())
            .collect(),
    }
}

fn all_http_methods() -> Vec<HttpMethod> {
    vec![
        HttpMethod::Delete,
        HttpMethod::Get,
        HttpMethod::Head,
        HttpMethod::Options,
        HttpMethod::Patch,
        HttpMethod::Post,
        HttpMethod::Put,
    ]
}

fn json_error_response(status: StatusCode, message: &str) -> Response {
    Response::builder()
        .status(status)
        .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
        .body(Body::from(
            serde_json::json!({
                "code": "gateway_proxy_error",
                "message": message
            })
            .to_string(),
        ))
        .expect("gateway json error response should build")
}

fn upstream_websocket_url(base_url: &str, path_and_query: &str) -> Result<String, String> {
    let upstream_base_url = if let Some(value) = base_url.strip_prefix("http://") {
        format!("ws://{value}")
    } else if let Some(value) = base_url.strip_prefix("https://") {
        format!("wss://{value}")
    } else if base_url.starts_with("ws://") || base_url.starts_with("wss://") {
        base_url.to_owned()
    } else {
        return Err(format!(
            "unsupported upstream websocket scheme in {base_url}"
        ));
    };

    Ok(format!(
        "{}{}",
        upstream_base_url.trim_end_matches('/'),
        path_and_query
    ))
}

fn copy_websocket_headers(
    source_headers: &header::HeaderMap,
    target_headers: &mut header::HeaderMap,
) {
    for (name, value) in source_headers.iter() {
        if websocket_header_should_forward(name) {
            target_headers.insert(name, value.clone());
        }
    }
}

fn websocket_header_should_forward(name: &header::HeaderName) -> bool {
    !matches!(
        *name,
        header::HOST
            | header::CONNECTION
            | header::UPGRADE
            | header::CONTENT_LENGTH
            | header::SEC_WEBSOCKET_ACCEPT
            | header::SEC_WEBSOCKET_EXTENSIONS
            | header::SEC_WEBSOCKET_KEY
            | header::SEC_WEBSOCKET_VERSION
    )
}

fn downstream_to_upstream_message(message: Message) -> tungstenite::Message {
    match message {
        Message::Text(text) => tungstenite::Message::Text(text.to_string().into()),
        Message::Binary(bytes) => tungstenite::Message::Binary(bytes),
        Message::Ping(payload) => tungstenite::Message::Ping(payload),
        Message::Pong(payload) => tungstenite::Message::Pong(payload),
        Message::Close(frame) => {
            tungstenite::Message::Close(frame.map(|frame| tungstenite::protocol::CloseFrame {
                code: frame.code.into(),
                reason: frame.reason.to_string().into(),
            }))
        }
    }
}

fn upstream_to_downstream_message(message: tungstenite::Message) -> Option<Message> {
    match message {
        tungstenite::Message::Text(text) => Some(Message::Text(text.to_string().into())),
        tungstenite::Message::Binary(bytes) => Some(Message::Binary(bytes)),
        tungstenite::Message::Ping(payload) => Some(Message::Ping(payload)),
        tungstenite::Message::Pong(payload) => Some(Message::Pong(payload)),
        tungstenite::Message::Close(frame) => Some(Message::Close(frame.map(|frame| CloseFrame {
            code: frame.code.into(),
            reason: Utf8Bytes::from(frame.reason.to_string()),
        }))),
        tungstenite::Message::Frame(_) => None,
    }
}

async fn fetch_service_openapi_documents(
    state: &GatewayState,
) -> Result<Vec<ServiceOpenApiDocument>, Response> {
    let fetches = state.config.upstreams.iter().map(|upstream| {
        let service_id = upstream.service_id.clone();
        async move {
            (
                service_id.clone(),
                fetch_service_openapi_document(state, service_id.as_str()).await,
            )
        }
    });
    let mut documents = Vec::new();
    for (service_id, result) in futures_util::future::join_all(fetches).await {
        match result {
            Ok(document) => documents.push(ServiceOpenApiDocument {
                service_id,
                document,
            }),
            Err(error) if state.config.strict_startup => return Err(error),
            Err(_) => continue,
        }
    }
    Ok(documents)
}

async fn fetch_service_openapi_document(
    state: &GatewayState,
    service_id: &str,
) -> Result<Value, Response> {
    let Some(base_url) = state.config.upstream_base_url(service_id) else {
        return Err(json_error_response(
            StatusCode::NOT_FOUND,
            format!("service schema upstream is not configured for {service_id}").as_str(),
        ));
    };
    let url = format!("{}/openapi.json", base_url.trim_end_matches('/'));
    let response = state
        .client
        .get(url)
        .timeout(Duration::from_secs(2))
        .send()
        .await
        .map_err(|error| {
            json_error_response(
                StatusCode::BAD_GATEWAY,
                format!("failed to fetch upstream schema for {service_id}: {error}").as_str(),
            )
        })?;
    let status = response.status();
    if !status.is_success() {
        return Err(json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("upstream schema request for {service_id} returned {status}").as_str(),
        ));
    }
    response.json::<Value>().await.map_err(|error| {
        json_error_response(
            StatusCode::BAD_GATEWAY,
            format!("failed to decode upstream schema for {service_id}: {error}").as_str(),
        )
    })
}

fn build_aggregate_openapi_document(documents: &[ServiceOpenApiDocument]) -> Value {
    let mut tags = std::collections::BTreeMap::<String, Value>::new();
    let mut paths = Map::new();
    let mut security_schemes = Map::new();
    let mut schemas = gateway_discovery_schema_components();

    for document in documents {
        if let Some(service_tags) = document.document.get("tags").and_then(Value::as_array) {
            for tag in service_tags {
                if let Some(name) = tag.get("name").and_then(Value::as_str) {
                    tags.entry(name.to_owned()).or_insert_with(|| tag.clone());
                }
            }
        }

        if let Some(service_paths) = document.document.get("paths").and_then(Value::as_object) {
            for (path, operations) in service_paths {
                let path_item = paths
                    .entry(path.clone())
                    .or_insert_with(|| Value::Object(Map::new()));
                let path_object = path_item
                    .as_object_mut()
                    .expect("aggregate path entry should always be an object");
                if let Some(operations_object) = operations.as_object() {
                    for (method, operation) in operations_object {
                        let mut operation_value = operation.clone();
                        if let Some(operation_object) = operation_value.as_object_mut() {
                            operation_object
                                .entry("x-sdkwork-service".to_owned())
                                .or_insert(Value::String(document.service_id.clone()));
                        }
                        path_object.insert(method.clone(), operation_value);
                    }
                }
            }
        }

        if let Some(schemes) = document
            .document
            .get("components")
            .and_then(|value| value.get("securitySchemes"))
            .and_then(Value::as_object)
        {
            for (name, scheme) in schemes {
                security_schemes
                    .entry(name.clone())
                    .or_insert_with(|| scheme.clone());
            }
        }
    }

    merge_gateway_discovery_openapi(&mut tags, &mut paths);

    let mut document = Map::new();
    document.insert("openapi".to_owned(), Value::String("3.1.0".to_owned()));
    document.insert(
        "info".to_owned(),
        json!({
            "title": "Craw Chat Unified Gateway API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Aggregate OpenAPI contract assembled by the web-gateway from live upstream service schemas."
        }),
    );
    document.insert("servers".to_owned(), json!([{ "url": "/" }]));
    document.insert(
        "tags".to_owned(),
        Value::Array(tags.into_values().collect()),
    );
    document.insert("paths".to_owned(), Value::Object(paths));

    if !security_schemes.is_empty() || !schemas.is_empty() {
        let mut components = Map::new();
        if !security_schemes.is_empty() {
            components.insert(
                "securitySchemes".to_owned(),
                Value::Object(security_schemes),
            );
        }
        if !schemas.is_empty() {
            components.insert(
                "schemas".to_owned(),
                Value::Object(std::mem::take(&mut schemas)),
            );
        }
        document.insert("components".to_owned(), Value::Object(components));
    }

    Value::Object(document)
}

fn service_schema_index_entries(
    config: &WebGatewayConfig,
    registry: &RouteRegistry,
) -> Vec<ServiceSchemaIndexEntry> {
    config
        .upstreams
        .iter()
        .map(|upstream| {
            let service_routes = registry
                .entries()
                .iter()
                .filter(|entry| entry.service_id == upstream.service_id)
                .collect::<Vec<_>>();

            ServiceSchemaIndexEntry {
                service_id: upstream.service_id.clone(),
                contract_kind: ContractKind::UpstreamOperational,
                schema_url: format!("/openapi/services/{}.openapi.json", upstream.service_id),
                docs_url: format!("/docs/services/{}", upstream.service_id),
                visibility: service_visibility(service_routes.as_slice())
                    .unwrap_or_else(|| visibility_for_service(upstream.service_id.as_str())),
                route_count: service_routes.len(),
                operation_groups: service_routes
                    .iter()
                    .map(|entry| entry.operation_group.clone())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                sdk_targets: service_routes
                    .iter()
                    .flat_map(|entry| entry.sdk_targets.iter().copied())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                protocols: service_routes
                    .iter()
                    .map(|entry| entry.protocol)
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
                websocket_subprotocols: service_routes
                    .iter()
                    .flat_map(|entry| entry.websocket_subprotocols.iter().cloned())
                    .collect::<BTreeSet<_>>()
                    .into_iter()
                    .collect(),
            }
        })
        .collect()
}

fn merge_gateway_discovery_openapi(
    tags: &mut std::collections::BTreeMap<String, Value>,
    paths: &mut Map<String, Value>,
) {
    tags.entry("gatewayDiscovery".to_owned()).or_insert_with(|| {
        json!({
            "name": "gatewayDiscovery",
            "description": "Gateway discovery operations exposed directly by the unified web-gateway."
        })
    });

    paths.insert(
        "/openapi/index.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayOpenapiIndex",
                "summary": "Get gateway service schema index",
                "tags": ["gatewayDiscovery"],
                "responses": {
                    "200": {
                        "description": "Gateway service schema index",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/GatewayOpenapiIndex"
                                }
                            }
                        }
                    }
                },
                "x-sdkwork-service": "web-gateway"
            }
        }),
    );
    paths.insert(
        "/openapi/runtime-summary.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayRuntimeSummary",
                "summary": "Get gateway runtime discovery summary",
                "tags": ["gatewayDiscovery"],
                "responses": {
                    "200": {
                        "description": "Gateway runtime summary",
                        "content": {
                            "application/json": {
                                "schema": {
                                    "$ref": "#/components/schemas/GatewayRuntimeSummary"
                                }
                            }
                        }
                    }
                },
                "x-sdkwork-service": "web-gateway"
            }
        }),
    );
    paths.insert(
        "/openapi/services/{serviceId}.openapi.json".to_owned(),
        json!({
            "get": {
                "operationId": "getGatewayServiceSchema",
                "summary": "Get gateway service OpenAPI schema",
                "tags": ["gatewayDiscovery"],
                "parameters": [
                    {
                        "name": "serviceId",
                        "in": "path",
                        "required": true,
                        "schema": { "type": "string" },
                        "description": "Gateway service identifier"
                    }
                ],
                "responses": {
                    "200": {
                        "description": "Service OpenAPI schema"
                    }
                },
                "x-sdkwork-service": "web-gateway"
            }
        }),
    );
}

fn gateway_discovery_schema_components() -> Map<String, Value> {
    let mut schemas = Map::new();

    schemas.insert(
        "GatewayOpenapiIndex".to_owned(),
        json!({
            "type": "object",
            "required": ["sdkContracts", "services", "routes", "surfaceGroups"],
            "properties": {
                "sdkContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkContractSummary" }
                },
                "services": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayServiceSchemaIndexEntry" }
                },
                "routes": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteSummary" }
                },
                "surfaceGroups": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySurfaceGroupSummary" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRuntimeSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "bindAddr",
                "baseUrl",
                "aggregateOpenapiUrl",
                "openapiIndexUrl",
                "runtimeSummaryUrl",
                "docsUrl",
                "runtimeMode",
                "sdkContracts",
                "upstreams",
                "serviceContracts",
                "publicEndpoints",
                "surfaceGroups"
            ],
            "properties": {
                "bindAddr": { "type": "string" },
                "baseUrl": { "type": "string", "format": "uri" },
                "aggregateOpenapiUrl": { "type": "string", "format": "uri" },
                "openapiIndexUrl": { "type": "string", "format": "uri" },
                "runtimeSummaryUrl": { "type": "string", "format": "uri" },
                "docsUrl": { "type": "string", "format": "uri" },
                "runtimeMode": { "$ref": "#/components/schemas/GatewayRuntimeMode" },
                "sdkContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkContractSummary" }
                },
                "upstreams": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayUpstreamBinding" }
                },
                "serviceContracts": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayServiceContractSummary" }
                },
                "publicEndpoints": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayPublicEndpointSummary" }
                },
                "surfaceGroups": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySurfaceGroupSummary" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayServiceSchemaIndexEntry".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "contractKind",
                "schemaUrl",
                "docsUrl",
                "visibility",
                "routeCount",
                "operationGroups",
                "sdkTargets",
                "protocols"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string" },
                "docsUrl": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "routeCount": { "type": "integer", "minimum": 0 },
                "operationGroups": {
                    "type": "array",
                    "items": { "type": "string" }
                },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "protocols": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteProtocol" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRouteSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "operationGroup",
                "visibility",
                "pathPattern",
                "methods",
                "protocol",
                "sdkTargets"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "operationGroup": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "pathPattern": { "type": "string" },
                "methods": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayHttpMethod" }
                },
                "protocol": { "$ref": "#/components/schemas/GatewayRouteProtocol" },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewaySurfaceGroupSummary".to_owned(),
        json!({
            "type": "object",
            "required": [
                "serviceId",
                "operationGroup",
                "visibility",
                "routeCount",
                "sdkTargets",
                "protocols"
            ],
            "properties": {
                "serviceId": { "type": "string" },
                "operationGroup": { "type": "string" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "routeCount": { "type": "integer", "minimum": 0 },
                "sdkTargets": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewaySdkTarget" }
                },
                "protocols": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayRouteProtocol" }
                },
                "websocketSubprotocols": {
                    "type": "array",
                    "items": { "type": "string" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayServiceContractSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["serviceId", "contractKind", "schemaUrl", "docsUrl"],
            "properties": {
                "serviceId": { "type": "string" },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string", "format": "uri" },
                "docsUrl": { "type": "string", "format": "uri" }
            }
        }),
    );
    schemas.insert(
        "GatewaySdkContractSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["groupId", "contractKind", "schemaUrl", "apiPrefix", "sdkTarget"],
            "properties": {
                "groupId": { "type": "string", "enum": ["im-open-api", "im-app-api", "im-backend-api"] },
                "contractKind": { "$ref": "#/components/schemas/GatewayContractKind" },
                "schemaUrl": { "type": "string" },
                "apiPrefix": { "type": "string" },
                "sdkTarget": { "$ref": "#/components/schemas/GatewaySdkTarget" }
            }
        }),
    );
    schemas.insert(
        "GatewayPublicEndpointSummary".to_owned(),
        json!({
            "type": "object",
            "required": ["serviceId", "pathPattern", "protocol", "visibility", "methods"],
            "properties": {
                "serviceId": { "type": "string" },
                "pathPattern": { "type": "string" },
                "protocol": { "$ref": "#/components/schemas/GatewayRouteProtocol" },
                "visibility": { "$ref": "#/components/schemas/GatewayRouteVisibility" },
                "methods": {
                    "type": "array",
                    "items": { "$ref": "#/components/schemas/GatewayHttpMethod" }
                }
            }
        }),
    );
    schemas.insert(
        "GatewayRuntimeMode".to_owned(),
        json!({
            "type": "string",
            "enum": ["split", "embedded"]
        }),
    );
    schemas.insert(
        "GatewayUpstreamBinding".to_owned(),
        json!({
            "type": "array",
            "prefixItems": [
                { "type": "string" },
                { "type": "string", "format": "uri" }
            ],
            "minItems": 2,
            "maxItems": 2
        }),
    );
    schemas.insert(
        "GatewayRouteVisibility".to_owned(),
        json!({
            "type": "string",
            "enum": ["public", "partner", "internal"]
        }),
    );
    schemas.insert(
        "GatewayRouteProtocol".to_owned(),
        json!({
            "type": "string",
            "enum": ["http", "websocket"]
        }),
    );
    schemas.insert(
        "GatewaySdkTarget".to_owned(),
        json!({
            "type": "string",
            "enum": ["sdkworkImSdk", "sdkworkImAppSdk", "sdkworkImBackendSdk", "sdkworkRtcAppSdk", "sdkworkDriveAppSdk", "none"]
        }),
    );
    schemas.insert(
        "GatewayContractKind".to_owned(),
        json!({
            "type": "string",
            "enum": ["sdk", "upstreamOperational"]
        }),
    );
    schemas.insert(
        "GatewayHttpMethod".to_owned(),
        json!({
            "type": "string",
            "enum": ["delete", "get", "head", "options", "patch", "post", "put"]
        }),
    );

    schemas
}

fn service_visibility(service_routes: &[&RouteDescriptor]) -> Option<RouteVisibility> {
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Public)
    {
        return Some(RouteVisibility::Public);
    }
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Partner)
    {
        return Some(RouteVisibility::Partner);
    }
    if service_routes
        .iter()
        .any(|entry| entry.visibility == RouteVisibility::Internal)
    {
        return Some(RouteVisibility::Internal);
    }
    None
}

fn visibility_for_service(service_id: &str) -> RouteVisibility {
    match service_id {
        "control-plane-api" | "audit-service" | "ops-service" => RouteVisibility::Internal,
        _ => RouteVisibility::Public,
    }
}

fn aggregate_gateway_openapi_spec() -> OpenApiServiceSpec<'static> {
    OpenApiServiceSpec {
        title: "Craw Chat Unified Gateway API",
        version: env!("CARGO_PKG_VERSION"),
        description: "Aggregate OpenAPI contract served by the web-gateway for the unified Craw Chat external HTTP surface.",
        openapi_path: "/openapi.json",
        docs_path: "/docs",
    }
}

fn service_openapi_spec(service_id: &str) -> OpenApiServiceSpec<'static> {
    let title = Box::leak(format!("Craw Chat {} Service Contract", service_id).into_boxed_str());
    let description = Box::leak(
        format!("Gateway-hosted service contract view for {service_id}.").into_boxed_str(),
    );
    let openapi_path =
        Box::leak(format!("/openapi/services/{service_id}.openapi.json").into_boxed_str());
    let docs_path = Box::leak(format!("/docs/services/{service_id}").into_boxed_str());
    OpenApiServiceSpec {
        title,
        version: env!("CARGO_PKG_VERSION"),
        description,
        openapi_path,
        docs_path,
    }
}

fn request_base_url(request: &Request) -> String {
    let scheme = forwarded_header_value(
        request.headers(),
        header::HeaderName::from_static("x-forwarded-proto"),
    )
    .or_else(|| request.uri().scheme_str().map(str::to_owned))
    .unwrap_or_else(|| "http".to_owned());
    let authority = forwarded_header_value(
        request.headers(),
        header::HeaderName::from_static("x-forwarded-host"),
    )
    .or_else(|| {
        request
            .headers()
            .get(header::HOST)
            .and_then(|value| value.to_str().ok())
            .map(str::to_owned)
    })
    .or_else(|| {
        request
            .uri()
            .authority()
            .map(|value| value.as_str().to_owned())
    })
    .unwrap_or_else(|| "localhost".to_owned());
    format!("{scheme}://{authority}")
}

fn forwarded_header_value(headers: &header::HeaderMap, name: header::HeaderName) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.split(',').next())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
}

struct ServiceOpenApiDocument {
    service_id: String,
    document: Value,
}
