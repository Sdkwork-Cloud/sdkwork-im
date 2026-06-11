use im_app_context::DualTokenRequestBuilderExt;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;

#[tokio::test]
async fn test_local_minimal_ops_diagnostics_exposes_runtime_provider_bindings() {
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/diagnostics")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops diagnostics should return response");
    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("ops diagnostics body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("ops diagnostics should be valid json");

    let provider_bindings = json["providerBindings"]
        .as_array()
        .expect("providerBindings should be an array");
    assert_eq!(
        provider_bindings.len(),
        1,
        "local-minimal runtime should expose one global provider binding snapshot"
    );

    let global_snapshot = &provider_bindings[0];
    assert_eq!(
        global_snapshot["interfaceVersion"], "provider-registry/v1",
        "ops diagnostics should reuse provider-registry contract version"
    );
    assert_eq!(
        global_snapshot["tenantId"],
        serde_json::Value::Null,
        "local-minimal runtime should publish a global provider snapshot"
    );
    assert_eq!(
        global_snapshot["precedence"],
        serde_json::json!(["tenant_override", "deployment_profile", "global_default"])
    );

    let effective_bindings = global_snapshot["effectiveBindings"]
        .as_array()
        .expect("effectiveBindings should be an array");
    assert!(
        effective_bindings.iter().any(|binding| {
            binding["domain"] == "rtc"
                && binding["selectedPluginId"] == "rtc-volcengine"
                && binding["selectionSource"] == "global_default"
        }),
        "rtc binding should expose the selected volcengine provider"
    );
    assert!(
        effective_bindings.iter().any(|binding| {
            binding["domain"] == "object-storage"
                && binding["selectedPluginId"] == "object-storage-volcengine"
                && binding["selectionSource"] == "deployment_profile"
        }),
        "object-storage binding should expose the selected volcengine provider"
    );
    assert!(
        effective_bindings.iter().any(|binding| {
            binding["domain"] == "principal-profile"
                && binding["selectedPluginId"] == "principal-profile-upstream-context"
                && binding["selectionSource"] == "global_default"
        }),
        "principal-profile binding should expose the local provider"
    );
    assert_eq!(
        json["providerBindingDrift"]["items"],
        serde_json::json!([]),
        "local-minimal global snapshot should not report tenant drift by default"
    );
}

#[tokio::test]
async fn test_local_minimal_exposes_standalone_ops_provider_binding_routes() {
    let app = local_minimal_node::build_default_app();

    let provider_bindings_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/provider_bindings")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider_bindings should return response");
    assert_eq!(
        provider_bindings_response.status(),
        StatusCode::OK,
        "local-minimal should expose standalone provider_bindings route"
    );

    let provider_bindings_body = provider_bindings_response
        .into_body()
        .collect()
        .await
        .expect("provider_bindings body should collect")
        .to_bytes();
    let provider_bindings_json: serde_json::Value = serde_json::from_slice(&provider_bindings_body)
        .expect("provider_bindings body should be valid json");
    let items = provider_bindings_json["items"]
        .as_array()
        .expect("provider_bindings items should be an array");
    assert_eq!(
        items.len(),
        1,
        "standalone provider_bindings route should publish the same global snapshot as diagnostics"
    );

    let provider_binding_drift_response = app
        .oneshot(
            Request::builder()
                .uri("/backend/v3/api/ops/provider_bindings/drift")
                .with_dual_token_tenant("t_demo")
                .with_dual_token_user("u_ops_demo")
                .with_dual_token_actor_kind("user")
                .with_dual_token_permission_scope("ops.read")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("ops provider_bindings drift should return response");
    assert_eq!(
        provider_binding_drift_response.status(),
        StatusCode::OK,
        "local-minimal should expose standalone provider-binding drift route"
    );

    let provider_binding_drift_body = provider_binding_drift_response
        .into_body()
        .collect()
        .await
        .expect("provider_bindings drift body should collect")
        .to_bytes();
    let provider_binding_drift_json: serde_json::Value =
        serde_json::from_slice(&provider_binding_drift_body)
            .expect("provider_bindings drift body should be valid json");
    assert_eq!(
        provider_binding_drift_json["items"],
        serde_json::json!([]),
        "standalone provider-binding drift route should mirror diagnostic drift view"
    );
}
