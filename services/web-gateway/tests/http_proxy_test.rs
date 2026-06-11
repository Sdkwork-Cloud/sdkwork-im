use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, Method, Request, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{any, get},
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use http_body_util::BodyExt;
use im_app_context::{
    AppContext, build_dual_token_headers_for_context, local_service_app_context,
    resolve_app_context,
};
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

const SDKWORK_INTERNAL_HEADER_PROBE: &str = "x-sdkwork-tenant-id";

#[derive(Clone)]
struct UpstreamState {
    service_id: Arc<str>,
}

fn gateway_test_app_context() -> AppContext {
    let mut context = local_service_app_context(
        "tenant_real",
        "user_real",
        "user",
        Some("device_real"),
        ["*"],
    );
    context.session_id = Some("session_real".to_owned());
    context.app_id = Some("sdkwork-chat-pc".to_owned());
    context
}

fn gateway_test_auth_headers() -> HeaderMap {
    let context = gateway_test_app_context();
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn gateway_test_authorization_header() -> String {
    gateway_test_auth_headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("test auth token should be present")
        .to_owned()
}

fn gateway_test_access_token_header() -> String {
    gateway_test_auth_headers()
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .expect("test access token should be present")
        .to_owned()
}

fn gateway_numeric_auth_headers() -> HeaderMap {
    let mut context = local_service_app_context(
        "20001",
        "user_numeric",
        "user",
        Some("device_numeric"),
        ["*"],
    );
    context.organization_id = Some("30001".to_owned());
    context.session_id = Some("session_numeric".to_owned());
    context.app_id = Some("sdkwork-chat-pc".to_owned());
    build_dual_token_headers_for_context(&context, context.permission_scope.iter())
}

fn gateway_numeric_authorization_header() -> String {
    gateway_numeric_auth_headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .expect("numeric test auth token should be present")
        .to_owned()
}

fn gateway_numeric_access_token_header() -> String {
    gateway_numeric_auth_headers()
        .get("access-token")
        .and_then(|value| value.to_str().ok())
        .expect("numeric test access token should be present")
        .to_owned()
}

#[tokio::test]
async fn gateway_exposes_health_and_readiness_endpoints() {
    let app = web_gateway::build_app(test_gateway_config(Vec::new()));

    let healthz = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/healthz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("healthz request should succeed");
    assert_eq!(healthz.status(), StatusCode::OK);

    let readyz = app
        .oneshot(
            Request::builder()
                .uri("/readyz")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("readyz request should succeed");
    assert_eq!(readyz.status(), StatusCode::OK);
}

#[tokio::test]
async fn gateway_routes_control_requests_to_control_plane_api() {
    let control_plane = spawn_upstream("control-plane-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "control-plane-api",
        control_plane.base_url.as_str(),
    )]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/backend/v3/api/control/protocol-registry")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("response body should collect")
        .to_bytes();
    let value: serde_json::Value =
        serde_json::from_slice(&body).expect("response body should be valid json");
    assert_eq!(value["serviceId"], "control-plane-api");
}

#[tokio::test]
async fn gateway_routes_conversation_reads_and_writes_to_different_upstreams() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let projection = spawn_upstream("projection-service").await;
    let conversation_runtime = spawn_upstream("conversation-runtime").await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
        service_upstream("projection-service", projection.base_url.as_str()),
        service_upstream(
            "conversation-runtime",
            conversation_runtime.base_url.as_str(),
        ),
    ]));

    let read_response = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/chat/conversations/c_1/messages")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("read request should succeed");
    assert_eq!(read_response.status(), StatusCode::OK);
    let read_value: serde_json::Value = serde_json::from_slice(
        &read_response
            .into_body()
            .collect()
            .await
            .expect("read response body should collect")
            .to_bytes(),
    )
    .expect("read response should be valid json");
    assert_eq!(read_value["serviceId"], "projection-service");

    let write_response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/im/v3/api/chat/conversations/c_1/messages")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("write request should succeed");
    assert_eq!(write_response.status(), StatusCode::OK);
    let write_value: serde_json::Value = serde_json::from_slice(
        &write_response
            .into_body()
            .collect()
            .await
            .expect("write response body should collect")
            .to_bytes(),
    )
    .expect("write response should be valid json");
    assert_eq!(write_value["serviceId"], "conversation-runtime");
}

#[tokio::test]
async fn gateway_routes_im_app_iam_requests_to_appbase_app_api() {
    let appbase = spawn_upstream("sdkwork-appbase-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-appbase-app-api",
        appbase.base_url.as_str(),
    )]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/sessions")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(value["serviceId"], "sdkwork-appbase-app-api");
    assert_eq!(value["path"], "/app/v3/api/auth/sessions");
}

