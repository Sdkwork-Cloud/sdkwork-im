use std::sync::{Mutex, OnceLock};

fn runtime_config_env_guard() -> std::sync::MutexGuard<'static, ()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD
        .get_or_init(|| Mutex::new(()))
        .lock()
        .expect("runtime config env guard should not be poisoned")
}

#[test]
fn test_resolve_bind_addr_defaults_and_supports_container_override() {
    let _guard = runtime_config_env_guard();
    let key = "CRAW_CHAT_BIND_ADDR";
    let previous = std::env::var(key).ok();
    unsafe {
        std::env::remove_var(key);
    }

    let default_addr = local_minimal_node::resolve_bind_addr();
    assert_eq!(default_addr, "127.0.0.1:18090");

    unsafe {
        std::env::set_var(key, "0.0.0.0:18090");
    }
    let container_addr = local_minimal_node::resolve_bind_addr();
    assert_eq!(container_addr, "0.0.0.0:18090");

    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[test]
fn test_resolve_runtime_dir_defaults_and_supports_override() {
    let _guard = runtime_config_env_guard();
    let key = "CRAW_CHAT_RUNTIME_DIR";
    let previous = std::env::var(key).ok();
    unsafe {
        std::env::remove_var(key);
    }

    let default_runtime_dir = local_minimal_node::resolve_runtime_dir();
    assert_eq!(
        default_runtime_dir,
        std::path::PathBuf::from(".runtime").join("local-minimal")
    );

    unsafe {
        std::env::set_var(key, "custom-runtime-dir");
    }
    let overridden_runtime_dir = local_minimal_node::resolve_runtime_dir();
    assert_eq!(
        overridden_runtime_dir,
        std::path::PathBuf::from("custom-runtime-dir")
    );

    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[test]
fn test_resolve_app_openapi_schema_source_path_defaults_and_supports_override() {
    let _guard = runtime_config_env_guard();
    let key = "CRAW_CHAT_APP_OPENAPI_SCHEMA_PATH";
    let previous = std::env::var(key).ok();
    unsafe {
        std::env::remove_var(key);
    }

    let default_schema_path = local_minimal_node::resolve_app_openapi_schema_source_path();
    assert!(
        default_schema_path
            .to_string_lossy()
            .ends_with("sdks/sdkwork-im-sdk/openapi/craw-chat-app.openapi.yaml"),
        "default schema path should point at the workspace authority contract: {}",
        default_schema_path.display()
    );

    unsafe {
        std::env::set_var(key, "custom-openapi/runtime-schema.yaml");
    }
    let overridden_schema_path = local_minimal_node::resolve_app_openapi_schema_source_path();
    assert_eq!(
        overridden_schema_path,
        std::path::PathBuf::from("custom-openapi/runtime-schema.yaml")
    );

    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[test]
fn test_resolve_public_browser_origins_defaults_and_supports_override() {
    let _guard = runtime_config_env_guard();
    let key = "CRAW_CHAT_BROWSER_ORIGINS";
    let previous = std::env::var(key).ok();
    unsafe {
        std::env::remove_var(key);
    }

    let default_origins = local_minimal_node::resolve_public_browser_origins();
    assert_eq!(
        default_origins,
        vec![
            "http://127.0.0.1:4176".to_string(),
            "http://localhost:4176".to_string(),
        ]
    );

    unsafe {
        std::env::set_var(
            key,
            " https://portal.example.com , tauri://LOCALHOST/ , https://portal.example.com/ ",
        );
    }
    let overridden_origins = local_minimal_node::resolve_public_browser_origins();
    assert_eq!(
        overridden_origins,
        vec![
            "https://portal.example.com".to_string(),
            "tauri://localhost".to_string(),
        ]
    );

    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[test]
fn test_resolve_public_browser_origins_rejects_invalid_explicit_entries() {
    let _guard = runtime_config_env_guard();
    let key = "CRAW_CHAT_BROWSER_ORIGINS";
    let previous = std::env::var(key).ok();
    unsafe {
        std::env::set_var(key, "https://portal.example.com/app");
    }

    let result = std::panic::catch_unwind(local_minimal_node::resolve_public_browser_origins);
    assert!(result.is_err());

    match previous {
        Some(value) => unsafe {
            std::env::set_var(key, value);
        },
        None => unsafe {
            std::env::remove_var(key);
        },
    }
}

#[test]
fn test_resolve_user_center_runtime_config_defaults_to_builtin_local_standard_contract() {
    let _guard = runtime_config_env_guard();
    let keys = [
        "SDKWORK_USER_CENTER_MODE",
        "SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH",
        "SDKWORK_USER_CENTER_PROVIDER_KEY",
        "SDKWORK_USER_CENTER_AUTHORIZATION_HEADER_NAME",
        "SDKWORK_USER_CENTER_ACCESS_TOKEN_HEADER_NAME",
        "SDKWORK_USER_CENTER_REFRESH_TOKEN_HEADER_NAME",
        "SDKWORK_USER_CENTER_SESSION_HEADER_NAME",
        "SDKWORK_USER_CENTER_AUTHORIZATION_SCHEME",
        "SDKWORK_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN",
        "SDKWORK_USER_CENTER_APP_API_BASE_URL",
        "SDKWORK_USER_CENTER_EXTERNAL_BASE_URL",
        "SDKWORK_USER_CENTER_APP_ID",
        "SDKWORK_USER_CENTER_SECRET_ID",
        "SDKWORK_USER_CENTER_SHARED_SECRET",
        "SDKWORK_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS",
    ];
    let previous = keys
        .iter()
        .map(|key| (*key, std::env::var(key).ok()))
        .collect::<Vec<_>>();
    for key in keys {
        unsafe {
            std::env::remove_var(key);
        }
    }

    let config = local_minimal_node::resolve_user_center_runtime_config()
        .expect("default user-center runtime config should resolve");

    assert_eq!(config.mode.as_str(), "builtin-local");
    assert_eq!(config.provider_kind.as_str(), "local");
    assert_eq!(config.local_api_base_path, "/api/app/v1/user-center");
    assert_eq!(config.provider_key, "craw-chat-local");
    assert_eq!(config.authorization_header_name, "Authorization");
    assert_eq!(config.access_token_header_name, "Access-Token");
    assert_eq!(config.refresh_token_header_name, "Refresh-Token");
    assert_eq!(config.authorization_scheme, "Bearer");
    assert!(config.allow_authorization_fallback_to_access_token);
    assert!(config.app_api_base_url.is_none());
    assert!(config.external_base_url.is_none());
    assert!(config.secret_id.is_none());
    assert!(config.shared_secret.is_none());

    for (key, value) in previous {
        match value {
            Some(value) => unsafe {
                std::env::set_var(key, value);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }
    }
}

#[test]
fn test_resolve_user_center_runtime_config_supports_complete_app_api_mode() {
    let _guard = runtime_config_env_guard();
    let values = [
        (
            "SDKWORK_USER_CENTER_MODE",
            Some("sdkwork-cloud-app-api".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_LOCAL_API_BASE_PATH",
            Some("/api/custom/user-center".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_PROVIDER_KEY",
            Some("craw-app-api".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_APP_API_BASE_URL",
            Some("https://app-api.sdkwork.local/craw".to_string()),
        ),
        ("SDKWORK_USER_CENTER_APP_ID", Some("craw-chat".to_string())),
        ("SDKWORK_USER_CENTER_SECRET_ID", Some("secret-501".to_string())),
        (
            "SDKWORK_USER_CENTER_SHARED_SECRET",
            Some("shared-secret-501".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_HANDSHAKE_FRESHNESS_WINDOW_MS",
            Some("45000".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_ALLOW_AUTHORIZATION_FALLBACK_TO_ACCESS_TOKEN",
            Some("false".to_string()),
        ),
    ];
    let keys = values.iter().map(|(key, _)| *key).collect::<Vec<_>>();
    let previous = keys
        .iter()
        .map(|key| (*key, std::env::var(key).ok()))
        .collect::<Vec<_>>();

    for (key, value) in &values {
        match value {
            Some(value) => unsafe {
                std::env::set_var(key, value);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }
    }

    let config = local_minimal_node::resolve_user_center_runtime_config()
        .expect("complete app-api user-center runtime config should resolve");

    assert_eq!(config.mode.as_str(), "sdkwork-cloud-app-api");
    assert_eq!(config.provider_kind.as_str(), "sdkwork-cloud-app-api");
    assert_eq!(config.local_api_base_path, "/api/custom/user-center");
    assert_eq!(config.provider_key, "craw-app-api");
    assert_eq!(
        config.app_api_base_url.as_deref(),
        Some("https://app-api.sdkwork.local/craw")
    );
    assert_eq!(config.app_id, "craw-chat");
    assert_eq!(config.secret_id.as_deref(), Some("secret-501"));
    assert_eq!(config.shared_secret.as_deref(), Some("shared-secret-501"));
    assert_eq!(config.handshake_freshness_window_ms, 45_000);
    assert!(!config.allow_authorization_fallback_to_access_token);

    for (key, value) in previous {
        match value {
            Some(value) => unsafe {
                std::env::set_var(key, value);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }
    }
}

#[test]
fn test_resolve_user_center_runtime_config_rejects_incomplete_remote_mode() {
    let _guard = runtime_config_env_guard();
    let values = [
        (
            "SDKWORK_USER_CENTER_MODE",
            Some("sdkwork-cloud-app-api".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_APP_API_BASE_URL",
            Some("https://app-api.sdkwork.local/craw".to_string()),
        ),
        (
            "SDKWORK_USER_CENTER_PROVIDER_KEY",
            Some("craw-app-api".to_string()),
        ),
        ("SDKWORK_USER_CENTER_SECRET_ID", Some("secret-501".to_string())),
        ("SDKWORK_USER_CENTER_SHARED_SECRET", None),
    ];
    let keys = values.iter().map(|(key, _)| *key).collect::<Vec<_>>();
    let previous = keys
        .iter()
        .map(|key| (*key, std::env::var(key).ok()))
        .collect::<Vec<_>>();

    for (key, value) in &values {
        match value {
            Some(value) => unsafe {
                std::env::set_var(key, value);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }
    }

    let error = local_minimal_node::resolve_user_center_runtime_config()
        .expect_err("incomplete remote user-center config should fail closed");
    assert!(
        error.contains("SDKWORK_USER_CENTER_SHARED_SECRET"),
        "remote mode validation should surface the missing shared secret env. actual error: {error}"
    );

    for (key, value) in previous {
        match value {
            Some(value) => unsafe {
                std::env::set_var(key, value);
            },
            None => unsafe {
                std::env::remove_var(key);
            },
        }
    }
}
