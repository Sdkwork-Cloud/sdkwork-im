use std::fs;
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::{HeaderMap, Request, StatusCode};
use axum::response::IntoResponse;
use axum::routing::post;
use craw_chat_cli::{CommandOutput, execute_command, parse_cli_args};
use serde_json::{Value, json};
use tokio::net::TcpListener;

#[derive(Clone, Default)]
struct CaptureState {
    requests: Arc<Mutex<Vec<CapturedRequest>>>,
}

#[derive(Clone, Debug)]
struct CapturedRequest {
    path: String,
    authorization: Option<String>,
    body: Value,
}

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("tool dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

async fn spawn_server(app: Router) -> (String, tokio::task::JoinHandle<()>) {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.expect("server should run");
    });
    (format!("http://127.0.0.1:{}", address.port()), handle)
}

async fn reserve_closed_base_url() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("listener should bind");
    let address = listener
        .local_addr()
        .expect("listener should expose local address");
    drop(listener);
    format!("http://127.0.0.1:{}", address.port())
}

fn command_output_json(output: CommandOutput) -> Value {
    match output {
        CommandOutput::Json(value) => value,
        other => panic!("expected json output, got {other:?}"),
    }
}

fn assert_no_authority_fields(body: &Value, context: &str) {
    for forbidden in [
        "tenantId",
        "tenant_id",
        "userId",
        "deviceId",
        "sessionId",
        "sender",
        "senderId",
    ] {
        assert!(
            body.get(forbidden).is_none(),
            "{context} must not carry authority field {forbidden}: {body}"
        );
    }
}

async fn capture_request(
    State(state): State<CaptureState>,
    headers: HeaderMap,
    request: Request<Body>,
) -> impl IntoResponse {
    let path = request.uri().path().to_owned();
    let authorization = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let bytes = axum::body::to_bytes(request.into_body(), usize::MAX)
        .await
        .expect("request body should collect");
    let body = if bytes.is_empty() {
        json!({})
    } else {
        serde_json::from_slice::<Value>(&bytes).expect("request body should be valid json")
    };

    state
        .requests
        .lock()
        .expect("captured request store should stay available")
        .push(CapturedRequest {
            path: path.clone(),
            authorization: authorization.clone(),
            body: body.clone(),
        });

    (
        StatusCode::OK,
        axum::Json(json!({
            "path": path,
            "authorization": authorization,
            "body": body
        })),
    )
}

#[test]
fn test_step12_cli_docs_freeze_authority_model_and_validation_paths() {
    let root = workspace_root();
    let doc_path = root
        .join("docs")
        .join("部署")
        .join("CLI聊天验证与兼容矩阵.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 12 CLI doc: {}", doc_path.display()));
    let readme_path = root.join("README.md");
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", readme_path.display()));

    for required_text in [
        "craw-chat-cli",
        "chat-session",
        "watch",
        "open-chat-test",
        "--public-bearer-secret",
        "tenant_id",
        "auth_bind",
        "compatibility matrix",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 12 CLI doc must contain {required_text}"
        );
    }

    assert!(
        readme.contains("CLI聊天验证与兼容矩阵.md"),
        "README must link to the Step 12 CLI guide"
    );
}

#[test]
fn test_step12_sdk_readmes_freeze_facade_boundaries_and_workspace_entry_points() {
    let root = workspace_root();
    let sdk_index_path = root.join("sdks").join("README.md");
    let sdk_index = fs::read_to_string(&sdk_index_path)
        .unwrap_or_else(|_| panic!("missing SDK index README: {}", sdk_index_path.display()));
    let app_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk")
        .join("README.md");
    let app_sdk = fs::read_to_string(&app_sdk_path)
        .unwrap_or_else(|_| panic!("missing app SDK README: {}", app_sdk_path.display()));
    let admin_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk-admin")
        .join("README.md");
    let admin_sdk = fs::read_to_string(&admin_sdk_path)
        .unwrap_or_else(|_| panic!("missing admin SDK README: {}", admin_sdk_path.display()));
    let readme_path = root.join("README.md");
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", readme_path.display()));

    for required_text in [
        "sdkwork-craw-chat-sdk",
        "sdkwork-craw-chat-sdk-admin",
        "compatibility matrix",
        "TypeScript",
        "Flutter",
    ] {
        assert!(
            sdk_index.contains(required_text),
            "SDK index README must contain {required_text}"
        );
    }

    for required_text in [
        "sdkwork-craw-chat-sdk",
        "TypeScript",
        "Flutter",
        "payload.json",
        "ccp/ws/1",
        "bearer",
        "compatibility matrix",
    ] {
        assert!(
            app_sdk.contains(required_text),
            "app SDK README must contain {required_text}"
        );
    }

    for required_text in [
        "sdkwork-craw-chat-sdk-admin",
        "TypeScript",
        "Flutter",
        "control-plane",
        "protocol governance",
        "compatibility matrix",
    ] {
        assert!(
            admin_sdk.contains(required_text),
            "admin SDK README must contain {required_text}"
        );
    }

    assert!(
        readme.contains("sdks/README.md"),
        "README must link to the SDK index"
    );
}