#[tokio::test]
async fn embedded_gateway_delegates_oauth_device_authorization_to_embedded_appbase_without_upstream()
 {
    let embedded_runtime = sdkwork_iam_http::build_sdkwork_appbase_app_api_router();
    let product_runtime = Router::new()
        .route(
            "/app/v3/api/oauth/device_authorizations",
            any(echo_upstream),
        )
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        Some(product_runtime),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/oauth/device_authorizations")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(value["code"], "2000");
    assert_eq!(value["data"]["status"], "pending");
    let session_key = value["data"]["sessionKey"]
        .as_str()
        .expect("OAuth device authorization response must include sessionKey");
    assert!(
        !session_key.starts_with("sdkwork-local-qr-"),
        "embedded appbase OAuth device authorization must not expose a local sequence session key"
    );
    let qr_content = value["data"]["qrContent"]["content"]
        .as_str()
        .expect("OAuth device authorization response must include mobile-openable content");
    assert_eq!(value["data"]["qrContent"]["mode"], "fallback_url");
    assert_eq!(value["data"]["fallbackUrl"], qr_content);
    assert!(
        qr_content.starts_with("http://") || qr_content.starts_with("https://"),
        "QR content must be a mobile-openable web URL: {qr_content}"
    );
    assert!(qr_content.contains("/auth/qr/"));
    assert!(qr_content.contains(&format!("session_key={session_key}")));
    assert!(
        !qr_content.contains("/app/v3/api/oauth/device_authorizations"),
        "QR content must not point at the session status API: {qr_content}"
    );
    assert!(
        value.get("serviceId").is_none(),
        "appbase-owned IAM routes must not fall through to product runtime: {value}"
    );
}

#[tokio::test]
async fn embedded_gateway_keeps_retired_open_platform_qr_auth_namespace_out_of_product_fallback() {
    let product_runtime = Router::new()
        .route(
            "/app/v3/api/open_platform/qr_auth/{*path}",
            any(echo_upstream),
        )
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        None,
        Some(product_runtime),
    );

    for path in [
        "/app/v3/api/open_platform/qr_auth/sessions",
        "/app/v3/api/open_platform/qr_auth/unknown",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(Method::POST)
                    .uri(path)
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .expect("gateway request should return");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "retired appbase QR auth namespace must not be proxied to product runtime: {path}"
        );
    }
}

