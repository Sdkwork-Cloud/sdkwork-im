use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{Method, Request, StatusCode},
    routing::any,
};
use craw_chat_gateway_config::{GatewayRuntimeMode, WebGatewayConfig, service_upstream};
use craw_chat_gateway_observability::{
    build_startup_summary_with_registry, format_startup_summary,
};
use craw_chat_runtime_link::LINK_WEBSOCKET_SUBPROTOCOL;
use http_body_util::BodyExt;
use serde_json::json;
use std::sync::Arc;
use tower::ServiceExt;

#[derive(Clone)]
struct OpenApiUpstreamState {
    service_id: Arc<str>,
    openapi: serde_json::Value,
}

#[tokio::test]
async fn gateway_exposes_aggregate_openapi_json() {
    let control_plane = spawn_openapi_upstream(
        "control-plane-api",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/api/v1/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let projection = spawn_openapi_upstream(
        "projection-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Craw Chat Projection Service API", "version": "0.1.0" },
            "paths": {
                "/api/v1/conversations/{conversation_id}/messages": {
                    "get": { "summary": "Get messages", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let runtime = spawn_openapi_upstream(
        "conversation-runtime",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Craw Chat Conversation Runtime API", "version": "0.1.0" },
            "paths": {
                "/api/v1/conversations/{conversation_id}/messages": {
                    "post": { "summary": "Post message", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("control-plane-api", control_plane.base_url.as_str()),
        service_upstream("projection-service", projection.base_url.as_str()),
        service_upstream("conversation-runtime", runtime.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("aggregate openapi request should succeed");

    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("aggregate openapi body should collect")
            .to_bytes(),
    )
    .expect("aggregate openapi should be valid json");

    assert_eq!(value["openapi"], "3.1.0");
    assert_eq!(value["info"]["title"], "Craw Chat Unified Gateway API");
    assert!(value["paths"]["/openapi/runtime-summary.json"]["get"].is_object());
    assert_eq!(
        value["paths"]["/openapi/index.json"]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"],
        "#/components/schemas/GatewayOpenapiIndex"
    );
    assert_eq!(
        value["paths"]["/openapi/runtime-summary.json"]["get"]["responses"]["200"]["content"]["application/json"]
            ["schema"]["$ref"],
        "#/components/schemas/GatewayRuntimeSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["services"]["items"]["$ref"],
        "#/components/schemas/GatewayServiceSchemaIndexEntry"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["routes"]["items"]["$ref"],
        "#/components/schemas/GatewayRouteSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayOpenapiIndex"]["properties"]["surfaceGroups"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySurfaceGroupSummary"
    );
    assert_eq!(
        value["components"]["schemas"]["GatewayRuntimeSummary"]["properties"]["surfaceGroups"]["items"]
            ["$ref"],
        "#/components/schemas/GatewaySurfaceGroupSummary"
    );
    assert!(value["paths"]["/api/v1/control/protocol-registry"]["get"].is_object());
    assert!(value["paths"]["/api/v1/conversations/{conversation_id}/messages"]["get"].is_object());
    assert!(value["paths"]["/api/v1/conversations/{conversation_id}/messages"]["post"].is_object());
}

#[tokio::test]
async fn gateway_exposes_openapi_service_index_and_service_schema_proxy() {
    let control_plane = spawn_openapi_upstream(
        "control-plane-api",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/api/v1/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "control-plane-api",
        control_plane.base_url.as_str(),
    )]));

    let index_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");
    assert_eq!(index_value["services"][0]["serviceId"], "control-plane-api");
    assert_eq!(
        index_value["services"][0]["schemaUrl"],
        "/openapi/services/control-plane-api.openapi.json"
    );
    assert_eq!(
        index_value["services"][0]["docsUrl"],
        "/docs/services/control-plane-api"
    );
    assert_eq!(index_value["services"][0]["visibility"], "internal");
    assert_eq!(index_value["services"][0]["routeCount"], 1);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["control"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["controlPlaneSdk"])
    );
    assert_eq!(index_value["services"][0]["protocols"], json!(["http"]));
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "control-plane-api"
                    && route["operationGroup"] == "control"
                    && route["pathPattern"] == "/api/v1/control/{*path}"
                    && route["methods"]
                        == json!(["delete", "get", "head", "options", "patch", "post", "put"])
                    && route["protocol"] == "http"
                    && route["sdkTargets"] == json!(["controlPlaneSdk"])
            })
    );
    assert!(
        index_value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "control-plane-api"
                    && group["operationGroup"] == "control"
                    && group["visibility"] == "internal"
                    && group["routeCount"] == 1
                    && group["sdkTargets"] == json!(["controlPlaneSdk"])
                    && group["protocols"] == json!(["http"])
            })
    );

    let service_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/services/control-plane-api.openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("service schema proxy request should succeed");
    assert_eq!(service_response.status(), StatusCode::OK);
    let service_value: serde_json::Value = serde_json::from_slice(
        &service_response
            .into_body()
            .collect()
            .await
            .expect("service schema body should collect")
            .to_bytes(),
    )
    .expect("service schema should be valid json");
    assert_eq!(service_value["info"]["title"], "Control Plane API");
}

