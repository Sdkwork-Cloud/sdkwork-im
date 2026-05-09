use std::collections::BTreeSet;
use std::sync::Arc;

use axum::Router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use im_platform_contracts::{RuntimeProviderRegistry, StaticProviderRegistry};
use session_gateway::RealtimeClusterBridge;
use tower::ServiceExt;

struct StatusExpectation<'a> {
    method: &'a str,
    uri: &'a str,
    tenant_id: Option<&'a str>,
    user_id: Option<&'a str>,
    permission: Option<&'a str>,
    body: Option<&'a str>,
    expected_http: StatusCode,
    expected_status: &'a str,
}

async fn request_status(app: Router, expectation: &StatusExpectation<'_>) -> (StatusCode, String) {
    let mut request = Request::builder()
        .method(expectation.method)
        .uri(expectation.uri);
    if let Some(tenant_id) = expectation.tenant_id {
        request = request.header("x-tenant-id", tenant_id);
    }
    if let Some(user_id) = expectation.user_id {
        request = request.header("x-user-id", user_id);
        request = request.header("x-actor-kind", "user");
    }
    if let Some(permission) = expectation.permission {
        request = request.header("x-permissions", permission);
    }
    if expectation.body.is_some() {
        request = request.header("content-type", "application/json");
    }

    let response = app
        .oneshot(
            request
                .body(
                    expectation
                        .body
                        .map(|value| Body::from(value.to_owned()))
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

async fn assert_status(app: Router, expectation: StatusExpectation<'_>) -> String {
    let (status_code, status) = request_status(app, &expectation).await;
    assert_eq!(
        status_code, expectation.expected_http,
        "{} {} should return the expected HTTP status",
        expectation.method, expectation.uri
    );
    assert_eq!(
        status, expectation.expected_status,
        "{} {} should return the expected top-level provider status",
        expectation.method, expectation.uri
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
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-registry",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_status: "registry",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-bindings",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_status: "bindings",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-policies/preview",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::OK,
                expected_status: "preview",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-bindings",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::OK,
                expected_status: "applied",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-bindings",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(
                    r#"{"domain":"object-storage","pluginId":"object-storage-volcengine","expectedBaseVersion":2}"#,
                ),
                expected_http: StatusCode::OK,
                expected_status: "noop",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-policies",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_status: "history",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-policies/diff?fromVersion=1&toVersion=2",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::OK,
                expected_status: "diff",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-policies/rollback",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(r#"{"targetVersion":1}"#),
                expected_http: StatusCode::OK,
                expected_status: "rolled_back",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-bindings",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"rtc","pluginId":"object-storage-aws"}"#),
                expected_http: StatusCode::BAD_REQUEST,
                expected_status: "invalid",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-policies/diff?fromVersion=1&toVersion=9",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.read"),
                body: None,
                expected_http: StatusCode::CONFLICT,
                expected_status: "conflict",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            static_app.clone(),
            StatusExpectation {
                method: "POST",
                uri: "/api/v1/control/provider-policies/preview",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: Some("control.write"),
                body: Some(r#"{"domain":"object-storage","pluginId":"object-storage-volcengine"}"#),
                expected_http: StatusCode::SERVICE_UNAVAILABLE,
                expected_status: "unavailable",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app.clone(),
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-registry",
                tenant_id: Some("t_demo"),
                user_id: Some("u_admin"),
                permission: None,
                body: None,
                expected_http: StatusCode::FORBIDDEN,
                expected_status: "forbidden",
            },
        )
        .await,
    );
    observed.insert(
        assert_status(
            runtime_app,
            StatusExpectation {
                method: "GET",
                uri: "/api/v1/control/provider-registry",
                tenant_id: None,
                user_id: None,
                permission: None,
                body: None,
                expected_http: StatusCode::UNAUTHORIZED,
                expected_status: "unauthorized",
            },
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