#[test]
fn test_step12_cli_and_sdk_docs_freeze_recovery_baseline() {
    let root = workspace_root();
    let cli_doc_path = root
        .join("docs")
        .join("部署")
        .join("CLI聊天验证与兼容矩阵.md");
    let cli_doc = fs::read_to_string(&cli_doc_path)
        .unwrap_or_else(|_| panic!("missing Step 12 CLI doc: {}", cli_doc_path.display()));
    let app_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk")
        .join("README.md");
    let app_sdk = fs::read_to_string(&app_sdk_path)
        .unwrap_or_else(|_| panic!("missing app SDK README: {}", app_sdk_path.display()));
    let admin_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk-admin")
        .join("README.md");
    let admin_sdk = fs::read_to_string(&admin_sdk_path)
        .unwrap_or_else(|_| panic!("missing admin SDK README: {}", admin_sdk_path.display()));

    for required_text in [
        "session.disconnect",
        "realtime.overload",
        "goaway",
        "resume fallback",
    ] {
        assert!(
            cli_doc.contains(required_text),
            "Step 12 CLI doc must contain recovery baseline text {required_text}"
        );
    }

    for required_text in [
        "session.disconnect",
        "realtime.overload",
        "goaway",
        "resume fallback",
    ] {
        assert!(
            app_sdk.contains(required_text),
            "app SDK README must contain recovery baseline text {required_text}"
        );
    }

    for required_text in [
        "session.disconnect",
        "realtime.overload",
        "goaway",
        "compatibility matrix",
    ] {
        assert!(
            admin_sdk.contains(required_text),
            "admin SDK README must contain recovery baseline text {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_docs_freeze_detailed_recovery_registry_baseline() {
    let root = workspace_root();
    let cli_doc_path = root
        .join("docs")
        .join("部署")
        .join("CLI聊天验证与兼容矩阵.md");
    let cli_doc = fs::read_to_string(&cli_doc_path)
        .unwrap_or_else(|_| panic!("missing Step 12 CLI doc: {}", cli_doc_path.display()));
    let app_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk")
        .join("README.md");
    let app_sdk = fs::read_to_string(&app_sdk_path)
        .unwrap_or_else(|_| panic!("missing app SDK README: {}", app_sdk_path.display()));
    let admin_sdk_path = root
        .join("sdks")
        .join("sdkwork-craw-chat-sdk-admin")
        .join("README.md");
    let admin_sdk = fs::read_to_string(&admin_sdk_path)
        .unwrap_or_else(|_| panic!("missing admin SDK README: {}", admin_sdk_path.display()));
    let index_doc_path = root
        .join("docs")
        .join("部署")
        .join("兼容矩阵与SDK-CLI-operator验证索引.md");
    let index_doc = fs::read_to_string(&index_doc_path)
        .unwrap_or_else(|_| panic!("missing validation index doc: {}", index_doc_path.display()));

    for required_text in [
        "4001",
        "session.disconnect",
        "reconnect_required",
        "pull-only",
        "events.pull",
    ] {
        assert!(
            cli_doc.contains(required_text),
            "Step 12 CLI doc must contain detailed recovery baseline text {required_text}"
        );
    }

    for required_text in [
        "4001",
        "session.disconnect",
        "reconnect_required",
        "pull-only",
        "events.pull",
    ] {
        assert!(
            app_sdk.contains(required_text),
            "app SDK README must contain detailed recovery baseline text {required_text}"
        );
    }

    for required_text in [
        "4001",
        "session.disconnect",
        "reconnect_required",
        "pull-only",
        "events.pull",
    ] {
        assert!(
            admin_sdk.contains(required_text),
            "admin SDK README must contain detailed recovery baseline text {required_text}"
        );
    }

    for required_text in [
        "4001",
        "session.disconnect",
        "reconnect_required",
        "pull-only",
        "events.pull",
    ] {
        assert!(
            index_doc.contains(required_text),
            "validation index doc must contain detailed recovery baseline text {required_text}"
        );
    }
}

#[test]
fn test_continuous_optimization_docs_freeze_single_validation_index() {
    let root = workspace_root();
    let index_doc_path = root
        .join("docs")
        .join("部署")
        .join("兼容矩阵与SDK-CLI-operator验证索引.md");
    let index_doc = fs::read_to_string(&index_doc_path).unwrap_or_else(|_| {
        panic!(
            "missing continuous optimization validation index doc: {}",
            index_doc_path.display()
        )
    });
    let deploy_readme_path = root.join("docs").join("部署").join("README.md");
    let deploy_readme = fs::read_to_string(&deploy_readme_path)
        .unwrap_or_else(|_| panic!("missing deploy README: {}", deploy_readme_path.display()));
    let sdk_index_path = root.join("sdks").join("README.md");
    let sdk_index = fs::read_to_string(&sdk_index_path)
        .unwrap_or_else(|_| panic!("missing SDK index README: {}", sdk_index_path.display()));
    let repo_readme_path = root.join("README.md");
    let repo_readme = fs::read_to_string(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));

    for required_text in [
        "compatibility matrix",
        "sdkwork-craw-chat-sdk",
        "sdkwork-craw-chat-sdk-admin",
        "craw-chat-cli",
        "open-chat-test",
        "chat_cli_contract_test.rs",
        "chat_cli_e2e_test.rs",
        "protocol_registry_test.rs",
        "protocol_governance_test.rs",
    ] {
        assert!(
            index_doc.contains(required_text),
            "validation index doc must contain {required_text}"
        );
    }

    for doc in [&deploy_readme, &sdk_index, &repo_readme] {
        assert!(
            doc.contains("兼容矩阵与SDK-CLI-operator验证索引.md"),
            "all public entry readmes must link to the single validation index doc"
        );
    }
}

#[test]
fn test_continuous_optimization_docs_freeze_release_bundle_archive_convention() {
    let root = workspace_root();
    let bundle_readme_path = root.join("artifacts").join("releases").join("README.md");
    let bundle_readme = fs::read_to_string(&bundle_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle README: {}",
            bundle_readme_path.display()
        )
    });
    let wave_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let wave_bundle_manifest =
        fs::read_to_string(&wave_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing wave bundle manifest: {}",
                wave_bundle_manifest_path.display()
            )
        });
    let repo_readme_path = root.join("README.md");
    let repo_readme = fs::read_to_string(&repo_readme_path)
        .unwrap_or_else(|_| panic!("missing repository README: {}", repo_readme_path.display()));

    for required_text in [
        "artifacts/releases",
        "wave-d-2026-04-08",
        "step-13-release-readiness-2026-04-08.md",
        "wave-d-93-总验收-2026-04-08.md",
        "go / no-go",
        "可审计",
        "可回滚",
    ] {
        assert!(
            bundle_readme.contains(required_text) || wave_bundle_manifest.contains(required_text),
            "release bundle archive assets must contain {required_text}"
        );
    }

    assert!(
        repo_readme.contains("artifacts/releases/README.md"),
        "README must link to the release bundle archive convention"
    );
}

