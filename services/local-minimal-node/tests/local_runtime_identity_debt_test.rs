use std::fs;
use std::path::PathBuf;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

#[test]
fn test_local_runtime_scripts_do_not_keep_public_bearer_debt() {
    let root = workspace_root();
    for relative_path in [
        "bin/start-local.sh",
        "bin/start-local.ps1",
        "bin/init-config-local.sh",
        "bin/init-config-local.ps1",
        "bin/chat-window.sh",
        "bin/chat-window.ps1",
        "bin/chat-window-gui.ps1",
    ] {
        let path = root.join(relative_path);
        let script = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing local runtime script: {}", path.display()));
        for legacy_token in [
            "CRAW_CHAT_PUBLIC_BEARER",
            "PUBLIC_BEARER",
            "--public-bearer-secret",
            "PublicBearerSecret",
            "public_bearer",
            "PublicBearer",
        ] {
            assert!(
                !script.contains(legacy_token),
                "{relative_path} must not keep legacy craw-chat IAM/Public Bearer token `{legacy_token}` after AppContext integration"
            );
        }
    }
}

#[test]
fn test_local_runtime_configs_keep_only_domain_cursor_secret() {
    let root = workspace_root();
    for relative_path in [
        "deployments/templates/local-minimal.env.example",
        "deployments/templates/local-default.env.example",
        "deployments/docker-compose/local-minimal.yml",
    ] {
        let path = root.join(relative_path);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing local runtime config: {}", path.display()));
        assert!(
            content.contains("CRAW_CHAT_FRIEND_REQUEST_CURSOR_HS256_SECRET"),
            "{relative_path} must keep the domain cursor signing secret"
        );
        assert!(
            !content.contains("CRAW_CHAT_PUBLIC_BEARER"),
            "{relative_path} must not configure legacy craw-chat IAM/Public Bearer secrets"
        );
    }
}

#[test]
fn test_smoke_scripts_use_app_context_projection_headers() {
    let root = workspace_root();
    for relative_path in [
        "tools/smoke/local_stack_smoke.sh",
        "tools/smoke/local_stack_smoke.ps1",
    ] {
        let path = root.join(relative_path);
        let script = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing smoke script: {}", path.display()));
        for required_header in [
            "x-sdkwork-tenant-id",
            "x-sdkwork-user-id",
            "x-sdkwork-session-id",
            "x-sdkwork-device-id",
            "x-sdkwork-permission-scope",
        ] {
            assert!(
                script.contains(required_header),
                "{relative_path} must send sdkwork AppContext projection header `{required_header}`"
            );
        }
        for legacy_token in [
            "CRAW_CHAT_PUBLIC_BEARER",
            "PUBLIC_BEARER",
            "--public-bearer-secret",
            "Authorization",
            "actor_kind",
        ] {
            assert!(
                !script.contains(legacy_token),
                "{relative_path} must not keep legacy signed Public Bearer smoke token `{legacy_token}`"
            );
        }
    }
}

#[test]
fn test_backend_regression_tests_do_not_use_legacy_local_bearer_fixtures() {
    let root = workspace_root();
    for relative_path in [
        "services/local-minimal-node/tests/access_control_e2e_test.rs",
        "services/local-minimal-node/tests/chat_runtime_session_namespace_test.rs",
        "services/local-minimal-node/tests/disconnect_fence_persistence_test.rs",
        "services/local-minimal-node/tests/http_e2e_test.rs",
        "services/local-minimal-node/tests/live_subscription_recovery_persistence_test.rs",
        "services/local-minimal-node/tests/performance_quant_baseline_test.rs",
        "services/local-minimal-node/tests/task10_capabilities_e2e_test.rs",
        "tools/smoke/end_to_end_smoke.ps1",
    ] {
        let path = root.join(relative_path);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing backend regression fixture: {}", path.display()));
        for legacy_token in [
            "Bearer ey",
            "DEMO_BEARER",
            "OWNER_BEARER",
            "OTHER_BEARER",
            "AUTOMATION_BEARER",
            "PRIVILEGED_BEARER",
            r#".header("authorization""#,
            r#""Authorization" ="#,
        ] {
            assert!(
                !content.contains(legacy_token),
                "{relative_path} must use sdkwork AppContext projection fixtures, not legacy local bearer fixture `{legacy_token}`"
            );
        }
    }
}