#[tokio::test]
async fn gateway_service_index_surfaces_session_websocket_metadata() {
    let session_gateway = spawn_openapi_upstream(
        "session-gateway",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Craw Chat Session Gateway API", "version": "0.1.0" },
            "paths": {
                "/api/v1/sessions/resume": {
                    "post": { "summary": "Resume session", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/presence/me": {
                    "get": { "summary": "Get current presence", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/realtime/ws": {
                    "get": {
                        "summary": "Open realtime websocket session",
                        "responses": { "101": { "description": "websocket upgrade successful" } },
                        "x-craw-chat-protocol": "websocket",
                        "x-craw-chat-websocket-subprotocols": ["ccp"]
                    }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "session-gateway",
        session_gateway.base_url.as_str(),
    )]));

    let index_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");

    assert_eq!(index_value["services"][0]["serviceId"], "session-gateway");
    assert_eq!(index_value["services"][0]["visibility"], "public");
    assert_eq!(index_value["services"][0]["routeCount"], 4);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["presence", "realtime", "sessions"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["crawChatAppSdk"])
    );
    assert_eq!(
        index_value["services"][0]["protocols"],
        json!(["http", "websocket"])
    );
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "session-gateway"
                    && route["operationGroup"] == "realtime"
                    && route["pathPattern"] == "/api/v1/realtime/ws"
                    && route["protocol"] == "websocket"
                    && route["websocketSubprotocols"] == json!([LINK_WEBSOCKET_SUBPROTOCOL])
            })
    );
    assert!(
        index_value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "session-gateway"
                    && group["operationGroup"] == "realtime"
                    && group["routeCount"] == 2
                    && group["protocols"] == json!(["http", "websocket"])
                    && group["websocketSubprotocols"] == json!([LINK_WEBSOCKET_SUBPROTOCOL])
            })
    );
    let websocket_subprotocols = index_value["services"][0]["websocketSubprotocols"]
        .as_array()
        .expect("websocket subprotocols should be an array");
    assert_eq!(websocket_subprotocols.len(), 1);
    assert!(
        websocket_subprotocols[0]
            .as_str()
            .is_some_and(|value| !value.is_empty()),
        "websocket subprotocol should be non-empty"
    );
}