#[test]
fn test_continuous_optimization_docs_freeze_sdk_release_catalog_contract() {
    let root = workspace_root();
    let sdk_index_path = root.join("sdks").join("README.md");
    let sdk_index = fs::read_to_string(&sdk_index_path)
        .unwrap_or_else(|_| panic!("missing SDK index README: {}", sdk_index_path.display()));
    let release_readme_path = root.join("artifacts").join("releases").join("README.md");
    let release_readme = fs::read_to_string(&release_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle README: {}",
            release_readme_path.display()
        )
    });
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing wave bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("sdk-release-catalog.json");
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing SDK release catalog: {}", catalog_path.display()));
    let catalog_json: Value = serde_json::from_str(&catalog).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog json: {}",
            catalog_path.display()
        )
    });

    assert_eq!(catalog_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(catalog_json["artifact"], "sdk-release-catalog");
    assert_eq!(catalog_json["state"], "template_only_pending_generation");

    let artifacts = catalog_json["sdkArtifacts"]
        .as_array()
        .expect("sdkArtifacts should be an array");
    for (id, audience, language, package_name, readme_path) in [
        (
            "app-typescript",
            "app",
            "typescript",
            "sdkwork-craw-chat-sdk-typescript",
            "sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-typescript/README.md",
        ),
        (
            "app-flutter",
            "app",
            "flutter",
            "sdkwork-craw-chat-sdk-flutter",
            "sdks/sdkwork-craw-chat-sdk/sdkwork-craw-chat-sdk-flutter/README.md",
        ),
        (
            "admin-typescript",
            "admin",
            "typescript",
            "sdkwork-craw-chat-sdk-admin-typescript",
            "sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-typescript/README.md",
        ),
        (
            "admin-flutter",
            "admin",
            "flutter",
            "sdkwork-craw-chat-sdk-admin-flutter",
            "sdks/sdkwork-craw-chat-sdk-admin/sdkwork-craw-chat-sdk-admin-flutter/README.md",
        ),
    ] {
        assert!(
            artifacts.iter().any(|artifact| {
                artifact["id"] == id
                    && artifact["audience"] == audience
                    && artifact["language"] == language
                    && artifact["package"] == package_name
                    && artifact["readmePath"] == readme_path
                    && artifact["generationStatus"] == "template_only_pending_generation"
                    && artifact["releaseStatus"] == "not_published"
            }),
            "SDK release catalog must contain {id} / {audience} / {language}"
        );
    }

    for doc in [&sdk_index, &release_readme, &bundle_manifest] {
        assert!(
            doc.contains("sdk-release-catalog.json"),
            "public SDK/release docs must link to sdk-release-catalog.json"
        );
    }
}

#[test]
fn test_continuous_optimization_docs_freeze_sdk_release_catalog_schema_contract() {
    let root = workspace_root();
    let release_readme_path = root.join("artifacts").join("releases").join("README.md");
    let release_readme = fs::read_to_string(&release_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle README: {}",
            release_readme_path.display()
        )
    });
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing wave bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("sdk-release-catalog.schema.json");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|_| {
        panic!(
            "missing SDK release catalog schema: {}",
            schema_path.display()
        )
    });
    let schema_json: Value = serde_json::from_str(&schema).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog schema json: {}",
            schema_path.display()
        )
    });
    let catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("sdk-release-catalog.json");
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing SDK release catalog: {}", catalog_path.display()));
    let catalog_json: Value = serde_json::from_str(&catalog).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog json: {}",
            catalog_path.display()
        )
    });

    assert_eq!(
        catalog_json["$schema"],
        "../schemas/sdk-release-catalog.schema.json"
    );
    assert_eq!(
        schema_json["$id"],
        "artifacts/releases/schemas/sdk-release-catalog.schema.json"
    );
    assert_eq!(schema_json["title"], "craw-chat sdk release catalog");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "sdk-release-catalog"
    );

    let state_enum = schema_json["properties"]["state"]["enum"]
        .as_array()
        .expect("state enum should be defined");
    for required_state in [
        "template_only_pending_generation",
        "generated_pending_publication",
        "published",
    ] {
        assert!(
            state_enum.iter().any(|value| value == required_state),
            "SDK release catalog schema must declare state {required_state}"
        );
    }

    let release_status_enum =
        schema_json["properties"]["sdkArtifacts"]["items"]["properties"]["releaseStatus"]["enum"]
            .as_array()
            .expect("releaseStatus enum should be defined");
    for required_status in ["not_published", "published"] {
        assert!(
            release_status_enum
                .iter()
                .any(|value| value == required_status),
            "SDK release catalog schema must declare releaseStatus {required_status}"
        );
    }

    for doc in [&release_readme, &bundle_manifest] {
        assert!(
            doc.contains("sdk-release-catalog.schema.json"),
            "release docs must link to sdk-release-catalog.schema.json"
        );
    }
}

