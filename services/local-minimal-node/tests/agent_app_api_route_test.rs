use axum::body::Body;
use axum::http::header::CONTENT_TYPE;
use axum::http::{Method, Request, StatusCode};
use im_app_context::DualTokenRequestBuilderExt;
use local_minimal_node::build_default_app;
use tower::ServiceExt;

#[tokio::test]
async fn local_minimal_node_does_not_mount_foundation_gateway_routes() {
    let app = build_default_app();

    for (method, uri, body) in [
        (Method::GET, "/app/v3/api/ai/agents?tenant_id=1", ""),
        (
            Method::POST,
            "/app/v3/api/ai/agents/agent.local.runtime/preview_responses?tenant_id=1",
            r#"{"content":"hello"}"#,
        ),
        (Method::GET, "/app/v3/api/iot/devices", ""),
        (Method::GET, "/backend/v3/api/iot/devices", ""),
    ] {
        let request = Request::builder()
            .method(method)
            .uri(uri)
            .header(CONTENT_TYPE, "application/json")
            .with_dual_token_tenant("1")
            .with_dual_token_user("u-agent-local")
            .with_dual_token_actor_kind("user")
            .body(Body::from(body))
            .expect("foundation route request should be built");
        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("foundation route request should return a response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{uri} must be served by sdkwork-api-gateway, not local-minimal-node"
        );
    }
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
            .with_dual_token_tenant("1")
            .with_dual_token_user("u-agent-local")
            .with_dual_token_actor_kind("user")
            .body(Body::from(body))
            .expect("guardrail request should be built");
        let response = app
            .clone()
            .oneshot(request)
            .await
            .expect("guardrail request should return a response");

        assert_eq!(
            response.status(),
            StatusCode::NOT_FOUND,
            "{uri} must remain owned by sdkwork-appbase, not local-minimal-node"
        );
    }
}