#[tokio::test]
async fn embedded_gateway_serves_appbase_iam_directory_from_local_store() {
    let (embedded_runtime, _) =
        web_gateway::build_embedded_appbase_runtime_with_principal_profile_provider();
    let product_runtime = Router::new()
        .route(
            "/app/v3/api/iam/organization_memberships",
            any(echo_upstream),
        )
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        Some(product_runtime),
    );

    let registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "confirmPassword": "dev123456",
                        "email": "gateway-directory-user@sdkwork-iam.local",
                        "password": "dev123456",
                        "username": "gateway-directory-user"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase registration should return response");
    assert_eq!(registration.status(), StatusCode::OK);
    let registration = read_json_body(registration).await;
    assert_eq!(registration["code"], "2000");
    let auth_token = registration["data"]["authToken"]
        .as_str()
        .expect("registration should return auth token")
        .to_owned();
    let access_token = registration["data"]["accessToken"]
        .as_str()
        .expect("registration should return access token")
        .to_owned();
    let organization_id = registration["data"]["context"]["organizationId"]
        .as_str()
        .expect("registration should return organization context")
        .to_owned();
    let user_id = registration["data"]["user"]["id"]
        .as_str()
        .expect("registration should return user id")
        .to_owned();
    let memberships_uri = format!(
        "/app/v3/api/iam/organization_memberships?organizationId={organization_id}&userId={user_id}"
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(memberships_uri.as_str())
                .header("authorization", format!("Bearer {auth_token}"))
                .header("access-token", access_token.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway IAM directory request should return response");
    assert_eq!(response.status(), StatusCode::OK);
    let value = read_json_body(response).await;
    assert_eq!(value["code"], "2000");
    assert_eq!(value["data"]["items"][0]["organizationId"], organization_id);
    assert_eq!(value["data"]["items"][0]["userId"], user_id);
    assert!(
        value.get("serviceId").is_none(),
        "appbase-owned IAM directory routes must not fall through to product runtime: {value}"
    );

    let body_text = value.to_string();
    for forbidden in [
        "membership_demo",
        "org_demo",
        "t_demo",
        "user_local_default",
    ] {
        assert!(
            !body_text.contains(forbidden),
            "embedded gateway must not expose demo directory token {forbidden}: {value}"
        );
    }
}

#[tokio::test]
async fn embedded_gateway_reports_configured_appbase_runtime_context() {
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(web_gateway::build_embedded_appbase_im_runtime_router()),
        Some(Router::new()),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/app/v3/api/system/iam/runtime")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway runtime context request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let value = read_json_body(response).await;
    assert_eq!(value["code"], "2000");
    assert_eq!(value["data"]["appId"], "chat");
    assert_eq!(value["data"]["tenantId"], "20001");
    assert_eq!(value["data"]["organizationId"], "30001");
    assert_eq!(value["data"]["deploymentMode"], "local");
    assert_eq!(value["data"]["runtime"], "embedded");
}

#[tokio::test]
async fn embedded_gateway_clears_outer_path_params_before_delegating_nested_runtime_routes() {
    let embedded_runtime = Router::new();
    let product_runtime = Router::new()
        .route("/app/v3/api/portal/{section}", get(echo_runtime_section))
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        Some(product_runtime),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/app/v3/api/portal/local-section")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(value["serviceId"], "sdkwork-api-product-runtime");
    assert_eq!(value["section"], "local-section");
}

#[tokio::test]
async fn embedded_gateway_delegates_app_portal_api_to_product_runtime() {
    let embedded_runtime = Router::new();
    let product_runtime = Router::new()
        .route("/app/v3/api/portal/home", get(echo_upstream))
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        Some(product_runtime),
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/app/v3/api/portal/home")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(value["serviceId"], "sdkwork-api-product-runtime");
    assert_eq!(value["path"], "/app/v3/api/portal/home");
}

#[tokio::test]
async fn embedded_gateway_rejects_product_runtime_auth_registration_instead_of_seeding_social_search()
 {
    let product_runtime = Router::new()
        .route("/app/v3/api/auth/registrations", any(echo_upstream))
        .route("/app/v3/api/auth/sessions", any(echo_upstream))
        .with_state(UpstreamState {
            service_id: Arc::<str>::from("sdkwork-api-product-runtime"),
        });
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(web_gateway::build_embedded_appbase_im_runtime_router()),
        Some(product_runtime),
    );

    let target_registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "channel": "EMAIL",
                        "confirmPassword": "dev123456",
                        "email": "target-user@sdkwork-iam.local",
                        "name": "Target User",
                        "password": "dev123456",
                        "username": "target-user"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase-owned registration request should return response");
    assert_eq!(target_registration.status(), StatusCode::OK);
    let target_registration = read_json_body(target_registration).await;
    assert_eq!(target_registration["code"], "2000");
    assert_eq!(
        target_registration["data"]["user"]["email"],
        "target-user@sdkwork-iam.local"
    );
    assert_eq!(target_registration["data"]["user"]["name"], "Target User");
    assert!(
        target_registration.get("serviceId").is_none(),
        "appbase registration must be served by embedded appbase, not product runtime: {target_registration}"
    );

    let searcher_registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "channel": "EMAIL",
                        "confirmPassword": "dev123456",
                        "email": "searcher-user@sdkwork-iam.local",
                        "name": "Searcher User",
                        "password": "dev123456",
                        "username": "searcher-user"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase-owned searcher registration request should return response");
    assert_eq!(searcher_registration.status(), StatusCode::OK);
    let searcher_registration = read_json_body(searcher_registration).await;
    assert_eq!(searcher_registration["code"], "2000");
    assert!(
        searcher_registration.get("serviceId").is_none(),
        "appbase searcher registration must be served by embedded appbase, not product runtime: {searcher_registration}"
    );

    let searcher_session = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/sessions")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "grantType": "password",
                        "email": "searcher-user@sdkwork-iam.local",
                        "password": "dev123456"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase-owned login request should return response");
    let searcher_session_status = searcher_session.status();
    let searcher_session = read_json_body(searcher_session).await;
    assert_eq!(
        searcher_session_status,
        StatusCode::OK,
        "appbase-owned login should succeed: {searcher_session}"
    );
    assert_eq!(searcher_session["code"], "2000");
    assert_eq!(
        searcher_session["data"]["user"]["email"],
        "searcher-user@sdkwork-iam.local"
    );
    let searcher_auth_token = searcher_session["data"]["authToken"]
        .as_str()
        .expect("searcher login should return auth token")
        .to_owned();
    let searcher_access_token = searcher_session["data"]["accessToken"]
        .as_str()
        .expect("searcher login should return access token")
        .to_owned();
    assert!(
        searcher_session.get("serviceId").is_none(),
        "appbase login must be served by embedded appbase, not product runtime: {searcher_session}"
    );

    let search_response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/social/users?q=target-user&limit=20")
                .header("authorization", format!("Bearer {searcher_auth_token}"))
                .header("access-token", searcher_access_token.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social user search should return response");
    assert_eq!(search_response.status(), StatusCode::OK);
    let search = read_json_body(search_response).await;
    let items = search["items"]
        .as_array()
        .expect("social user search should return items");
    let target_item = items
        .iter()
        .find(|item| item["userId"] == target_registration["data"]["user"]["userId"])
        .unwrap_or_else(|| panic!("social search must return the appbase IAM user: {search}"));
    assert!(
        target_item["userId"]
            .as_str()
            .is_some_and(|user_id| user_id.starts_with("iamu_")),
        "social search must expose the real opaque IAM user id: {search}"
    );
    assert_ne!(target_item["userId"], "user_target_user");
    assert_eq!(target_item["displayName"], "Target User");
    assert_eq!(target_item["email"], "target-user@sdkwork-iam.local");
    assert_eq!(
        target_item["metadata"]["source"],
        "sdkwork-appbase-local-iam"
    );
    assert_eq!(target_item["metadata"]["username"], "target-user");
    assert_eq!(
        target_item["tenantId"], searcher_session["data"]["context"]["tenantId"],
        "social search must use the appbase token tenant"
    );
    assert_ne!(target_item["tenantId"], "t_demo");
    assert_ne!(target_item["chatId"], target_item["userId"]);
    assert!(
        target_item.get("serviceId").is_none(),
        "social search result must not be product-runtime echo data: {search}"
    );
}

#[tokio::test]
async fn embedded_gateway_derives_im_social_context_from_appbase_dual_tokens_not_client_headers() {
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(web_gateway::build_embedded_appbase_im_runtime_router()),
        None,
    );

    let registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "channel": "EMAIL",
                        "confirmPassword": "dev123456",
                        "email": "gateway-token-context@sdkwork-iam.local",
                        "name": "Gateway Token Context",
                        "password": "dev123456",
                        "username": "gateway-token-context"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase registration should return response");
    assert_eq!(registration.status(), StatusCode::OK);
    let registration = read_json_body(registration).await;
    let auth_token = registration["data"]["authToken"]
        .as_str()
        .expect("registration should return auth token")
        .to_owned();
    let access_token = registration["data"]["accessToken"]
        .as_str()
        .expect("registration should return access token")
        .to_owned();

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/social/users?q=user_test006_a_com&limit=20")
                .header("authorization", format!("Bearer {auth_token}"))
                .header("access-token", access_token.as_str())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "sdkwork-chat-pc")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "sdkwork_iam_session_test006")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("social user search should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let value = read_json_body(response).await;
    let response_text = value.to_string();
    assert!(
        !response_text.contains("t_demo"),
        "IM social search must not trust client-supplied tenant header: {value}"
    );
    assert!(
        !response_text.contains("user_test006_a_com"),
        "IM social search must not trust client-supplied user header: {value}"
    );
    assert_eq!(
        value["items"],
        json!([]),
        "spoofed current user query must not synthesize a self search result: {value}"
    );
}

