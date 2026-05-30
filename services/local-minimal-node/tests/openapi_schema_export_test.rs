use std::fs;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use serde_json::Value;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const IM_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_IM_OPENAPI_SCHEMA_PATH";
const APP_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_APP_API_OPENAPI_SCHEMA_PATH";
const BACKEND_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_BACKEND_API_OPENAPI_SCHEMA_PATH";

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

async fn openapi_schema_env_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

async fn request_json_schema(app: axum::Router, path: &str) -> Value {
    let response = app
        .oneshot(Request::builder().uri(path).body(Body::empty()).unwrap())
        .await
        .expect("openapi schema route should respond");

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .expect("content-type header should be present");
    assert!(
        content_type.starts_with("application/json"),
        "expected json content-type, got {content_type}"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("schema body should collect")
        .to_bytes();
    serde_json::from_slice(&body).expect("schema body should be json")
}

#[tokio::test]
async fn test_local_minimal_node_exports_im_openapi_schema() {
    let _guard = openapi_schema_env_guard().await;
    let app = local_minimal_node::build_public_app();

    let body_json = request_json_schema(app, "/im/v3/openapi.json").await;

    assert!(
        body_json["openapi"]
            .as_str()
            .is_some_and(|value| value.starts_with("3.")),
        "schema must be an OpenAPI 3.x document"
    );
    assert!(
        body_json["paths"]
            .as_object()
            .is_some_and(|paths| paths.contains_key("/im/v3/api/chat/conversations")),
        "schema must include IM open-platform /im/v3/api chat routes"
    );
    assert!(
        body_json["paths"].as_object().is_some_and(|paths| {
            paths.keys().all(|path| path.starts_with("/im/v3/api/"))
                && !paths.contains_key("/app/v3/api/chat/conversations")
                && !paths.contains_key("/backend/v3/api/ops/health")
        }),
        "IM OpenAPI schema must only include IM open-platform business paths"
    );
}

#[tokio::test]
async fn test_local_minimal_node_exports_app_api_openapi_schema() {
    let _guard = openapi_schema_env_guard().await;
    let app = local_minimal_node::build_public_app();

    let body_json = request_json_schema(app, "/app/v3/openapi.json").await;

    assert!(
        body_json["openapi"]
            .as_str()
            .is_some_and(|value| value.starts_with("3.")),
        "schema must be an OpenAPI 3.x document"
    );
    assert!(
        body_json["paths"].as_object().is_some_and(|paths| {
            paths.contains_key("/app/v3/api/portal/access")
                && paths.contains_key("/app/v3/api/automation/executions")
                && paths.contains_key("/app/v3/api/notifications/requests")
                && paths.contains_key("/app/v3/api/devices/{deviceId}/twin")
        }),
        "schema must include app-business /app/v3/api routes"
    );
    assert!(
        body_json["paths"].as_object().is_some_and(|paths| {
            paths.keys().all(|path| path.starts_with("/app/v3/api/"))
                && !paths.contains_key("/app/v3/api/chat/conversations")
                && !paths.contains_key("/app/v3/api/social/friend_requests")
                && !paths.contains_key("/app/v3/api/device/sessions/resume")
                && !paths.contains_key("/backend/v3/api/ops/health")
        }),
        "app OpenAPI schema must only include app-api business paths outside IM standard APIs"
    );
}

#[tokio::test]
async fn test_local_minimal_node_exports_backend_api_openapi_schema() {
    let _guard = openapi_schema_env_guard().await;
    let app = local_minimal_node::build_public_app();

    let body_json = request_json_schema(app, "/backend/v3/openapi.json").await;

    assert!(
        body_json["openapi"]
            .as_str()
            .is_some_and(|value| value.starts_with("3.")),
        "schema must be an OpenAPI 3.x document"
    );
    assert!(
        body_json["paths"]
            .as_object()
            .is_some_and(|paths| paths.contains_key("/backend/v3/api/ops/health")),
        "schema must include backend-api ops routes"
    );
    assert!(
        body_json["paths"].as_object().is_some_and(|paths| {
            paths
                .keys()
                .all(|path| path.starts_with("/backend/v3/api/"))
                && !paths.contains_key("/app/v3/api/chat/conversations")
        }),
        "backend OpenAPI schema must only include backend-api business paths"
    );
}

#[tokio::test]
async fn test_local_minimal_node_exports_im_openapi_schema_from_runtime_override_path() {
    let _guard = openapi_schema_env_guard().await;
    let schema_fixture_dir = std::env::temp_dir().join(format!(
        "craw-chat-im-openapi-export-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&schema_fixture_dir).expect("schema fixture directory should be created");

    let schema_fixture_path = schema_fixture_dir.join("im.openapi.yaml");
    let schema_fixture = r#"openapi: 3.0.3
info:
  title: Runtime IM Export Override
  version: 9.9.9
paths:
  /im/v3/api/runtime_override:
    get:
      operationId: runtimeOverride.retrieve
      responses:
        '200':
          description: ok
"#;
    fs::write(&schema_fixture_path, schema_fixture).expect("schema fixture should be written");

    let _schema_override = ScopedEnvVar::set(
        IM_OPENAPI_SCHEMA_PATH_ENV,
        &schema_fixture_path.to_string_lossy(),
    );
    let app = local_minimal_node::build_public_app();

    let body_json = request_json_schema(app, "/im/v3/openapi.json").await;

    assert!(
        body_json["paths"]
            .as_object()
            .is_some_and(|paths| paths.contains_key("/im/v3/api/runtime_override")),
        "runtime override schema should be exported as json"
    );
}

#[tokio::test]
async fn test_local_minimal_node_exports_app_openapi_schema_from_runtime_override_path() {
    let _guard = openapi_schema_env_guard().await;
    let schema_fixture_dir = std::env::temp_dir().join(format!(
        "craw-chat-openapi-export-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&schema_fixture_dir).expect("schema fixture directory should be created");

    let schema_fixture_path = schema_fixture_dir.join("app.openapi.yaml");
    let schema_fixture = r#"openapi: 3.0.3
info:
  title: Runtime Export Override
  version: 9.9.9
paths:
  /app/v3/api/runtime_override:
    get:
      operationId: runtimeOverride.retrieve
      responses:
        '200':
          description: ok
"#;
    fs::write(&schema_fixture_path, schema_fixture).expect("schema fixture should be written");

    let _schema_override = ScopedEnvVar::set(
        APP_OPENAPI_SCHEMA_PATH_ENV,
        &schema_fixture_path.to_string_lossy(),
    );
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/openapi.json")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi schema route should respond");

    assert_eq!(response.status(), StatusCode::OK);

    let body = response
        .into_body()
        .collect()
        .await
        .expect("schema body should collect")
        .to_bytes();
    let body_json: Value = serde_json::from_slice(&body).expect("schema body should be json");

    assert!(
        body_json["paths"]
            .as_object()
            .is_some_and(|paths| paths.contains_key("/app/v3/api/runtime_override")),
        "runtime override schema should be exported as json"
    );
}

#[tokio::test]
async fn test_local_minimal_node_exports_backend_api_openapi_schema_from_runtime_override_path() {
    let _guard = openapi_schema_env_guard().await;
    let schema_fixture_dir = std::env::temp_dir().join(format!(
        "craw-chat-backend-openapi-export-test-{}",
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock should be after unix epoch")
            .as_nanos()
    ));
    fs::create_dir_all(&schema_fixture_dir).expect("schema fixture directory should be created");

    let schema_fixture_path = schema_fixture_dir.join("backend.openapi.yaml");
    let schema_fixture = r#"openapi: 3.0.3
info:
  title: Runtime Backend Export Override
  version: 9.9.9
paths:
  /backend/v3/api/runtime_override:
    get:
      operationId: runtimeOverride.retrieve
      responses:
        '200':
          description: ok
"#;
    fs::write(&schema_fixture_path, schema_fixture).expect("schema fixture should be written");

    let _schema_override = ScopedEnvVar::set(
        BACKEND_OPENAPI_SCHEMA_PATH_ENV,
        &schema_fixture_path.to_string_lossy(),
    );
    let app = local_minimal_node::build_public_app();

    let body_json = request_json_schema(app, "/backend/v3/openapi.json").await;

    assert!(
        body_json["paths"]
            .as_object()
            .is_some_and(|paths| paths.contains_key("/backend/v3/api/runtime_override")),
        "runtime override schema should be exported as json"
    );
}

#[tokio::test]
async fn test_local_minimal_node_does_not_expose_legacy_or_single_surface_openapi_schema_paths() {
    let _guard = openapi_schema_env_guard().await;
    let app = local_minimal_node::build_default_app();

    for legacy_path in [
        "/openapi/craw-chat-app.openapi.yaml",
        "/im/v3/api/openapi.json",
        "/app/v3/api/openapi.json",
        "/backend/v3/api/openapi.json",
    ] {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri(legacy_path)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .expect("legacy openapi schema route should return a response");

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }
}