#[test]
fn test_continuous_optimization_sdk_leaf_readmes_freeze_release_catalog_boundary() {
    let root = workspace_root();
    let catalog_path = "artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json";

    for (label, path, required_texts) in [
        (
            "app TypeScript README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-typescript")
                .join("README.md"),
            [
                "sdk-release-catalog.json",
                "template_only_pending_generation",
                "not_published",
                catalog_path,
            ],
        ),
        (
            "app Flutter README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-flutter")
                .join("README.md"),
            [
                "sdk-release-catalog.json",
                "template_only_pending_generation",
                "not_published",
                catalog_path,
            ],
        ),
        (
            "admin TypeScript README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-typescript")
                .join("README.md"),
            [
                "sdk-release-catalog.json",
                "template_only_pending_generation",
                "not_published",
                catalog_path,
            ],
        ),
        (
            "admin Flutter README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-flutter")
                .join("README.md"),
            [
                "sdk-release-catalog.json",
                "template_only_pending_generation",
                "not_published",
                catalog_path,
            ],
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in required_texts {
            assert!(
                readme.contains(required_text),
                "{label} must contain release catalog boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_sdk_container_readmes_freeze_release_catalog_boundary() {
    let root = workspace_root();
    let catalog_path = "artifacts/releases/wave-d-2026-04-08/sdk-release-catalog.json";

    for (label, path) in [
        (
            "app SDK container README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("README.md"),
        ),
        (
            "admin SDK container README",
            root.join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("README.md"),
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in [
            "sdk-release-catalog.json",
            "template_only_pending_generation",
            "not_published",
            catalog_path,
        ] {
            assert!(
                readme.contains(required_text),
                "{label} must contain release catalog boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_sdk_release_catalog_freezes_version_placeholder_contract() {
    let root = workspace_root();
    let sdk_index_path = root.join("sdks").join("README.md");
    let sdk_index = fs::read_to_string(&sdk_index_path)
        .unwrap_or_else(|_| panic!("missing SDK index README: {}", sdk_index_path.display()));
    let release_readme_path = root.join("artifacts").join("releases").join("README.md");
    let release_readme = fs::read_to_string(&release_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle README: {}",
            release_readme_path.display()
        )
    });
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing wave bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("sdk-release-catalog.schema.json");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|_| {
        panic!(
            "missing SDK release catalog schema: {}",
            schema_path.display()
        )
    });
    let schema_json: Value = serde_json::from_str(&schema).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog schema json: {}",
            schema_path.display()
        )
    });
    let catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("sdk-release-catalog.json");
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing SDK release catalog: {}", catalog_path.display()));
    let catalog_json: Value = serde_json::from_str(&catalog).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog json: {}",
            catalog_path.display()
        )
    });

    let artifact_properties = &schema_json["properties"]["sdkArtifacts"]["items"]["properties"];
    assert_eq!(
        artifact_properties["versionStatus"]["type"], "string",
        "SDK release catalog schema must define versionStatus"
    );
    let version_status_enum = artifact_properties["versionStatus"]["enum"]
        .as_array()
        .expect("versionStatus enum should be defined");
    assert!(
        version_status_enum
            .iter()
            .any(|value| value == "version_unassigned_pending_freeze"),
        "SDK release catalog schema must declare version_unassigned_pending_freeze"
    );
    assert_eq!(
        artifact_properties["plannedVersion"]["type"],
        serde_json::json!(["string", "null"]),
        "SDK release catalog schema must define plannedVersion"
    );

    let artifacts = catalog_json["sdkArtifacts"]
        .as_array()
        .expect("sdkArtifacts should be an array");
    for artifact in artifacts {
        assert_eq!(
            artifact["versionStatus"], "version_unassigned_pending_freeze",
            "SDK release artifacts must freeze versionStatus before real version assignment"
        );
        assert!(
            artifact["plannedVersion"].is_null(),
            "SDK release artifacts must keep plannedVersion null before version freeze"
        );
    }

    for doc in [&sdk_index, &release_readme, &bundle_manifest] {
        assert!(
            doc.contains("version_unassigned_pending_freeze"),
            "public SDK/release docs must mention version_unassigned_pending_freeze"
        );
        assert!(
            doc.contains("plannedVersion"),
            "public SDK/release docs must mention plannedVersion"
        );
    }
}

#[test]
fn test_continuous_optimization_sdk_release_catalog_freezes_version_decision_source_contract() {
    let root = workspace_root();
    let sdk_index_path = root.join("sdks").join("README.md");
    let sdk_index = fs::read_to_string(&sdk_index_path)
        .unwrap_or_else(|_| panic!("missing SDK index README: {}", sdk_index_path.display()));
    let release_readme_path = root.join("artifacts").join("releases").join("README.md");
    let release_readme = fs::read_to_string(&release_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle README: {}",
            release_readme_path.display()
        )
    });
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing wave bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("sdk-release-catalog.schema.json");
    let schema = fs::read_to_string(&schema_path).unwrap_or_else(|_| {
        panic!(
            "missing SDK release catalog schema: {}",
            schema_path.display()
        )
    });
    let schema_json: Value = serde_json::from_str(&schema).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog schema json: {}",
            schema_path.display()
        )
    });
    let catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("sdk-release-catalog.json");
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing SDK release catalog: {}", catalog_path.display()));
    let catalog_json: Value = serde_json::from_str(&catalog).unwrap_or_else(|_| {
        panic!(
            "invalid SDK release catalog json: {}",
            catalog_path.display()
        )
    });

    let artifact_properties = &schema_json["properties"]["sdkArtifacts"]["items"]["properties"];
    assert_eq!(
        artifact_properties["versionDecisionSourcePath"]["type"],
        serde_json::json!(["string", "null"]),
        "SDK release catalog schema must define versionDecisionSourcePath"
    );

    let artifacts = catalog_json["sdkArtifacts"]
        .as_array()
        .expect("sdkArtifacts should be an array");
    for artifact in artifacts {
        assert!(
            artifact["versionDecisionSourcePath"].is_null(),
            "SDK release artifacts must keep versionDecisionSourcePath null before freeze evidence is assigned"
        );
    }

    for doc in [&sdk_index, &release_readme, &bundle_manifest] {
        assert!(
            doc.contains("versionDecisionSourcePath"),
            "public SDK/release docs must mention versionDecisionSourcePath"
        );
        assert!(
            doc.contains("null"),
            "public SDK/release docs must mention the unresolved null placeholder"
        );
    }
}

