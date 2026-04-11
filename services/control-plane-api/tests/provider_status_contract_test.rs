use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{RuntimeProviderRegistry, StaticProviderRegistry};
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

async fn request_status(
    app: Router,
    method: &str,
    uri: &str,
    tenant_id: Option<&str>,
    user_id: Option<&str>,
    permission: Option<&str>,
    body: Option<&str>,
) -> (StatusCode, String) {
    let mut request = Request::builder().method(method).uri(uri);
    if let Some(tenant_id) = tenant_id {
        request = request.header("x-tenant-id", tenant_id);
    }
    if let Some(user_id) = user_id {
        request = request.header("x-user-id", user_id);
    }
    if let Some(permission) = permission {
        request = request.header("x-permissions", permission);
    }
    if body.is_some() {
        request = request.header("content-type", "application/json");
    }

    let response = app
        .oneshot(
            request
                .body(
                    body.map(|value| Body::from(value.to_owned()))
                        .unwrap_or_else(Body::empty),
                )
                .unwrap(),
        )
        .await
        .expect("provider control-plane request should return a response");
    let status_code = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider control-plane body should collect")
        .to_bytes();
    let json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider control-plane body should be valid json");
    let status = json["status"]
        .as_str()
        .expect("provider control-plane response should expose top-level status")
        .to_owned();

    (status_code, status)
}

async fn assert_status(
    app: Router,
    method: &str,
    uri: &str,
    tenant_id: Option<&str>,
    user_id: Option<&str>,
    permission: Option<&str>,
    body: Option<&str>,
    expected_http: StatusCode,
    expected_status: &str,
) -> String {
    let (status_code, status) =
        request_status(app, method, uri, tenant_id, user_id, permission, body).await;
    assert_eq!(
        status_code, expected_http,
        "{method} {uri} should return the expected HTTP status"
    );
    assert_eq!(
        status, expected_status,
        "{method} {uri} should return the expected top-level provider status"
    );
    status
}

#[tokio::test]
async fn test_provider_control_plane_status_contract_covers_read_write_and_error_routes() {
    let runtime_app = control_plane_api::build_app_with_cluster_and_runtime_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(RuntimeProviderRegistry::platform_default()),
    );
    let static_app = control_plane_api::build_app_with_cluster_and_provider_registry(
        Arc::new(RealtimeClusterBridge::default()),
        Arc::new(StaticProviderRegistry::platform_default()),
    );

    let mut observed = BTreeSet::new();

    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-registry",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.read"),
            None,
            StatusCode::OK,
            "registry",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-bindings",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.read"),
            None,
            StatusCode::OK,
            "bindings",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "POST",
            "/api/v1/control/provider-policies/preview",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
            StatusCode::OK,
            "preview",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "POST",
            "/api/v1/control/provider-bindings",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
            StatusCode::OK,
            "applied",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "POST",
            "/api/v1/control/provider-bindings",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine","expectedBaseVersion":2}"#),
            StatusCode::OK,
            "noop",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-policies",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.read"),
            None,
            StatusCode::OK,
            "history",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-policies/diff?fromVersion=1&toVersion=2",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.read"),
            None,
            StatusCode::OK,
            "diff",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "POST",
            "/api/v1/control/provider-policies/rollback",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"targetVersion":1}"#),
            StatusCode::OK,
            "rolled_back",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "POST",
            "/api/v1/control/provider-bindings",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"domain":"rtc","pluginId":"object-storage-aws"}"#),
            StatusCode::BAD_REQUEST,
            "invalid",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-policies/diff?fromVersion=1&toVersion=9",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.read"),
            None,
            StatusCode::CONFLICT,
            "conflict",
        )
        .await,
    );
    observed.insert(
        assert_status(
            static_app.clone(),
            "POST",
            "/api/v1/control/provider-policies/preview",
            Some("t_demo"),
            Some("u_admin"),
            Some("control.write"),
            Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
            StatusCode::SERVICE_UNAVAILABLE,
            "unavailable",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            "GET",
            "/api/v1/control/provider-registry",
            Some("t_demo"),
            Some("u_admin"),
            None,
            None,
            StatusCode::FORBIDDEN,
            "forbidden",
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app,
            "GET",
            "/api/v1/control/provider-registry",
            None,
            None,
            None,
            None,
            StatusCode::UNAUTHORIZED,
            "unauthorized",
        )
        .await,
    );

    assert_eq!(
        observed,
        BTreeSet::from([
            "applied".to_owned(),
            "bindings".to_owned(),
            "conflict".to_owned(),
            "diff".to_owned(),
            "forbidden".to_owned(),
            "history".to_owned(),
            "invalid".to_owned(),
            "noop".to_owned(),
            "preview".to_owned(),
            "registry".to_owned(),
            "rolled_back".to_owned(),
            "unavailable".to_owned(),
            "unauthorized".to_owned(),
        ]),
        "provider control-plane routes should expose the consolidated top-level status vocabulary"
    );
}
