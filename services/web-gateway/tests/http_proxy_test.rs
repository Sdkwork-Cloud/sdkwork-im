use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    routing::{any, get},
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use http_body_util::BodyExt;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Clone)]
struct UpstreamState {
    service_id: Arc<str>,
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
    let projection = spawn_upstream("projection-service").await;
    let conversation_runtime = spawn_upstream("conversation-runtime").await;
    let app = web_gateway::build_app(test_gateway_config(vec![
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
async fn embedded_gateway_rejects_im_app_iam_requests_when_appbase_upstream_is_absent() {
    let embedded_runtime = Router::new();
    let product_runtime = Router::new()
        .route(
            "/app/v3/api/open_platform/qr_auth/sessions",
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
                .uri("/app/v3/api/open_platform/qr_auth/sessions")
                .header("content-type", "application/json")
                .body(Body::from("{}"))
                .unwrap(),
        )
        .await
        .expect("gateway request should succeed");

    assert_eq!(response.status(), StatusCode::BAD_GATEWAY);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("response body should collect")
            .to_bytes(),
    )
    .expect("response body should be valid json");
    assert_eq!(
        value["message"],
        "upstream target is not configured for sdkwork-appbase-app-api"
    );
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
        Some(local_minimal_node::build_default_app()),
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
                        "username": "target-user@sdkwork-iam.local",
                        "displayName": "Target Local User",
                        "password": "dev123456"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase-owned registration request should return response");
    assert_eq!(target_registration.status(), StatusCode::BAD_GATEWAY);
    let target_registration = read_json_body(target_registration).await;
    assert_eq!(
        target_registration["message"],
        "upstream target is not configured for sdkwork-appbase-app-api"
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
                        "username": "searcher-user@sdkwork-iam.local",
                        "password": "dev123456"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .expect("appbase-owned login request should return response");
    assert_eq!(searcher_session.status(), StatusCode::BAD_GATEWAY);

    let search_response = app
        .oneshot(
            Request::builder()
                .method(Method::GET)
                .uri("/im/v3/api/social/users?q=target-user&limit=20")
                .header("access-token", "access-token")
                .header("x-sdkwork-app-id", "sdkwork-chat-pc")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-session-id", "sdkwork_iam_session_demo")
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
    assert!(
        items.is_empty(),
        "social search must not be seeded by product-runtime auth registration; response: {search}"
    );
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
                .uri("/app/v3/api/open_platform/qr_auth/sessions")
                .header("origin", origin)
                .header("access-control-request-method", "POST")
                .header(
                    "access-control-request-headers",
                    "content-type,access-token,x-sdkwork-tenant-id,x-sdkwork-session-id",
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
    for expected in [
        "content-type",
        "access-token",
        "x-sdkwork-tenant-id",
        "x-sdkwork-session-id",
    ] {
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
                .uri("/app/v3/api/open_platform/qr_auth/sessions")
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

async fn spawn_upstream(service_id: &str) -> TestUpstream {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test upstream should bind local port");
    let local_addr = listener
        .local_addr()
        .expect("test upstream should expose local addr");
    let state = UpstreamState {
        service_id: Arc::<str>::from(service_id),
    };
    let app = Router::new()
        .route("/", any(echo_upstream))
        .route("/{*path}", any(echo_upstream))
        .with_state(state);

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("test upstream server should run");
    });

    TestUpstream {
        base_url: format!("http://{local_addr}"),
    }
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

async fn echo_runtime_section(
    State(state): State<UpstreamState>,
    axum::extract::Path(section): axum::extract::Path<String>,
) -> Json<serde_json::Value> {
    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "section": section,
    }))
}
