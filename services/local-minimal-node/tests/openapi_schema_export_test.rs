use std::fs;
use std::sync::OnceLock;
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

const APP_OPENAPI_SCHEMA_PATH_ENV: &str = "CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH";

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

#[tokio::test]
async fn test_local_minimal_node_exports_app_openapi_schema() {
    let _guard = openapi_schema_env_guard().await;
    let app = local_minimal_node::build_public_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/openapi/craw-chat-app.openapi.yaml")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("openapi schema route should respond");

    assert_eq!(response.status(), StatusCode::OK);

    let content_type = response
        .headers()
        .get(axum::http::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .expect("content-type header should be present");
    assert!(
        content_type.starts_with("application/yaml"),
        "expected yaml content-type, got {content_type}"
    );

    let body = response
        .into_body()
        .collect()
        .await
        .expect("schema body should collect")
        .to_bytes();
    let body_text = String::from_utf8(body.to_vec()).expect("schema body should be utf-8");

    assert!(
        body_text.starts_with("openapi: 3."),
        "schema must be an OpenAPI 3.x document"
    );
    assert!(
        body_text.contains("/api/v1/conversations:"),
        "schema must include app-facing conversation routes"
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
  /api/v1/runtime-override:
    get:
      operationId: getRuntimeOverride
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
                .uri("/openapi/craw-chat-app.openapi.yaml")
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
    let body_text = String::from_utf8(body.to_vec()).expect("schema body should be utf-8");

    assert_eq!(body_text, schema_fixture);
    assert!(
        body_text.contains("/api/v1/runtime-override:"),
        "runtime override schema should be exported"
    );
}
