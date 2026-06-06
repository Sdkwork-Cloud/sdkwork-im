use axum::body::{Body, to_bytes};
use axum::http::header::CONTENT_TYPE;
use axum::http::{HeaderValue, Method, Request, StatusCode};
use local_minimal_node::build_default_app;
use serde_json::{Value, json};
use tower::ServiceExt;

fn agent_auth_headers(mut request: Request<Body>) -> Request<Body> {
    let headers = request.headers_mut();
    headers.insert("x-sdkwork-tenant-id", HeaderValue::from_static("1"));
    headers.insert(
        "x-sdkwork-user-id",
        HeaderValue::from_static("u-agent-local"),
    );
    headers.insert("x-sdkwork-actor-kind", HeaderValue::from_static("user"));
    request
}

fn agent_manifest(agent_id: &str, display_name: &str) -> Value {
    json!({
        "schema_version": "1.0.0",
        "manifest_type": "agent",
        "agent_id": agent_id,
        "name": agent_id,
        "display_name": display_name,
        "description": "local runtime agent app api contract",
        "version": "0.1.0",
        "domain": "intelligence",
        "required_capabilities": [{ "capability_id": "model.chat" }],
        "optional_capabilities": [{ "capability_id": "tool.invoke" }],
        "event_families": ["agent.lifecycle"],
        "owner": { "name": "sdkwork" },
        "status": "active"
    })
}

#[tokio::test]
async fn local_minimal_node_mounts_agent_app_api_for_sdkwork_agent_app_sdk() {
    let app = build_default_app();
    let create_body = json!({
        "agentId": "agent.local.runtime",
        "organizationId": "10",
        "ownerUserId": "100",
        "code": "agent-local-runtime",
        "displayName": "Local Runtime Agent",
        "description": "created through local-minimal-node agent app api",
        "manifest": agent_manifest("agent.local.runtime", "Local Runtime Agent"),
        "defaultCodeTaskIntent": {
            "prompt": "Use the local runtime Agent App API",
            "contextPaths": ["services/local-minimal-node"],
            "constraints": ["sdkwork-agent-app-sdk compatible"]
        },
        "implementationKind": "manifest-only",
        "visibility": "private",
        "tags": ["local-runtime"],
        "requestedAt": "2026-06-05T00:00:00Z"
    });

    let create_request = Request::builder()
        .method(Method::POST)
        .uri("/app/v3/api/ai/agents?tenant_id=1")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(create_body.to_string()))
        .expect("create request should be built");
    let create_response = app
        .clone()
        .oneshot(agent_auth_headers(create_request))
        .await
        .expect("agent create request should return a response");

    assert_eq!(create_response.status(), StatusCode::CREATED);

    let retrieve_request = Request::builder()
        .method(Method::GET)
        .uri("/app/v3/api/ai/agents/agent.local.runtime?tenant_id=1")
        .body(Body::empty())
        .expect("retrieve request should be built");
    let retrieve_response = app
        .clone()
        .oneshot(agent_auth_headers(retrieve_request))
        .await
        .expect("agent retrieve request should return a response");

    assert_eq!(retrieve_response.status(), StatusCode::OK);
    let body_bytes = to_bytes(retrieve_response.into_body(), usize::MAX)
        .await
        .expect("agent response body should be readable");
    let body_json: Value =
        serde_json::from_slice(&body_bytes).expect("agent response body should be valid json");
    assert_eq!(body_json["data"]["agentId"], "agent.local.runtime");
    assert_eq!(body_json["data"]["displayName"], "Local Runtime Agent");
    assert_eq!(body_json["data"]["implementationKind"], "manifest-only");

    let preview_body = json!({
        "executionId": "execution.local.runtime.preview.1",
        "content": "hello local agent",
        "debugMode": true,
        "memoryEnabled": false,
        "model": "model.local",
        "temperature": 0.2,
        "inputPayload": {
            "agent": {
                "id": "agent.local.runtime",
                "name": "Local Runtime Agent"
            },
            "content": "hello local agent"
        },
        "requestedAt": "2026-06-05T00:00:01Z"
    });
    let preview_request = Request::builder()
        .method(Method::POST)
        .uri("/app/v3/api/ai/agents/agent.local.runtime/preview_responses?tenant_id=1")
        .header(CONTENT_TYPE, "application/json")
        .body(Body::from(preview_body.to_string()))
        .expect("preview request should be built");
    let preview_response = app
        .oneshot(agent_auth_headers(preview_request))
        .await
        .expect("agent preview request should return a response");

    assert_eq!(preview_response.status(), StatusCode::OK);
    let body_bytes = to_bytes(preview_response.into_body(), usize::MAX)
        .await
        .expect("agent preview response body should be readable");
    let body_json: Value = serde_json::from_slice(&body_bytes)
        .expect("agent preview response body should be valid json");
    assert_eq!(
        body_json["data"]["executionId"],
        "execution.local.runtime.preview.1"
    );
    assert_eq!(body_json["data"]["agentId"], "agent.local.runtime");
    assert_eq!(body_json["data"]["operation"], "preview_response");
    assert_eq!(
        body_json["data"]["outputPayload"]["content"],
        "hello local agent"
    );
}

#[tokio::test]
async fn local_minimal_node_does_not_mount_unrelated_appbase_business_routes() {
    let app = build_default_app();

    for (method, uri, body) in [
        (Method::GET, "/app/v3/api/notifications", ""),
        (
            Method::POST,
            "/app/v3/api/automation/executions",
            r#"{"workflowId":"agent-preview"}"#,
        ),
    ] {
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .expect("guardrail request should be built");
        let response = app
            .clone()
            .oneshot(agent_auth_headers(request))
            .await
            .expect("guardrail request should return a response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{uri} must remain owned by sdkwork-appbase, not local-minimal-node"
        );
    }
}