#[test]
fn test_continuous_optimization_sdk_leaf_readmes_freeze_version_decision_source_boundary() {
    for (label, path) in [
        (
            "app TypeScript README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-typescript")
                .join("README.md"),
        ),
        (
            "app Flutter README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-flutter")
                .join("README.md"),
        ),
        (
            "admin TypeScript README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-typescript")
                .join("README.md"),
        ),
        (
            "admin Flutter README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-flutter")
                .join("README.md"),
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in ["versionDecisionSourcePath", "null"] {
            assert!(
                readme.contains(required_text),
                "{label} must contain version decision source boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_sdk_leaf_readmes_freeze_version_placeholder_boundary() {
    for (label, path) in [
        (
            "app TypeScript README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-typescript")
                .join("README.md"),
        ),
        (
            "app Flutter README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("sdkwork-craw-chat-sdk-flutter")
                .join("README.md"),
        ),
        (
            "admin TypeScript README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-typescript")
                .join("README.md"),
        ),
        (
            "admin Flutter README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("sdkwork-craw-chat-sdk-admin-flutter")
                .join("README.md"),
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in [
            "plannedVersion",
            "null",
            "versionStatus",
            "version_unassigned_pending_freeze",
        ] {
            assert!(
                readme.contains(required_text),
                "{label} must contain version placeholder boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_sdk_container_readmes_freeze_version_placeholder_boundary() {
    for (label, path) in [
        (
            "app SDK container README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("README.md"),
        ),
        (
            "admin SDK container README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("README.md"),
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in [
            "plannedVersion",
            "null",
            "versionStatus",
            "version_unassigned_pending_freeze",
        ] {
            assert!(
                readme.contains(required_text),
                "{label} must contain version placeholder boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_continuous_optimization_sdk_container_readmes_freeze_version_decision_source_boundary() {
    for (label, path) in [
        (
            "app SDK container README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk")
                .join("README.md"),
        ),
        (
            "admin SDK container README",
            workspace_root()
                .join("sdks")
                .join("sdkwork-craw-chat-sdk-admin")
                .join("README.md"),
        ),
    ] {
        let readme = fs::read_to_string(&path)
            .unwrap_or_else(|_| panic!("missing {label}: {}", path.display()));
        for required_text in ["versionDecisionSourcePath", "null"] {
            assert!(
                readme.contains(required_text),
                "{label} must contain version decision source boundary text {required_text}"
            );
        }
    }
}

#[test]
fn test_step12_compatibility_matrix_doc_freezes_control_plane_mapping_and_client_rows() {
    let root = workspace_root();
    let doc_path = root
        .join("docs")
        .join("部署")
        .join("CLI聊天验证与兼容矩阵.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 12 compatibility doc: {}", doc_path.display()));

    for required_text in [
        "web",
        "desktop",
        "mobile",
        "backend",
        "ccp/sse/1",
        "ccp/mqtt/1",
        "payload.cbor",
        "/api/v1/control/protocol-registry",
        "/api/v1/control/protocol-governance",
        "sdkCompatibilityBaseline",
        "sdkwork-craw-chat-sdk",
        "sdkwork-craw-chat-sdk-admin",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 12 compatibility doc must contain {required_text}"
        );
    }
}

#[test]
fn test_step12_open_chat_test_scripts_freeze_scripted_validation_contract() {
    let root = workspace_root();
    let doc_path = root
        .join("docs")
        .join("部署")
        .join("CLI聊天验证与兼容矩阵.md");
    let doc = fs::read_to_string(&doc_path)
        .unwrap_or_else(|_| panic!("missing Step 12 compatibility doc: {}", doc_path.display()));
    let open_chat_test_ps1_path = root.join("bin").join("open-chat-test.ps1");
    let open_chat_test_ps1 = fs::read_to_string(&open_chat_test_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing open-chat-test PowerShell script: {}",
            open_chat_test_ps1_path.display()
        )
    });
    let open_chat_test_sh_path = root.join("bin").join("open-chat-test.sh");
    let open_chat_test_sh = fs::read_to_string(&open_chat_test_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing open-chat-test bash script: {}",
            open_chat_test_sh_path.display()
        )
    });
    let chat_window_sh_path = root.join("bin").join("chat-window.sh");
    let chat_window_sh = fs::read_to_string(&chat_window_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing chat-window bash script: {}",
            chat_window_sh_path.display()
        )
    });

    for required_text in ["-ScriptedValidation", "-ValidationMessage", "-Json"] {
        assert!(
            open_chat_test_ps1.contains(required_text),
            "open-chat-test.ps1 must contain {required_text}"
        );
    }

    for required_text in [
        "--scripted-validation",
        "--validation-message",
        "--json",
        "watchFrameTypes",
        "--owner-login",
        "--owner-password",
        "--guest-login",
        "--guest-password",
        "login",
    ] {
        assert!(
            open_chat_test_sh.contains(required_text),
            "open-chat-test.sh must contain {required_text}"
        );
    }

    for required_text in ["--bearer-token", "chat-session"] {
        assert!(
            chat_window_sh.contains(required_text),
            "chat-window.sh must contain {required_text}"
        );
    }

    for required_text in [
        "-ScriptedValidation",
        "--scripted-validation",
        "ValidationMessage",
        "watchFrameTypes",
        "realtime.connected",
        "event.window",
        "open-chat-test",
        "OwnerPassword",
        "GuestPassword",
    ] {
        assert!(
            doc.contains(required_text),
            "Step 12 compatibility doc must contain {required_text}"
        );
    }
}

