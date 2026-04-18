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
            .ends_with("sdks/sdkwork-craw-chat-sdk/openapi/craw-chat-app.openapi.yaml"),
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