#[tokio::test]
async fn gateway_service_index_surfaces_projection_device_metadata() {
    let projection = spawn_openapi_upstream(
        "projection-service",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Craw Chat Projection Service API", "version": "0.1.0" },
            "paths": {
                "/api/v1/devices/register": {
                    "post": { "summary": "Register device", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/devices/{device_id}/sync-feed": {
                    "get": { "summary": "Get device sync feed", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/inbox": {
                    "get": { "summary": "Get inbox", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![service_upstream(
        "projection-service",
        projection.base_url.as_str(),
    )]));

    let index_response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/index.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi index request should succeed");
    assert_eq!(index_response.status(), StatusCode::OK);
    let index_value: serde_json::Value = serde_json::from_slice(
        &index_response
            .into_body()
            .collect()
            .await
            .expect("openapi index body should collect")
            .to_bytes(),
    )
    .expect("openapi index should be valid json");

    assert_eq!(
        index_value["services"][0]["serviceId"],
        "projection-service"
    );
    assert_eq!(index_value["services"][0]["visibility"], "public");
    assert_eq!(index_value["services"][0]["routeCount"], 5);
    assert_eq!(
        index_value["services"][0]["operationGroups"],
        json!(["conversations", "devices"])
    );
    assert_eq!(
        index_value["services"][0]["sdkTargets"],
        json!(["crawChatAppSdk"])
    );
    assert_eq!(index_value["services"][0]["protocols"], json!(["http"]));
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "projection-service"
                    && route["operationGroup"] == "devices"
                    && route["pathPattern"] == "/api/v1/devices/register"
                    && route["methods"] == json!(["post"])
                    && route["protocol"] == "http"
                    && route["sdkTargets"] == json!(["crawChatAppSdk"])
            })
    );
    assert!(
        index_value["routes"]
            .as_array()
            .expect("routes should be an array")
            .iter()
            .any(|route| {
                route["serviceId"] == "projection-service"
                    && route["operationGroup"] == "devices"
                    && route["pathPattern"] == "/api/v1/devices/{device_id}/sync-feed"
                    && route["methods"] == json!(["get"])
                    && route["protocol"] == "http"
                    && route["sdkTargets"] == json!(["crawChatAppSdk"])
            })
    );
    assert!(
        index_value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "projection-service"
                    && group["operationGroup"] == "devices"
                    && group["routeCount"] == 2
                    && group["protocols"] == json!(["http"])
                    && group["sdkTargets"] == json!(["crawChatAppSdk"])
            })
    );
}

#[tokio::test]
async fn gateway_exposes_runtime_summary_json() {
    let control_plane = spawn_openapi_upstream(
        "control-plane-api",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Control Plane API", "version": "0.1.0" },
            "paths": {
                "/api/v1/control/protocol-registry": {
                    "get": { "summary": "Get protocol registry", "responses": { "200": { "description": "ok" } } }
                }
            }
        }),
    )
    .await;
    let session_gateway = spawn_openapi_upstream(
        "session-gateway",
        json!({
            "openapi": "3.1.0",
            "info": { "title": "Craw Chat Session Gateway API", "version": "0.1.0" },
            "paths": {
                "/api/v1/sessions/resume": {
                    "post": { "summary": "Resume session", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/presence/me": {
                    "get": { "summary": "Get current presence", "responses": { "200": { "description": "ok" } } }
                },
                "/api/v1/realtime/ws": {
                    "get": {
                        "summary": "Open realtime websocket session",
                        "responses": { "101": { "description": "websocket upgrade successful" } },
                        "x-craw-chat-protocol": "websocket",
                        "x-craw-chat-websocket-subprotocols": ["ccp"]
                    }
                }
            }
        }),
    )
    .await;
    let app = web_gateway::build_app(test_gateway_config(vec![
        service_upstream("control-plane-api", control_plane.base_url.as_str()),
        service_upstream("session-gateway", session_gateway.base_url.as_str()),
    ]));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/runtime-summary.json")
                .header("host", "gateway.example:18079")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("runtime summary request should succeed");
    assert_eq!(response.status(), StatusCode::OK);
    let value: serde_json::Value = serde_json::from_slice(
        &response
            .into_body()
            .collect()
            .await
            .expect("runtime summary body should collect")
            .to_bytes(),
    )
    .expect("runtime summary should be valid json");

    assert_eq!(value["baseUrl"], "http://gateway.example:18079");
    assert_eq!(
        value["aggregateOpenapiUrl"],
        "http://gateway.example:18079/openapi.json"
    );
    assert_eq!(
        value["openapiIndexUrl"],
        "http://gateway.example:18079/openapi/index.json"
    );
    assert_eq!(
        value["runtimeSummaryUrl"],
        "http://gateway.example:18079/openapi/runtime-summary.json"
    );
    assert_eq!(
        value["serviceContracts"][0]["schemaUrl"],
        "http://gateway.example:18079/openapi/services/control-plane-api.openapi.json"
    );
    assert!(
        value["publicEndpoints"]
            .as_array()
            .expect("public endpoints should be an array")
            .iter()
            .any(|endpoint| {
                endpoint["pathPattern"] == "/api/v1/realtime/ws"
                    && endpoint["protocol"] == "websocket"
                    && endpoint["visibility"] == "public"
            })
    );
    assert!(
        value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "session-gateway"
                    && group["operationGroup"] == "realtime"
                    && group["routeCount"] == 2
            })
    );
    assert!(
        value["surfaceGroups"]
            .as_array()
            .expect("surface groups should be an array")
            .iter()
            .any(|group| {
                group["serviceId"] == "control-plane-api"
                    && group["operationGroup"] == "control"
                    && group["visibility"] == "internal"
            })
    );
}

