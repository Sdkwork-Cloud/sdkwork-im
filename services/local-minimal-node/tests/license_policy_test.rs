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
fn test_workspace_license_policy_requires_agpl_or_later_with_commercial_authorization() {
    let root = workspace_root();
    let cargo_toml_path = root.join("Cargo.toml");
    let readme_path = root.join("README.md");
    let license_path = root.join("LICENSE");
    let commercial_license_path = root.join("COMMERCIAL-LICENSE.md");
    let cargo_toml = fs::read_to_string(&cargo_toml_path)
        .unwrap_or_else(|_| panic!("missing workspace manifest: {}", cargo_toml_path.display()));
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));
    let license = fs::read_to_string(&license_path)
        .unwrap_or_else(|_| panic!("missing project LICENSE: {}", license_path.display()));
    let commercial_license = fs::read_to_string(&commercial_license_path).unwrap_or_else(|_| {
        panic!(
            "missing commercial license policy: {}",
            commercial_license_path.display()
        )
    });

    assert!(
        cargo_toml.contains("license = \"AGPL-3.0-or-later\""),
        "workspace manifest must publish the SPDX license expression AGPL-3.0-or-later"
    );
    assert!(
        !cargo_toml.contains("license = \"MIT\""),
        "workspace manifest must not keep the old MIT declaration"
    );

    for required_text in [
        "SPDX-License-Identifier: AGPL-3.0-or-later",
        "GNU Affero General Public License",
        "Version 3",
        "or later",
        "https://www.gnu.org/licenses/agpl-3.0.html",
    ] {
        assert!(
            license.contains(required_text),
            "LICENSE must contain {required_text}"
        );
    }

    for required_text in [
        "AGPL-3.0-or-later",
        "Commercial use requires a separate commercial license",
        "production deployment",
        "paid SaaS",
        "commercial distribution",
        "commercial authorization",
    ] {
        assert!(
            commercial_license.contains(required_text),
            "COMMERCIAL-LICENSE.md must contain {required_text}"
        );
    }

    assert!(
        readme.contains("AGPL-3.0-or-later"),
        "README must expose the current AGPL license policy"
    );
    assert!(
        readme.contains("Commercial use requires a separate commercial license"),
        "README must expose the commercial authorization boundary"
    );
    assert!(
        !readme.contains("MIT"),
        "README must not advertise the old MIT repository policy"
    );
}

#[test]
fn test_project_owned_sdk_license_metadata_follows_repository_policy() {
    let root = workspace_root();
    for relative_path in [
        "sdks/sdkwork-im-admin-sdk/sdkwork-im-admin-sdk-typescript/generated/server-openapi/package.json",
        "docs/sites/.vitepress/config.mjs",
    ] {
        let path = root.join(relative_path);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing project-owned SDK manifest: {}", path.display()));
        assert!(
            content.contains("AGPL-3.0-or-later"),
            "{relative_path} must follow the repository AGPL-3.0-or-later policy"
        );
        assert!(
            !content.contains("\"MIT\"")
                && !content.contains(">MIT<")
                && !content.contains("license = \"MIT\""),
            "{relative_path} must not keep the old MIT project license"
        );
    }

    {
        let relative_path =
            "sdks/sdkwork-im-admin-sdk/bin/materialize-im-admin-typescript-workspace.mjs";
        let path = root.join(relative_path);
        let content = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing SDK generator: {}", path.display()));
        assert!(
            content.contains("AGPL-3.0-or-later"),
            "{relative_path} must generate the repository license policy"
        );
        assert!(
            !content.contains("license: 'MIT'") && !content.contains("MIT\n"),
            "{relative_path} must not keep hardcoded MIT project license output"
        );
    }
}
