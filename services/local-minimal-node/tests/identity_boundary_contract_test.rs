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

fn read_workspace_file(relative_path: &str) -> String {
    let path = workspace_root().join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()))
}

fn marker(parts: &[&str]) -> String {
    parts.concat()
}

#[test]
fn test_server_lifecycle_scripts_do_not_render_local_identity_config() {
    for relative_path in [
        "bin/init-config-server.ps1",
        "bin/init-config-server.sh",
        "bin/start-server.ps1",
        "bin/start-server.sh",
    ] {
        let content = read_workspace_file(relative_path);
        let forbidden_terms = vec![
            marker(&["USER", "_CENTER"]),
            marker(&["User", "Center"]),
            marker(&["user", "_center"]),
            marker(&["user", "-center"]),
            marker(&["/api/app", "/v1"]),
        ];
        for forbidden in forbidden_terms {
            assert!(
                !content.contains(forbidden.as_str()),
                "{relative_path} must not keep sdkwork-im owned identity config debt `{forbidden}`"
            );
        }
    }
}

#[test]
fn test_principal_profile_provider_contract_is_read_only_and_not_user_lifecycle() {
    for relative_path in [
        "crates/im-platform-contracts/src/provider.rs",
        "services/local-minimal-node/src/node.rs",
        "services/local-minimal-node/src/node/build.rs",
        "sdks/sdkwork-im-backend-sdk/openapi/sdkwork-im-backend-api.openapi.yaml",
    ] {
        let content = read_workspace_file(relative_path);
        let forbidden_terms = vec![
            marker(&["User", "Module"]),
            marker(&["user", "_module"]),
            marker(&["user", "-module"]),
            marker(&["user", "Module"]),
            marker(&["User", "ModuleCreateOrBindRequest"]),
            marker(&["User", "ModuleUpdateProfileRequest"]),
            marker(&["create", "_or_bind_user"]),
            marker(&["update", "_user_profile"]),
            marker(&["disable", "_user"]),
            marker(&["Local", "UserModuleProvider"]),
        ];
        for forbidden in forbidden_terms {
            assert!(
                !content.contains(forbidden.as_str()),
                "{relative_path} must not keep sdkwork-im owned user lifecycle/provider debt `{forbidden}`"
            );
        }
    }

    let legacy_user_lifecycle_module = workspace_root().join(marker(&[
        "services/local-minimal-node/src/node/user",
        "_module.rs",
    ]));
    assert!(
        !legacy_user_lifecycle_module.exists(),
        "local-minimal-node must not keep a legacy local user lifecycle runtime implementation at {}",
        legacy_user_lifecycle_module.display()
    );
}

#[test]
fn test_sdkwork_im_does_not_keep_local_token_or_iam_context_runtime() {
    for relative_path in [
        "Cargo.toml",
        "crates/im-app-context/Cargo.toml",
        "crates/im-app-context/src/lib.rs",
        "services/local-minimal-node/src/node.rs",
        "services/local-minimal-node/src/node/build.rs",
        "services/session-gateway/src/lib.rs",
        "services/session-gateway/src/presence_routes.rs",
    ] {
        let content = read_workspace_file(relative_path);
        let forbidden_terms = vec![
            marker(&["im", "-auth-context"]),
            marker(&["im", "_auth_context"]),
            marker(&["im", "-principal-context"]),
            marker(&["im", "_principal_context"]),
            marker(&["Principal", "Context"]),
            marker(&["resolve", "_principal_context"]),
            marker(&["resolve", "_trusted_principal_headers"]),
            marker(&["PUBLIC", "_BEARER"]),
            marker(&["SDKWORK_IM_PUBLIC", "_BEARER"]),
            marker(&["encode", "_hs256_bearer_token"]),
            marker(&["resolve", "_public_bearer_auth_context"]),
            marker(&["resolve", "_bearer_auth_context"]),
            marker(&["jsonwebtoken"]),
            marker(&["require", "_principal_context"]),
            marker(&["x-tenant", "-id"]),
            marker(&["x-user", "-id"]),
            marker(&["x-actor", "-id"]),
            marker(&["x-actor", "-kind"]),
            marker(&["x-scope"]),
            marker(&["x-scopes"]),
        ];
        for forbidden in forbidden_terms {
            assert!(
                !content.contains(forbidden.as_str()),
                "{relative_path} must not keep sdkwork-im-owned token/IAM runtime debt `{forbidden}`"
            );
        }
    }

    let legacy_auth_context_crate = workspace_root().join(marker(&["crates/im", "-auth-context"]));
    assert!(
        !legacy_auth_context_crate.exists(),
        "sdkwork-im must not keep a local IAM/auth-context crate at {}",
        legacy_auth_context_crate.display()
    );

    let legacy_principal_context_crate =
        workspace_root().join(marker(&["crates/im", "-principal-context"]));
    assert!(
        !legacy_principal_context_crate.exists(),
        "sdkwork-im must not keep a principal-context crate at {}",
        legacy_principal_context_crate.display()
    );

    let app_context_crate = workspace_root().join("crates/im-app-context");
    assert!(
        app_context_crate.exists(),
        "sdkwork-im should keep an app-context crate that only consumes sdkwork-appbase AppContext projection"
    );

    let app_context_source = read_workspace_file("crates/im-app-context/src/lib.rs");
    for required in [
        "AppContext",
        "resolve_app_context",
        "x-sdkwork-tenant-id",
        "x-sdkwork-user-id",
    ] {
        assert!(
            app_context_source.contains(required),
            "crates/im-app-context/src/lib.rs must expose sdkwork-appbase AppContext projection `{required}`"
        );
    }
}

#[test]
fn test_backend_openapi_security_helpers_are_named_for_app_context_not_bearer() {
    for relative_path in [
        "services/audit-service/src/lib.rs",
        "services/automation-service/src/lib.rs",
        "services/sdkwork-comms-conversation-service/src/runtime/http.rs",
        "services/media-service/src/lib.rs",
        "services/notification-service/src/lib.rs",
        "services/ops-service/src/lib.rs",
        "services/projection-service/src/http.rs",
        "services/session-gateway/src/lib.rs",
        "services/streaming-service/src/lib.rs",
    ] {
        let content = read_workspace_file(relative_path);
        let forbidden = marker(&["requires", "_bearer"]);
        assert!(
            !content.contains(forbidden.as_str()),
            "{relative_path} must name protected OpenAPI routes as AppContext requirements, not `{forbidden}`"
        );
    }
}