#[tokio::test]
async fn embedded_gateway_searches_registered_iam_user_by_copied_public_chat_id() {
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(web_gateway::build_embedded_appbase_im_runtime_router()),
        None,
    );

    let target_registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "channel": "EMAIL",
                        "confirmPassword": "dev123456",
                        "email": "copy-target@sdkwork-iam.local",
                        "name": "Copy Target",
                        "password": "dev123456",
                        "username": "copy-target"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("target appbase registration should return response");
    assert_eq!(target_registration.status(), StatusCode::OK);
    let target_registration = read_json_body(target_registration).await;
    let target_user_id = target_registration["data"]["user"]["userId"]
        .as_str()
        .expect("target registration should return real IAM user id")
        .to_owned();
    assert!(
        target_user_id.starts_with("iamu_"),
        "target registration should return a real opaque IAM user id: {target_registration}"
    );
    assert_ne!(target_user_id, "user_copy_target");

    let searcher_registration = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/registrations")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "channel": "EMAIL",
                        "confirmPassword": "dev123456",
                        "email": "copy-searcher@sdkwork-iam.local",
                        "name": "Copy Searcher",
                        "password": "dev123456",
                        "username": "copy-searcher"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("searcher appbase registration should return response");
    assert_eq!(searcher_registration.status(), StatusCode::OK);

    let searcher_session = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/auth/sessions")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "grantType": "password",
                        "email": "copy-searcher@sdkwork-iam.local",
                        "password": "dev123456"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("searcher login should return response");
    assert_eq!(searcher_session.status(), StatusCode::OK);
    let searcher_session = read_json_body(searcher_session).await;
    let searcher_auth_token = searcher_session["data"]["authToken"]
        .as_str()
        .expect("searcher login should return auth token")
        .to_owned();
    let searcher_access_token = searcher_session["data"]["accessToken"]
        .as_str()
        .expect("searcher login should return access token")
        .to_owned();

    let target_lookup = app
        .clone()
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/social/users?q=copy-target&limit=20")
                .header("authorization", format!("Bearer {searcher_auth_token}"))
                .header("access-token", searcher_access_token.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("target name lookup should return response");
    assert_eq!(target_lookup.status(), StatusCode::OK);
    let target_lookup = read_json_body(target_lookup).await;
    let target_items = target_lookup["items"]
        .as_array()
        .expect("target name lookup should return items");
    let target_item = target_items
        .iter()
        .find(|item| item["userId"] == target_user_id)
        .unwrap_or_else(|| panic!("target user should be searchable by name: {target_lookup}"));
    let copied_chat_id = target_item["chatId"]
        .as_str()
        .expect("target search result should expose copied public chat id")
        .to_owned();
    assert_ne!(copied_chat_id, target_user_id);

    let copied_id_lookup = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri(format!(
                    "/im/v3/api/social/users?q={copied_chat_id}&limit=20"
                ))
                .header("authorization", format!("Bearer {searcher_auth_token}"))
                .header("access-token", searcher_access_token.as_str())
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("copied chat id lookup should return response");
    assert_eq!(copied_id_lookup.status(), StatusCode::OK);
    let copied_id_lookup = read_json_body(copied_id_lookup).await;
    let copied_id_items = copied_id_lookup["items"]
        .as_array()
        .expect("copied chat id lookup should return items");
    assert_eq!(
        copied_id_items.len(),
        1,
        "copied public chat id must resolve exactly one IAM user: {copied_id_lookup}"
    );
    assert_eq!(copied_id_items[0]["userId"], target_user_id);
    assert_eq!(copied_id_items[0]["chatId"], copied_chat_id);
    assert_eq!(
        copied_id_items[0]["tenantId"], searcher_session["data"]["context"]["tenantId"],
        "social search must use the appbase token tenant"
    );
    assert_ne!(copied_id_items[0]["tenantId"], "t_demo");
}

