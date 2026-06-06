use std::fs;
use std::path::PathBuf;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt;
use tokio::sync::{Mutex, MutexGuard};
use tower::ServiceExt;

static UNIQUE_RUNTIME_DIR_COUNTER: AtomicU64 = AtomicU64::new(0);

async fn principal_profile_env_guard() -> MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(())).lock().await
}

fn unique_runtime_dir() -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    let counter = UNIQUE_RUNTIME_DIR_COUNTER.fetch_add(1, Ordering::Relaxed);
    std::env::temp_dir().join(format!(
        "craw_chat_principal_profile_provider_http_{unique}_{counter}"
    ))
}

fn restore_env(key: &str, previous: Option<String>) {
    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[tokio::test]
async fn test_local_minimal_profile_gets_principal_profile_provider_health_over_http() {
    let _guard = principal_profile_env_guard().await;
    let app = local_minimal_node::build_default_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/principal/profiles/provider_health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("principal-profile provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("principal-profile provider health body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("principal-profile provider health response should be valid json");

    assert_eq!(json["pluginId"], "principal-profile-upstream-context");
    assert_eq!(json["status"], "healthy");
    assert_eq!(json["details"]["providerKind"], "upstream-context");
}

#[tokio::test]
async fn test_local_minimal_profile_gets_unavailable_external_principal_profile_provider_health_over_http_when_catalog_path_missing()
 {
    let _guard = principal_profile_env_guard().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let provider_key = "CRAW_CHAT_PRINCIPAL_PROFILE_PROVIDER";
    let catalog_key = "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH";
    let system_key = "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM";
    let previous_provider = std::env::var(provider_key).ok();
    let previous_catalog = std::env::var(catalog_key).ok();
    let previous_system = std::env::var(system_key).ok();

    unsafe {
        std::env::set_var(provider_key, "external-catalog");
        std::env::remove_var(catalog_key);
        std::env::set_var(system_key, "corp-idp");
    }
    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    restore_env(provider_key, previous_provider);
    restore_env(catalog_key, previous_catalog);
    restore_env(system_key, previous_system);

    let response = app
        .oneshot(
            Request::builder()
                .uri("/app/v3/api/principal/profiles/provider_health")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_demo")
                .header("x-sdkwork-actor-kind", "user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .expect("principal-profile provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("principal-profile provider health body should collect")
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body)
        .expect("principal-profile provider health response should be valid json");

    assert_eq!(json["pluginId"], "principal-profile-external-catalog");
    assert_eq!(json["status"], "unavailable");
    assert_eq!(json["details"]["providerMode"], "external-catalog");
    assert_eq!(json["details"]["configKey"], catalog_key);
    assert!(
        json["details"]["error"]
            .as_str()
            .is_some_and(|message| message.contains(catalog_key)),
        "unavailable principal-profile provider health should surface the missing catalog env. actual body: {json}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