#[test]
fn test_chat_cli_wrappers_rebuild_when_sources_are_newer_than_local_binary() {
    let root = workspace_root();
    let chat_cli_local_ps1_path = root.join("bin").join("chat-cli-local.ps1");
    let chat_cli_local_ps1 = fs::read_to_string(&chat_cli_local_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing chat-cli-local PowerShell wrapper: {}",
            chat_cli_local_ps1_path.display()
        )
    });
    let chat_cli_local_sh_path = root.join("bin").join("chat-cli-local.sh");
    let chat_cli_local_sh = fs::read_to_string(&chat_cli_local_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing chat-cli-local bash wrapper: {}",
            chat_cli_local_sh_path.display()
        )
    });
    let chat_window_gui_ps1_path = root.join("bin").join("chat-window-gui.ps1");
    let chat_window_gui_ps1 = fs::read_to_string(&chat_window_gui_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing chat-window-gui PowerShell launcher: {}",
            chat_window_gui_ps1_path.display()
        )
    });

    for required_text in [
        "Test-ChatCliExecutableNeedsBuild",
        "LastWriteTimeUtc",
        "tools\\chat-cli\\src",
        "Cargo.lock",
    ] {
        assert!(
            chat_cli_local_ps1.contains(required_text),
            "chat-cli-local.ps1 must contain stale-binary rebuild guard text {required_text}"
        );
    }

    for required_text in [
        "chat_cli_binary_needs_build",
        "tools/chat-cli/src",
        "Cargo.lock",
        "build -p craw-chat-cli",
    ] {
        assert!(
            chat_cli_local_sh.contains(required_text),
            "chat-cli-local.sh must contain stale-binary rebuild guard text {required_text}"
        );
    }

    for required_text in [
        "Test-ChatCliExecutableNeedsBuild",
        "LastWriteTimeUtc",
        "tools\\chat-cli\\src",
        "Resolve-ChatCliExecutablePath",
    ] {
        assert!(
            chat_window_gui_ps1.contains(required_text),
            "chat-window-gui.ps1 must contain stale-binary rebuild guard text {required_text}"
        );
    }
}

#[test]
fn test_chat_cli_bash_wrapper_avoids_windows_find_exe_for_source_scan() {
    let root = workspace_root();
    let chat_cli_local_sh_path = root.join("bin").join("chat-cli-local.sh");
    let chat_cli_local_sh = fs::read_to_string(&chat_cli_local_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing chat-cli-local bash wrapper: {}",
            chat_cli_local_sh_path.display()
        )
    });

    assert!(
        chat_cli_local_sh.contains("shopt -s globstar nullglob"),
        "chat-cli-local.sh must enable bash-native recursive globbing instead of relying on external find"
    );
    assert!(
        !chat_cli_local_sh.contains("find \"${input_path}\" -type f -print0"),
        "chat-cli-local.sh must not call external find for source scanning because Windows find.exe breaks the bash wrapper"
    );
}

#[test]
fn test_open_chat_test_ps1_contains_managed_runtime_self_heal_guards() {
    let root = workspace_root();
    let open_chat_test_ps1_path = root.join("bin").join("open-chat-test.ps1");
    let open_chat_test_ps1 = fs::read_to_string(&open_chat_test_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing open-chat-test PowerShell script: {}",
            open_chat_test_ps1_path.display()
        )
    });

    for required_text in [
        "Resolve-LocalMinimalRuntimeDir",
        "Invoke-RepairLocalRuntime",
        "Reset-LocalRuntimeState",
        "Test-IsManagedRuntimeRecoveryCandidate",
        "Invoke-ManagedScriptedValidationWithRecovery",
        "chat-cli invocation timed out after",
        "Managed runtime still failed after repair",
        "repair-runtime-local.ps1",
        "scripted-validation-reset-",
    ] {
        assert!(
            open_chat_test_ps1.contains(required_text),
            "open-chat-test.ps1 must contain managed runtime self-heal guard text {required_text}"
        );
    }
}

#[test]
fn test_open_chat_test_gui_launch_prefers_manual_login_over_prefetched_bearer_tokens() {
    let root = workspace_root();
    let open_chat_test_ps1_path = root.join("bin").join("open-chat-test.ps1");
    let open_chat_test_ps1 = fs::read_to_string(&open_chat_test_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing open-chat-test PowerShell script: {}",
            open_chat_test_ps1_path.display()
        )
    });

    assert!(
        open_chat_test_ps1.contains("\"-Login\", $resolvedOwnerLogin"),
        "open-chat-test.ps1 GUI owner launch must preserve the real login identifier so operators can click Login against the prepared conversation"
    );
    assert!(
        open_chat_test_ps1.contains("\"-Login\", $resolvedGuestLogin"),
        "open-chat-test.ps1 GUI guest launch must preserve the real login identifier so operators can click Login against the prepared conversation"
    );
    assert!(
        !open_chat_test_ps1.contains("\"-BearerToken\", $ownerAuth.BearerToken"),
        "open-chat-test.ps1 GUI owner launch must not inject a prefetched bearer token because that bypasses the manual login flow under test"
    );
    assert!(
        !open_chat_test_ps1.contains("\"-BearerToken\", $guestAuth.BearerToken"),
        "open-chat-test.ps1 GUI guest launch must not inject a prefetched bearer token because that bypasses the manual login flow under test"
    );
}