#[tokio::test]
async fn embedded_gateway_derives_drive_context_from_embedded_appbase_current_session() {
    let embedded_runtime = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let drive = spawn_app_upstream(Router::new().route(
        "/app/v3/api/drive/uploader/uploads",
        any(echo_context_upstream),
    ))
    .await;
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: vec![service_upstream(
                "sdkwork-drive-app-api",
                drive.base_url.as_str(),
            )],
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        None,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/drive/uploader/uploads")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway drive upload request should return");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

#[tokio::test]
async fn embedded_gateway_fails_closed_when_drive_app_api_upstream_is_missing() {
    let embedded_runtime = Router::new()
        .route(
            "/app/v3/api/auth/sessions/current",
            get(appbase_current_session),
        )
        .route(
            "/app/v3/api/drive/uploader/uploads",
            any(echo_context_upstream),
        );
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        None,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/drive/uploader/uploads")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway drive upload request should return");

    assert_eq!(
        response.status(),
        StatusCode::BAD_GATEWAY,
        "missing Drive dependency must fail before falling through to embedded runtime routes"
    );
    assert!(
        response
            .headers()
            .get("x-craw-chat-upstream-service")
            .is_none(),
        "missing Drive dependency must not be treated as a proxied upstream"
    );
    let value = read_json_body(response).await;
    assert_eq!(value["code"], "gateway_proxy_error");
    assert!(
        value["message"]
            .as_str()
            .is_some_and(|message| message.contains("sdkwork-drive-app-api")),
        "missing Drive dependency error should identify the unconfigured upstream: {value}"
    );
}

#[tokio::test]
async fn embedded_gateway_derives_notary_context_from_configured_upstream() {
    let embedded_runtime = Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    );
    let notary = spawn_app_upstream(
        Router::new().route("/app/v3/api/notary/cases", any(echo_context_upstream)),
    )
    .await;
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: vec![service_upstream(
                "sdkwork-notary-app-api",
                notary.base_url.as_str(),
            )],
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        None,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/notary/cases")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway notary case request should return");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

#[tokio::test]
async fn embedded_gateway_fails_closed_when_notary_app_api_upstream_is_missing() {
    let embedded_runtime = Router::new()
        .route(
            "/app/v3/api/auth/sessions/current",
            get(appbase_current_session),
        )
        .route("/app/v3/api/notary/cases", any(echo_context_upstream));
    let app = web_gateway::build_app_with_registry_and_runtime_routers(
        WebGatewayConfig {
            bind_addr: "127.0.0.1:0".to_owned(),
            runtime_mode: GatewayRuntimeMode::Embedded,
            strict_startup: true,
            upstreams: Vec::new(),
        },
        web_gateway::build_gateway_registry().expect("gateway registry should build"),
        Some(embedded_runtime),
        None,
    );

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/notary/cases")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway notary case request should return");

    assert_eq!(
        response.status(),
        StatusCode::BAD_GATEWAY,
        "missing Notary dependency must fail before falling through to embedded runtime routes"
    );
    assert!(
        response
            .headers()
            .get("x-craw-chat-upstream-service")
            .is_none(),
        "missing Notary dependency must not be treated as a proxied upstream"
    );
    let value = read_json_body(response).await;
    assert_eq!(value["code"], "gateway_proxy_error");
    assert!(
        value["message"]
            .as_str()
            .is_some_and(|message| message.contains("sdkwork-notary-app-api")),
        "missing Notary dependency error should identify the unconfigured upstream: {value}"
    );
}