#[test]
fn startup_summary_lists_gateway_openapi_endpoints() {
    let registry =
        web_gateway::build_gateway_registry().expect("gateway route registry should build");
    let config = test_gateway_config(vec![service_upstream(
        "control-plane-api",
        "http://127.0.0.1:18081",
    )]);
    let summary = build_startup_summary_with_registry(&config, &registry, "http://127.0.0.1:18079");
    let text = format_startup_summary(&summary);

    assert!(text.contains("OpenAPI 3.1 Schemas"));
    assert!(text.contains("http://127.0.0.1:18079/openapi.json"));
    assert!(text.contains("http://127.0.0.1:18079/openapi/index.json"));
    assert!(text.contains("http://127.0.0.1:18079/openapi/runtime-summary.json"));
    assert!(text.contains("http://127.0.0.1:18079/"));
    assert!(text.contains("http://127.0.0.1:18079/admin/"));
    assert!(text.contains("Gateway Endpoints"));
    assert!(text.contains("/api/v1/realtime/ws"));
    assert!(text.contains("Gateway Surface Groups"));
    assert!(text.contains(
        "public session-gateway realtime [sdk:crawChatAppSdk] [protocols:http,websocket]: 2 routes"
    ));
    assert!(text.contains(
        "internal control-plane-api control [sdk:controlPlaneSdk] [protocols:http]: 1 routes"
    ));
}

#[test]
fn startup_summary_lists_per_service_schema_and_docs_endpoints() {
    let registry =
        web_gateway::build_gateway_registry().expect("gateway route registry should build");
    let config = test_gateway_config(vec![
        service_upstream("control-plane-api", "http://127.0.0.1:18081"),
        service_upstream("conversation-runtime", "http://127.0.0.1:18082"),
    ]);
    let summary = build_startup_summary_with_registry(&config, &registry, "http://127.0.0.1:18079");
    let text = format_startup_summary(&summary);

    assert!(text.contains("Service Contracts"));
    assert!(text.contains(
        "control-plane-api schema: http://127.0.0.1:18079/openapi/services/control-plane-api.openapi.json"
    ));
    assert!(text.contains(
        "control-plane-api docs: http://127.0.0.1:18079/docs/services/control-plane-api"
    ));
    assert!(text.contains(
        "conversation-runtime schema: http://127.0.0.1:18079/openapi/services/conversation-runtime.openapi.json"
    ));
    assert!(text.contains(
        "conversation-runtime docs: http://127.0.0.1:18079/docs/services/conversation-runtime"
    ));
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

struct TestOpenApiUpstream {
    base_url: String,
}

async fn spawn_openapi_upstream(
    service_id: &str,
    openapi: serde_json::Value,
) -> TestOpenApiUpstream {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("openapi upstream should bind local port");
    let local_addr = listener
        .local_addr()
        .expect("openapi upstream should expose local addr");
    let app = Router::new()
        .route("/", any(openapi_upstream))
        .route("/{*path}", any(openapi_upstream))
        .with_state(OpenApiUpstreamState {
            service_id: Arc::<str>::from(service_id),
            openapi,
        });

    tokio::spawn(async move {
        axum::serve(listener, app)
            .await
            .expect("openapi upstream server should run");
    });

    TestOpenApiUpstream {
        base_url: format!("http://{local_addr}"),
    }
}

async fn openapi_upstream(
    State(state): State<OpenApiUpstreamState>,
    method: Method,
    request: Request<Body>,
) -> Json<serde_json::Value> {
    if method == Method::GET && request.uri().path() == "/openapi.json" {
        return Json(state.openapi);
    }

    Json(json!({
        "serviceId": state.service_id.as_ref(),
        "path": request.uri().path()
    }))
}