#[tokio::test]
async fn test_chat_cli_timeline_connect_failure_surfaces_actionable_service_unreachable_hint() {
    let base_url = reserve_closed_base_url().await;

    let error = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_guest",
            "--session-id",
            "s_guest",
            "--device-id",
            "d_guest",
            "timeline",
            "--conversation-id",
            "c_cli_connectivity_demo",
        ])
        .expect("timeline args should parse"),
    )
    .await
    .expect_err("timeline against closed port should fail");

    let message = error.to_string();
    assert!(
        message.contains("unable to connect to craw-chat service"),
        "connectivity failure should explain root cause\nmessage:\n{message}"
    );
    assert!(
        message.contains(base_url.as_str()),
        "connectivity failure should echo the requested base url\nmessage:\n{message}"
    );
    assert!(
        message.contains("verify the service is running"),
        "connectivity failure should suggest verifying server availability\nmessage:\n{message}"
    );
    assert!(
        message.contains("--base-url"),
        "connectivity failure should point users at base-url diagnosis\nmessage:\n{message}"
    );
}

#[tokio::test]
async fn test_chat_cli_watch_connect_failure_surfaces_actionable_realtime_unreachable_hint() {
    let base_url = reserve_closed_base_url().await;

    let error = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_guest",
            "--session-id",
            "s_guest",
            "--device-id",
            "d_guest",
            "watch",
            "--conversation-id",
            "c_cli_connectivity_demo",
            "--event-type",
            "message.posted",
            "--exit-after-events",
            "1",
            "--idle-timeout-seconds",
            "1",
        ])
        .expect("watch args should parse"),
    )
    .await
    .expect_err("watch against closed port should fail");

    let message = error.to_string();
    assert!(
        message.contains("unable to connect realtime websocket"),
        "realtime connectivity failure should explain root cause\nmessage:\n{message}"
    );
    assert!(
        message.contains(base_url.as_str()),
        "realtime connectivity failure should echo the requested base url\nmessage:\n{message}"
    );
    assert!(
        message.contains("/api/v1/realtime/ws"),
        "realtime connectivity failure should identify the websocket endpoint\nmessage:\n{message}"
    );
    assert!(
        message.contains("verify the service is running"),
        "realtime connectivity failure should suggest verifying server availability\nmessage:\n{message}"
    );
    assert!(
        message.contains("--base-url"),
        "realtime connectivity failure should point users at base-url diagnosis\nmessage:\n{message}"
    );
}

#[tokio::test]
async fn test_chat_cli_http_commands_keep_authority_in_token_not_business_body() {
    let state = CaptureState::default();
    let app = Router::new()
        .route("/api/v1/conversations", post(capture_request))
        .route(
            "/api/v1/conversations/{conversation_id}/messages",
            post(capture_request),
        )
        .with_state(state.clone());
    let (base_url, handle) = spawn_server(app).await;

    let create_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "create-conversation",
            "--conversation-id",
            "c_cli_contract_demo",
            "--conversation-type",
            "group",
        ])
        .expect("create args should parse"),
    )
    .await
    .expect("create conversation should succeed");
    let create_json = command_output_json(create_output);
    assert_eq!(create_json["path"], "/api/v1/conversations");

    let send_output = execute_command(
        parse_cli_args([
            "craw-chat-cli",
            "--base-url",
            base_url.as_str(),
            "--tenant-id",
            "t_demo",
            "--user-id",
            "u_owner",
            "--session-id",
            "s_owner",
            "--device-id",
            "d_owner",
            "--public-bearer-secret",
            "local-chat-cli-secret",
            "send-message",
            "--conversation-id",
            "c_cli_contract_demo",
            "--summary",
            "hello from cli contract test",
            "--text",
            "hello from cli contract test",
            "--client-msg-id",
            "cli_contract_msg_1",
        ])
        .expect("send args should parse"),
    )
    .await
    .expect("send message should succeed");
    let send_json = command_output_json(send_output);
    assert_eq!(
        send_json["conversationId"], "c_cli_contract_demo",
        "send command should preserve conversation id in result envelope"
    );

    let captured = state
        .requests
        .lock()
        .expect("captured request store should stay available")
        .clone();
    assert_eq!(
        captured.len(),
        2,
        "create and send should both hit HTTP endpoints"
    );

    let create_request = captured
        .iter()
        .find(|request| request.path == "/api/v1/conversations")
        .expect("create request should be captured");
    assert_no_authority_fields(&create_request.body, "create-conversation body");
    assert_eq!(create_request.body["conversationId"], "c_cli_contract_demo");
    assert_eq!(create_request.body["conversationType"], "group");
    assert!(
        create_request
            .authorization
            .as_deref()
            .is_some_and(|value| value.starts_with("Bearer ")),
        "create-conversation must send bearer authorization header"
    );

    let send_request = captured
        .iter()
        .find(|request| request.path == "/api/v1/conversations/c_cli_contract_demo/messages")
        .expect("send-message request should be captured");
    assert_no_authority_fields(&send_request.body, "send-message body");
    assert_eq!(send_request.body["clientMsgId"], "cli_contract_msg_1");
    assert_eq!(send_request.body["summary"], "hello from cli contract test");
    assert_eq!(send_request.body["text"], "hello from cli contract test");
    assert!(
        send_request
            .authorization
            .as_deref()
            .is_some_and(|value| value.starts_with("Bearer ")),
        "send-message must send bearer authorization header"
    );

    handle.abort();
    let _ = handle.await;
}