#[tokio::test]
async fn gateway_derives_proxied_im_http_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(
        Router::new().route("/im/v3/api/realtime/events", get(echo_context_upstream)),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

#[tokio::test]
async fn gateway_drops_sdkwork_internal_headers_when_signature_secret_is_configured() {
    let _signature_secret = ScopedEnvVar::set(
        "CRAW_CHAT_APP_CONTEXT_SIGNATURE_SECRET",
        "gateway-signing-secret",
    );
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(Router::new().route(
        "/im/v3/api/realtime/events",
        get(require_signed_context_upstream),
    ))
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed-signature")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "tenant_real");
    assert_eq!(context["userId"], "user_real");
    assert_eq!(context["sessionId"], "session_real");
    assert_eq!(context["signaturePresent"], false);
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
}

#[tokio::test]
async fn gateway_accepts_numeric_appbase_session_context_ids_for_proxied_im_routes() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_numeric_current_session),
    ))
    .await;
    let session_gateway = spawn_app_upstream(
        Router::new().route("/im/v3/api/realtime/events", get(echo_context_upstream)),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/realtime/events?afterSeq=0&limit=1")
                .header(
                    header::AUTHORIZATION,
                    gateway_numeric_authorization_header(),
                )
                .header("access-token", gateway_numeric_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "org_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let context = read_json_body(response).await;
    assert_eq!(context["tenantId"], "20001");
    assert_eq!(context["organizationId"], "30001");
    assert_eq!(context["userId"], "user_numeric");
    assert_eq!(context["sdkworkInternalHeadersForwarded"], false);
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["organizationId"], "org_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

#[tokio::test]
async fn gateway_derives_proxied_im_calls_context_from_appbase_dual_tokens_not_client_headers() {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let rtc = spawn_app_upstream(
        Router::new()
            .route(
                "/im/v3/api/calls/sessions/rtc_demo/signals",
                any(echo_context_upstream),
            )
            .route(
                "/im/v3/api/calls/sessions/rtc_demo/credentials",
                any(echo_context_upstream),
            ),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("im-calls-service", rtc.base_url.as_str()),
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
    ]));

    for (method, path) in [
        (Method::GET, "/im/v3/api/calls/sessions/rtc_demo/signals"),
        (
            Method::POST,
            "/im/v3/api/calls/sessions/rtc_demo/credentials",
        ),
    ] {
        let body = if method == Method::GET {
            Body::empty()
        } else {
            Body::from(json!({ "participantId": "user_real" }).to_string())
        };
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .header(header::AUTHORIZATION, gateway_test_authorization_header())
                    .header("access-token", gateway_test_access_token_header())
                    .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                    .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                    .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                    .body(body)
                    .unwrap(),
            )
            .await
            .unwrap_or_else(|error| {
                panic!("gateway IM calls request should succeed for {path}: {error}")
            });

        assert_eq!(
            response.status(),
            StatusCode::OK,
            "gateway IM calls request should succeed for {path}"
        );
        let context = read_json_body(response).await;
        assert_eq!(context["tenantId"], "tenant_real", "{path} tenant context");
        assert_eq!(context["userId"], "user_real", "{path} user context");
        assert_eq!(
            context["sessionId"], "session_real",
            "{path} session context"
        );
        assert_eq!(
            context["sdkworkInternalHeadersForwarded"], false,
            "{path} must not receive client-supplied SDKWork SDKWork internal headers"
        );
        assert_ne!(context["tenantId"], "t_demo");
        assert_ne!(context["userId"], "user_test006_a_com");
    }
}

#[tokio::test]
async fn gateway_derives_proxied_chat_data_context_from_appbase_dual_tokens_not_client_headers() {
    for (service_id, method, path, route) in [
        (
            "conversation-runtime",
            Method::POST,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "projection-service",
            Method::GET,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "streaming-service",
            Method::POST,
            "/im/v3/api/streams",
            "/im/v3/api/streams",
        ),
        (
            "media-service",
            Method::POST,
            "/im/v3/api/media/uploads",
            "/im/v3/api/media/uploads",
        ),
        (
            "notification-service",
            Method::GET,
            "/app/v3/api/notifications",
            "/app/v3/api/notifications",
        ),
        (
            "automation-service",
            Method::POST,
            "/app/v3/api/automation/jobs",
            "/app/v3/api/automation/jobs",
        ),
        (
            "sdkwork-drive-app-api",
            Method::POST,
            "/app/v3/api/drive/uploader/uploads",
            "/app/v3/api/drive/uploader/uploads",
        ),
    ] {
        assert_gateway_derives_context_for_configured_upstream(
            service_id,
            method.clone(),
            path,
            route,
        )
        .await;
    }
}

