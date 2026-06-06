use std::fs;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::{Path, PathBuf};
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
        "craw_chat_principal_profile_runtime_selection_{unique}_{counter}"
    ))
}

fn state_file(runtime_dir: &Path, file_name: &str) -> PathBuf {
    runtime_dir.join("state").join(file_name)
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
async fn test_default_app_uses_configured_external_principal_profile_provider() {
    let _guard = principal_profile_env_guard().await;
    let runtime_dir = unique_runtime_dir();
    fs::create_dir_all(&runtime_dir).expect("runtime dir should be created");

    let external_catalog_path = runtime_dir.join("external-principal-profile.json");
    fs::write(
        &external_catalog_path,
        r#"{
            "externalSystem":"corp-idp",
            "profiles":[
                {
                    "tenantId":"t_demo",
                    "principalId":"u_runtime_external_owner",
                    "displayName":"Runtime External Owner",
                    "externalPrincipalId":"ext::u_runtime_external_owner",
                    "attributes":{"source":"external","department":"federated"}
                },
                {
                    "tenantId":"t_demo",
                    "principalId":"u_runtime_external_member",
                    "displayName":"Runtime External Member",
                    "externalPrincipalId":"ext::u_runtime_external_member",
                    "attributes":{"source":"external","department":"federated"}
                }
            ]
        }"#,
    )
    .expect("external catalog should be written");

    let provider_key = "CRAW_CHAT_PRINCIPAL_PROFILE_PROVIDER";
    let catalog_key = "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH";
    let system_key = "CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_SYSTEM";
    let previous_provider = std::env::var(provider_key).ok();
    let previous_catalog = std::env::var(catalog_key).ok();
    let previous_system = std::env::var(system_key).ok();

    unsafe {
        std::env::set_var(provider_key, "external-catalog");
        std::env::set_var(catalog_key, &external_catalog_path);
        std::env::set_var(system_key, "corp-idp");
    }
    let app = local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path());
    restore_env(provider_key, previous_provider);
    restore_env(catalog_key, previous_catalog);
    restore_env(system_key, previous_system);

    let create_conversation = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_runtime_external_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_runtime_external_owner")
                .header("x-sdkwork-session-id", "s_runtime_external_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_runtime_external_default",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should succeed");
    assert_eq!(create_conversation.status(), StatusCode::OK);

    let add_member = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_runtime_external_default/members/add")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_runtime_external_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_runtime_external_owner")
                .header("x-sdkwork-session-id", "s_runtime_external_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "principalId":"u_runtime_external_member",
                        "principalKind":"user",
                        "role":"member"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("add member should succeed");
    assert_eq!(add_member.status(), StatusCode::OK);
    let add_member_body = add_member
        .into_body()
        .collect()
        .await
        .expect("add member body should collect")
        .to_bytes();
    let add_member_json: serde_json::Value =
        serde_json::from_slice(&add_member_body).expect("member response should be valid json");
    assert_eq!(
        add_member_json["attributes"]["displayName"],
        "Runtime External Member"
    );
    assert_eq!(add_member_json["attributes"]["externalSystem"], "corp-idp");
    assert_eq!(
        add_member_json["attributes"]["externalPrincipalId"],
        "ext::u_runtime_external_member"
    );
    assert_eq!(
        add_member_json["attributes"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let post_message = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations/c_runtime_external_default/messages")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_runtime_external_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_runtime_external_owner")
                .header("x-sdkwork-session-id", "s_runtime_external_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "clientMsgId":"client_runtime_external_default",
                        "summary":"runtime external",
                        "text":"runtime external"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("post message should succeed");
    assert_eq!(post_message.status(), StatusCode::OK);

    let journal_content =
        fs::read_to_string(state_file(runtime_dir.as_path(), "commit-journal.json"))
            .expect("commit journal should be readable");
    let journal_events = journal_content
        .lines()
        .map(|line| {
            serde_json::from_str::<serde_json::Value>(line)
                .expect("commit journal line should be valid json")
        })
        .collect::<Vec<_>>();
    let message_posted = journal_events
        .iter()
        .find(|item| item["event_type"] == "message.posted")
        .expect("message.posted event should exist");
    let payload: serde_json::Value = serde_json::from_str(
        message_posted["payload"]
            .as_str()
            .expect("payload should be serialized json"),
    )
    .expect("message payload should be valid json");
    assert_eq!(
        payload["sender"]["metadata"]["displayName"],
        "Runtime External Owner"
    );
    assert_eq!(payload["sender"]["metadata"]["externalSystem"], "corp-idp");
    assert_eq!(
        payload["sender"]["metadata"]["externalPrincipalId"],
        "ext::u_runtime_external_owner"
    );
    assert_eq!(
        payload["sender"]["metadata"]["principalProfilePluginId"],
        "principal-profile-external-catalog"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_app_boots_with_external_principal_profile_provider_missing_catalog_path_and_returns_provider_unavailable()
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
    let build_result = catch_unwind(AssertUnwindSafe(|| {
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path())
    }));
    restore_env(provider_key, previous_provider);
    restore_env(catalog_key, previous_catalog);
    restore_env(system_key, previous_system);

    let app = build_result.expect(
        "missing external principal-profile catalog path should boot with an unavailable provider instead of panicking",
    );

    let create_conversation = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/im/v3/api/chat/conversations")
                .header("x-sdkwork-tenant-id", "t_demo")
                .header("x-sdkwork-user-id", "u_runtime_external_owner")
                .header("x-sdkwork-actor-kind", "user")
                .header("x-sdkwork-device-id", "d_runtime_external_owner")
                .header("x-sdkwork-session-id", "s_runtime_external_owner")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{
                        "conversationId":"c_runtime_external_missing_catalog",
                        "conversationType":"group"
                    }"#,
                ))
                .unwrap(),
        )
        .await
        .expect("create conversation should return a structured error response");
    assert_eq!(
        create_conversation.status(),
        StatusCode::SERVICE_UNAVAILABLE
    );

    let response_body = create_conversation
        .into_body()
        .collect()
        .await
        .expect("error body should collect")
        .to_bytes();
    let response_json: serde_json::Value =
        serde_json::from_slice(&response_body).expect("error response should be valid json");
    assert_eq!(response_json["code"], "provider_unavailable");
    assert!(
        response_json["message"]
            .as_str()
            .is_some_and(|message| message.contains(catalog_key)),
        "provider_unavailable message should mention the missing catalog path env. actual body: {response_json}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}

#[tokio::test]
async fn test_default_app_boots_with_invalid_principal_profile_provider_mode_and_surfaces_config_error()
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
        std::env::set_var(provider_key, "rogue-provider");
        std::env::remove_var(catalog_key);
        std::env::remove_var(system_key);
    }
    let build_result = catch_unwind(AssertUnwindSafe(|| {
        local_minimal_node::build_default_app_with_runtime_dir(runtime_dir.as_path())
    }));
    restore_env(provider_key, previous_provider);
    restore_env(catalog_key, previous_catalog);
    restore_env(system_key, previous_system);

    let app = build_result.expect(
        "invalid principal-profile provider mode should boot with an unavailable provider instead of panicking",
    );

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
        .expect("provider health request should return response");

    assert_eq!(response.status(), StatusCode::OK);
    let body = response
        .into_body()
        .collect()
        .await
        .expect("provider health body should collect")
        .to_bytes();
    let response_json: serde_json::Value =
        serde_json::from_slice(&body).expect("provider health response should be valid json");

    assert_eq!(response_json["status"], "unavailable");
    assert_eq!(response_json["details"]["configKey"], provider_key);
    assert_eq!(
        response_json["details"]["configuredValue"],
        "rogue-provider"
    );
    assert!(
        response_json["details"]["error"]
            .as_str()
            .is_some_and(|message| message.contains("upstream-context, external-catalog")),
        "invalid provider mode should surface the allowed values. actual body: {response_json}"
    );

    let _ = fs::remove_dir_all(runtime_dir);
}