#[tokio::test]
async fn test_chat_cli_token_command_freezes_header_and_token_only_contract() {
    let generated_default = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_token",
                "--user-id",
                "u_token",
                "--session-id",
                "s_token",
                "--device-id",
                "d_token",
                "--public-bearer-secret",
                "local-chat-cli-secret",
                "token",
            ])
            .expect("default token args should parse"),
        )
        .await
        .expect("default token command should succeed"),
    );
    let generated_authorization = generated_default["authorization"]
        .as_str()
        .expect("default authorization should be a string");
    let generated_token = generated_default["token"]
        .as_str()
        .expect("default token should be a string");
    assert_eq!(generated_default["source"], "generatedBearerToken");
    assert!(
        generated_authorization.starts_with("Bearer "),
        "default token command must expose bearer-header form"
    );
    assert_eq!(
        generated_authorization,
        format!("Bearer {generated_token}"),
        "default token command must keep authorization aligned with token"
    );
    assert_eq!(generated_default["claims"]["tenant_id"], "t_token");
    assert_eq!(generated_default["claims"]["sub"], "u_token");
    assert_eq!(generated_default["claims"]["sid"], "s_token");
    assert_eq!(generated_default["claims"]["did"], "d_token");

    let generated_token_only = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_token",
                "--user-id",
                "u_token",
                "--session-id",
                "s_token",
                "--device-id",
                "d_token",
                "--public-bearer-secret",
                "local-chat-cli-secret",
                "token",
                "--token-only",
            ])
            .expect("token-only args should parse"),
        )
        .await
        .expect("token-only command should succeed"),
    );
    let generated_token_only_authorization = generated_token_only["authorization"]
        .as_str()
        .expect("token-only authorization should be a string");
    let generated_token_only_token = generated_token_only["token"]
        .as_str()
        .expect("token-only token should be a string");
    assert_eq!(generated_token_only["source"], "generatedBearerToken");
    assert_eq!(
        generated_token_only_authorization, generated_token_only_token,
        "--token-only must return the bare token instead of a bearer header"
    );
    assert!(
        !generated_token_only_authorization.starts_with("Bearer "),
        "--token-only must not keep the Bearer prefix"
    );

    let provided_token_only = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_token",
                "--user-id",
                "u_token",
                "--session-id",
                "s_token",
                "--device-id",
                "d_token",
                "--bearer-token",
                "Bearer provided_token_demo",
                "token",
                "--token-only",
            ])
            .expect("provided token-only args should parse"),
        )
        .await
        .expect("provided token-only command should succeed"),
    );
    assert_eq!(provided_token_only["source"], "providedBearerToken");
    assert_eq!(provided_token_only["authorization"], "provided_token_demo");
    assert_eq!(provided_token_only["token"], "provided_token_demo");
}

#[tokio::test]
async fn test_chat_cli_token_command_normalizes_lowercase_bearer_prefix() {
    let provided_default = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_token",
                "--user-id",
                "u_token",
                "--session-id",
                "s_token",
                "--device-id",
                "d_token",
                "--bearer-token",
                "bearer lower_case_token_demo",
                "token",
            ])
            .expect("provided lowercase bearer args should parse"),
        )
        .await
        .expect("provided lowercase bearer token command should succeed"),
    );
    assert_eq!(provided_default["source"], "providedBearerToken");
    assert_eq!(
        provided_default["authorization"],
        "Bearer lower_case_token_demo"
    );
    assert_eq!(provided_default["token"], "lower_case_token_demo");

    let provided_token_only = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_token",
                "--user-id",
                "u_token",
                "--session-id",
                "s_token",
                "--device-id",
                "d_token",
                "--bearer-token",
                "bearer lower_case_token_demo",
                "token",
                "--token-only",
            ])
            .expect("provided lowercase bearer token-only args should parse"),
        )
        .await
        .expect("provided lowercase bearer token-only command should succeed"),
    );
    assert_eq!(provided_token_only["source"], "providedBearerToken");
    assert_eq!(
        provided_token_only["authorization"],
        "lower_case_token_demo"
    );
    assert_eq!(provided_token_only["token"], "lower_case_token_demo");
}

#[tokio::test]
async fn test_chat_cli_token_command_does_not_synthesize_claims_for_provided_bearer_tokens() {
    let provided_default = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_local_context",
                "--user-id",
                "u_local_context",
                "--session-id",
                "s_local_context",
                "--device-id",
                "d_local_context",
                "--bearer-token",
                "Bearer externally_supplied_token",
                "token",
            ])
            .expect("provided token args should parse"),
        )
        .await
        .expect("provided token command should succeed"),
    );
    assert_eq!(provided_default["source"], "providedBearerToken");
    assert_eq!(
        provided_default["authorization"],
        "Bearer externally_supplied_token"
    );
    assert_eq!(provided_default["token"], "externally_supplied_token");
    assert!(
        provided_default["claims"].is_null(),
        "provided bearer token output must not pretend local CLI inputs are decoded token claims: {}",
        provided_default
    );

    let provided_token_only = command_output_json(
        execute_command(
            parse_cli_args([
                "craw-chat-cli",
                "--tenant-id",
                "t_local_context",
                "--user-id",
                "u_local_context",
                "--session-id",
                "s_local_context",
                "--device-id",
                "d_local_context",
                "--bearer-token",
                "Bearer externally_supplied_token",
                "token",
                "--token-only",
            ])
            .expect("provided token-only args should parse"),
        )
        .await
        .expect("provided token-only command should succeed"),
    );
    assert_eq!(provided_token_only["source"], "providedBearerToken");
    assert_eq!(
        provided_token_only["authorization"],
        "externally_supplied_token"
    );
    assert_eq!(provided_token_only["token"], "externally_supplied_token");
    assert!(
        provided_token_only["claims"].is_null(),
        "provided bearer token-only output must not synthesize claims from local CLI inputs: {}",
        provided_token_only
    );
}
