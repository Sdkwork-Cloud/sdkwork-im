#[test]
fn test_resolve_bind_addr_defaults_and_supports_container_override() {
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