#[tokio::test]
async fn gateway_derives_context_for_protected_routes_without_appbase_session_lookup() {
    for (service_id, method, path, route) in [
        (
            "session-gateway",
            Method::GET,
            "/im/v3/api/realtime/events?afterSeq=0&limit=1",
            "/im/v3/api/realtime/events",
        ),
        (
            "conversation-runtime",
            Method::POST,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "projection-service",
            Method::GET,
            "/im/v3/api/chat/conversations/c_demo/messages",
            "/im/v3/api/chat/conversations/{id}/messages",
        ),
        (
            "streaming-service",
            Method::POST,
            "/im/v3/api/streams",
            "/im/v3/api/streams",
        ),
        (
            "media-service",
            Method::POST,
            "/im/v3/api/media/uploads",
            "/im/v3/api/media/uploads",
        ),
        (
            "im-calls-service",
            Method::GET,
            "/im/v3/api/calls/sessions/rtc_demo/signals",
            "/im/v3/api/calls/sessions/{id}/signals",
        ),
        (
            "notification-service",
            Method::GET,
            "/app/v3/api/notifications",
            "/app/v3/api/notifications",
        ),
        (
            "automation-service",
            Method::POST,
            "/app/v3/api/automation/jobs",
            "/app/v3/api/automation/jobs",
        ),
        (
            "sdkwork-drive-app-api",
            Method::POST,
            "/app/v3/api/drive/uploader/uploads",
            "/app/v3/api/drive/uploader/uploads",
        ),
    ] {
        assert_gateway_derives_context_without_appbase_session_lookup(
            service_id,
            method.clone(),
            path,
            route,
        )
        .await;
    }
}

#[tokio::test]
async fn gateway_handles_browser_cors_preflight_for_im_app_iam_routes() {
    let appbase = spawn_upstream("sdkwork-appbase-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-appbase-app-api",
        appbase.base_url.as_str(),
    )]));
    let origin = "http://127.0.0.1:1620";

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/app/v3/api/oauth/device_authorizations")
                .header("origin", origin)
                .header("access-control-request-method", "POST")
                .header(
                    "access-control-request-headers",
                    "authorization,content-type,access-token",
                )
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("gateway CORS preflight should succeed");

    assert!(matches!(
        response.status(),
        StatusCode::OK | StatusCode::NO_CONTENT
    ));
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|value| value.to_str().ok()),
        Some(origin)
    );
    assert!(
        response
            .headers()
            .get("x-craw-chat-upstream-service")
            .is_none(),
        "gateway should answer browser preflight itself instead of proxying it to appbase"
    );

    let allow_methods = response
        .headers()
        .get("access-control-allow-methods")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_uppercase();
    assert!(allow_methods.contains("POST"));
    assert!(allow_methods.contains("OPTIONS"));

    let allow_headers = response
        .headers()
        .get("access-control-allow-headers")
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default()
        .to_ascii_lowercase();
    for expected in ["authorization", "content-type", "access-token"] {
        assert!(
            allow_headers.contains(expected),
            "gateway CORS preflight must allow {expected}, got {allow_headers}"
        );
    }
}

#[tokio::test]
async fn gateway_adds_browser_cors_headers_to_im_app_iam_responses() {
    let appbase = spawn_upstream("sdkwork-appbase-app-api").await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "sdkwork-appbase-app-api",
        appbase.base_url.as_str(),
    )]));
    let origin = "http://127.0.0.1:1620";

    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/app/v3/api/oauth/device_authorizations")
                .header("origin", origin)
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(
        response
            .headers()
            .get("access-control-allow-origin")
            .and_then(|value| value.to_str().ok()),
        Some(origin)
    );
    assert_eq!(
        response
            .headers()
            .get("x-craw-chat-upstream-service")
            .and_then(|value| value.to_str().ok()),
        Some("sdkwork-appbase-app-api")
    );
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

async fn read_json_body(response: axum::response::Response) -> serde_json::Value {
    serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json")
}

struct TestUpstream {
    base_url: String,
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

async fn spawn_upstream(service_id: &str) -> TestUpstream {
    spawn_app_upstream(
        Router::new()
            .route("/", any(echo_upstream))
            .route("/{*path}", any(echo_upstream))
            .with_state(UpstreamState {
                service_id: Arc::<str>::from(service_id),
            }),
    )
    .await
}

async fn spawn_app_upstream(app: Router) -> TestUpstream {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test upstream should bind local port");
    let local_addr = listener
        .local_addr()
        .expect("test upstream should expose local addr");

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("test upstream server should run");
    });

    TestUpstream {
        base_url: format!("http://{local_addr}"),
    }
}

async fn assert_gateway_derives_context_for_configured_upstream(
    service_id: &str,
    method: Method,
    path: &str,
    upstream_route: &str,
) {
    let appbase = spawn_app_upstream(Router::new().route(
        "/app/v3/api/auth/sessions/current",
        get(appbase_current_session),
    ))
    .await;
    let upstream =
        spawn_app_upstream(Router::new().route(upstream_route, any(echo_context_upstream))).await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream(service_id, upstream.base_url.as_str()),
        service_upstream("sdkwork-appbase-app-api", appbase.base_url.as_str()),
    ]));
    let body = if method == Method::GET {
        Body::empty()
    } else {
        Body::from("{}")
    };
    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap_or_else(|error| panic!("gateway request should succeed for {service_id}: {error}"));

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "gateway request should succeed for {service_id}"
    );
    let context = read_json_body(response).await;
    assert_eq!(
        context["tenantId"], "tenant_real",
        "{service_id} must receive dual-token tenant context"
    );
    assert_eq!(
        context["userId"], "user_real",
        "{service_id} must receive dual-token user context"
    );
    assert_eq!(
        context["sessionId"], "session_real",
        "{service_id} must receive dual-token session context"
    );
    assert_eq!(
        context["sdkworkInternalHeadersForwarded"], false,
        "{service_id} must not receive client-supplied SDKWork SDKWork internal headers"
    );
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

