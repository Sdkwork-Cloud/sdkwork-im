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

fn read_repo_file(relative_path: &str) -> String {
    let path = workspace_root().join(relative_path);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("missing repository file: {}", path.display()))
}

fn marker(parts: &[&str]) -> String {
    parts.concat()
}

#[test]
fn test_provider_docs_use_principal_profile_provider_names_and_paths() {
    let docs = [
        "docs/sites/features/capabilities.md",
        "docs/sites/features/index.md",
        "docs/sites/architecture/runtime-topology.md",
        "docs/sites/architecture/overview.md",
        "docs/sites/api-reference/platform-api.md",
        "docs/sites/api-reference/app/provider-health.md",
        "docs/sites/api-reference/operations/app/provider-health/get-principal-profile-provider-health.md",
        "docs/sites/deployment/profiles-and-env.md",
    ];

    for relative_path in docs {
        let content = read_repo_file(relative_path);
        let forbidden_terms = vec![
            marker(&["user", "-module"]),
            marker(&["user", "_module"]),
            marker(&["User", "Module"]),
            marker(&["user", "Module"]),
            marker(&["CRAW_CHAT_USER", "_MODULE"]),
            marker(&["/backend/v3/api/user", "_module/provider_health"]),
        ];
        for forbidden in forbidden_terms {
            assert!(
                !content.contains(forbidden.as_str()),
                "{relative_path} must not keep legacy principal provider wording `{forbidden}`"
            );
        }
    }

    let provider_health = read_repo_file("docs/sites/api-reference/app/provider-health.md");
    assert!(
        provider_health.contains("/app/v3/api/principal/profiles/provider_health"),
        "provider health docs must expose the principal-profile app route"
    );
    assert!(
        provider_health.contains("getPrincipalProfileProviderHealth")
            || provider_health.contains("principalProfileHealth.retrieve"),
        "provider health docs must expose the principal-profile operation id"
    );

    let deployment = read_repo_file("docs/sites/deployment/profiles-and-env.md");
    assert!(
        deployment.contains("CRAW_CHAT_PRINCIPAL_PROFILE_PROVIDER")
            && deployment.contains("CRAW_CHAT_PRINCIPAL_PROFILE_EXTERNAL_CATALOG_PATH"),
        "deployment docs must document principal-profile provider env keys"
    );
}

#[test]
fn test_provider_plugin_matrix_docs_keep_runtime_provider_ids_visible() {
    let overview = read_repo_file("docs/sites/architecture/overview.md");
    let topology = read_repo_file("docs/sites/architecture/runtime-topology.md");
    let capabilities = read_repo_file("docs/sites/features/capabilities.md");

    for required in [
        "principal-profile",
        "principal-profile-upstream-context",
        "principal-profile-external-catalog",
        "object-storage",
        "rtc",
    ] {
        assert!(
            overview.contains(required)
                || topology.contains(required)
                || capabilities.contains(required),
            "provider matrix docs must keep `{required}` visible"
        );
    }
}