async fn assert_gateway_derives_context_without_appbase_session_lookup(
    service_id: &str,
    method: Method,
    path: &str,
    upstream_route: &str,
) {
    let upstream =
        spawn_app_upstream(Router::new().route(upstream_route, any(echo_context_upstream))).await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        service_id,
        upstream.base_url.as_str(),
    )]));
    let body = if method == Method::GET {
        Body::empty()
    } else {
        Body::from("{}")
    };

    let response = app
        .oneshot(
            Request::builder()
                .method(method)
                .uri(path)
                .header(header::AUTHORIZATION, gateway_test_authorization_header())
                .header("access-token", gateway_test_access_token_header())
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "t_demo")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "user_test006_a_com")
                .header(SDKWORK_INTERNAL_HEADER_PROBE, "spoofed_session")
                .body(body)
                .unwrap(),
        )
        .await
        .unwrap_or_else(|error| panic!("gateway request should succeed for {service_id}: {error}"));

    assert_eq!(
        response.status(),
        StatusCode::OK,
        "{service_id} must derive request context directly from dual tokens"
    );
    let context = read_json_body(response).await;
    assert_eq!(
        context["tenantId"], "tenant_real",
        "{service_id} must receive dual-token tenant context without appbase session lookup"
    );
    assert_eq!(
        context["userId"], "user_real",
        "{service_id} must receive dual-token user context without appbase session lookup"
    );
    assert_eq!(
        context["sessionId"], "session_real",
        "{service_id} must receive dual-token session context without appbase session lookup"
    );
    assert_eq!(
        context["sdkworkInternalHeadersForwarded"], false,
        "{service_id} must not receive client-supplied SDKWork SDKWork internal headers"
    );
    assert_ne!(context["tenantId"], "t_demo");
    assert_ne!(context["userId"], "user_test006_a_com");
}

async fn echo_upstream(
    State(state): State<UpstreamState>,
    method: Method,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "method": method.as_str(),
        "path": request.uri().path(),
    }))
}

async fn appbase_current_session(headers: HeaderMap) -> Response {
    let Ok(context) = resolve_app_context(&headers) else {
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
    };

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": context.tenant_id,
                    "organizationId": context.organization_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "appId": context.app_id,
                    "actorKind": context.actor_kind
                }
            }
        })),
    )
        .into_response()
}

async fn appbase_numeric_current_session(headers: HeaderMap) -> Response {
    let Ok(context) = resolve_app_context(&headers) else {
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
    };

    (
        StatusCode::OK,
        Json(json!({
            "data": {
                "context": {
                    "tenantId": context.tenant_id,
                    "organizationId": context.organization_id,
                    "userId": context.user_id,
                    "sessionId": context.session_id,
                    "appId": context.app_id,
                    "actorKind": context.actor_kind
                }
            }
        })),
    )
        .into_response()
}

async fn echo_context_upstream(headers: HeaderMap) -> Json<serde_json::Value> {
    match resolve_app_context(&headers) {
        Ok(context) => Json(json!({
            "tenantId": context.tenant_id,
            "organizationId": context.organization_id,
            "userId": context.user_id,
            "sessionId": context.session_id,
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        })),
        Err(error) => Json(json!({
            "code": error.code(),
            "message": error.message(),
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        })),
    }
}

async fn require_signed_context_upstream(headers: HeaderMap) -> Response {
    match im_app_context::resolve_app_context_with_signature_config(
        &headers,
        im_app_context::AppContextSignatureConfig {
            require_signature: true,
            shared_secret: Some("gateway-signing-secret".to_owned()),
        },
    ) {
        Ok(context) => Json(json!({
            "tenantId": context.tenant_id,
            "userId": context.user_id,
            "sessionId": context.session_id,
            "signaturePresent": header_value(&headers, "x-sdkwork-context-signature").is_some(),
            "sdkworkInternalHeadersForwarded": has_sdkwork_internal_header(&headers),
        }))
        .into_response(),
        Err(error) => (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "code": error.code(),
                "message": error.message(),
            })),
        )
            .into_response(),
    }
}

fn header_value(headers: &HeaderMap, name: &str) -> Option<String> {
    headers
        .get(name)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned)
}

fn has_sdkwork_internal_header(headers: &HeaderMap) -> bool {
    [
        "x-sdkwork-tenant-id",
        "x-sdkwork-organization-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-context-signature",
    ]
    .iter()
    .any(|name| headers.contains_key(*name))
}

async fn echo_runtime_section(
    State(state): State<UpstreamState>,
    axum::extract::Path(section): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "section": section,
    }))
}
