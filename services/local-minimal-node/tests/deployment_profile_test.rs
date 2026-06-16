use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use axum::{
    Json, Router,
    extract::Path as AxumPath,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use im_app_context::{AppContextSignatureConfig, resolve_app_context_with_signature_config};
use tokio::net::TcpListener;

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("service dir should have parent")
        .parent()
        .expect("workspace root should exist")
        .to_path_buf()
}

fn first_non_empty_line(content: &str) -> &str {
    content
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .expect("content should contain at least one non-empty line")
}

fn unique_temp_root(prefix: &str) -> PathBuf {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system time should be after epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("sdkwork_im_{prefix}_{unique}"))
}

fn write_file_with_parents(path: &Path, content: &str) {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).expect("parent dir should be created");
    }
    fs::write(path, content).expect("file should be written");
}

fn wait_for_path(path: &Path, timeout: Duration) -> bool {
    let deadline = Instant::now() + timeout;
    while Instant::now() < deadline {
        if path.exists() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    path.exists()
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

fn dual_token_smoke_app() -> Router {
    Router::new()
        .route("/healthz", get(|| async { "ok" }))
        .route(
            "/im/v3/api/chat/conversations",
            post(move |headers: HeaderMap| async move {
                match require_dual_token_context(headers) {
                    Some(response) => response,
                    None => Json(serde_json::json!({})).into_response(),
                }
            }),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}/messages",
            post(
                move |headers: HeaderMap, AxumPath(_conversation_id): AxumPath<String>| async move {
                    match require_dual_token_context(headers) {
                        Some(response) => response,
                        None => Json(serde_json::json!({})).into_response(),
                    }
                },
            ),
        )
        .route(
            "/im/v3/api/chat/conversations/{conversation_id}",
            get(
                move |headers: HeaderMap, AxumPath(conversation_id): AxumPath<String>| async move {
                    match require_dual_token_context(headers) {
                        Some(response) => response,
                        None => Json(serde_json::json!({
                            "conversationId": conversation_id,
                            "lastSummary": "smoke"
                        }))
                        .into_response(),
                    }
                },
            ),
        )
}

fn require_dual_token_context(headers: HeaderMap) -> Option<Response> {
    resolve_app_context_with_signature_config(
        &headers,
        AppContextSignatureConfig {
            require_signature: false,
            shared_secret: None,
        },
    )
    .err()
    .map(|error| {
        (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "code": error.code(),
                "message": error.message()
            })),
        )
            .into_response()
    })
}

#[test]
fn test_status_local_help_texts_share_runtime_ops_contract_across_platform_scripts() {
    let root = workspace_root();
    let status_ps1_path = root.join("bin").join("status-local.ps1");
    let status_sh_path = root.join("bin").join("status-local.sh");

    let status_ps1 = fs::read_to_string(&status_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", status_ps1_path.display()));
    let status_sh = fs::read_to_string(&status_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", status_sh_path.display()));

    let shared_contract = "Show local-minimal-node pid, config, stdout/stderr logs, health status, and the next runtime-dir inspection/repair/list/archive/prune/preview/restore steps.";
    assert!(
        status_ps1.contains(shared_contract),
        "status-local.ps1 help must describe the full runtime operations contract"
    );
    assert!(
        status_sh.contains(shared_contract),
        "status-local.sh help must describe the same runtime operations contract as status-local.ps1"
    );
}

#[test]
fn test_quick_start_doc_freezes_full_local_command_surface() {
    let root = workspace_root();
    let quick_start_doc_path = root.join("docs").join("部署").join("快速启动脚本.md");
    let readme_path = root.join("README.md");

    let quick_start_doc = fs::read_to_string(&quick_start_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", quick_start_doc_path.display()));
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));

    for command in [
        "install-local",
        "init-config-local",
        "start-local",
        "status-local",
        "restart-local",
        "stop-local",
        "inspect-runtime-local",
        "repair-runtime-local",
        "list-runtime-backups-local",
        "archive-runtime-backup-local",
        "prune-runtime-archives-local",
        "preview-runtime-restore-local",
        "restore-runtime-local",
    ] {
        assert!(
            quick_start_doc.contains(command),
            "快速启动脚本.md must freeze the documented local command surface for {command}"
        );
    }

    for profile_contract in [
        "local-default",
        "--profile <local-minimal|local-default>",
        "-ProfileName <local-minimal|local-default>",
    ] {
        assert!(quick_start_doc.contains(profile_contract));
    }

    for smoke_contract in ["--smoke-base-url <url>", "-SmokeBaseUrl <url>"] {
        assert!(quick_start_doc.contains(smoke_contract));
    }

    for command in [
        "install-local",
        "init-config-local",
        "start-local",
        "status-local",
        "restart-local",
        "stop-local",
    ] {
        assert!(
            readme.contains(command),
            "README.md must advertise the core local lifecycle command {command}"
        );
    }
}

#[test]
fn test_quick_start_doc_surfaces_local_default_profile_examples_across_lifecycle_commands() {
    let root = workspace_root();
    let quick_start_doc_path = root.join("docs").join("部署").join("快速启动脚本.md");
    let readme_path = root.join("README.md");

    let quick_start_doc = fs::read_to_string(&quick_start_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", quick_start_doc_path.display()));
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));

    for example in [
        "./bin/install-local.ps1 -ProfileName local-default",
        "./bin/init-config-local.ps1 -ProfileName local-default",
        "./bin/start-local.ps1 -ProfileName local-default",
        "./bin/restart-local.ps1 -ProfileName local-default",
        "./bin/stop-local.ps1 -ProfileName local-default",
        "bash bin/install-local.sh --profile local-default",
        "bash bin/init-config-local.sh --profile local-default",
        "bash bin/start-local.sh --profile local-default",
        "bash bin/restart-local.sh --profile local-default",
        "bash bin/stop-local.sh --profile local-default",
        "bin\\install-local.cmd --profile local-default",
        "bin\\init-config-local.cmd --profile local-default",
        "bin\\start-local.cmd --profile local-default",
        "bin\\restart-local.cmd --profile local-default",
        "bin\\stop-local.cmd --profile local-default",
    ] {
        assert!(
            quick_start_doc.contains(example),
            "快速启动脚本.md must surface the local-default lifecycle example `{example}` after lifecycle profile support was implemented"
        );
    }

    for contract in [
        ".runtime/local-default/config/local-default.env",
        ".runtime/local-minimal",
        "local-default` 仍复用 `local-minimal`",
    ] {
        assert!(
            quick_start_doc.contains(contract),
            "快速启动脚本.md must explain the current local-default config/runtime compatibility contract `{contract}`"
        );
    }

    for example in [
        "./bin/install-local.ps1 -ProfileName local-default",
        "./bin/start-local.ps1 -ProfileName local-default",
        "./bin/restart-local.ps1 -ProfileName local-default",
        "./bin/install-local.sh --profile local-default",
        "./bin/start-local.sh --profile local-default",
        "./bin/restart-local.sh --profile local-default",
    ] {
        assert!(
            readme.contains(example),
            "README.md must surface the local-default lifecycle example `{example}` so the top-level operator entry stays aligned with the shipped scripts"
        );
    }
}

#[test]
fn test_deployment_profiles_and_templates_document_local_minimal_and_local_default_contracts() {
    let root = workspace_root();
    let local_default_compose_path = root
        .join("deployments")
        .join("docker-compose")
        .join("local-default.yml");
    let profile_doc_path = root
        .join("docs")
        .join("部署")
        .join("多环境Profile与配置模板.md");
    let deployment_readme_path = root.join("docs").join("部署").join("README.md");
    let local_minimal_template_path = root
        .join("deployments")
        .join("templates")
        .join("local-minimal.env.example");
    let local_default_template_path = root
        .join("deployments")
        .join("templates")
        .join("local-default.env.example");
    let site_profiles_env_doc_path = root
        .join("docs")
        .join("sites")
        .join("deployment")
        .join("profiles-and-env.md");
    let readme_path = root.join("README.md");

    let local_default_compose =
        fs::read_to_string(&local_default_compose_path).unwrap_or_else(|_| {
            panic!(
                "missing compose profile: {}",
                local_default_compose_path.display()
            )
        });
    let profile_doc = fs::read_to_string(&profile_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", profile_doc_path.display()));
    let deployment_readme = fs::read_to_string(&deployment_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment doc: {}",
            deployment_readme_path.display()
        )
    });
    let local_minimal_template =
        fs::read_to_string(&local_minimal_template_path).unwrap_or_else(|_| {
            panic!(
                "missing template file: {}",
                local_minimal_template_path.display()
            )
        });
    let local_default_template =
        fs::read_to_string(&local_default_template_path).unwrap_or_else(|_| {
            panic!(
                "missing template file: {}",
                local_default_template_path.display()
            )
        });
    let site_profiles_env_doc = fs::read_to_string(&site_profiles_env_doc_path)
        .unwrap_or_else(|_| panic!("missing site doc: {}", site_profiles_env_doc_path.display()));
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));

    assert!(local_default_compose.contains("file: local-minimal.yml"));
    assert!(local_default_compose.contains("service: local-minimal-node"));

    for profile_name in [
        "local-minimal",
        "local-default",
        "private-saas-single-cell",
        "cloud-shared-cell",
        "cloud-dedicated-cell",
    ] {
        assert!(
            profile_doc.contains(profile_name),
            "多环境Profile与配置模板.md must document the profile contract for {profile_name}"
        );
    }

    for template_content in [&local_minimal_template, &local_default_template] {
        assert!(template_content.contains("SDKWORK_IM_BIND_ADDR="));
        assert!(template_content.contains("SDKWORK_IM_RUNTIME_DIR="));
        assert!(template_content.contains("SDKWORK_IM_RUNTIME_PROFILE="));
        assert!(template_content.contains("SDKWORK_IM_BROWSER_ORIGINS="));
        assert!(!template_content.contains("SDKWORK_IM_PUBLIC_BEARER"));
        assert!(template_content.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET="));
        assert!(template_content.contains("SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true"));
        assert!(template_content.contains("SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET="));
        assert!(
            template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS=")
        );
        assert!(
            template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS=")
        );
        assert!(
            template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS=")
        );
        assert!(template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS="));
        assert!(
            template_content
                .contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED=")
        );
        assert!(
            template_content.contains(
                "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS="
            )
        );
        assert!(
            template_content
                .contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS=")
        );
        assert!(template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT="));
        assert!(
            template_content.contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY=")
        );
        assert!(
            template_content
                .contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS=")
        );
        assert!(
            template_content
                .contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES=")
        );
        assert!(
            template_content
                .contains("SDKWORK_IM_SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS=")
        );
        assert!(template_content.contains("SDKWORK_IM_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP="));
    }

    for env_name in [
        "SDKWORK_IM_BROWSER_ORIGINS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_REQUESTS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_WINDOW_SECONDS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_RATE_LIMIT_MAX_BUCKETS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_HTTP_TIMEOUT_MILLIS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_ENABLED",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_INTERVAL_MILLIS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_STALE_RECLAIM_SCHEDULER_JITTER_MILLIS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_DISPATCH_WORKER_COUNT",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_DISPATCH_QUEUE_CAPACITY",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_RETENTION_MILLIS",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_DELIVERED_LEDGER_MAX_ENTRIES",
        "SDKWORK_IM_SHARED_CHANNEL_SYNC_PENDING_RETRY_COOLDOWN_MILLIS",
        "SDKWORK_IM_ALLOW_INSECURE_SHARED_CHANNEL_SYNC_HTTP",
        "SDKWORK_IM_RUNTIME_PROFILE",
        "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE",
        "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET",
    ] {
        assert!(
            site_profiles_env_doc.contains(env_name),
            "docs/sites/deployment/profiles-and-env.md must document {env_name}"
        );
    }

    assert!(
        deployment_readme.contains("Profile"),
        "docs/部署/README.md must advertise the multi-profile/template contract doc"
    );
    assert!(
        readme.contains("local-default"),
        "README.md must surface local-default as part of the current deployment profile matrix"
    );
}

#[test]
fn test_security_and_audit_api_docs_cover_app_context_shared_sync_and_chain_verification_contracts()
{
    let root = workspace_root();
    let auth_and_errors_doc_path = root
        .join("docs")
        .join("sites")
        .join("api-reference")
        .join("auth-and-errors.md");
    let audit_doc_path = root
        .join("docs")
        .join("sites")
        .join("api-reference")
        .join("platform")
        .join("audit.md");
    let platform_schema_path = root
        .join("docs")
        .join("sites")
        .join(".vitepress")
        .join("theme")
        .join("api-schemas")
        .join("platform-business.ts");

    let auth_and_errors_doc = fs::read_to_string(&auth_and_errors_doc_path).unwrap_or_else(|_| {
        panic!(
            "missing auth and errors api doc: {}",
            auth_and_errors_doc_path.display()
        )
    });
    let audit_doc = fs::read_to_string(&audit_doc_path)
        .unwrap_or_else(|_| panic!("missing audit api doc: {}", audit_doc_path.display()));
    let platform_schema = fs::read_to_string(&platform_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing platform schema: {}",
            platform_schema_path.display()
        )
    });

    for token in [
        "Authorization: Bearer",
        "Access-Token",
        "private signed trusted-edge projection",
        "shared_channel_sync_permission_denied",
        "shared_channel_sync_actor_invalid",
        "shared_channel_sync_rate_limited",
        "conversation.shared_channel.sync",
    ] {
        assert!(
            auth_and_errors_doc.contains(token),
            "docs/sites/api-reference/auth-and-errors.md must document {token}"
        );
    }

    for forbidden_client_projection_header in [
        "x-sdkwork-tenant-id",
        "x-sdkwork-user-id",
        "x-sdkwork-session-id",
        "x-sdkwork-device-id",
        "x-sdkwork-actor-kind",
    ] {
        assert!(
            !auth_and_errors_doc.contains(forbidden_client_projection_header),
            "docs/sites/api-reference/auth-and-errors.md must not document client-controlled identity projection header `{forbidden_client_projection_header}`"
        );
    }

    for token in [
        "/backend/v3/api/audit/verify",
        "AuditChainVerification",
        "chainHeadHash",
        "chainValid",
    ] {
        assert!(
            audit_doc.contains(token),
            "docs/sites/api-reference/platform/audit.md must document {token}"
        );
    }

    for token in [
        "AuditChainVerification",
        "chainPrevHash",
        "chainHash",
        "chainHeadHash",
        "chainValid",
    ] {
        assert!(
            platform_schema.contains(token),
            "docs/sites/.vitepress/theme/api-schemas/platform-business.ts must include {token}"
        );
    }
}

#[test]
fn test_shell_lifecycle_scripts_use_args_based_process_identity_for_portability() {
    let root = workspace_root();

    for script_name in ["start-local.sh", "status-local.sh", "stop-local.sh"] {
        let script_path = root.join("bin").join(script_name);
        let script = fs::read_to_string(&script_path)
            .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));

        assert!(
            script.contains("ps -p \"$pid\" -o args="),
            "{script_name} must use ps args output so long process names stay portable across Bash environments"
        );
        assert!(
            script.contains("process_path=\"${process_name%% *}\""),
            "{script_name} must normalize the first argv token before basename extraction"
        );
        assert!(
            !script.contains("ps -p \"$pid\" -o comm="),
            "{script_name} must not rely on ps comm output because BSD/macOS may truncate command names"
        );
    }
}

#[test]
fn test_local_default_post_release_verification_samples_are_documented_and_archived() {
    let root = workspace_root();
    let deployment_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let deployment_readme_path = root.join("docs").join("部署").join("README.md");
    let readme_path = root.join("README.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");

    let deployment_doc = fs::read_to_string(&deployment_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", deployment_doc_path.display()));
    let deployment_readme = fs::read_to_string(&deployment_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment doc: {}",
            deployment_readme_path.display()
        )
    });
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });

    for expected in [
        "local-default",
        "post-release",
        "deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090",
        "deploy-local.sh --profile local-default --smoke-base-url http://127.0.0.1:28090",
        "status-local.ps1 -ProfileName local-default",
        "status-local.sh --profile local-default",
        "tools\\smoke\\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090",
        "tools/smoke/local_stack_smoke.sh --base-url http://127.0.0.1:28090",
        "open-chat-test.ps1",
        "inspect-runtime-local.ps1 -ProfileName local-default",
    ] {
        assert!(
            deployment_doc.contains(expected),
            "local-default发布后验证样本.md must document {expected}"
        );
    }

    assert!(
        deployment_doc.contains(
            "当前 `local-default` 仍复用 `local-minimal` 的 compose 服务合同与 smoke 链路"
        ),
        "local-default发布后验证样本.md must keep the current local-default boundary explicit"
    );
    assert!(
        deployment_readme.contains("local-default"),
        "docs/部署/README.md must advertise the local-default post-release verification samples doc"
    );
    assert!(
        readme.contains("local-default"),
        "README.md must surface the local-default post-release verification samples doc"
    );
    assert!(
        release_bundle_manifest.contains("docs/部署/local-default发布后验证样本.md"),
        "Wave D bundle manifest must reference the local-default post-release verification samples doc"
    );
}

#[test]
fn test_local_default_operator_execution_record_template_is_documented_and_archived() {
    let root = workspace_root();
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let deployment_readme_path = root.join("docs").join("部署").join("README.md");
    let readme_path = root.join("README.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");

    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let deployment_readme = fs::read_to_string(&deployment_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment doc: {}",
            deployment_readme_path.display()
        )
    });
    let readme = fs::read_to_string(&readme_path)
        .unwrap_or_else(|_| panic!("missing README: {}", readme_path.display()));
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });

    for expected in [
        "local-default",
        "执行记录模板",
        "验证窗口",
        "Go / No-Go",
        "Go / No-Go",
        "证据链接",
        "deploy-local.ps1 -ProfileName local-default -SmokeBaseUrl http://127.0.0.1:28090",
        "status-local.ps1 -ProfileName local-default",
        "tools\\smoke\\local_stack_smoke.ps1 -BaseUrl http://127.0.0.1:28090",
        "open-chat-test.ps1",
        "截图",
        "日志",
    ] {
        assert!(
            template_doc.contains(expected),
            "local-default发布后验证执行记录模板.md must document {expected}"
        );
    }

    assert!(
        sample_doc.contains("local-default发布后验证执行记录模板.md"),
        "local-default发布后验证样本.md must point operators to the execution record template"
    );
    assert!(
        deployment_readme.contains("local-default"),
        "docs/部署/README.md must advertise the local-default operator execution record template"
    );
    assert!(
        readme.contains("local-default"),
        "README.md must surface the local-default operator execution record template"
    );
    assert!(
        release_bundle_manifest.contains("docs/部署/local-default发布后验证执行记录模板.md"),
        "Wave D bundle manifest must reference the local-default operator execution record template"
    );
}

#[test]
fn test_local_default_release_bundle_contains_machine_readable_evidence_index() {
    let root = workspace_root();
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");

    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });

    assert_eq!(evidence_index_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(evidence_index_json["profile"], "local-default");
    assert_eq!(
        evidence_index_json["state"],
        "template_only_pending_collection"
    );
    assert_eq!(
        evidence_index_json["sampleDoc"],
        "docs/部署/local-default发布后验证样本.md"
    );
    assert_eq!(
        evidence_index_json["recordTemplate"],
        "docs/部署/local-default发布后验证执行记录模板.md"
    );
    assert!(
        evidence_index_json["boundary"]
            .as_str()
            .expect("boundary should be string")
            .contains("local-minimal"),
        "evidence index must keep the current local-default boundary explicit"
    );

    let slots = evidence_index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots should be an array");
    assert!(
        slots.len() >= 5,
        "evidenceSlots must freeze a useful minimum evidence set"
    );

    for expected_slot in [
        "deploy_local_ps1_log",
        "status_local_ps1_output",
        "local_stack_smoke_ps1_output",
        "open_chat_test_operator_record",
        "inspect_runtime_ps1_output",
    ] {
        assert!(
            slots.iter().any(|slot| slot["id"] == expected_slot),
            "evidenceSlots must contain slot {expected_slot}"
        );
    }

    assert!(
        sample_doc.contains("local-default-post-release-evidence-index.json"),
        "local-default发布后验证样本.md must point to the machine-readable evidence index"
    );
    assert!(
        template_doc.contains("local-default-post-release-evidence-index.json"),
        "local-default发布后验证执行记录模板.md must point to the machine-readable evidence index"
    );
    assert!(
        release_bundle_manifest.contains("local-default-post-release-evidence-index.json"),
        "Wave D bundle manifest must reference the machine-readable evidence index"
    );
}

#[test]
fn test_local_default_release_bundle_freezes_evidence_index_schema_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });

    assert_eq!(
        evidence_index_json["$schema"],
        "../schemas/post-release-evidence-index.schema.json"
    );
    assert_eq!(
        schema_json["title"],
        "sdkwork-im post-release evidence index"
    );
    assert_eq!(schema_json["type"], "object");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "post-release-evidence-index"
    );

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "wave",
        "profile",
        "artifact",
        "state",
        "boundary",
        "sampleDoc",
        "recordTemplate",
        "evidenceSlots",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "schema required fields must contain {field}"
        );
    }

    let slot_required = schema_json["properties"]["evidenceSlots"]["items"]["required"]
        .as_array()
        .expect("slot required fields should be an array");
    for field in ["id", "kind", "required", "status", "command"] {
        assert!(
            slot_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "slot schema required fields must contain {field}"
        );
    }

    assert!(
        release_bundle_manifest
            .contains("artifacts/releases/schemas/post-release-evidence-index.schema.json"),
        "Wave D bundle manifest must reference the evidence-index schema"
    );
    assert!(
        releases_readme
            .contains("artifacts/releases/schemas/post-release-evidence-index.schema.json"),
        "artifacts/releases/README.md must document the evidence-index schema path"
    );
}

#[test]
fn test_local_default_release_bundle_freezes_evidence_slot_collection_metadata_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));

    let slot_properties = &schema_json["properties"]["evidenceSlots"]["items"]["properties"];
    for field in ["artifactPath", "collectedAt", "checksumSha256"] {
        let field_schema = &slot_properties[field];
        assert!(
            field_schema.is_object(),
            "slot schema must define metadata field {field}"
        );
        assert!(
            field_schema["type"]
                .as_array()
                .expect("metadata field type should be an array")
                .iter()
                .any(|entry| entry.as_str() == Some("null")),
            "metadata field {field} must allow null for template-only pending collection"
        );
    }

    let slots = evidence_index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots should be an array");
    assert!(
        !slots.is_empty(),
        "evidenceSlots must stay populated when freezing metadata placeholders"
    );
    for slot in slots {
        assert!(
            slot.get("artifactPath").is_some(),
            "every slot must expose artifactPath placeholder"
        );
        assert!(
            slot.get("collectedAt").is_some(),
            "every slot must expose collectedAt placeholder"
        );
        assert!(
            slot.get("checksumSha256").is_some(),
            "every slot must expose checksumSha256 placeholder"
        );
        assert!(
            slot["artifactPath"].is_null(),
            "template-only slots must keep artifactPath null before collection"
        );
        assert!(
            slot["collectedAt"].is_null(),
            "template-only slots must keep collectedAt null before collection"
        );
        assert!(
            slot["checksumSha256"].is_null(),
            "template-only slots must keep checksumSha256 null before collection"
        );
    }

    for expected in ["artifactPath", "collectedAt", "checksumSha256"] {
        assert!(
            template_doc.contains(expected),
            "execution record template must document metadata field {expected}"
        );
        assert!(
            sample_doc.contains(expected),
            "verification sample must document metadata field {expected}"
        );
    }
}

#[test]
fn test_local_default_release_bundle_freezes_evidence_artifact_root_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    assert!(
        required
            .iter()
            .any(|entry| entry.as_str() == Some("artifactRoot")),
        "schema required fields must contain artifactRoot"
    );
    assert_eq!(schema_json["properties"]["artifactRoot"]["type"], "string");
    assert_eq!(
        evidence_index_json["artifactRoot"],
        "artifacts/releases/wave-d-2026-04-08/evidence/local-default"
    );
    assert!(
        artifact_root_readme.contains("artifactPath"),
        "artifact root readme must explain how artifactPath values resolve under the root"
    );
    assert!(
        artifact_root_readme.contains("template_only_pending_collection"),
        "artifact root readme must keep the template-only collection boundary explicit"
    );
    assert!(
        release_bundle_manifest
            .contains("artifacts/releases/wave-d-2026-04-08/evidence/local-default/README.md"),
        "Wave D bundle manifest must reference the artifact-root placeholder readme"
    );
    for expected in [
        "artifacts/releases/wave-d-2026-04-08/evidence/local-default",
        "artifactRoot",
    ] {
        assert!(
            sample_doc.contains(expected),
            "verification sample must document {expected}"
        );
        assert!(
            template_doc.contains(expected),
            "execution record template must document {expected}"
        );
    }
}

#[test]
fn test_local_default_release_bundle_freezes_evidence_slot_suggested_relative_path_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let suggested_path_schema =
        &schema_json["properties"]["evidenceSlots"]["items"]["properties"]["suggestedRelativePath"];
    assert!(
        suggested_path_schema.is_object(),
        "slot schema must define suggestedRelativePath"
    );
    assert_eq!(suggested_path_schema["type"], "string");

    let slots = evidence_index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots should be an array");
    for slot in slots {
        let suggested_path = slot["suggestedRelativePath"]
            .as_str()
            .expect("every slot must expose suggestedRelativePath as string");
        assert!(
            !suggested_path.is_empty(),
            "suggestedRelativePath must not be empty"
        );
        assert!(
            !suggested_path.contains('\\'),
            "suggestedRelativePath must use forward slashes"
        );
        assert!(
            !suggested_path.starts_with('/'),
            "suggestedRelativePath must stay relative to artifactRoot"
        );
    }

    for expected in ["suggestedRelativePath", "artifactRoot"] {
        assert!(
            artifact_root_readme.contains(expected),
            "artifact root readme must document {expected}"
        );
        assert!(
            sample_doc.contains(expected),
            "verification sample must document {expected}"
        );
        assert!(
            template_doc.contains(expected),
            "execution record template must document {expected}"
        );
    }
}

#[test]
fn test_local_default_release_bundle_freezes_evidence_slot_size_bytes_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let size_bytes_schema =
        &schema_json["properties"]["evidenceSlots"]["items"]["properties"]["sizeBytes"];
    assert!(
        size_bytes_schema.is_object(),
        "slot schema must define sizeBytes"
    );
    assert_eq!(
        size_bytes_schema["type"],
        serde_json::json!(["integer", "null"])
    );
    assert_eq!(size_bytes_schema["minimum"], 0);

    let slots = evidence_index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots should be an array");
    for slot in slots {
        assert!(
            slot.get("sizeBytes").is_some(),
            "every slot must expose sizeBytes"
        );
        assert!(
            slot["sizeBytes"].is_null(),
            "template-only slots must freeze sizeBytes as null"
        );
    }

    let expected = "sizeBytes";
    assert!(
        artifact_root_readme.contains(expected),
        "artifact root readme must document {expected}"
    );
    assert!(
        sample_doc.contains(expected),
        "verification sample must document {expected}"
    );
    assert!(
        template_doc.contains(expected),
        "execution record template must document {expected}"
    );
}

#[test]
fn test_local_default_release_bundle_freezes_checksum_manifest_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    assert!(
        required
            .iter()
            .any(|entry| entry.as_str() == Some("checksumManifestPath")),
        "schema required fields must contain checksumManifestPath"
    );
    assert_eq!(
        schema_json["properties"]["checksumManifestPath"]["type"],
        "string"
    );

    let checksum_manifest_path = evidence_index_json["checksumManifestPath"]
        .as_str()
        .expect("evidence index must expose checksumManifestPath");
    assert_eq!(
        checksum_manifest_path,
        "artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt"
    );
    assert!(
        checksum_manifest_path.starts_with(
            evidence_index_json["artifactRoot"]
                .as_str()
                .expect("evidence index must expose artifactRoot")
        ),
        "checksumManifestPath must stay under artifactRoot"
    );

    let checksum_manifest = fs::read_to_string(root.join(checksum_manifest_path))
        .unwrap_or_else(|_| panic!("missing checksum manifest: {}", checksum_manifest_path));
    for expected in [
        "template_only_pending_collection",
        "sha256:<digest>  <suggestedRelativePath>",
    ] {
        assert!(
            checksum_manifest.contains(expected),
            "checksum manifest placeholder must document {expected}"
        );
    }

    for expected in ["checksumManifestPath", "checksum-manifest.txt"] {
        assert!(
            artifact_root_readme.contains(expected),
            "artifact root readme must document {expected}"
        );
        assert!(
            sample_doc.contains(expected),
            "verification sample must document {expected}"
        );
        assert!(
            template_doc.contains(expected),
            "execution record template must document {expected}"
        );
    }
    assert!(
        release_bundle_manifest.contains(
            "artifacts/releases/wave-d-2026-04-08/evidence/local-default/checksum-manifest.txt"
        ),
        "Wave D bundle manifest must reference the checksum manifest placeholder"
    );
}

#[test]
fn test_local_default_release_bundle_freezes_artifact_file_list_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let release_bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let release_bundle_manifest =
        fs::read_to_string(&release_bundle_manifest_path).unwrap_or_else(|_| {
            panic!(
                "missing release bundle manifest: {}",
                release_bundle_manifest_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    assert!(
        required
            .iter()
            .any(|entry| entry.as_str() == Some("artifactFileListPath")),
        "schema required fields must contain artifactFileListPath"
    );
    assert_eq!(
        schema_json["properties"]["artifactFileListPath"]["type"],
        "string"
    );

    let artifact_file_list_path = evidence_index_json["artifactFileListPath"]
        .as_str()
        .expect("evidence index must expose artifactFileListPath");
    assert_eq!(
        artifact_file_list_path,
        "artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt"
    );
    assert!(
        artifact_file_list_path.starts_with(
            evidence_index_json["artifactRoot"]
                .as_str()
                .expect("evidence index must expose artifactRoot")
        ),
        "artifactFileListPath must stay under artifactRoot"
    );

    let artifact_file_list = fs::read_to_string(root.join(artifact_file_list_path))
        .unwrap_or_else(|_| panic!("missing artifact file list: {}", artifact_file_list_path));
    for expected in [
        "template_only_pending_collection",
        "deploy-local/deploy-local.ps1.log",
        "status-local/status-local.ps1.txt",
        "smoke/local_stack_smoke.ps1.txt",
        "open-chat-test/open-chat-test.ps1.md",
        "inspect-runtime/inspect-runtime-local.ps1.txt",
        "screenshots/runtime-window.png",
    ] {
        assert!(
            artifact_file_list.contains(expected),
            "artifact file list placeholder must document {expected}"
        );
    }

    for expected in ["artifactFileListPath", "artifact-file-list.txt"] {
        assert!(
            artifact_root_readme.contains(expected),
            "artifact root readme must document {expected}"
        );
        assert!(
            sample_doc.contains(expected),
            "verification sample must document {expected}"
        );
        assert!(
            template_doc.contains(expected),
            "execution record template must document {expected}"
        );
    }
    assert!(
        release_bundle_manifest.contains(
            "artifacts/releases/wave-d-2026-04-08/evidence/local-default/artifact-file-list.txt"
        ),
        "Wave D bundle manifest must reference the artifact file list placeholder"
    );
}

#[test]
fn test_local_default_release_bundle_freezes_collection_summary_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("post-release-evidence-index.schema.json");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    assert!(
        required
            .iter()
            .any(|entry| entry.as_str() == Some("collectionSummary")),
        "schema required fields must contain collectionSummary"
    );
    let collection_summary_schema = &schema_json["properties"]["collectionSummary"];
    assert!(
        collection_summary_schema.is_object(),
        "schema must define collectionSummary"
    );
    for field in [
        "totalSlots",
        "requiredSlots",
        "optionalSlots",
        "collectedSlots",
        "pendingSlots",
        "skippedOptionalSlots",
    ] {
        let field_schema = &collection_summary_schema["properties"][field];
        assert!(
            field_schema.is_object(),
            "collectionSummary must define {field}"
        );
        assert_eq!(field_schema["type"], "integer");
        assert_eq!(field_schema["minimum"], 0);
    }

    let collection_summary = &evidence_index_json["collectionSummary"];
    assert_eq!(collection_summary["totalSlots"], 6);
    assert_eq!(collection_summary["requiredSlots"], 5);
    assert_eq!(collection_summary["optionalSlots"], 1);
    assert_eq!(collection_summary["collectedSlots"], 0);
    assert_eq!(collection_summary["pendingSlots"], 6);
    assert_eq!(collection_summary["skippedOptionalSlots"], 0);

    let expected = "collectionSummary";
    assert!(
        artifact_root_readme.contains(expected),
        "artifact root readme must document {expected}"
    );
    assert!(
        sample_doc.contains(expected),
        "verification sample must document {expected}"
    );
    assert!(
        template_doc.contains(expected),
        "execution record template must document {expected}"
    );
}

#[test]
fn test_local_default_release_bundle_collection_summary_matches_slot_statuses() {
    let root = workspace_root();
    let release_readme_path = root.join("artifacts").join("releases").join("README.md");
    let evidence_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("local-default-post-release-evidence-index.json");
    let artifact_root_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("evidence")
        .join("local-default")
        .join("README.md");
    let sample_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证样本.md");
    let template_doc_path = root
        .join("docs")
        .join("部署")
        .join("local-default发布后验证执行记录模板.md");

    let release_readme = fs::read_to_string(&release_readme_path)
        .unwrap_or_else(|_| panic!("missing release doc: {}", release_readme_path.display()));
    let evidence_index = fs::read_to_string(&evidence_index_path).unwrap_or_else(|_| {
        panic!(
            "missing release artifact: {}",
            evidence_index_path.display()
        )
    });
    let evidence_index_json: serde_json::Value = serde_json::from_str(&evidence_index)
        .unwrap_or_else(|_| {
            panic!(
                "invalid json evidence index: {}",
                evidence_index_path.display()
            )
        });
    let artifact_root_readme =
        fs::read_to_string(&artifact_root_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing artifact root readme: {}",
                artifact_root_readme_path.display()
            )
        });
    let sample_doc = fs::read_to_string(&sample_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", sample_doc_path.display()));
    let template_doc = fs::read_to_string(&template_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", template_doc_path.display()));

    let slots = evidence_index_json["evidenceSlots"]
        .as_array()
        .expect("evidenceSlots should be an array");
    let summary = &evidence_index_json["collectionSummary"];

    let total_slots = slots.len() as i64;
    let required_slots = slots.iter().filter(|slot| slot["required"] == true).count() as i64;
    let optional_slots = slots
        .iter()
        .filter(|slot| slot["required"] == false)
        .count() as i64;
    let collected_slots = slots
        .iter()
        .filter(|slot| slot["status"] == "collected")
        .count() as i64;
    let pending_slots = slots
        .iter()
        .filter(|slot| slot["status"] == "pending_collection")
        .count() as i64;
    let skipped_optional_slots = slots
        .iter()
        .filter(|slot| slot["status"] == "skipped_optional")
        .count() as i64;

    assert_eq!(summary["totalSlots"], total_slots);
    assert_eq!(summary["requiredSlots"], required_slots);
    assert_eq!(summary["optionalSlots"], optional_slots);
    assert_eq!(summary["collectedSlots"], collected_slots);
    assert_eq!(summary["pendingSlots"], pending_slots);
    assert_eq!(summary["skippedOptionalSlots"], skipped_optional_slots);

    for doc in [
        &release_readme,
        &artifact_root_readme,
        &sample_doc,
        &template_doc,
    ] {
        assert!(
            doc.contains("collectionSummary"),
            "collection summary consistency docs must mention collectionSummary"
        );
        assert!(
            doc.contains("evidenceSlots"),
            "collection summary consistency docs must mention evidenceSlots as the source of truth"
        );
        assert!(
            doc.contains("status"),
            "collection summary consistency docs must mention slot status consistency"
        );
    }
}

#[test]
fn test_deploy_local_scripts_expose_profile_selection_contract() {
    let root = workspace_root();
    let deploy_ps1_path = root.join("bin").join("deploy-local.ps1");
    let deploy_sh_path = root.join("bin").join("deploy-local.sh");
    let bootstrap_path = root
        .join("deployments")
        .join("scripts")
        .join("bootstrap-local.ps1");

    let deploy_ps1 = fs::read_to_string(&deploy_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", deploy_ps1_path.display()));
    let deploy_sh = fs::read_to_string(&deploy_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", deploy_sh_path.display()));
    let bootstrap = fs::read_to_string(&bootstrap_path)
        .unwrap_or_else(|_| panic!("missing bootstrap script: {}", bootstrap_path.display()));

    assert!(
        deploy_ps1.contains("ProfileName"),
        "deploy-local.ps1 must expose a profile selector for local-minimal/local-default"
    );
    assert!(
        deploy_ps1.contains("local-default"),
        "deploy-local.ps1 must document local-default as a supported deployment profile"
    );
    assert!(
        deploy_sh.contains("--profile"),
        "deploy-local.sh must expose a --profile selector for deployment profile choice"
    );
    assert!(
        deploy_sh.contains("local-default"),
        "deploy-local.sh must document local-default as a supported deployment profile"
    );
    assert!(
        bootstrap.contains("ProfileName"),
        "bootstrap-local.ps1 must accept a forwarded deployment profile selector"
    );
}

#[test]
fn test_deploy_local_scripts_expose_repeatable_smoke_base_url_contract() {
    let root = workspace_root();
    let deploy_ps1_path = root.join("bin").join("deploy-local.ps1");
    let deploy_sh_path = root.join("bin").join("deploy-local.sh");
    let bootstrap_path = root
        .join("deployments")
        .join("scripts")
        .join("bootstrap-local.ps1");

    let deploy_ps1 = fs::read_to_string(&deploy_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", deploy_ps1_path.display()));
    let deploy_sh = fs::read_to_string(&deploy_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", deploy_sh_path.display()));
    let bootstrap = fs::read_to_string(&bootstrap_path)
        .unwrap_or_else(|_| panic!("missing bootstrap script: {}", bootstrap_path.display()));

    assert!(
        deploy_ps1.contains("SmokeBaseUrl"),
        "deploy-local.ps1 must expose a smoke base-url override for repeatable smoke verification"
    );
    assert!(
        deploy_ps1.contains("-SmokeBaseUrl <url>"),
        "deploy-local.ps1 help must document the smoke base-url override"
    );
    assert!(
        deploy_sh.contains("--smoke-base-url"),
        "deploy-local.sh must expose a smoke base-url override for repeatable smoke verification"
    );
    assert!(
        bootstrap.contains("SmokeBaseUrl"),
        "bootstrap-local.ps1 must accept a forwarded smoke base-url override"
    );
}

#[cfg(windows)]
#[tokio::test]
async fn test_local_stack_smoke_ps1_executes_against_public_app_with_app_context_projection() {
    let root = workspace_root();
    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;
    let root_for_command = root.clone();
    let base_url_for_command = base_url.clone();

    let output = tokio::time::timeout(
        tokio::time::Duration::from_secs(30),
        tokio::task::spawn_blocking(move || {
            Command::new("powershell")
                .current_dir(&root_for_command)
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    "tools\\smoke\\local_stack_smoke.ps1",
                    "-BaseUrl",
                    base_url_for_command.as_str(),
                ])
                .output()
        }),
    )
    .await
    .expect("local_stack_smoke.ps1 should finish before timeout")
    .expect("local_stack_smoke.ps1 should execute task")
    .expect("local_stack_smoke.ps1 should execute");

    handle.abort();
    let _ = handle.await;

    assert!(
        output.status.success(),
        "local_stack_smoke.ps1 should succeed against build_public_app. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("local stack smoke check passed."),
        "local_stack_smoke.ps1 should report a successful smoke run"
    );
}

#[tokio::test]
async fn test_local_stack_smoke_sh_executes_against_public_app_with_app_context_projection() {
    let root = workspace_root();
    let app = local_minimal_node::build_public_app();
    let (base_url, handle) = spawn_server(app).await;

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping local_stack_smoke.sh execution regression because no usable bash runtime is available"
        );
        handle.abort();
        let _ = handle.await;
        return;
    };
    let root_for_command = root.clone();
    let base_url_for_command = base_url.clone();

    let output = tokio::time::timeout(
        tokio::time::Duration::from_secs(30),
        tokio::task::spawn_blocking(move || {
            Command::new(&bash_path)
                .current_dir(&root_for_command)
                .args([
                    "tools/smoke/local_stack_smoke.sh",
                    "--base-url",
                    base_url_for_command.as_str(),
                ])
                .output()
        }),
    )
    .await
    .expect("local_stack_smoke.sh should finish before timeout")
    .expect("local_stack_smoke.sh should execute task")
    .expect("local_stack_smoke.sh should execute");

    handle.abort();
    let _ = handle.await;

    assert!(
        output.status.success(),
        "local_stack_smoke.sh should succeed against build_public_app. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("local stack smoke check passed."),
        "local_stack_smoke.sh should report a successful smoke run"
    );
}

#[cfg(windows)]
#[tokio::test]
async fn test_local_stack_smoke_ps1_executes_against_dual_token_service() {
    let root = workspace_root();
    let app = dual_token_smoke_app();
    let (base_url, handle) = spawn_server(app).await;
    let root_for_command = root.clone();
    let base_url_for_command = base_url.clone();

    let output = tokio::time::timeout(
        tokio::time::Duration::from_secs(30),
        tokio::task::spawn_blocking(move || {
            Command::new("powershell")
                .current_dir(&root_for_command)
                .args([
                    "-NoProfile",
                    "-ExecutionPolicy",
                    "Bypass",
                    "-File",
                    "tools\\smoke\\local_stack_smoke.ps1",
                    "-BaseUrl",
                    base_url_for_command.as_str(),
                ])
                .output()
        }),
    )
    .await
    .expect("dual-token local_stack_smoke.ps1 should finish before timeout")
    .expect("dual-token local_stack_smoke.ps1 should execute task")
    .expect("dual-token local_stack_smoke.ps1 should execute");

    handle.abort();
    let _ = handle.await;

    assert!(
        output.status.success(),
        "local_stack_smoke.ps1 should send dual-token headers accepted by the service. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("local stack smoke check passed."),
        "dual-token local_stack_smoke.ps1 should report a successful smoke run"
    );
}

#[tokio::test]
async fn test_local_stack_smoke_sh_executes_against_dual_token_service() {
    let root = workspace_root();
    let app = dual_token_smoke_app();
    let (base_url, handle) = spawn_server(app).await;

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping dual-token local_stack_smoke.sh execution regression because no usable bash runtime is available"
        );
        handle.abort();
        let _ = handle.await;
        return;
    };
    let root_for_command = root.clone();
    let base_url_for_command = base_url.clone();

    let output = tokio::time::timeout(
        tokio::time::Duration::from_secs(30),
        tokio::task::spawn_blocking(move || {
            Command::new(&bash_path)
                .current_dir(&root_for_command)
                .args([
                    "tools/smoke/local_stack_smoke.sh",
                    "--base-url",
                    base_url_for_command.as_str(),
                ])
                .output()
        }),
    )
    .await
    .expect("dual-token local_stack_smoke.sh should finish before timeout")
    .expect("dual-token local_stack_smoke.sh should execute task")
    .expect("dual-token local_stack_smoke.sh should execute");

    handle.abort();
    let _ = handle.await;

    assert!(
        output.status.success(),
        "local_stack_smoke.sh should send dual-token headers accepted by the service. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        String::from_utf8_lossy(&output.stdout).contains("local stack smoke check passed."),
        "dual-token local_stack_smoke.sh should report a successful smoke run"
    );
}

#[test]
fn test_local_minimal_compose_injects_only_domain_cursor_secret_for_public_smoke_contract() {
    let root = workspace_root();
    let compose_path = root
        .join("deployments")
        .join("docker-compose")
        .join("local-minimal.yml");
    let compose = fs::read_to_string(&compose_path)
        .unwrap_or_else(|_| panic!("missing compose profile: {}", compose_path.display()));

    assert!(
        !compose.contains("SDKWORK_IM_PUBLIC_BEARER"),
        "local-minimal.yml must not configure sdkwork-im IAM/Public Bearer secrets after AppContext integration"
    );
    assert!(
        compose.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"),
        "local-minimal.yml must inject SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET so friend request cursors stay stable across restarts and replicas"
    );
}

#[test]
fn test_local_stack_smoke_scripts_require_dual_token_contract() {
    let root = workspace_root();
    let smoke_ps1_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.ps1");
    let smoke_sh_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.sh");

    let smoke_ps1 = fs::read_to_string(&smoke_ps1_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_ps1_path.display()));
    let smoke_sh = fs::read_to_string(&smoke_sh_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_sh_path.display()));

    for script in [&smoke_ps1, &smoke_sh] {
        assert!(
            !script.contains("eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0."),
            "local stack smoke scripts must not embed alg=none bearer tokens"
        );
        assert!(!script.contains("SDKWORK_IM_PUBLIC_BEARER"));
        assert!(script.contains("Authorization"));
        assert!(script.contains("Access-Token"));
        assert!(!script.contains("x-sdkwork-tenant-id"));
        assert!(!script.contains("x-sdkwork-user-id"));
        assert!(!script.contains("x-sdkwork-session-id"));
        assert!(!script.contains("x-sdkwork-device-id"));
    }
}

#[test]
fn test_local_minimal_install_doc_describes_dual_token_client_contract() {
    let root = workspace_root();
    let install_doc_path = root.join("docs").join("部署").join("本地最小安装与运行.md");
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", install_doc_path.display()));

    assert!(
        !install_doc.contains("SDKWORK_IM_PUBLIC_BEARER"),
        "本地最小安装与运行.md must not document sdkwork-im-owned Public Bearer secrets"
    );
    assert!(
        install_doc.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"),
        "install doc must document the friend request cursor signing secret contract"
    );
    assert!(
        install_doc.contains("Authorization: Bearer"),
        "本地最小安装与运行.md must describe sdkwork AppContext tenant projection"
    );
    assert!(
        install_doc.contains("Access-Token"),
        "本地最小安装与运行.md must describe sdkwork AppContext user projection"
    );
    assert!(
        !install_doc.contains("x-sdkwork-tenant-id"),
        "install doc must not document client-controlled tenant projection headers"
    );
    assert!(
        !install_doc.contains("x-sdkwork-user-id"),
        "install doc must not document client-controlled user projection headers"
    );
    assert!(
        !install_doc.contains("x-sdkwork-session-id"),
        "install doc must not document client-controlled session projection headers"
    );
    assert!(
        !install_doc.contains("x-sdkwork-device-id"),
        "install doc must not document client-controlled device projection headers"
    );
    assert!(
        !install_doc.contains("x-sdkwork-actor-kind"),
        "install doc must not document client-controlled actor projection headers"
    );
    assert!(
        !install_doc.contains("alg=none"),
        "本地最小安装与运行.md must not claim that local-minimal skips bearer signature verification"
    );
    assert!(
        !install_doc.contains("eyJhbGciOiJub25lIiwidHlwIjoiSldUIn0."),
        "本地最小安装与运行.md must not document alg=none bearer examples for local-minimal"
    );
}

#[test]
fn test_runtime_operation_scripts_expose_profile_selection_contract() {
    let root = workspace_root();
    let quick_start_doc_path = root.join("docs").join("部署").join("快速启动脚本.md");
    let quick_start_doc = fs::read_to_string(&quick_start_doc_path)
        .unwrap_or_else(|_| panic!("missing deployment doc: {}", quick_start_doc_path.display()));

    for script_name in [
        "status-local.ps1",
        "inspect-runtime-local.ps1",
        "repair-runtime-local.ps1",
        "list-runtime-backups-local.ps1",
        "archive-runtime-backup-local.ps1",
        "prune-runtime-archives-local.ps1",
        "preview-runtime-restore-local.ps1",
        "restore-runtime-local.ps1",
    ] {
        let script_path = root.join("bin").join(script_name);
        let script = fs::read_to_string(&script_path)
            .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));
        assert!(
            script.contains("ProfileName"),
            "{script_name} must expose a profile selector so runtime operations can target local-minimal/local-default consistently"
        );
        assert!(
            script.contains("local-default"),
            "{script_name} must document local-default as a supported runtime operations profile"
        );
    }

    for script_name in [
        "status-local.sh",
        "inspect-runtime-local.sh",
        "repair-runtime-local.sh",
        "list-runtime-backups-local.sh",
        "archive-runtime-backup-local.sh",
        "prune-runtime-archives-local.sh",
        "preview-runtime-restore-local.sh",
        "restore-runtime-local.sh",
    ] {
        let script_path = root.join("bin").join(script_name);
        let script = fs::read_to_string(&script_path)
            .unwrap_or_else(|_| panic!("missing bin script: {}", script_path.display()));
        assert!(
            script.contains("--profile"),
            "{script_name} must expose a --profile selector so runtime operations can target local-minimal/local-default consistently"
        );
        assert!(
            script.contains("local-default"),
            "{script_name} must document local-default as a supported runtime operations profile"
        );
    }

    assert!(
        quick_start_doc.contains(
            "运行时运维脚本同样支持 `--profile <local-minimal|local-default>` / `-ProfileName <local-minimal|local-default>`"
        ),
        "快速启动脚本.md must document the runtime operations profile-selection contract"
    );
}

fn resolve_usable_bash() -> Option<PathBuf> {
    let mut candidates = Vec::new();
    #[cfg(windows)]
    {
        candidates.push(PathBuf::from(r"C:\Program Files\Git\bin\bash.exe"));
        candidates.push(PathBuf::from(r"C:\Program Files\Git\usr\bin\bash.exe"));
    }
    candidates.push(PathBuf::from("bash"));

    candidates.into_iter().find(|candidate| {
        Command::new(candidate)
            .arg("--version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    })
}

fn parse_captured_cli_calls(content: &str) -> Vec<Vec<String>> {
    content
        .split("__CALL__")
        .filter_map(|chunk| {
            let call = chunk
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>();
            (!call.is_empty()).then_some(call)
        })
        .collect()
}

#[cfg(windows)]
fn windows_system_executable(file_name: &str) -> PathBuf {
    let windows_dir = std::env::var_os("WINDIR")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from(r"C:\Windows"));
    windows_dir.join("System32").join(file_name)
}

#[test]
fn test_restart_local_sh_propagates_stop_failure_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_sh_stop_failure_behavior");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.sh"),
        bin_dir.join("restart-local.sh"),
    )
    .expect("restart-local.sh should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\necho \"simulated stop failure\" >&2\nexit 17\n",
    )
    .expect("stub stop-local.sh should be written");
    fs::write(
        bin_dir.join("start-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nROOT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")/..\" && pwd)\"\ntouch \"${ROOT_DIR}/start-invoked.marker\"\n",
    )
    .expect("stub start-local.sh should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping restart-local.sh behavior regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("bin/restart-local.sh")
        .output()
        .expect("restart-local.sh should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.sh should fail when stop-local.sh exits non-zero"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.sh must not invoke start-local.sh after stop-local.sh fails"
    );

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("simulated stop failure"),
        "restart-local.sh should surface stop-local.sh stderr on failure. actual stderr: {stderr}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_restart_local_sh_forwards_profile_selection_to_stop_and_start_scripts() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_sh_profile_forward");
    let bin_dir = temp_root.join("bin");
    let stop_args_path = temp_root.join("stop-args.txt");
    let start_args_path = temp_root.join("start-args.txt");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.sh"),
        bin_dir.join("restart-local.sh"),
    )
    .expect("restart-local.sh should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nROOT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")/..\" && pwd)\"\nprintf '%s\\n' \"$@\" > \"${ROOT_DIR}/stop-args.txt\"\n",
    )
    .expect("stub stop-local.sh should be written");
    fs::write(
        bin_dir.join("start-local.sh"),
        "#!/usr/bin/env bash\nset -euo pipefail\nROOT_DIR=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")/..\" && pwd)\"\nprintf '%s\\n' \"$@\" > \"${ROOT_DIR}/start-args.txt\"\n",
    )
    .expect("stub start-local.sh should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping restart-local.sh profile forwarding regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .args([
            "bin/restart-local.sh",
            "--profile",
            "local-default",
            "--release",
            "--foreground",
            "--bind-addr",
            "127.0.0.1:19190",
        ])
        .output()
        .expect("restart-local.sh should execute in temp workspace");
    assert!(
        output.status.success(),
        "restart-local.sh should forward profile-aware lifecycle flags. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stop_args: Vec<_> = fs::read_to_string(&stop_args_path)
        .unwrap_or_else(|_| panic!("missing stop args file: {}", stop_args_path.display()))
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        stop_args,
        vec!["--profile".to_string(), "local-default".to_string()],
        "restart-local.sh must pass only the selected runtime profile to stop-local.sh"
    );

    let start_args: Vec<_> = fs::read_to_string(&start_args_path)
        .unwrap_or_else(|_| panic!("missing start args file: {}", start_args_path.display()))
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        start_args,
        vec![
            "--profile".to_string(),
            "local-default".to_string(),
            "--release".to_string(),
            "--foreground".to_string(),
            "--bind-addr".to_string(),
            "127.0.0.1:19190".to_string()
        ],
        "restart-local.sh must forward profile-aware startup flags to start-local.sh in order"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restart_local_ps1_propagates_terminating_stop_failure_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_ps1_stop_throw");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.ps1"),
        bin_dir.join("restart-local.ps1"),
    )
    .expect("restart-local.ps1 should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.ps1"),
        "Write-Host 'stub stop'\r\nthrow 'simulated terminating stop failure'\r\n",
    )
    .expect("stub stop-local.ps1 should be written");
    fs::write(
        bin_dir.join("start-local.ps1"),
        "$root = Split-Path -Parent $PSScriptRoot\r\nNew-Item -ItemType File -Force -Path (Join-Path $root 'start-invoked.marker') | Out-Null\r\nWrite-Host 'stub start'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\restart-local.ps1",
        ])
        .output()
        .expect("restart-local.ps1 should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.ps1 should fail when stop-local.ps1 throws"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.ps1 must not invoke start-local.ps1 after a terminating stop-local.ps1 failure"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        combined.contains("simulated terminating stop failure"),
        "restart-local.ps1 should surface stop-local.ps1 terminating failure details. actual output: {combined}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restart_local_ps1_forwards_profile_name_to_stop_and_start_scripts() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_ps1_profile_forward");
    let bin_dir = temp_root.join("bin");
    let stop_profile_path = temp_root.join("stop-profile.txt");
    let start_args_path = temp_root.join("start-args.txt");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.ps1"),
        bin_dir.join("restart-local.ps1"),
    )
    .expect("restart-local.ps1 should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.ps1"),
        "param([string]$ProfileName = 'local-minimal')\r\n$root = Split-Path -Parent $PSScriptRoot\r\nSet-Content -Path (Join-Path $root 'stop-profile.txt') -Value $ProfileName\r\nWrite-Host 'stub stop'\r\n",
    )
    .expect("stub stop-local.ps1 should be written");
    fs::write(
        bin_dir.join("start-local.ps1"),
        "param([string]$ProfileName = 'local-minimal', [switch]$Release, [switch]$Foreground, [string]$BindAddress)\r\n$root = Split-Path -Parent $PSScriptRoot\r\n@(\"ProfileName=$ProfileName\", \"Release=$Release\", \"Foreground=$Foreground\", \"BindAddress=$BindAddress\") | Set-Content -Path (Join-Path $root 'start-args.txt')\r\nWrite-Host 'stub start'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\restart-local.ps1",
            "-ProfileName",
            "local-default",
            "-Release",
            "-Foreground",
            "-BindAddress",
            "127.0.0.1:19190",
        ])
        .output()
        .expect("restart-local.ps1 should execute in temp workspace");
    assert!(
        output.status.success(),
        "restart-local.ps1 should forward profile-aware lifecycle flags. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stop_profile = fs::read_to_string(&stop_profile_path)
        .unwrap_or_else(|_| panic!("missing stop profile file: {}", stop_profile_path.display()));
    assert_eq!(
        stop_profile.trim(),
        "local-default",
        "restart-local.ps1 must pass the selected runtime profile to stop-local.ps1"
    );

    let start_args: Vec<_> = fs::read_to_string(&start_args_path)
        .unwrap_or_else(|_| panic!("missing start args file: {}", start_args_path.display()))
        .lines()
        .map(str::to_owned)
        .collect();
    assert_eq!(
        start_args,
        vec![
            "ProfileName=local-default".to_string(),
            "Release=True".to_string(),
            "Foreground=True".to_string(),
            "BindAddress=127.0.0.1:19190".to_string()
        ],
        "restart-local.ps1 must pass profile-aware startup flags to start-local.ps1 in order"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restart_local_ps1_propagates_non_zero_stop_exit_before_starting_new_instance() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restart_ps1_stop_exit");
    let bin_dir = temp_root.join("bin");
    let start_marker = temp_root.join("start-invoked.marker");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    fs::copy(
        root.join("bin").join("restart-local.ps1"),
        bin_dir.join("restart-local.ps1"),
    )
    .expect("restart-local.ps1 should be copied into temp workspace");

    fs::write(
        bin_dir.join("stop-local.ps1"),
        "Write-Host 'stub stop'\r\nexit 9\r\n",
    )
    .expect("stub stop-local.ps1 should be written");
    fs::write(
        bin_dir.join("start-local.ps1"),
        "$root = Split-Path -Parent $PSScriptRoot\r\nNew-Item -ItemType File -Force -Path (Join-Path $root 'start-invoked.marker') | Out-Null\r\nWrite-Host 'stub start'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\restart-local.ps1",
        ])
        .output()
        .expect("restart-local.ps1 should execute in temp workspace");

    assert!(
        !output.status.success(),
        "restart-local.ps1 should fail when stop-local.ps1 exits non-zero"
    );
    assert!(
        !start_marker.exists(),
        "restart-local.ps1 must not invoke start-local.ps1 after a non-zero stop-local.ps1 exit"
    );
    assert_eq!(
        output.status.code(),
        Some(9),
        "restart-local.ps1 should preserve the stop-local.ps1 exit code"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("stub stop"),
        "restart-local.ps1 should surface stop-local.ps1 output before exiting. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_stop_local_ps1_does_not_kill_unmanaged_process_from_stale_pid_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("stop_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    fs::copy(
        root.join("bin").join("stop-local.ps1"),
        bin_dir.join("stop-local.ps1"),
    )
    .expect("stop-local.ps1 should be copied into temp workspace");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\stop-local.ps1",
        ])
        .output()
        .expect("stop-local.ps1 should execute in temp workspace");

    assert!(
        output.status.success(),
        "stop-local.ps1 should treat an unmanaged process PID as stale metadata"
    );
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "stop-local.ps1 must not kill an unrelated process that reused or occupied the pid file"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "stop-local.ps1 should remove a stale pid file that points to an unmanaged process"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("local-minimal-node is not running."),
        "stop-local.ps1 should normalize unmanaged pid-file targets as not running. actual stdout: {stdout}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_status_local_ps1_treats_unmanaged_process_from_stale_pid_file_as_stopped() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    fs::copy(
        root.join("bin").join("status-local.ps1"),
        bin_dir.join("status-local.ps1"),
    )
    .expect("status-local.ps1 should be copied into temp workspace");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\status-local.ps1",
        ])
        .output()
        .expect("status-local.ps1 should execute in temp workspace");

    assert!(output.status.success(), "status-local.ps1 should succeed");
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "status-local.ps1 must not disturb an unrelated process from a stale pid file"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "status-local.ps1 should remove a stale pid file that points to an unmanaged process"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("status: stopped"),
        "status-local.ps1 should treat an unmanaged pid-file target as stopped. actual stdout: {stdout}"
    );
    assert!(
        !stdout.contains("status: running"),
        "status-local.ps1 must not report an unmanaged pid-file target as running. actual stdout: {stdout}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_status_local_ps1_uses_local_default_profile_config_when_requested() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_ps1_profile");
    let bin_dir = temp_root.join("bin");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_minimal_config_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");
    let local_minimal_runtime_dir = temp_root.join("runtime-from-local-minimal");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");
    fs::create_dir_all(&local_minimal_config_dir).expect("local-minimal config dir should exist");

    for file_name in ["status-local.ps1", "_runtime-profile-common.ps1"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\nSDKWORK_IM_BIND_ADDR=127.0.0.1:19090\r\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");
    fs::write(
        local_minimal_config_dir.join("local-minimal.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\nSDKWORK_IM_BIND_ADDR=127.0.0.1:18090\r\n",
            local_minimal_runtime_dir.display()
        ),
    )
    .expect("local-minimal config should be written");

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\status-local.ps1",
            "-ProfileName",
            "local-default",
        ])
        .output()
        .expect("status-local.ps1 should execute in temp workspace");
    assert!(
        output.status.success(),
        "status-local.ps1 should support profile selection. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let expected_config_path = local_default_config_dir.join("local-default.env");
    let expected_stdout_log = local_default_runtime_dir
        .join("logs")
        .join("local-minimal-node.out.log");
    let expected_stderr_log = local_default_runtime_dir
        .join("logs")
        .join("local-minimal-node.err.log");
    assert!(
        stdout.contains(expected_config_path.to_string_lossy().as_ref()),
        "status-local.ps1 must report the selected local-default config path. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("bind: 127.0.0.1:19090"),
        "status-local.ps1 must report the bind address from the selected local-default profile. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(expected_stdout_log.to_string_lossy().as_ref()),
        "status-local.ps1 must resolve log paths from the selected local-default runtime dir. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(expected_stderr_log.to_string_lossy().as_ref()),
        "status-local.ps1 must resolve stderr log paths from the selected local-default runtime dir. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("health: http://127.0.0.1:19090/healthz"),
        "status-local.ps1 must derive healthz url from the selected local-default bind address. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("status: stopped"),
        "status-local.ps1 should still report a stopped profile when no managed pid exists. actual stdout: {stdout}"
    );
    assert!(
        !stdout.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()),
        "status-local.ps1 must not fall back to the local-minimal runtime dir when a local-default config exists. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_status_local_cmd_supports_profile_switch() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_cmd_profile");
    let bin_dir = temp_root.join("bin");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");

    for file_name in [
        "status-local.ps1",
        "status-local.cmd",
        "_cmd-forward-powershell.cmd",
        "_runtime-profile-common.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\nSDKWORK_IM_BIND_ADDR=127.0.0.1:19090\r\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");

    let output = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\status-local.cmd", "--profile", "local-default"])
        .output()
        .expect("status-local.cmd should execute");
    assert!(
        output.status.success(),
        "status-local.cmd should normalize --profile to the PowerShell profile selector. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(local_default_runtime_dir.to_string_lossy().as_ref()),
        "status-local.cmd must forward the selected local-default runtime dir through the underlying PowerShell script. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("bind: 127.0.0.1:19090"),
        "status-local.cmd must forward the selected local-default bind address through the underlying PowerShell script. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_status_local_sh_uses_local_default_profile_config_when_requested() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_sh_profile");
    let bin_dir = temp_root.join("bin");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_minimal_config_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");
    let local_minimal_runtime_dir = temp_root.join("runtime-from-local-minimal");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");
    fs::create_dir_all(&local_minimal_config_dir).expect("local-minimal config dir should exist");

    for file_name in ["status-local.sh", "_runtime-profile-common.sh"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\nSDKWORK_IM_BIND_ADDR=127.0.0.1:19090\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");
    fs::write(
        local_minimal_config_dir.join("local-minimal.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\nSDKWORK_IM_BIND_ADDR=127.0.0.1:18090\n",
            local_minimal_runtime_dir.display()
        ),
    )
    .expect("local-minimal config should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping status-local.sh profile regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .args(["bin/status-local.sh", "--profile", "local-default"])
        .output()
        .expect("status-local.sh should execute");
    assert!(
        output.status.success(),
        "status-local.sh should support profile selection. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(local_default_runtime_dir.to_string_lossy().as_ref()),
        "status-local.sh must resolve paths from the selected local-default runtime dir. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("bind: 127.0.0.1:19090"),
        "status-local.sh must report the bind address from the selected local-default profile. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains("health: http://127.0.0.1:19090/healthz"),
        "status-local.sh must derive healthz url from the selected local-default bind address. actual stdout: {stdout}"
    );
    assert!(
        !stdout.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()),
        "status-local.sh must not fall back to the local-minimal runtime dir when a local-default config exists. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_ignores_unmanaged_process_from_stale_pid_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_unmanaged_pid");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");
    let pid_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");
    fs::create_dir_all(&pid_dir).expect("temp pid dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let mut unrelated_process = Command::new("powershell")
        .args(["-NoProfile", "-Command", "Start-Sleep -Seconds 30"])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("unrelated powershell process should start");

    fs::write(
        pid_dir.join("local-minimal-node.pid"),
        unrelated_process.id().to_string(),
    )
    .expect("pid file should be written");

    let whoami_path = windows_system_executable("whoami.exe");
    fs::copy(
        &whoami_path,
        target_debug_dir.join("local-minimal-node.exe"),
    )
    .unwrap_or_else(|_| {
        panic!(
            "failed to copy fake node binary from {}",
            whoami_path.display()
        )
    });

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .output()
        .expect("start-local.ps1 should execute against temp workspace");

    assert!(
        !output.status.success(),
        "fake binary should still fail readiness after stale unmanaged pid metadata is ignored"
    );
    assert!(
        unrelated_process
            .try_wait()
            .expect("unrelated process wait state should be readable")
            .is_none(),
        "start-local.ps1 must not treat an unrelated process from the pid file as a managed running instance"
    );
    assert!(
        !pid_dir.join("local-minimal-node.pid").exists(),
        "start-local.ps1 should clear stale unmanaged pid metadata during failed startup"
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    let combined = format!("{stdout}\n{stderr}");
    assert!(
        !combined.contains("already running"),
        "start-local.ps1 must not reject startup because of an unrelated process in the stale pid file. actual output: {combined}"
    );
    assert!(
        combined.contains("Starting local-minimal-node in background"),
        "start-local.ps1 should proceed to launch after clearing the unmanaged pid file. actual output: {combined}"
    );

    let _ = unrelated_process.kill();
    let _ = unrelated_process.wait();
    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_local_minimal_deployment_assets_exist_and_reference_expected_entrypoints() {
    let root = workspace_root();
    let dockerfile_path = root
        .join("deployments")
        .join("docker")
        .join("local-minimal.Dockerfile");
    let compose_path = root
        .join("deployments")
        .join("docker-compose")
        .join("local-minimal.yml");
    let bootstrap_path = root
        .join("deployments")
        .join("scripts")
        .join("bootstrap-local.ps1");
    let bin_install_ps1_path = root.join("bin").join("install-local.ps1");
    let bin_install_sh_path = root.join("bin").join("install-local.sh");
    let bin_install_cmd_path = root.join("bin").join("install-local.cmd");
    let bin_deploy_ps1_path = root.join("bin").join("deploy-local.ps1");
    let bin_deploy_sh_path = root.join("bin").join("deploy-local.sh");
    let bin_deploy_cmd_path = root.join("bin").join("deploy-local.cmd");
    let bin_start_ps1_path = root.join("bin").join("start-local.ps1");
    let bin_start_sh_path = root.join("bin").join("start-local.sh");
    let bin_start_cmd_path = root.join("bin").join("start-local.cmd");
    let bin_stop_ps1_path = root.join("bin").join("stop-local.ps1");
    let bin_stop_sh_path = root.join("bin").join("stop-local.sh");
    let bin_stop_cmd_path = root.join("bin").join("stop-local.cmd");
    let bin_cmd_forwarder_path = root.join("bin").join("_cmd-forward-powershell.cmd");
    let bin_restart_ps1_path = root.join("bin").join("restart-local.ps1");
    let bin_restart_sh_path = root.join("bin").join("restart-local.sh");
    let bin_restart_cmd_path = root.join("bin").join("restart-local.cmd");
    let bin_status_ps1_path = root.join("bin").join("status-local.ps1");
    let bin_status_sh_path = root.join("bin").join("status-local.sh");
    let bin_status_cmd_path = root.join("bin").join("status-local.cmd");
    let bin_inspect_runtime_ps1_path = root.join("bin").join("inspect-runtime-local.ps1");
    let bin_inspect_runtime_sh_path = root.join("bin").join("inspect-runtime-local.sh");
    let bin_inspect_runtime_cmd_path = root.join("bin").join("inspect-runtime-local.cmd");
    let bin_repair_runtime_ps1_path = root.join("bin").join("repair-runtime-local.ps1");
    let bin_repair_runtime_sh_path = root.join("bin").join("repair-runtime-local.sh");
    let bin_repair_runtime_cmd_path = root.join("bin").join("repair-runtime-local.cmd");
    let bin_restore_runtime_ps1_path = root.join("bin").join("restore-runtime-local.ps1");
    let bin_restore_runtime_sh_path = root.join("bin").join("restore-runtime-local.sh");
    let bin_restore_runtime_cmd_path = root.join("bin").join("restore-runtime-local.cmd");
    let bin_preview_restore_runtime_ps1_path =
        root.join("bin").join("preview-runtime-restore-local.ps1");
    let bin_preview_restore_runtime_sh_path =
        root.join("bin").join("preview-runtime-restore-local.sh");
    let bin_preview_restore_runtime_cmd_path =
        root.join("bin").join("preview-runtime-restore-local.cmd");
    let bin_list_runtime_backups_ps1_path = root.join("bin").join("list-runtime-backups-local.ps1");
    let bin_list_runtime_backups_sh_path = root.join("bin").join("list-runtime-backups-local.sh");
    let bin_list_runtime_backups_cmd_path = root.join("bin").join("list-runtime-backups-local.cmd");
    let bin_init_config_ps1_path = root.join("bin").join("init-config-local.ps1");
    let bin_init_config_sh_path = root.join("bin").join("init-config-local.sh");
    let bin_init_config_cmd_path = root.join("bin").join("init-config-local.cmd");
    let smoke_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.ps1");
    let smoke_sh_path = root
        .join("tools")
        .join("smoke")
        .join("local_stack_smoke.sh");
    let local_memory_adapter_path = root
        .join("adapters")
        .join("local-memory")
        .join("Cargo.toml");
    let redpanda_readme_path = root
        .join("adapters")
        .join("journal-redpanda")
        .join("README.md");
    let cockroach_readme_path = root
        .join("adapters")
        .join("meta-cockroach")
        .join("README.md");
    let scylla_readme_path = root
        .join("adapters")
        .join("timeline-scylla")
        .join("README.md");

    let dockerfile = fs::read_to_string(&dockerfile_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment dockerfile: {}",
            dockerfile_path.display()
        )
    });
    let compose = fs::read_to_string(&compose_path)
        .unwrap_or_else(|_| panic!("missing compose profile: {}", compose_path.display()));
    let bootstrap = fs::read_to_string(&bootstrap_path)
        .unwrap_or_else(|_| panic!("missing bootstrap script: {}", bootstrap_path.display()));
    let bin_install_ps1 = fs::read_to_string(&bin_install_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_ps1_path.display()));
    let bin_install_sh = fs::read_to_string(&bin_install_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_sh_path.display()));
    let bin_install_cmd = fs::read_to_string(&bin_install_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_install_cmd_path.display()));
    let bin_deploy_ps1 = fs::read_to_string(&bin_deploy_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_ps1_path.display()));
    let bin_deploy_sh = fs::read_to_string(&bin_deploy_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_sh_path.display()));
    let bin_deploy_cmd = fs::read_to_string(&bin_deploy_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_deploy_cmd_path.display()));
    let bin_start_ps1 = fs::read_to_string(&bin_start_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_ps1_path.display()));
    let bin_start_sh = fs::read_to_string(&bin_start_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_sh_path.display()));
    let bin_start_cmd = fs::read_to_string(&bin_start_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_start_cmd_path.display()));
    let bin_stop_ps1 = fs::read_to_string(&bin_stop_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_ps1_path.display()));
    let bin_stop_sh = fs::read_to_string(&bin_stop_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_sh_path.display()));
    let bin_stop_cmd = fs::read_to_string(&bin_stop_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_stop_cmd_path.display()));
    let bin_cmd_forwarder = fs::read_to_string(&bin_cmd_forwarder_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_cmd_forwarder_path.display()));
    let bin_restart_ps1 = fs::read_to_string(&bin_restart_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_ps1_path.display()));
    let bin_restart_sh = fs::read_to_string(&bin_restart_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_sh_path.display()));
    let bin_restart_cmd = fs::read_to_string(&bin_restart_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_restart_cmd_path.display()));
    let bin_status_ps1 = fs::read_to_string(&bin_status_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_ps1_path.display()));
    let bin_status_sh = fs::read_to_string(&bin_status_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_sh_path.display()));
    let bin_status_cmd = fs::read_to_string(&bin_status_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_status_cmd_path.display()));
    let bin_inspect_runtime_ps1 =
        fs::read_to_string(&bin_inspect_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_ps1_path.display()
            )
        });
    let bin_inspect_runtime_sh =
        fs::read_to_string(&bin_inspect_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_sh_path.display()
            )
        });
    let bin_inspect_runtime_cmd =
        fs::read_to_string(&bin_inspect_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_inspect_runtime_cmd_path.display()
            )
        });
    let bin_repair_runtime_ps1 =
        fs::read_to_string(&bin_repair_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_ps1_path.display()
            )
        });
    let bin_repair_runtime_sh =
        fs::read_to_string(&bin_repair_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_sh_path.display()
            )
        });
    let bin_repair_runtime_cmd =
        fs::read_to_string(&bin_repair_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_repair_runtime_cmd_path.display()
            )
        });
    let bin_restore_runtime_ps1 =
        fs::read_to_string(&bin_restore_runtime_ps1_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_ps1_path.display()
            )
        });
    let bin_restore_runtime_sh =
        fs::read_to_string(&bin_restore_runtime_sh_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_sh_path.display()
            )
        });
    let bin_restore_runtime_cmd =
        fs::read_to_string(&bin_restore_runtime_cmd_path).unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_restore_runtime_cmd_path.display()
            )
        });
    let bin_preview_restore_runtime_ps1 = fs::read_to_string(&bin_preview_restore_runtime_ps1_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_ps1_path.display()
            )
        });
    let bin_preview_restore_runtime_sh = fs::read_to_string(&bin_preview_restore_runtime_sh_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_sh_path.display()
            )
        });
    let bin_preview_restore_runtime_cmd = fs::read_to_string(&bin_preview_restore_runtime_cmd_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_preview_restore_runtime_cmd_path.display()
            )
        });
    let bin_list_runtime_backups_ps1 = fs::read_to_string(&bin_list_runtime_backups_ps1_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_ps1_path.display()
            )
        });
    let bin_list_runtime_backups_sh = fs::read_to_string(&bin_list_runtime_backups_sh_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_sh_path.display()
            )
        });
    let bin_list_runtime_backups_cmd = fs::read_to_string(&bin_list_runtime_backups_cmd_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing bin script: {}",
                bin_list_runtime_backups_cmd_path.display()
            )
        });
    let bin_init_config_ps1 = fs::read_to_string(&bin_init_config_ps1_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_ps1_path.display()));
    let bin_init_config_sh = fs::read_to_string(&bin_init_config_sh_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_sh_path.display()));
    let bin_init_config_cmd = fs::read_to_string(&bin_init_config_cmd_path)
        .unwrap_or_else(|_| panic!("missing bin script: {}", bin_init_config_cmd_path.display()));
    let smoke = fs::read_to_string(&smoke_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_path.display()));
    let smoke_sh = fs::read_to_string(&smoke_sh_path)
        .unwrap_or_else(|_| panic!("missing smoke script: {}", smoke_sh_path.display()));
    let local_memory_adapter =
        fs::read_to_string(&local_memory_adapter_path).unwrap_or_else(|_| {
            panic!(
                "missing local memory adapter cargo manifest: {}",
                local_memory_adapter_path.display()
            )
        });
    let redpanda_readme = fs::read_to_string(&redpanda_readme_path)
        .unwrap_or_else(|_| panic!("missing adapter README: {}", redpanda_readme_path.display()));
    let cockroach_readme = fs::read_to_string(&cockroach_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing adapter README: {}",
            cockroach_readme_path.display()
        )
    });
    let scylla_readme = fs::read_to_string(&scylla_readme_path)
        .unwrap_or_else(|_| panic!("missing adapter README: {}", scylla_readme_path.display()));

    assert!(dockerfile.contains("local-minimal-node"));
    assert!(dockerfile.contains("cargo build --release -p local-minimal-node"));
    assert!(dockerfile.contains("EXPOSE 18090"));

    assert!(compose.contains("local-minimal-node"));
    assert!(compose.contains("18090:18090"));
    assert!(compose.contains("healthz"));

    assert!(bootstrap.contains(r#"$composeFile = "deployments/docker-compose/$ProfileName.yml""#));
    assert!(bootstrap.contains("Missing compose profile: $composeFile"));
    assert!(bootstrap.contains("docker compose"));
    assert!(bootstrap.contains("docker compose version"));
    assert!(bootstrap.contains("docker --version"));
    assert!(bootstrap.contains("docker info"));
    assert!(bootstrap.contains("Docker daemon"));
    assert!(bootstrap.contains("Docker compose failed"));
    assert!(bootstrap.contains("docker compose -f $composeFile ps"));
    assert!(bootstrap.contains("logs --tail 200"));
    assert!(bootstrap.contains("Smoke verification failed"));
    assert!(bootstrap.contains("without smoke verification"));
    assert!(bootstrap.contains("local_stack_smoke.ps1"));
    assert_eq!(first_non_empty_line(&bootstrap), "param(");

    assert!(bin_install_ps1.contains("cargo build -p local-minimal-node --offline"));
    assert!(bin_install_ps1.contains(".runtime"));
    assert_eq!(first_non_empty_line(&bin_install_ps1), "param(");
    assert!(bin_install_ps1.contains("[string]$ProfileName = \"local-minimal\""));
    assert!(bin_install_ps1.contains("$PSBoundParameters.ContainsKey('BindAddress')"));
    assert!(bin_install_ps1.contains("_runtime-profile-common.ps1"));
    assert!(bin_install_ps1.contains("Resolve-RuntimeDirFromProfile"));
    assert!(bin_install_ps1.contains("-ProfileName $ProfileName"));
    assert!(bin_install_ps1.contains("-Force:$bindAddressProvided"));
    assert!(bin_install_sh.contains("cargo build -p local-minimal-node --offline"));
    assert!(bin_install_sh.contains(".runtime"));
    assert_eq!(first_non_empty_line(&bin_install_sh), "#!/usr/bin/env bash");
    assert!(bin_install_sh.contains("profile_name=\"local-minimal\""));
    assert!(bin_install_sh.contains("bind_addr_provided=0"));
    assert!(bin_install_sh.contains("_runtime-profile-common.sh"));
    assert!(bin_install_sh.contains("resolve_runtime_dir_from_profile"));
    assert!(bin_install_sh.contains("--profile \"$profile_name\""));
    assert!(bin_install_sh.contains("--force"));
    assert!(bin_install_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_install_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_deploy_ps1.contains("bootstrap-local.ps1"));
    assert!(bin_deploy_ps1.contains("docker compose"));
    assert_eq!(first_non_empty_line(&bin_deploy_ps1), "param(");
    assert!(
        bin_deploy_sh.contains(r#"COMPOSE_FILE="deployments/docker-compose/${profile_name}.yml""#)
    );
    assert!(bin_deploy_sh.contains("Unsupported deployment profile"));
    assert!(bin_deploy_sh.contains("docker compose"));
    assert!(bin_deploy_sh.contains("docker compose version"));
    assert!(bin_deploy_sh.contains("tools/smoke/local_stack_smoke.sh"));
    assert!(bin_deploy_sh.contains("docker compose -f \"$COMPOSE_FILE\" ps"));
    assert!(bin_deploy_sh.contains("logs --tail 200"));
    assert!(bin_deploy_sh.contains("Smoke verification failed"));
    assert_eq!(first_non_empty_line(&bin_deploy_sh), "#!/usr/bin/env bash");
    assert!(bin_deploy_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_deploy_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_start_ps1.contains("local-minimal-node"));
    assert!(bin_start_ps1.contains("Start-Process"));
    assert!(
        !bin_start_ps1.contains("$host ="),
        "start-local.ps1 must not assign the automatic $Host variable"
    );
    assert!(
        bin_start_ps1.contains("-RedirectStandardOutput"),
        "start-local.ps1 must redirect Start-Process stdout to the documented log file on Windows"
    );
    assert!(
        bin_start_ps1.contains("-RedirectStandardError"),
        "start-local.ps1 must redirect Start-Process stderr to the documented log file on Windows"
    );
    assert!(bin_start_ps1.contains("UseBasicParsing"));
    assert_eq!(first_non_empty_line(&bin_start_ps1), "param(");
    assert!(bin_start_ps1.contains("[string]$ProfileName = \"local-minimal\""));
    assert!(bin_start_ps1.contains("$PSBoundParameters.ContainsKey('BindAddress')"));
    assert!(bin_start_ps1.contains("_runtime-profile-common.ps1"));
    assert!(bin_start_ps1.contains("Resolve-RuntimeProfileConfigFiles"));
    assert!(bin_start_ps1.contains("Resolve-RuntimeDirFromProfile"));
    assert!(bin_start_ps1.contains("-ProfileName $ProfileName"));
    assert!(!bin_start_ps1.contains("$installBindAddress ="));
    assert!(bin_start_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_start_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert!(bin_start_ps1.contains("Stop-ManagedProcessAndRemovePidFile"));
    assert!(bin_start_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(!bin_start_ps1.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(bin_start_ps1.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"));
    assert!(bin_start_sh.contains("local-minimal-node"));
    assert!(bin_start_sh.contains("nohup"));
    assert_eq!(first_non_empty_line(&bin_start_sh), "#!/usr/bin/env bash");
    assert!(bin_start_sh.contains("profile_name=\"local-minimal\""));
    assert!(bin_start_sh.contains("bind_addr_provided=0"));
    assert!(bin_start_sh.contains("_runtime-profile-common.sh"));
    assert!(bin_start_sh.contains("runtime_profile_config_files"));
    assert!(bin_start_sh.contains("resolve_runtime_dir_from_profile"));
    assert!(bin_start_sh.contains("--profile \"$profile_name\""));
    assert!(bin_start_sh.contains("if [[ \"$bind_addr_provided\" -eq 1 ]]; then"));
    assert!(bin_start_sh.contains("command -v wget"));
    assert!(bin_start_sh.contains("wget -q -O /dev/null"));
    assert!(bin_start_sh.contains("Neither curl nor wget is available for health verification."));
    assert!(bin_start_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(!bin_start_sh.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(bin_start_sh.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"));
    assert!(bin_start_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_start_sh.contains("pid_matches_expected_process"));
    assert!(bin_start_sh.contains("stop_managed_process_and_remove_pid_file"));
    assert!(bin_start_sh.contains("kill -9 \"$pid\""));
    assert!(bin_start_sh.contains("return 1"));
    assert!(bin_start_sh.contains("ps -p \"$pid\" -o args="));
    assert!(bin_start_sh.contains("process_path=\"${process_name%% *}\""));
    assert!(bin_start_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_start_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_stop_ps1.contains("local-minimal-node.pid"));
    assert!(bin_stop_ps1.contains("Stop-Process"));
    assert!(bin_stop_ps1.contains("Wait-Process"));
    assert!(bin_stop_ps1.contains("did not exit within"));
    assert!(bin_stop_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_stop_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert_eq!(first_non_empty_line(&bin_stop_ps1), "param(");
    assert!(bin_stop_ps1.contains("[string]$ProfileName = \"local-minimal\""));
    assert!(bin_stop_ps1.contains("_runtime-profile-common.ps1"));
    assert!(bin_stop_ps1.contains("Resolve-RuntimeDirFromProfile"));
    assert!(bin_stop_sh.contains("local-minimal-node.pid"));
    assert!(bin_stop_sh.contains("kill"));
    assert!(bin_stop_sh.contains("for _ in $(seq 1 30)"));
    assert!(bin_stop_sh.contains("kill -0 \"$pid\""));
    assert!(bin_stop_sh.contains("did not exit within 30 seconds"));
    assert!(bin_stop_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_stop_sh.contains("pid_matches_expected_process"));
    assert!(bin_stop_sh.contains("ps -p \"$pid\" -o args="));
    assert!(bin_stop_sh.contains("process_path=\"${process_name%% *}\""));
    assert_eq!(first_non_empty_line(&bin_stop_sh), "#!/usr/bin/env bash");
    assert!(bin_stop_sh.contains("profile_name=\"local-minimal\""));
    assert!(bin_stop_sh.contains("_runtime-profile-common.sh"));
    assert!(bin_stop_sh.contains("resolve_runtime_dir_from_profile"));
    assert!(bin_stop_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_stop_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_restart_ps1.contains("stop-local.ps1"));
    assert!(bin_restart_ps1.contains("start-local.ps1"));
    assert!(bin_restart_ps1.contains("$stopExitCode"));
    assert!(bin_restart_ps1.contains("exit $stopExitCode"));
    assert_eq!(first_non_empty_line(&bin_restart_ps1), "param(");
    assert!(bin_restart_ps1.contains("[string]$ProfileName = \"local-minimal\""));
    assert!(bin_restart_ps1.contains("-ProfileName"));
    assert!(bin_restart_ps1.contains("$ProfileName"));
    assert!(bin_restart_sh.contains("stop-local.sh"));
    assert!(bin_restart_sh.contains("start-local.sh"));
    assert!(
        !bin_restart_sh.contains("|| true"),
        "restart-local.sh must not swallow stop-local.sh failures before starting a new instance"
    );
    assert_eq!(first_non_empty_line(&bin_restart_sh), "#!/usr/bin/env bash");
    assert!(bin_restart_sh.contains("profile_name=\"local-minimal\""));
    assert!(bin_restart_sh.contains("--profile"));
    assert!(bin_restart_sh.contains("\"$profile_name\""));
    assert!(bin_restart_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_restart_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_status_ps1.contains("local-minimal-node.pid"));
    assert!(bin_status_ps1.contains("stdout"));
    assert!(bin_status_ps1.contains("health status:"));
    assert!(bin_status_ps1.contains("Invoke-WebRequest"));
    assert!(bin_status_ps1.contains("inspect-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("repair-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("list-runtime-backups-local.ps1"));
    assert!(bin_status_ps1.contains("preview-runtime-restore-local.ps1"));
    assert!(bin_status_ps1.contains("restore-runtime-local.ps1"));
    assert!(bin_status_ps1.contains("ExpectedPreviewFingerprint"));
    assert!(bin_status_ps1.contains("ExpectedProcessName = \"local-minimal-node\""));
    assert!(bin_status_ps1.contains("$process.ProcessName -ieq $ExpectedProcessName"));
    assert_eq!(first_non_empty_line(&bin_status_ps1), "param(");
    assert!(bin_status_sh.contains("local-minimal-node.pid"));
    assert!(bin_status_sh.contains("stdout"));
    assert!(bin_status_sh.contains("health status:"));
    assert!(bin_status_sh.contains("command -v wget"));
    assert!(bin_status_sh.contains("inspect-runtime-local.sh"));
    assert!(bin_status_sh.contains("repair-runtime-local.sh"));
    assert!(bin_status_sh.contains("list-runtime-backups-local.sh"));
    assert!(bin_status_sh.contains("preview-runtime-restore-local.sh"));
    assert!(bin_status_sh.contains("restore-runtime-local.sh"));
    assert!(bin_status_sh.contains("--expected-preview-fingerprint"));
    assert!(bin_status_sh.contains("EXPECTED_PROCESS_NAME=\"local-minimal-node\""));
    assert!(bin_status_sh.contains("pid_matches_expected_process"));
    assert!(bin_status_sh.contains("ps -p \"$pid\" -o args="));
    assert!(bin_status_sh.contains("process_path=\"${process_name%% *}\""));
    assert_eq!(first_non_empty_line(&bin_status_sh), "#!/usr/bin/env bash");
    assert!(bin_status_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_status_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_inspect_runtime_ps1.contains("inspect-runtime-dir"));
    assert!(bin_inspect_runtime_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_inspect_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_inspect_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_inspect_runtime_ps1), "param(");
    assert!(bin_inspect_runtime_sh.contains("inspect-runtime-dir"));
    assert!(bin_inspect_runtime_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_inspect_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_inspect_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_inspect_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_inspect_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_inspect_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_repair_runtime_ps1.contains("repair-runtime-dir"));
    assert!(bin_repair_runtime_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_repair_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_repair_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_repair_runtime_ps1), "param(");
    assert!(bin_repair_runtime_sh.contains("repair-runtime-dir"));
    assert!(bin_repair_runtime_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_repair_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_repair_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_repair_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_repair_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_repair_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_restore_runtime_ps1.contains("restore-runtime-dir"));
    assert!(bin_restore_runtime_ps1.contains("ExpectedPreviewFingerprint"));
    assert!(bin_restore_runtime_ps1.contains("--expected-preview-fingerprint"));
    assert!(bin_restore_runtime_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_restore_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_restore_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(first_non_empty_line(&bin_restore_runtime_ps1), "param(");
    assert!(bin_restore_runtime_sh.contains("restore-runtime-dir"));
    assert!(bin_restore_runtime_sh.contains("expected_preview_fingerprint"));
    assert!(bin_restore_runtime_sh.contains("--expected-preview-fingerprint"));
    assert!(bin_restore_runtime_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_restore_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_restore_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_restore_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_restore_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_restore_runtime_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_preview_restore_runtime_ps1.contains("preview-runtime-restore"));
    assert!(bin_preview_restore_runtime_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_preview_restore_runtime_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_preview_restore_runtime_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(
        first_non_empty_line(&bin_preview_restore_runtime_ps1),
        "param("
    );
    assert!(bin_preview_restore_runtime_sh.contains("preview-runtime-restore"));
    assert!(bin_preview_restore_runtime_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_preview_restore_runtime_sh.contains("target/release/local-minimal-node"));
    assert!(bin_preview_restore_runtime_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_preview_restore_runtime_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_preview_restore_runtime_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_preview_restore_runtime_cmd
            .contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_list_runtime_backups_ps1.contains("list-runtime-backups"));
    assert!(bin_list_runtime_backups_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_list_runtime_backups_ps1.contains("target\\release\\local-minimal-node.exe"));
    assert!(bin_list_runtime_backups_ps1.contains("target\\debug\\local-minimal-node.exe"));
    assert_eq!(
        first_non_empty_line(&bin_list_runtime_backups_ps1),
        "param("
    );
    assert!(bin_list_runtime_backups_sh.contains("list-runtime-backups"));
    assert!(bin_list_runtime_backups_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(bin_list_runtime_backups_sh.contains("target/release/local-minimal-node"));
    assert!(bin_list_runtime_backups_sh.contains("target/debug/local-minimal-node"));
    assert_eq!(
        first_non_empty_line(&bin_list_runtime_backups_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_list_runtime_backups_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(
        !bin_list_runtime_backups_cmd
            .contains("powershell -NoProfile -ExecutionPolicy Bypass -File")
    );

    assert!(bin_init_config_ps1.contains("SDKWORK_IM_BIND_ADDR"));
    assert!(bin_init_config_ps1.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(!bin_init_config_ps1.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(bin_init_config_ps1.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"));
    assert!(bin_init_config_ps1.contains("local-minimal.env"));
    assert!(bin_init_config_ps1.contains("local-default.env"));
    assert!(bin_init_config_ps1.contains("state"));
    assert_eq!(first_non_empty_line(&bin_init_config_ps1), "param(");
    assert!(bin_init_config_ps1.contains("[string]$ProfileName = \"local-minimal\""));
    assert!(bin_init_config_ps1.contains("_runtime-profile-common.ps1"));
    assert!(bin_init_config_ps1.contains("Resolve-RuntimeDirFromProfile"));
    assert!(bin_init_config_sh.contains("SDKWORK_IM_BIND_ADDR"));
    assert!(bin_init_config_sh.contains("SDKWORK_IM_RUNTIME_DIR"));
    assert!(!bin_init_config_sh.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(bin_init_config_sh.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET"));
    assert!(bin_init_config_sh.contains("local-minimal.env"));
    assert!(bin_init_config_sh.contains("local-default.env"));
    assert!(bin_init_config_sh.contains("state"));
    assert_eq!(
        first_non_empty_line(&bin_init_config_sh),
        "#!/usr/bin/env bash"
    );
    assert!(bin_init_config_sh.contains("profile_name=\"local-minimal\""));
    assert!(bin_init_config_sh.contains("_runtime-profile-common.sh"));
    assert!(bin_init_config_sh.contains("resolve_runtime_dir_from_profile"));
    assert!(bin_init_config_cmd.contains("_cmd-forward-powershell.cmd"));
    assert!(!bin_init_config_cmd.contains("powershell -NoProfile -ExecutionPolicy Bypass -File"));

    assert!(bin_cmd_forwarder.contains("/force"));
    assert!(bin_cmd_forwarder.contains("-Force"));
    assert!(bin_cmd_forwarder.contains("/skipSmoke"));
    assert!(bin_cmd_forwarder.contains("--skip-smoke"));
    assert!(bin_cmd_forwarder.contains("-SkipSmoke"));
    assert!(bin_cmd_forwarder.contains("/bindAddress"));
    assert!(bin_cmd_forwarder.contains("-BindAddress"));
    assert!(bin_cmd_forwarder.contains("/runtimeDir"));
    assert!(bin_cmd_forwarder.contains("-RuntimeDir"));
    assert!(bin_cmd_forwarder.contains("--runtime-dir"));
    assert!(bin_cmd_forwarder.contains("/json"));
    assert!(bin_cmd_forwarder.contains("-Json"));
    assert!(bin_cmd_forwarder.contains("--json"));
    assert!(bin_cmd_forwarder.contains("/profile"));
    assert!(bin_cmd_forwarder.contains("-ProfileName"));
    assert!(bin_cmd_forwarder.contains("--profile"));
    assert!(bin_cmd_forwarder.contains("/backupDir"));
    assert!(bin_cmd_forwarder.contains("-BackupDir"));
    assert!(bin_cmd_forwarder.contains("--backup-dir"));
    assert!(bin_cmd_forwarder.contains("/expectedPreviewFingerprint"));
    assert!(bin_cmd_forwarder.contains("-ExpectedPreviewFingerprint"));
    assert!(bin_cmd_forwarder.contains("--expected-preview-fingerprint"));
    assert_eq!(first_non_empty_line(&bin_cmd_forwarder), "@echo off");

    assert!(smoke.contains("http://127.0.0.1:18090/healthz"));
    assert!(smoke.contains("Authorization"));
    assert!(smoke.contains("Access-Token"));
    assert!(!smoke.contains("x-sdkwork-tenant-id"));
    assert!(!smoke.contains("x-sdkwork-user-id"));
    assert_eq!(first_non_empty_line(&smoke), "param(");
    assert!(smoke_sh.contains("http://127.0.0.1:18090/healthz"));
    assert!(!smoke_sh.contains("resolve_authorization_header"));
    assert!(!smoke_sh.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(smoke_sh.contains("Authorization"));
    assert!(smoke_sh.contains("Access-Token"));
    assert!(!smoke_sh.contains("x-sdkwork-tenant-id"));
    assert!(!smoke_sh.contains("x-sdkwork-user-id"));
    assert!(smoke_sh.contains("command -v wget"));
    assert!(smoke_sh.contains("/im/v3/api/chat/conversations"));
    assert_eq!(first_non_empty_line(&smoke_sh), "#!/usr/bin/env bash");

    assert!(local_memory_adapter.contains("name = \"im-adapters-local-memory\""));
    assert!(redpanda_readme.contains("# journal-redpanda"));
    assert!(cockroach_readme.contains("# meta-cockroach"));
    assert!(scylla_readme.contains("# timeline-scylla"));
}

#[cfg(windows)]
#[test]
fn test_inspect_runtime_local_ps1_uses_local_default_profile_config_when_requested() {
    let root = workspace_root();
    let temp_root = unique_temp_root("inspect_runtime_ps1_profile");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_minimal_config_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config");
    let captured_args_path = temp_root.join("cargo-args.txt");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");
    let local_minimal_runtime_dir = temp_root.join("runtime-from-local-minimal");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");
    fs::create_dir_all(&local_minimal_config_dir).expect("local-minimal config dir should exist");

    fs::copy(
        root.join("bin").join("inspect-runtime-local.ps1"),
        bin_dir.join("inspect-runtime-local.ps1"),
    )
    .expect("inspect-runtime-local.ps1 should be copied into temp workspace");

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\necho %* > \"%~dp0..\\cargo-args.txt\"\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");
    fs::write(
        local_minimal_config_dir.join("local-minimal.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\n",
            local_minimal_runtime_dir.display()
        ),
    )
    .expect("local-minimal config should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\inspect-runtime-local.ps1",
            "-ProfileName",
            "local-default",
        ])
        .status()
        .expect("inspect-runtime-local.ps1 should execute");
    assert!(status.success());

    let captured_args = fs::read_to_string(&captured_args_path).unwrap_or_else(|_| {
        panic!(
            "missing captured cargo args: {}",
            captured_args_path.display()
        )
    });
    assert!(
        captured_args.contains("inspect-runtime-dir"),
        "inspect-runtime-local.ps1 must invoke inspect-runtime-dir through local-minimal-node. actual args: {captured_args}"
    );
    assert!(
        captured_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()),
        "inspect-runtime-local.ps1 must resolve runtime dir from the local-default config when that profile is requested. actual args: {captured_args}"
    );
    assert!(
        !captured_args.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()),
        "inspect-runtime-local.ps1 must not fall back to local-minimal config when local-default config exists. actual args: {captured_args}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_inspect_runtime_local_cmd_supports_profile_switch() {
    let root = workspace_root();
    let temp_root = unique_temp_root("inspect_runtime_cmd_profile");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let captured_args_path = temp_root.join("cargo-args.txt");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");

    for file_name in [
        "inspect-runtime-local.ps1",
        "inspect-runtime-local.cmd",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\necho %* > \"%~dp0..\\cargo-args.txt\"\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "/c",
            "bin\\inspect-runtime-local.cmd",
            "--profile",
            "local-default",
        ])
        .status()
        .expect("inspect-runtime-local.cmd should execute");
    assert!(status.success());

    let captured_args = fs::read_to_string(&captured_args_path).unwrap_or_else(|_| {
        panic!(
            "missing captured cargo args: {}",
            captured_args_path.display()
        )
    });
    assert!(
        captured_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()),
        "inspect-runtime-local.cmd must normalize --profile to the real PowerShell profile parameter. actual args: {captured_args}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_repair_runtime_local_sh_uses_local_default_profile_config_when_requested() {
    let root = workspace_root();
    let temp_root = unique_temp_root("repair_runtime_sh_profile");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_minimal_config_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config");
    let captured_args_path = temp_root.join("cargo-args.txt");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");
    let local_minimal_runtime_dir = temp_root.join("runtime-from-local-minimal");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");
    fs::create_dir_all(&local_minimal_config_dir).expect("local-minimal config dir should exist");

    for file_name in ["repair-runtime-local.sh", "_runtime-profile-common.sh"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp workspace"));
    }

    fs::write(
        fake_tools_dir.join("cargo"),
        "#!/usr/bin/env bash\nscript_dir=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\"\nprintf '%s\\n' \"$@\" > \"${script_dir}/../cargo-args.txt\"\nexit 0\n",
    )
    .expect("fake cargo shell should be written");

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");
    fs::write(
        local_minimal_config_dir.join("local-minimal.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\n",
            local_minimal_runtime_dir.display()
        ),
    )
    .expect("local-minimal config should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping repair-runtime-local.sh profile regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let chmod_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!(
            "chmod +x \"{}\"",
            fake_tools_dir.join("cargo").display()
        ))
        .status()
        .expect("chmod should execute for fake cargo shell");
    assert!(chmod_status.success());

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{}:{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args(["bin/repair-runtime-local.sh", "--profile", "local-default"])
        .output()
        .expect("repair-runtime-local.sh should execute");
    assert!(
        output.status.success(),
        "repair-runtime-local.sh should execute through fake cargo. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let captured_args = fs::read_to_string(&captured_args_path).unwrap_or_else(|_| {
        panic!(
            "missing captured cargo args: {}",
            captured_args_path.display()
        )
    });
    assert!(
        captured_args.contains("repair-runtime-dir"),
        "repair-runtime-local.sh must invoke repair-runtime-dir through local-minimal-node. actual args: {captured_args}"
    );
    assert!(
        captured_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()),
        "repair-runtime-local.sh must resolve runtime dir from the local-default config when that profile is requested. actual args: {captured_args}"
    );
    assert!(
        !captured_args.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()),
        "repair-runtime-local.sh must not fall back to local-minimal config when local-default config exists. actual args: {captured_args}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_repair_runtime_local_sh_invokes_social_repair_after_generic_repair_when_social_journal_exists()
 {
    let root = workspace_root();
    let temp_root = unique_temp_root("repair_runtime_sh_social_repair");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let runtime_dir = temp_root.join("runtime");
    let social_state_dir = runtime_dir.join("state");
    let captured_calls_path = temp_root.join("cargo-calls.txt");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&social_state_dir).expect("social state dir should be created");

    for file_name in ["repair-runtime-local.sh", "_runtime-profile-common.sh"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(social_state_dir.join("social-commit-journal.json"), "")
        .expect("social commit journal should be written");

    fs::write(
        fake_tools_dir.join("cargo"),
        "#!/usr/bin/env bash\nscript_dir=\"$(cd \"$(dirname \"${BASH_SOURCE[0]}\")\" && pwd)\"\n{\n  printf '__CALL__\\n'\n  printf '%s\\n' \"$@\"\n} >> \"${script_dir}/../cargo-calls.txt\"\nexit 0\n",
    )
    .expect("fake cargo shell should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping repair-runtime-local.sh social repair regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    let chmod_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!(
            "chmod +x \"{}\"",
            fake_tools_dir.join("cargo").display()
        ))
        .status()
        .expect("chmod should execute for fake cargo shell");
    assert!(chmod_status.success());

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{}:{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "bin/repair-runtime-local.sh",
            "--runtime-dir",
            runtime_dir.to_string_lossy().as_ref(),
            "--json",
        ])
        .output()
        .expect("repair-runtime-local.sh should execute");
    assert!(
        output.status.success(),
        "repair-runtime-local.sh should execute through fake cargo. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let captured_calls = fs::read_to_string(&captured_calls_path).unwrap_or_else(|_| {
        panic!(
            "missing captured cargo calls: {}",
            captured_calls_path.display()
        )
    });
    let calls = parse_captured_cli_calls(&captured_calls);
    assert_eq!(
        calls.len(),
        2,
        "repair-runtime-local.sh must invoke generic repair and then social repair when the social journal exists. actual calls: {captured_calls}"
    );
    assert!(
        calls[0].contains(&"local-minimal-node".to_string()),
        "first call must target local-minimal-node. actual calls: {captured_calls}"
    );
    assert!(
        calls[0].contains(&"repair-runtime-dir".to_string()),
        "first call must invoke repair-runtime-dir. actual calls: {captured_calls}"
    );
    assert!(
        calls[1].contains(&"governance-service".to_string()),
        "second call must target governance-service. actual calls: {captured_calls}"
    );
    assert!(
        calls[1].contains(&"repair-social-runtime-dir".to_string()),
        "second call must invoke repair-social-runtime-dir. actual calls: {captured_calls}"
    );
    assert!(
        calls[1].contains(&runtime_dir.to_string_lossy().into_owned()),
        "social repair must reuse the same runtime dir. actual calls: {captured_calls}"
    );
    assert!(
        calls[1].contains(&"--json".to_string()),
        "social repair must receive --json when the wrapper is run in JSON mode. actual calls: {captured_calls}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_runtime_operation_ps1_wrappers_forward_profile_and_backup_arguments() {
    let root = workspace_root();
    let temp_root = unique_temp_root("runtime_ops_ps1_wrappers");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let local_default_config_dir = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config");
    let local_minimal_config_dir = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config");
    let captured_args_path = temp_root.join("cargo-args.txt");
    let local_default_runtime_dir = temp_root.join("runtime-from-local-default");
    let local_minimal_runtime_dir = temp_root.join("runtime-from-local-minimal");
    let backup_dir = temp_root.join("backup-source");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&local_default_config_dir).expect("local-default config dir should exist");
    fs::create_dir_all(&local_minimal_config_dir).expect("local-minimal config dir should exist");
    fs::create_dir_all(&backup_dir).expect("backup source dir should exist");

    for file_name in [
        "_runtime-profile-common.ps1",
        "repair-runtime-local.ps1",
        "archive-runtime-backup-local.ps1",
        "prune-runtime-archives-local.ps1",
        "preview-runtime-restore-local.ps1",
        "restore-runtime-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\necho %* > \"%~dp0..\\cargo-args.txt\"\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    fs::write(
        local_default_config_dir.join("local-default.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\n",
            local_default_runtime_dir.display()
        ),
    )
    .expect("local-default config should be written");
    fs::write(
        local_minimal_config_dir.join("local-minimal.env"),
        format!(
            "SDKWORK_IM_RUNTIME_DIR={}\r\n",
            local_minimal_runtime_dir.display()
        ),
    )
    .expect("local-minimal config should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let run_script = |script_name: &str, script_args: &[&str], expected_command: &str| -> String {
        let mut command_args = vec![
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            Box::leak(format!("bin\\{script_name}").into_boxed_str()),
        ];
        command_args.extend_from_slice(script_args);
        let output = Command::new("powershell")
            .current_dir(&temp_root)
            .env("PATH", &temp_path)
            .args(&command_args)
            .output()
            .unwrap_or_else(|_| panic!("{script_name} should execute"));
        assert!(
            output.status.success(),
            "{script_name} should execute through fake cargo. stdout: {} stderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        let captured_args = fs::read_to_string(&captured_args_path).unwrap_or_else(|_| {
            panic!(
                "missing captured cargo args for {script_name}: {}",
                captured_args_path.display()
            )
        });
        assert!(
            captured_args.contains(expected_command),
            "{script_name} must invoke {expected_command} through local-minimal-node. actual args: {captured_args}"
        );
        let _ = fs::remove_file(&captured_args_path);
        captured_args
    };

    let repair_args = run_script(
        "repair-runtime-local.ps1",
        &["-ProfileName", "local-default", "-Json"],
        "repair-runtime-dir",
    );
    assert!(repair_args.contains("--json"));
    assert!(repair_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()));
    assert!(!repair_args.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()));

    let backup_dir_string = backup_dir.to_string_lossy().into_owned();
    let archive_args = run_script(
        "archive-runtime-backup-local.ps1",
        &[
            "-ProfileName",
            "local-default",
            "-BackupDir",
            backup_dir_string.as_str(),
            "-RetentionDays",
            "30",
            "-LegalHold",
            "-Json",
        ],
        "archive-runtime-backup",
    );
    assert!(archive_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()));
    assert!(archive_args.contains(backup_dir_string.as_str()));
    assert!(archive_args.contains("--retention-days"));
    assert!(archive_args.contains("30"));
    assert!(archive_args.contains("--legal-hold"));
    assert!(archive_args.contains("--json"));

    let prune_args = run_script(
        "prune-runtime-archives-local.ps1",
        &["-ProfileName", "local-default", "-Json"],
        "prune-archived-runtime-backups",
    );
    assert!(prune_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()));
    assert!(!prune_args.contains(local_minimal_runtime_dir.to_string_lossy().as_ref()));
    assert!(prune_args.contains("--json"));

    let preview_args = run_script(
        "preview-runtime-restore-local.ps1",
        &[
            "-ProfileName",
            "local-default",
            "-BackupDir",
            backup_dir_string.as_str(),
            "-Json",
        ],
        "preview-runtime-restore",
    );
    assert!(preview_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()));
    assert!(preview_args.contains(backup_dir_string.as_str()));
    assert!(preview_args.contains("--json"));

    let restore_args = run_script(
        "restore-runtime-local.ps1",
        &[
            "-ProfileName",
            "local-default",
            "-BackupDir",
            backup_dir_string.as_str(),
            "-ExpectedPreviewFingerprint",
            "fingerprint-123",
            "-Json",
        ],
        "restore-runtime-dir",
    );
    assert!(restore_args.contains(local_default_runtime_dir.to_string_lossy().as_ref()));
    assert!(restore_args.contains(backup_dir_string.as_str()));
    assert!(restore_args.contains("--expected-preview-fingerprint"));
    assert!(restore_args.contains("fingerprint-123"));
    assert!(restore_args.contains("--json"));

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_repair_runtime_local_ps1_propagates_social_repair_failure_when_social_journal_exists() {
    let root = workspace_root();
    let temp_root = unique_temp_root("repair_runtime_ps1_social_failure");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let runtime_dir = temp_root.join("runtime");
    let social_state_dir = runtime_dir.join("state");
    let captured_calls_path = temp_root.join("cargo-calls.txt");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&social_state_dir).expect("social state dir should be created");

    fs::copy(
        root.join("bin").join("repair-runtime-local.ps1"),
        bin_dir.join("repair-runtime-local.ps1"),
    )
    .expect("repair-runtime-local.ps1 should be copied into temp workspace");

    fs::write(social_state_dir.join("social-commit-journal.json"), "")
        .expect("social commit journal should be written");

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nset args=%*\r\n>> \"%~dp0..\\cargo-calls.txt\" echo __CALL__\r\n>> \"%~dp0..\\cargo-calls.txt\" echo %args%\r\necho %args% | findstr /C:\"-p governance-service\" >nul\r\nif %errorlevel%==0 exit /b 23\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run runtime scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\repair-runtime-local.ps1",
            "-RuntimeDir",
            runtime_dir.to_string_lossy().as_ref(),
            "-Json",
        ])
        .output()
        .expect("repair-runtime-local.ps1 should execute");
    assert!(
        !output.status.success(),
        "repair-runtime-local.ps1 must fail when the appended social repair fails. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let captured_calls = fs::read_to_string(&captured_calls_path).unwrap_or_else(|_| {
        panic!(
            "missing captured cargo calls: {}",
            captured_calls_path.display()
        )
    });
    let calls = parse_captured_cli_calls(&captured_calls);
    assert_eq!(
        calls.len(),
        2,
        "repair-runtime-local.ps1 must invoke generic repair and then social repair when the social journal exists. actual calls: {captured_calls}"
    );
    assert!(
        calls[0]
            .iter()
            .any(|line| line.contains("local-minimal-node"))
            && calls[0]
                .iter()
                .any(|line| line.contains("repair-runtime-dir")),
        "first call must target local-minimal-node repair-runtime-dir. actual calls: {captured_calls}"
    );
    assert!(
        calls[1]
            .iter()
            .any(|line| line.contains("governance-service"))
            && calls[1]
                .iter()
                .any(|line| line.contains("repair-social-runtime-dir")),
        "second call must target governance-service repair-social-runtime-dir. actual calls: {captured_calls}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_init_config_local_ps1_uses_local_default_profile_when_requested() {
    let root = workspace_root();
    let temp_root = unique_temp_root("init_config_ps1_profile");
    let bin_dir = temp_root.join("bin");
    let local_default_config_file = temp_root
        .join(".runtime")
        .join("local-default")
        .join("config")
        .join("local-default.env");
    let local_minimal_runtime_dir = temp_root.join(".runtime").join("local-minimal");
    let local_minimal_config_file = local_minimal_runtime_dir
        .join("config")
        .join("local-minimal.env");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["init-config-local.ps1", "_runtime-profile-common.ps1"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let output = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\init-config-local.ps1",
            "-ProfileName",
            "local-default",
            "-BindAddress",
            "127.0.0.1:19101",
        ])
        .output()
        .expect("init-config-local.ps1 should execute");
    assert!(
        output.status.success(),
        "init-config-local.ps1 should support profile-aware config initialization. stdout: {} stderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let config = fs::read_to_string(&local_default_config_file).unwrap_or_else(|_| {
        panic!(
            "missing local-default config file: {}",
            local_default_config_file.display()
        )
    });
    assert!(
        config.contains("SDKWORK_IM_BIND_ADDR=127.0.0.1:19101"),
        "init-config-local.ps1 must write the selected bind address into the local-default config. actual config: {config}"
    );
    assert!(
        config.contains(
            format!(
                "SDKWORK_IM_RUNTIME_DIR={}",
                local_minimal_runtime_dir.display()
            )
            .as_str()
        ),
        "init-config-local.ps1 must preserve the current local-default runtime contract fallback to the local-minimal runtime dir. actual config: {config}"
    );
    assert!(
        !config.contains("SDKWORK_IM_PUBLIC_BEARER"),
        "init-config-local.ps1 must not materialize sdkwork-im IAM/Public Bearer secrets in the selected profile config. actual config: {config}"
    );
    assert!(
        config.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET="),
        "init-config-local.ps1 must materialize a stable friend request cursor signing secret in the selected profile config. actual config: {config}"
    );
    assert!(
        !local_minimal_config_file.exists(),
        "init-config-local.ps1 must not overwrite local-minimal.env when local-default is explicitly selected"
    );
    for dir_name in ["logs", "pids", "state"] {
        let dir = local_minimal_runtime_dir.join(dir_name);
        assert!(
            dir.is_dir(),
            "init-config-local.ps1 must prepare the shared runtime contract directory for local-default. missing: {}",
            dir.display()
        );
    }

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_init_config_cmd_normalizes_cmd_style_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("cmd_wrapper");
    let bin_dir = temp_root.join("bin");
    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "init-config-local.ps1",
        "init-config-local.cmd",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let seed_status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/bindAddress",
            "127.0.0.1:18090",
        ])
        .status()
        .expect("seed init-config-local.cmd should execute");
    assert!(seed_status.success());

    let overwrite_status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/force",
            "/bindAddress",
            "127.0.0.1:18101",
        ])
        .status()
        .expect("overwrite init-config-local.cmd should execute");
    assert!(overwrite_status.success());

    let config_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config")
        .join("local-minimal.env");
    let config = fs::read_to_string(&config_file)
        .unwrap_or_else(|_| panic!("missing temp config file: {}", config_file.display()));
    assert!(config.contains("SDKWORK_IM_BIND_ADDR=127.0.0.1:18101"));
    assert!(config.contains("SDKWORK_IM_RUNTIME_DIR="));
    assert!(!config.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(config.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET="));

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_install_local_cmd_rewrites_existing_config_when_bind_address_is_explicitly_provided() {
    let root = workspace_root();
    let temp_root = unique_temp_root("install_cmd_bind_override");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");

    for file_name in [
        "init-config-local.ps1",
        "init-config-local.cmd",
        "install-local.ps1",
        "install-local.cmd",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let seed_status = Command::new("cmd")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "/c",
            "bin\\init-config-local.cmd",
            "/bindAddress",
            "127.0.0.1:18090",
        ])
        .status()
        .expect("seed init-config-local.cmd should execute");
    assert!(seed_status.success());

    let install_status = Command::new("cmd")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "/c",
            "bin\\install-local.cmd",
            "/bindAddress",
            "127.0.0.1:18111",
        ])
        .status()
        .expect("install-local.cmd should execute");
    assert!(install_status.success());

    let config_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("config")
        .join("local-minimal.env");
    let config = fs::read_to_string(&config_file)
        .unwrap_or_else(|_| panic!("missing temp config file: {}", config_file.display()));
    assert!(config.contains("SDKWORK_IM_BIND_ADDR=127.0.0.1:18111"));
    assert!(config.contains("SDKWORK_IM_RUNTIME_DIR="));
    assert!(!config.contains("SDKWORK_IM_PUBLIC_BEARER"));
    assert!(config.contains("SDKWORK_IM_FRIEND_REQUEST_CURSOR_HS256_SECRET="));

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_install_local_cmd_help_surfaces_gnu_style_named_flags() {
    let root = workspace_root();
    let temp_root = unique_temp_root("install_cmd_help_surface");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "install-local.cmd",
        "install-local.ps1",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let output = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\install-local.cmd", "--help"])
        .output()
        .expect("install-local.cmd --help should execute");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "Usage: cmd /c .\\bin\\install-local.cmd [--profile <local-minimal|local-default>] [--release] [--bind-addr <host:port>]"
        ),
        "install-local.cmd --help must surface the documented GNU-style Windows usage. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(
            "Usage: powershell -ExecutionPolicy Bypass -File bin/install-local.ps1 [-ProfileName <local-minimal|local-default>] [-Release] [-BindAddress <host:port>]"
        ),
        "install-local.cmd --help should continue surfacing the native PowerShell usage. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_inspect_runtime_local_cmd_help_surfaces_gnu_style_named_flags() {
    let root = workspace_root();
    let temp_root = unique_temp_root("inspect_runtime_cmd_help_surface");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "inspect-runtime-local.cmd",
        "inspect-runtime-local.ps1",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let output = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\inspect-runtime-local.cmd", "--help"])
        .output()
        .expect("inspect-runtime-local.cmd --help should execute");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "Usage: cmd /c .\\bin\\inspect-runtime-local.cmd [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]"
        ),
        "inspect-runtime-local.cmd --help must surface the documented GNU-style Windows usage. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(
            "Usage: powershell -ExecutionPolicy Bypass -File bin/inspect-runtime-local.ps1 [-ProfileName <local-minimal|local-default>] [-RuntimeDir <path>] [-Json] [-Release]"
        ),
        "inspect-runtime-local.cmd --help should continue surfacing the native PowerShell usage. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_cmd_normalizes_skip_smoke_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_cmd_skip_smoke");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["deploy-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("deploy-local.ps1"),
        "param([switch]$SkipSmoke)\r\nif (-not $SkipSmoke) { throw 'SkipSmoke switch was not forwarded.' }\r\nWrite-Host 'skip smoke forwarded'\r\n",
    )
    .expect("stub deploy-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\deploy-local.cmd", "--skip-smoke"])
        .status()
        .expect("deploy-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_repair_runtime_local_cmd_help_surfaces_gnu_style_named_flags() {
    let root = workspace_root();
    let temp_root = unique_temp_root("repair_runtime_cmd_help_surface");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "repair-runtime-local.cmd",
        "repair-runtime-local.ps1",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let output = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\repair-runtime-local.cmd", "--help"])
        .output()
        .expect("repair-runtime-local.cmd --help should execute");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "Usage: cmd /c .\\bin\\repair-runtime-local.cmd [--profile <local-minimal|local-default>] [--runtime-dir <path>] [--json] [--release]"
        ),
        "repair-runtime-local.cmd --help must surface the documented GNU-style Windows usage. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(
            "Usage: powershell -ExecutionPolicy Bypass -File bin/repair-runtime-local.ps1 [-ProfileName <local-minimal|local-default>] [-RuntimeDir <path>] [-Json] [-Release]"
        ),
        "repair-runtime-local.cmd --help should continue surfacing the native PowerShell usage. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_cmd_help_surfaces_gnu_style_named_flags() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_cmd_help_surface");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in [
        "deploy-local.cmd",
        "deploy-local.ps1",
        "_cmd-forward-powershell.cmd",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let output = Command::new("cmd")
        .current_dir(&temp_root)
        .args(["/c", "bin\\deploy-local.cmd", "--help"])
        .output()
        .expect("deploy-local.cmd --help should execute");
    assert!(output.status.success());

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains(
            "Usage: cmd /c .\\bin\\deploy-local.cmd [--profile <local-minimal|local-default>] [--skip-smoke] [--smoke-base-url <url>]"
        ),
        "deploy-local.cmd --help must surface the documented GNU-style Windows usage. actual stdout: {stdout}"
    );
    assert!(
        stdout.contains(
            "Usage: powershell -ExecutionPolicy Bypass -File bin/deploy-local.ps1 [-ProfileName <local-minimal|local-default>] [-SkipSmoke] [-SmokeBaseUrl <url>]"
        ),
        "deploy-local.cmd --help should continue surfacing the native PowerShell usage. actual stdout: {stdout}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_ps1_forwards_profile_name_to_bootstrap_script() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_ps1_profile_forward");
    let bin_dir = temp_root.join("bin");
    let bootstrap_dir = temp_root.join("deployments").join("scripts");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&bootstrap_dir).expect("temp bootstrap dir should be created");

    fs::copy(
        root.join("bin").join("deploy-local.ps1"),
        bin_dir.join("deploy-local.ps1"),
    )
    .expect("deploy-local.ps1 should be copied into temp workspace");

    fs::write(
        bootstrap_dir.join("bootstrap-local.ps1"),
        "param([string]$ProfileName = 'local-minimal', [switch]$SkipSmoke)\r\nif ($ProfileName -ne 'local-default') { throw \"ProfileName was not forwarded: $ProfileName\" }\r\nif (-not $SkipSmoke) { throw 'SkipSmoke switch was not forwarded.' }\r\nWrite-Host 'profile forwarded'\r\n",
    )
    .expect("stub bootstrap-local.ps1 should be written");

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\deploy-local.ps1",
            "-ProfileName",
            "local-default",
            "-SkipSmoke",
        ])
        .status()
        .expect("deploy-local.ps1 should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_ps1_forwards_smoke_base_url_to_bootstrap_script() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_ps1_smoke_base_url_forward");
    let bin_dir = temp_root.join("bin");
    let bootstrap_dir = temp_root.join("deployments").join("scripts");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&bootstrap_dir).expect("temp bootstrap dir should be created");

    fs::copy(
        root.join("bin").join("deploy-local.ps1"),
        bin_dir.join("deploy-local.ps1"),
    )
    .expect("deploy-local.ps1 should be copied into temp workspace");

    fs::write(
        bootstrap_dir.join("bootstrap-local.ps1"),
        "param([string]$ProfileName = 'local-minimal', [switch]$SkipSmoke, [string]$SmokeBaseUrl = '')\r\nif ($SmokeBaseUrl -ne 'http://127.0.0.1:28090') { throw \"SmokeBaseUrl was not forwarded: $SmokeBaseUrl\" }\r\nif ($SkipSmoke) { throw 'SkipSmoke should remain disabled for smoke forwarding test.' }\r\nWrite-Host 'smoke base url forwarded'\r\n",
    )
    .expect("stub bootstrap-local.ps1 should be written");

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\deploy-local.ps1",
            "-SmokeBaseUrl",
            "http://127.0.0.1:28090",
        ])
        .status()
        .expect("deploy-local.ps1 should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_cmd_normalizes_profile_name_switch() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_cmd_profile_switch");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["deploy-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("deploy-local.ps1"),
        "param([string]$ProfileName = 'local-minimal', [switch]$SkipSmoke)\r\nif ($ProfileName -ne 'local-default') { throw \"ProfileName was not forwarded: $ProfileName\" }\r\nif (-not $SkipSmoke) { throw 'SkipSmoke switch was not forwarded.' }\r\nWrite-Host 'profile switch forwarded'\r\n",
    )
    .expect("stub deploy-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\deploy-local.cmd",
            "--profile",
            "local-default",
            "--skip-smoke",
        ])
        .status()
        .expect("deploy-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_deploy_local_cmd_normalizes_smoke_base_url_switch() {
    let root = workspace_root();
    let temp_root = unique_temp_root("deploy_cmd_smoke_base_url_switch");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["deploy-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("deploy-local.ps1"),
        "param([string]$SmokeBaseUrl = '')\r\nif ($SmokeBaseUrl -ne 'http://127.0.0.1:28090') { throw \"SmokeBaseUrl was not forwarded: $SmokeBaseUrl\" }\r\nWrite-Host 'smoke base url switch forwarded'\r\n",
    )
    .expect("stub deploy-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\deploy-local.cmd",
            "--smoke-base-url",
            "http://127.0.0.1:28090",
        ])
        .status()
        .expect("deploy-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_cmd_normalizes_documented_long_switches() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_cmd_long_switches");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["start-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("start-local.ps1"),
        "param([switch]$Release, [switch]$Foreground, [string]$BindAddress)\r\nif (-not $Release) { throw 'Release switch was not forwarded.' }\r\nif (-not $Foreground) { throw 'Foreground switch was not forwarded.' }\r\nif ($BindAddress -ne '127.0.0.1:19090') { throw \"BindAddress was not forwarded: $BindAddress\" }\r\nWrite-Host 'documented switches forwarded'\r\n",
    )
    .expect("stub start-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\start-local.cmd",
            "--release",
            "--foreground",
            "--bind-addr",
            "127.0.0.1:19090",
        ])
        .status()
        .expect("start-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_restore_runtime_local_cmd_normalizes_expected_preview_fingerprint_switch() {
    let root = workspace_root();
    let temp_root = unique_temp_root("restore_cmd_expected_preview_fingerprint");
    let bin_dir = temp_root.join("bin");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");

    for file_name in ["restore-runtime-local.cmd", "_cmd-forward-powershell.cmd"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        bin_dir.join("restore-runtime-local.ps1"),
        "param([string]$BackupDir = '', [string]$ExpectedPreviewFingerprint = '')\r\nif ($BackupDir -ne 'C:\\tmp\\backup') { throw \"BackupDir was not forwarded: $BackupDir\" }\r\nif ($ExpectedPreviewFingerprint -ne 'fingerprint-123') { throw \"ExpectedPreviewFingerprint was not forwarded: $ExpectedPreviewFingerprint\" }\r\nWrite-Host 'restore fingerprint switch forwarded'\r\n",
    )
    .expect("stub restore-runtime-local.ps1 should be written");

    let status = Command::new("cmd")
        .current_dir(&temp_root)
        .args([
            "/c",
            "bin\\restore-runtime-local.cmd",
            "--backup-dir",
            "C:\\tmp\\backup",
            "--expected-preview-fingerprint",
            "fingerprint-123",
        ])
        .status()
        .expect("restore-runtime-local.cmd should execute");
    assert!(status.success());

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_retired_ui_test_launchers_are_not_deployment_profile_assets() {
    let root = workspace_root();
    for retired_path in [
        root.join("bin").join("open-chat-test.ps1"),
        root.join("bin").join("open-chat-test.cmd"),
        root.join("bin").join("open-chat-test.sh"),
        root.join("bin").join("open-chat-test"),
        root.join("bin").join("chat-window-gui.ps1"),
        root.join("bin").join("chat-window-gui.cmd"),
    ] {
        assert!(
            !retired_path.exists(),
            "retired UI test launcher must not be required by deployment profile assets: {}",
            retired_path.display()
        );
    }
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_captures_background_process_stdout_into_documented_log_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_log_capture");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let whoami_path = windows_system_executable("whoami.exe");
    fs::copy(
        &whoami_path,
        target_debug_dir.join("local-minimal-node.exe"),
    )
    .unwrap_or_else(|_| {
        panic!(
            "failed to copy fake node binary from {}",
            whoami_path.display()
        )
    });

    let expected_stdout = Command::new(&whoami_path)
        .output()
        .expect("whoami.exe should run for expected stdout capture");
    assert!(
        expected_stdout.status.success(),
        "whoami.exe should succeed when collecting expected stdout"
    );
    let expected_stdout = String::from_utf8_lossy(&expected_stdout.stdout)
        .trim()
        .to_string();
    assert!(
        !expected_stdout.is_empty(),
        "whoami.exe output should not be empty"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");
    assert!(
        !status.success(),
        "fake binary should exit before readiness so the wrapper returns failure"
    );

    let stdout_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.out.log");
    let stderr_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.err.log");

    let stdout = fs::read_to_string(&stdout_log)
        .unwrap_or_else(|_| panic!("missing stdout log file: {}", stdout_log.display()));
    let stderr = fs::read_to_string(&stderr_log)
        .unwrap_or_else(|_| panic!("missing stderr log file: {}", stderr_log.display()));

    assert!(
        stdout.contains(&expected_stdout),
        "stdout log should capture child process stdout. expected fragment: {expected_stdout}, actual: {stdout}"
    );
    assert!(
        stderr.is_empty(),
        "stderr log should stay empty when fake process only writes stdout"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_captures_background_process_stderr_into_documented_log_file() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_stderr_capture");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let expected_stderr_line = "synthetic stderr from local-minimal-node probe";
    let probe_source = temp_root.join("stderr-probe.rs");
    fs::write(
        &probe_source,
        format!("fn main() {{ eprintln!(\"{expected_stderr_line}\"); std::process::exit(1); }}\n"),
    )
    .expect("stderr probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node.exe")
                .to_str()
                .expect("probe exe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the stderr probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the stderr probe executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");
    assert!(
        !status.success(),
        "fake binary should exit before readiness so the wrapper returns failure"
    );

    let stdout_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.out.log");
    let stderr_log = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("logs")
        .join("local-minimal-node.err.log");

    let stdout = fs::read_to_string(&stdout_log)
        .unwrap_or_else(|_| panic!("missing stdout log file: {}", stdout_log.display()));
    let stderr = fs::read_to_string(&stderr_log)
        .unwrap_or_else(|_| panic!("missing stderr log file: {}", stderr_log.display()));

    assert!(
        stdout.trim().is_empty(),
        "stdout log should stay empty when fake process only writes stderr. actual stdout: {stdout}"
    );
    assert!(
        stderr.contains(expected_stderr_line),
        "stderr log should capture child process stderr. expected fragment: {expected_stderr_line}, actual: {stderr}"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[cfg(windows)]
#[test]
fn test_start_local_ps1_stops_background_process_and_clears_pid_file_when_health_check_times_out() {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_ps1_health_timeout_cleanup");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in [
        "start-local.ps1",
        "install-local.ps1",
        "init-config-local.ps1",
    ] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let start_script_path = bin_dir.join("start-local.ps1");
    let start_script = fs::read_to_string(&start_script_path)
        .expect("copied start-local.ps1 should be readable for test acceleration");
    let accelerated_start_script = start_script
        .replacen(
            "$ErrorActionPreference = 'Stop'",
            "$ErrorActionPreference = 'Stop'\r\nfunction Invoke-WebRequest { throw 'synthetic health probe failure' }\r\n",
            1,
        )
        .replacen(
            "for ($attempt = 0; $attempt -lt 30; $attempt++) {",
            "for ($attempt = 0; $attempt -lt 20; $attempt++) {",
            1,
        )
        .replacen("Start-Sleep -Seconds 1", "Start-Sleep -Milliseconds 100", 1);
    assert_ne!(
        start_script, accelerated_start_script,
        "test acceleration should rewrite the copied start-local.ps1 readiness loop"
    );
    fs::write(&start_script_path, accelerated_start_script)
        .expect("accelerated start-local.ps1 should be written");

    fs::write(
        fake_tools_dir.join("cargo.cmd"),
        "@echo off\r\nexit /b 0\r\n",
    )
    .expect("fake cargo.cmd should be written");

    let probe_source = temp_root.join("health-timeout-probe.rs");
    fs::write(
        &probe_source,
        r#"
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

fn main() {
    let runtime_dir = env::var("SDKWORK_IM_RUNTIME_DIR").expect("runtime dir env should be present");
    let marker = PathBuf::from(runtime_dir).join("state").join("health-timeout-probe.pid");
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent).expect("marker parent dir should exist");
    }
    fs::write(&marker, process::id().to_string()).expect("marker should be written");
    thread::sleep(Duration::from_secs(300));
}
"#,
    )
    .expect("health-timeout probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node.exe")
                .to_str()
                .expect("probe exe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the health-timeout probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the health-timeout probe executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{};{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let status = Command::new("powershell")
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin\\start-local.ps1",
        ])
        .status()
        .expect("start-local.ps1 should execute against temp workspace");

    assert!(
        !status.success(),
        "health-timeout probe should cause the wrapper to fail startup"
    );

    let marker_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("state")
        .join("health-timeout-probe.pid");
    assert!(
        wait_for_path(&marker_file, Duration::from_secs(2)),
        "probe should record its pid before the wrapper returns"
    );

    let probe_pid = fs::read_to_string(&marker_file)
        .expect("probe pid marker should be readable")
        .trim()
        .parse::<u32>()
        .expect("probe pid marker should contain a numeric pid");
    let pid_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids")
        .join("local-minimal-node.pid");

    let probe_running_status = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "if ($null -eq (Get-Process -Id {probe_pid} -ErrorAction SilentlyContinue)) {{ exit 0 }} else {{ exit 1 }}"
            ),
        ])
        .status()
        .expect("probe running-state query should execute");
    let probe_still_running = !probe_running_status.success();

    let _ = Command::new("powershell")
        .args([
            "-NoProfile",
            "-Command",
            &format!(
                "$p = Get-Process -Id {probe_pid} -ErrorAction SilentlyContinue; if ($null -ne $p) {{ Stop-Process -Id {probe_pid} -Force -ErrorAction SilentlyContinue }}",
            ),
        ])
        .status();

    assert!(
        !probe_still_running,
        "start-local.ps1 must stop the launched local-minimal-node process when health readiness times out"
    );
    assert!(
        !pid_file.exists(),
        "start-local.ps1 must clear the pid file when startup fails after launching the process"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_start_local_sh_force_kills_background_process_and_clears_pid_file_when_health_check_times_out()
 {
    let root = workspace_root();
    let temp_root = unique_temp_root("start_sh_force_kill_cleanup");
    let bin_dir = temp_root.join("bin");
    let fake_tools_dir = temp_root.join("fake-tools");
    let target_debug_dir = temp_root.join("target").join("debug");

    fs::create_dir_all(&bin_dir).expect("temp bin dir should be created");
    fs::create_dir_all(&fake_tools_dir).expect("temp fake-tools dir should be created");
    fs::create_dir_all(&target_debug_dir).expect("temp debug target dir should be created");

    for file_name in ["start-local.sh", "install-local.sh", "init-config-local.sh"] {
        fs::copy(root.join("bin").join(file_name), bin_dir.join(file_name))
            .unwrap_or_else(|_| panic!("failed to copy {file_name} into temp bin dir"));
    }

    let start_script_path = bin_dir.join("start-local.sh");
    let start_script = fs::read_to_string(&start_script_path)
        .expect("copied start-local.sh should be readable for test acceleration");
    let accelerated_start_script = start_script
        .replacen("for _ in $(seq 1 30); do", "for _ in $(seq 1 5); do", 1)
        .replacen("for _ in $(seq 1 5); do", "for _ in $(seq 1 1); do", 1)
        .replace("sleep 1", "sleep 0.1");
    assert_ne!(
        start_script, accelerated_start_script,
        "test acceleration should rewrite the copied start-local.sh loops"
    );
    fs::write(&start_script_path, accelerated_start_script)
        .expect("accelerated start-local.sh should be written");

    fs::write(
        fake_tools_dir.join("cargo"),
        "#!/usr/bin/env bash\nexit 0\n",
    )
    .expect("fake cargo should be written");
    fs::write(fake_tools_dir.join("curl"), "#!/usr/bin/env bash\nexit 1\n")
        .expect("fake curl should be written");
    fs::write(fake_tools_dir.join("wget"), "#!/usr/bin/env bash\nexit 1\n")
        .expect("fake wget should be written");

    let Some(bash_path) = resolve_usable_bash() else {
        eprintln!(
            "skipping start-local.sh force-kill cleanup regression because no usable bash runtime is available"
        );
        let _ = fs::remove_dir_all(&temp_root);
        return;
    };

    for tool in ["cargo", "curl", "wget"] {
        let chmod_status = Command::new(&bash_path)
            .current_dir(&temp_root)
            .arg("-lc")
            .arg(format!(
                "chmod +x \"{}\"",
                fake_tools_dir.join(tool).display()
            ))
            .status()
            .expect("chmod should execute for fake shell tools");
        assert!(
            chmod_status.success(),
            "fake shell tool should become executable: {tool}"
        );
    }

    let probe_source = temp_root.join("health-timeout-force-kill-probe.rs");
    fs::write(
        &probe_source,
        r#"
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process;
use std::thread;
use std::time::Duration;

#[cfg(unix)]
unsafe extern "C" {
    fn signal(signum: i32, handler: usize) -> usize;
}

#[cfg(unix)]
const SIGTERM: i32 = 15;
#[cfg(unix)]
const SIG_IGN: usize = 1usize;

fn main() {
    let runtime_dir = env::var("SDKWORK_IM_RUNTIME_DIR").expect("runtime dir env should be present");
    let marker = PathBuf::from(runtime_dir).join("state").join("health-timeout-force-kill-probe.pid");
    if let Some(parent) = marker.parent() {
        fs::create_dir_all(parent).expect("marker parent dir should exist");
    }
    #[cfg(unix)]
    unsafe {
        signal(SIGTERM, SIG_IGN);
    }
    fs::write(&marker, process::id().to_string()).expect("marker should be written");
    thread::sleep(Duration::from_secs(300));
}
"#,
    )
    .expect("health-timeout force-kill probe source should be written");

    let rustc = std::env::var_os("RUSTC")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("rustc"));
    let rustc_status = Command::new(&rustc)
        .current_dir(&temp_root)
        .args([
            probe_source
                .to_str()
                .expect("probe source path should be valid"),
            "-o",
            target_debug_dir
                .join("local-minimal-node")
                .to_str()
                .expect("probe path should be valid"),
        ])
        .status()
        .expect("rustc should compile the shell force-kill probe executable");
    assert!(
        rustc_status.success(),
        "rustc should successfully compile the shell force-kill probe executable"
    );

    let chmod_probe_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!(
            "chmod +x \"{}\"",
            target_debug_dir.join("local-minimal-node").display()
        ))
        .status()
        .expect("chmod should execute for the shell probe binary");
    assert!(
        chmod_probe_status.success(),
        "compiled shell probe should become executable"
    );

    let original_path =
        std::env::var_os("PATH").expect("PATH must be available to run lifecycle scripts");
    let temp_path = format!(
        "{}:{}",
        fake_tools_dir.display(),
        PathBuf::from(original_path).display()
    );

    let output = Command::new(&bash_path)
        .current_dir(&temp_root)
        .env("PATH", &temp_path)
        .arg("bin/start-local.sh")
        .output()
        .expect("start-local.sh should execute against temp workspace");
    assert!(
        !output.status.success(),
        "health-timeout shell probe should cause the wrapper to fail startup"
    );

    let marker_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("state")
        .join("health-timeout-force-kill-probe.pid");
    assert!(
        wait_for_path(&marker_file, Duration::from_secs(2)),
        "shell probe should record its pid before the wrapper returns"
    );

    let probe_pid = fs::read_to_string(&marker_file)
        .expect("probe pid marker should be readable")
        .trim()
        .parse::<u32>()
        .expect("probe pid marker should contain a numeric pid");
    let pid_file = temp_root
        .join(".runtime")
        .join("local-minimal")
        .join("pids")
        .join("local-minimal-node.pid");

    let probe_running_status = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!("kill -0 {probe_pid}"))
        .status()
        .expect("probe running-state query should execute");
    let probe_still_running = probe_running_status.success();

    let _ = Command::new(&bash_path)
        .current_dir(&temp_root)
        .arg("-lc")
        .arg(format!("kill -9 {probe_pid} >/dev/null 2>&1 || true"))
        .status();

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        stderr.contains("local-minimal-node did not become healthy within 30 seconds")
            || stderr.contains("local-minimal-node exited before becoming ready"),
        "start-local.sh should surface startup rollback failure details. actual stderr: {stderr}"
    );
    assert!(
        !probe_still_running,
        "start-local.sh must force-kill the launched process when it ignores SIGTERM during startup rollback"
    );
    assert!(
        !pid_file.exists(),
        "start-local.sh must clear the pid file once startup rollback finishes"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_server_templates_freeze_cross_platform_contract() {
    let root = workspace_root();
    let server_template_path = root
        .join("deployments")
        .join("templates")
        .join("server.yaml.example");
    let server_env_template_path = root
        .join("deployments")
        .join("templates")
        .join("server.env.example");
    let postgresql_template_path = root
        .join("deployments")
        .join("templates")
        .join("postgresql.yaml.example");
    let quickstart_env_template_path = root
        .join("deployments")
        .join("templates")
        .join("quickstart-server-compose.env.example");

    let server_template = fs::read_to_string(&server_template_path).unwrap_or_else(|_| {
        panic!(
            "missing server template file: {}",
            server_template_path.display()
        )
    });
    let server_env_template = fs::read_to_string(&server_env_template_path).unwrap_or_else(|_| {
        panic!(
            "missing server env template file: {}",
            server_env_template_path.display()
        )
    });
    let postgresql_template = fs::read_to_string(&postgresql_template_path).unwrap_or_else(|_| {
        panic!(
            "missing postgresql template file: {}",
            postgresql_template_path.display()
        )
    });
    let quickstart_env_template =
        fs::read_to_string(&quickstart_env_template_path).unwrap_or_else(|_| {
            panic!(
                "missing quickstart server env template file: {}",
                quickstart_env_template_path.display()
            )
        });

    for contract in [
        "instance:",
        "name: default",
        "network:",
        "bindAddress:",
        "publicEndpoints:",
        "baseUrl:",
        "apiBaseUrl:",
        "websocketBaseUrl:",
        "runtime:",
        "dataDir:",
        "logDir:",
        "runDir:",
    ] {
        assert!(
            server_template.contains(contract),
            "server.yaml.example must freeze the server config contract `{contract}`"
        );
    }

    for contract in [
        "SDKWORK_IM_SERVER_INSTANCE=",
        "SDKWORK_IM_SERVER_CONFIG_DIR=",
        "SDKWORK_IM_SERVER_DATA_DIR=",
        "SDKWORK_IM_SERVER_LOG_DIR=",
        "SDKWORK_IM_SERVER_RUN_DIR=",
        "SDKWORK_IM_SERVER_BASE_URL=",
        "SDKWORK_IM_SERVER_API_BASE_URL=",
        "SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=",
        "SDKWORK_IM_BROWSER_ORIGINS=",
        "SDKWORK_IM_PC_API_UPSTREAM=",
        "SDKWORK_IM_APP_CONTEXT_REQUIRE_SIGNATURE=true",
        "SDKWORK_IM_APP_CONTEXT_SIGNATURE_SECRET=",
    ] {
        assert!(
            server_env_template.contains(contract),
            "server.env.example must freeze the env overlay contract `{contract}`"
        );
    }
    assert!(
        !server_env_template.contains(
            "SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=wss://realtime.example.com/im/v3/api/realtime/ws"
        ),
        "server.env.example must document the websocket base URL, not the SDK-owned realtime websocket endpoint"
    );

    for contract in [
        "provider: postgresql",
        "connection:",
        "passwordFile:",
        "schema:",
        "provisioningMode:",
        "migrationMode:",
        "verify-only",
        "bootstrap-schema",
        "create-db-and-schema",
    ] {
        assert!(
            postgresql_template.contains(contract),
            "postgresql.yaml.example must freeze the PostgreSQL contract `{contract}`"
        );
    }

    for contract in [
        "SDKWORK_IM_SERVER_IMAGE=",
        "SDKWORK_IM_SERVER_BASE_URL=",
        "SDKWORK_IM_SERVER_API_BASE_URL=",
        "SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=",
        "SDKWORK_IM_BROWSER_ORIGINS=",
        "SDKWORK_IM_PC_API_UPSTREAM=",
        "SDKWORK_IM_POSTGRES_HOST=",
        "SDKWORK_IM_POSTGRES_DATABASE=",
    ] {
        assert!(
            quickstart_env_template.contains(contract),
            "quickstart-server-compose.env.example must align with the same server config model `{contract}`"
        );
    }
    assert!(
        !quickstart_env_template.contains(
            "SDKWORK_IM_SERVER_WEBSOCKET_BASE_URL=ws://127.0.0.1:18080/im/v3/api/realtime/ws"
        ),
        "quickstart-server-compose.env.example must document the websocket base URL, not the SDK-owned realtime websocket endpoint"
    );
}

#[test]
fn test_server_install_scripts_expose_consistent_help_surface() {
    let root = workspace_root();

    for script in [
        root.join("bin").join("install-server.ps1"),
        root.join("bin").join("install-server.sh"),
        root.join("bin").join("init-config-server.ps1"),
        root.join("bin").join("init-config-server.sh"),
    ] {
        let content = fs::read_to_string(&script)
            .unwrap_or_else(|_| panic!("missing server lifecycle script: {}", script.display()));

        for contract in [
            "instance",
            "config",
            "data",
            "log",
            "run",
            "non-interactive",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "{} must advertise the `{contract}` help contract",
                script.display()
            );
        }
    }

    for script in [
        root.join("bin").join("init-config-server.ps1"),
        root.join("bin").join("init-config-server.sh"),
    ] {
        let content = fs::read_to_string(&script)
            .unwrap_or_else(|_| panic!("missing server config script: {}", script.display()));
        assert!(
            content.to_ascii_lowercase().contains("browser-origins"),
            "{} must advertise the browser CORS origin allowlist contract",
            script.display()
        );
        assert!(
            !content.contains("ws://127.0.0.1:18080/im/v3/api/realtime/ws"),
            "{} must default websocket-base-url to the websocket base origin, not the SDK-owned realtime endpoint",
            script.display()
        );
        assert!(
            content.contains("SDKWORK_IM_BROWSER_ORIGINS"),
            "{} must materialize the browser CORS origin allowlist in server.env",
            script.display()
        );
    }

    for script in [
        root.join("bin").join("install-server.cmd"),
        root.join("bin").join("init-config-server.cmd"),
    ] {
        let content = fs::read_to_string(&script)
            .unwrap_or_else(|_| panic!("missing CMD server wrapper: {}", script.display()));
        assert!(
            content.contains("_cmd-forward-powershell.cmd"),
            "{} must remain a thin forwarder to the PowerShell implementation",
            script.display()
        );
    }
}

#[test]
fn test_server_storage_and_verify_scripts_freeze_postgresql_contract() {
    let root = workspace_root();
    let init_storage_ps1_path = root.join("bin").join("init-storage-server.ps1");
    let init_storage_sh_path = root.join("bin").join("init-storage-server.sh");
    let verify_ps1_path = root.join("bin").join("verify-server.ps1");
    let verify_sh_path = root.join("bin").join("verify-server.sh");

    let init_storage_ps1 = fs::read_to_string(&init_storage_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing init-storage-server.ps1: {}",
            init_storage_ps1_path.display()
        )
    });
    let init_storage_sh = fs::read_to_string(&init_storage_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing init-storage-server.sh: {}",
            init_storage_sh_path.display()
        )
    });
    let verify_ps1 = fs::read_to_string(&verify_ps1_path)
        .unwrap_or_else(|_| panic!("missing verify-server.ps1: {}", verify_ps1_path.display()));
    let verify_sh = fs::read_to_string(&verify_sh_path)
        .unwrap_or_else(|_| panic!("missing verify-server.sh: {}", verify_sh_path.display()));

    for content in [&init_storage_ps1, &init_storage_sh] {
        for contract in [
            "verify-only",
            "bootstrap-schema",
            "create-db-and-schema",
            "postgresql",
            "storage",
            "report",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "init-storage-server scripts must freeze the PostgreSQL lifecycle contract `{contract}`"
            );
        }
    }

    for content in [&verify_ps1, &verify_sh] {
        for contract in [
            "config",
            "storage",
            "json",
            "text",
            "ready",
            "release-gate",
            "bundle",
            "decisionstatus",
            "platform",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "verify-server scripts must freeze the verification contract `{contract}`"
            );
        }
    }

    assert!(
        verify_ps1.contains("verify-server-release-contracts.mjs"),
        "verify-server.ps1 must delegate release-contract auditing to the shared helper"
    );
    assert!(
        verify_sh.contains("verify-server-release-contracts.mjs"),
        "verify-server.sh must delegate release-contract auditing to the shared helper"
    );
    assert!(
        !verify_ps1.contains("function Resolve-ReleaseWorkspaceRoot"),
        "verify-server.ps1 must not keep the obsolete inline release workspace resolver once helper delegation is in place"
    );
    assert!(
        !verify_sh.contains("resolve_release_workspace_root()"),
        "verify-server.sh must not keep the obsolete inline release workspace resolver once helper delegation is in place"
    );
}

#[test]
fn test_verify_server_ps1_can_audit_machine_readable_release_contract_bundle() {
    let root = workspace_root();
    let temp_root = unique_temp_root("verify_server_release_contracts");
    let config_dir = temp_root.join("config");
    let storage_dir = config_dir.join("storage");
    let secrets_dir = config_dir.join("secrets");
    fs::create_dir_all(&storage_dir).expect("storage dir should be created");
    fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");

    fs::write(
        config_dir.join("server.yaml"),
        r#"instance:
  name: demo
network:
  bindAddress: 127.0.0.1:8080
publicEndpoints:
  baseUrl: http://127.0.0.1:8080
runtime:
  dataRoot: ./data
"#,
    )
    .expect("server.yaml should be written");
    fs::write(
        storage_dir.join("postgresql.yaml"),
        r#"provider: postgresql
connection:
  host: 127.0.0.1
database: sdkwork_im
username: sdkwork_im
passwordFile: ./secrets/postgresql.password
migrationMode: validate
"#,
    )
    .expect("postgresql.yaml should be written");
    fs::write(secrets_dir.join("postgresql.password"), "demo-secret\n")
        .expect("postgresql password file should be written");

    let release_gate_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/verify-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-OutputFormat",
            "json",
            "-ReleaseGatePath",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
        ])
        .output()
        .expect("verify-server.ps1 should execute");

    assert!(
        output.status.success(),
        "verify-server.ps1 should accept release gate auditing and still emit a structured report"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "verify-server.ps1 should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["product"], "sdkwork-im-server");
    assert_eq!(report["instance"], "demo");
    assert_eq!(report["configValid"], true);
    assert_eq!(report["storageValid"], true);
    assert_eq!(report["ready"], true);
    assert_eq!(report["releaseContracts"]["enabled"], true);
    assert_eq!(
        report["releaseContracts"]["gatePath"],
        release_gate_path.display().to_string()
    );
    assert_eq!(report["releaseContracts"]["bundleId"], "wave-d-2026-04-08");
    assert_eq!(report["releaseContracts"]["wave"], "wave-d");
    assert_eq!(
        report["releaseContracts"]["decisionStatus"],
        "pending_go_no_go"
    );
    assert_eq!(
        report["releaseContracts"]["canonicalStartupCommand"],
        "sdkwork-im-server --config <config-root>/server.yaml"
    );
    assert_eq!(report["releaseContracts"]["packageArtifactCount"], 7);
    assert_eq!(report["releaseContracts"]["platformCount"], 3);
    assert_eq!(report["releaseContracts"]["contractsValid"], true);
    assert_eq!(report["releaseContracts"]["semanticIssueCount"], 0);
    assert!(
        report["releaseContracts"]["semanticCheckCount"]
            .as_u64()
            .expect("semanticCheckCount should be numeric")
            > 0,
        "verify-server.ps1 must report that semantic release-contract checks actually ran"
    );
    assert_eq!(
        report["releaseContracts"]["missing"]
            .as_array()
            .expect("releaseContracts.missing should be an array")
            .len(),
        0
    );

    let release_platforms = report["releaseContracts"]["platforms"]
        .as_array()
        .expect("releaseContracts.platforms should be an array");
    for platform in ["linux", "macos", "windows"] {
        assert!(
            release_platforms
                .iter()
                .any(|entry| entry.as_str() == Some(platform)),
            "releaseContracts.platforms must contain {platform}"
        );
    }
    assert_eq!(
        report["releaseContracts"]["semanticIssues"]
            .as_array()
            .expect("releaseContracts.semanticIssues should be an array")
            .len(),
        0
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_verify_server_ps1_detects_semantic_release_contract_mismatches_even_when_files_exist() {
    let root = workspace_root();
    let temp_root = unique_temp_root("verify_server_release_semantic_mismatch");
    let config_dir = temp_root.join("config");
    let storage_dir = config_dir.join("storage");
    let secrets_dir = config_dir.join("secrets");
    fs::create_dir_all(&storage_dir).expect("storage dir should be created");
    fs::create_dir_all(&secrets_dir).expect("secrets dir should be created");

    fs::write(
        config_dir.join("server.yaml"),
        r#"instance:
  name: demo
network:
  bindAddress: 127.0.0.1:8080
publicEndpoints:
  baseUrl: http://127.0.0.1:8080
runtime:
  dataRoot: ./data
"#,
    )
    .expect("server.yaml should be written");
    fs::write(
        storage_dir.join("postgresql.yaml"),
        r#"provider: postgresql
connection:
  host: 127.0.0.1
database: sdkwork_im
username: sdkwork_im
passwordFile: ./secrets/postgresql.password
migrationMode: validate
"#,
    )
    .expect("postgresql.yaml should be written");
    fs::write(secrets_dir.join("postgresql.password"), "demo-secret\n")
        .expect("postgresql password file should be written");

    for relative_path in [
        "artifacts/releases/wave-d-2026-04-08/server/release-gate.json",
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
        "docs/review/step-13-release-readiness-2026-04-08.md",
        "docs/review/step-13-go-no-go清单-2026-04-08.md",
    ] {
        let source_path = root.join(relative_path);
        let source_content = fs::read_to_string(&source_path)
            .unwrap_or_else(|_| panic!("missing source fixture: {}", source_path.display()));
        write_file_with_parents(&temp_root.join(relative_path), &source_content);
    }

    let release_gate_path = temp_root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");
    let mut gate_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&release_gate_path).unwrap_or_else(|_| {
            panic!(
                "missing temp release gate fixture: {}",
                release_gate_path.display()
            )
        }))
        .expect("temp release gate fixture should be valid json");
    gate_json["platformGateChecks"][2]["requiredPackageIds"] =
        serde_json::json!(["windows-zip", "windows-appx"]);
    fs::write(
        &release_gate_path,
        serde_json::to_string_pretty(&gate_json)
            .expect("mutated release gate should serialize cleanly"),
    )
    .expect("mutated release gate should be written");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/verify-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-OutputFormat",
            "json",
            "-ReleaseGatePath",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
        ])
        .output()
        .expect("verify-server.ps1 should execute");

    assert!(
        output.status.success(),
        "verify-server.ps1 should emit a structured report even when semantic release checks fail"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "verify-server.ps1 should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["configValid"], true);
    assert_eq!(report["storageValid"], true);
    assert_eq!(report["ready"], false);
    assert_eq!(report["releaseContracts"]["enabled"], true);
    assert_eq!(report["releaseContracts"]["contractsValid"], false);
    assert_eq!(
        report["releaseContracts"]["missing"]
            .as_array()
            .expect("releaseContracts.missing should be an array")
            .len(),
        0,
        "this regression must prove semantic validation, not missing-file validation"
    );
    assert!(
        report["releaseContracts"]["semanticIssueCount"]
            .as_u64()
            .expect("semanticIssueCount should be numeric")
            > 0,
        "semantic mismatch must increment semanticIssueCount"
    );
    assert!(
        report["releaseContracts"]["semanticIssues"]
            .as_array()
            .expect("releaseContracts.semanticIssues should be an array")
            .iter()
            .any(|entry| {
                entry.as_str() == Some("platform:windows:required-package-ids-mismatch")
            }),
        "verify-server.ps1 must report a stable semantic mismatch code for platform package-id drift"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_server_service_surface_is_frozen() {
    let root = workspace_root();
    let systemd_unit_path = root
        .join("deployments")
        .join("systemd")
        .join("sdkwork-im-server.service");
    let launchd_plist_path = root
        .join("deployments")
        .join("launchd")
        .join("com.sdkwork.im.server.plist");
    let windows_service_xml_path = root
        .join("deployments")
        .join("windows-service")
        .join("SdkworkImServer.xml");
    let install_service_ps1_path = root.join("bin").join("install-service-server.ps1");
    let install_service_sh_path = root.join("bin").join("install-service-server.sh");
    let status_server_ps1_path = root.join("bin").join("status-server.ps1");
    let status_server_sh_path = root.join("bin").join("status-server.sh");

    let systemd_unit = fs::read_to_string(&systemd_unit_path).unwrap_or_else(|_| {
        panic!(
            "missing systemd server unit template: {}",
            systemd_unit_path.display()
        )
    });
    let launchd_plist = fs::read_to_string(&launchd_plist_path).unwrap_or_else(|_| {
        panic!(
            "missing launchd server plist template: {}",
            launchd_plist_path.display()
        )
    });
    let windows_service_xml = fs::read_to_string(&windows_service_xml_path).unwrap_or_else(|_| {
        panic!(
            "missing windows service wrapper template: {}",
            windows_service_xml_path.display()
        )
    });
    let install_service_ps1 = fs::read_to_string(&install_service_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing install-service-server.ps1: {}",
            install_service_ps1_path.display()
        )
    });
    let install_service_sh = fs::read_to_string(&install_service_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing install-service-server.sh: {}",
            install_service_sh_path.display()
        )
    });
    let status_server_ps1 = fs::read_to_string(&status_server_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing status-server.ps1: {}",
            status_server_ps1_path.display()
        )
    });
    let status_server_sh = fs::read_to_string(&status_server_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing status-server.sh: {}",
            status_server_sh_path.display()
        )
    });

    for contract in [
        "[Unit]",
        "[Service]",
        "[Install]",
        "sdkwork-im-server",
        "server.env",
        "server.yaml",
    ] {
        assert!(
            systemd_unit.contains(contract),
            "sdkwork-im-server.service must freeze the systemd contract `{contract}`"
        );
    }

    for contract in [
        "<plist",
        "<key>Label</key>",
        "com.sdkwork.im.server",
        "<key>ProgramArguments</key>",
        "sdkwork-im-server",
        "server.yaml",
        "RunAtLoad",
        "KeepAlive",
        "StandardOutPath",
        "StandardErrorPath",
    ] {
        assert!(
            launchd_plist.contains(contract),
            "com.sdkwork.im.server.plist must freeze the launchd contract `{contract}`"
        );
    }

    for contract in [
        "<service>",
        "<id>SdkworkImServer</id>",
        "<name>SdkworkImServer</name>",
        "sdkwork-im-server.exe",
        "--config",
        "server.yaml",
        "<logpath>",
        "<onfailure",
    ] {
        assert!(
            windows_service_xml.contains(contract),
            "SdkworkImServer.xml must freeze the Windows Service wrapper contract `{contract}`"
        );
    }

    for content in [
        &install_service_ps1,
        &install_service_sh,
        &status_server_ps1,
        &status_server_sh,
    ] {
        for contract in [
            "service", "systemd", "launchd", "instance", "config", "status",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "server service scripts must freeze the contract `{contract}`"
            );
        }
    }

    for content in [&status_server_ps1, &status_server_sh] {
        for contract in [
            "release-gate",
            "bundle",
            "decisionstatus",
            "platform",
            "json",
            "output",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "status-server scripts must freeze the release contract audit surface `{contract}`"
            );
        }
    }

    assert!(
        status_server_ps1.contains("verify-server-release-contracts.mjs"),
        "status-server.ps1 must delegate release-contract auditing to the shared helper"
    );
    assert!(
        status_server_sh.contains("verify-server-release-contracts.mjs"),
        "status-server.sh must delegate release-contract auditing to the shared helper"
    );
    assert!(
        !status_server_ps1.contains("function Resolve-ReleaseWorkspaceRoot"),
        "status-server.ps1 must not keep the obsolete inline release workspace resolver once helper delegation is in place"
    );
    assert!(
        !status_server_sh.contains("resolve_release_workspace_root()"),
        "status-server.sh must not keep the obsolete inline release workspace resolver once helper delegation is in place"
    );
}

#[test]
fn test_status_server_ps1_can_summarize_machine_readable_release_contract_bundle() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_server_release_contracts");
    let config_dir = temp_root.join("config");
    let generated_dir = config_dir.join("generated");
    fs::create_dir_all(&generated_dir).expect("generated dir should be created");

    fs::write(
        generated_dir.join("sdkwork-im-server.service"),
        "[Unit]\nDescription=sdkwork-im-server\n",
    )
    .expect("generated systemd unit should be written");
    fs::write(
        generated_dir.join("com.sdkwork.im.server.plist"),
        "<plist><dict><key>Label</key><string>com.sdkwork.im.server</string></dict></plist>",
    )
    .expect("generated launchd plist should be written");
    fs::write(
        generated_dir.join("SdkworkImServer.xml"),
        "<service><id>SdkworkImServer</id></service>",
    )
    .expect("generated windows service xml should be written");
    fs::write(
        generated_dir.join("install-SdkworkImServer.ps1"),
        "Write-Host 'install service'\n",
    )
    .expect("generated install script should be written");
    fs::write(
        generated_dir.join("uninstall-SdkworkImServer.ps1"),
        "Write-Host 'uninstall service'\n",
    )
    .expect("generated uninstall script should be written");
    fs::write(
        config_dir.join("storage-init-report.json"),
        r#"{"product":"sdkwork-im-server","ready":true}"#,
    )
    .expect("storage-init-report.json should be written");

    let release_gate_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/status-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-OutputFormat",
            "json",
            "-ReleaseGatePath",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
        ])
        .output()
        .expect("status-server.ps1 should execute");

    assert!(
        output.status.success(),
        "status-server.ps1 should accept release gate auditing and emit a structured status report"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "status-server.ps1 should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["product"], "sdkwork-im-server");
    assert_eq!(report["instance"], "demo");
    assert_eq!(report["status"], "configuration-only skeleton");
    assert_eq!(report["config"], config_dir.display().to_string());
    assert_eq!(report["releaseContracts"]["enabled"], true);
    assert_eq!(report["releaseContracts"]["bundleId"], "wave-d-2026-04-08");
    assert_eq!(
        report["releaseContracts"]["decisionStatus"],
        "pending_go_no_go"
    );
    assert_eq!(report["releaseContracts"]["platformCount"], 3);
    assert_eq!(report["releaseContracts"]["contractsValid"], true);
    assert_eq!(report["releaseContracts"]["semanticIssueCount"], 0);
    assert!(
        report["releaseContracts"]["semanticCheckCount"]
            .as_u64()
            .expect("status-server semanticCheckCount should be numeric")
            > 0,
        "status-server.ps1 must surface that semantic release-contract checks actually ran"
    );
    assert_eq!(report["serviceContracts"]["systemd"]["exists"], true);
    assert_eq!(report["serviceContracts"]["launchd"]["exists"], true);
    assert_eq!(report["serviceContracts"]["windowsService"]["exists"], true);
    assert_eq!(report["storageReport"]["exists"], true);

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_status_server_ps1_reports_semantic_release_contract_drift() {
    let root = workspace_root();
    let temp_root = unique_temp_root("status_server_release_semantic_mismatch");
    let config_dir = temp_root.join("config");
    let generated_dir = config_dir.join("generated");
    fs::create_dir_all(&generated_dir).expect("generated dir should be created");

    fs::write(
        generated_dir.join("sdkwork-im-server.service"),
        "[Unit]\nDescription=sdkwork-im-server\n",
    )
    .expect("generated systemd unit should be written");
    fs::write(
        generated_dir.join("com.sdkwork.im.server.plist"),
        "<plist><dict><key>Label</key><string>com.sdkwork.im.server</string></dict></plist>",
    )
    .expect("generated launchd plist should be written");
    fs::write(
        generated_dir.join("SdkworkImServer.xml"),
        "<service><id>SdkworkImServer</id></service>",
    )
    .expect("generated windows service xml should be written");
    fs::write(
        generated_dir.join("install-SdkworkImServer.ps1"),
        "Write-Host 'install service'\n",
    )
    .expect("generated install script should be written");
    fs::write(
        generated_dir.join("uninstall-SdkworkImServer.ps1"),
        "Write-Host 'uninstall service'\n",
    )
    .expect("generated uninstall script should be written");
    fs::write(
        config_dir.join("storage-init-report.json"),
        r#"{"product":"sdkwork-im-server","ready":true}"#,
    )
    .expect("storage-init-report.json should be written");

    for relative_path in [
        "artifacts/releases/wave-d-2026-04-08/server/release-gate.json",
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
        "docs/review/step-13-release-readiness-2026-04-08.md",
        "docs/review/step-13-go-no-go清单-2026-04-08.md",
    ] {
        let source_path = root.join(relative_path);
        let source_content = fs::read_to_string(&source_path)
            .unwrap_or_else(|_| panic!("missing source fixture: {}", source_path.display()));
        write_file_with_parents(&temp_root.join(relative_path), &source_content);
    }

    let release_gate_path = temp_root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");
    let mut gate_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&release_gate_path).unwrap_or_else(|_| {
            panic!(
                "missing temp release gate fixture: {}",
                release_gate_path.display()
            )
        }))
        .expect("temp release gate fixture should be valid json");
    gate_json["platformGateChecks"][1]["requiredPackageIds"] =
        serde_json::json!(["macos-tar-gz", "macos-dmg"]);
    fs::write(
        &release_gate_path,
        serde_json::to_string_pretty(&gate_json)
            .expect("mutated release gate should serialize cleanly"),
    )
    .expect("mutated release gate should be written");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/status-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-OutputFormat",
            "json",
            "-ReleaseGatePath",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
        ])
        .output()
        .expect("status-server.ps1 should execute");

    assert!(
        output.status.success(),
        "status-server.ps1 should emit a structured report even when semantic release checks fail"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "status-server.ps1 should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["releaseContracts"]["enabled"], true);
    assert_eq!(report["releaseContracts"]["contractsValid"], false);
    assert!(
        report["releaseContracts"]["semanticIssueCount"]
            .as_u64()
            .expect("semanticIssueCount should be numeric")
            > 0,
        "status-server.ps1 must surface semantic drift through semanticIssueCount"
    );
    assert!(
        report["releaseContracts"]["semanticIssues"]
            .as_array()
            .expect("semanticIssues should be an array")
            .iter()
            .any(|entry| entry.as_str() == Some("platform:macos:required-package-ids-mismatch")),
        "status-server.ps1 must surface the stable semantic mismatch code"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_server_runtime_scripts_expose_consistent_help_surface() {
    let root = workspace_root();

    for script in [
        root.join("bin").join("start-server.ps1"),
        root.join("bin").join("start-server.sh"),
        root.join("bin").join("stop-server.ps1"),
        root.join("bin").join("stop-server.sh"),
        root.join("bin").join("restart-server.ps1"),
        root.join("bin").join("restart-server.sh"),
        root.join("bin").join("uninstall-service-server.ps1"),
        root.join("bin").join("uninstall-service-server.sh"),
    ] {
        let content = fs::read_to_string(&script)
            .unwrap_or_else(|_| panic!("missing server runtime script: {}", script.display()));

        for contract in ["instance", "config", "service", "status"] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "{} must advertise the runtime contract `{contract}`",
                script.display()
            );
        }
    }

    let start_ps1 = fs::read_to_string(root.join("bin").join("start-server.ps1"))
        .expect("start-server.ps1 should exist");
    let start_sh = fs::read_to_string(root.join("bin").join("start-server.sh"))
        .expect("start-server.sh should exist");
    for content in [&start_ps1, &start_sh] {
        for contract in ["foreground", "health", "binary", "log", "run"] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "start-server scripts must freeze the start contract `{contract}`"
            );
        }
    }

    let stop_ps1 = fs::read_to_string(root.join("bin").join("stop-server.ps1"))
        .expect("stop-server.ps1 should exist");
    let stop_sh = fs::read_to_string(root.join("bin").join("stop-server.sh"))
        .expect("stop-server.sh should exist");
    for content in [&stop_ps1, &stop_sh] {
        for contract in ["pid", "stop", "run"] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "stop-server scripts must freeze the stop contract `{contract}`"
            );
        }
    }

    for script in [
        root.join("bin").join("start-server.cmd"),
        root.join("bin").join("stop-server.cmd"),
        root.join("bin").join("restart-server.cmd"),
        root.join("bin").join("uninstall-service-server.cmd"),
    ] {
        let content = fs::read_to_string(&script)
            .unwrap_or_else(|_| panic!("missing CMD server wrapper: {}", script.display()));
        assert!(
            content.contains("_cmd-forward-powershell.cmd"),
            "{} must remain a thin forwarder to the PowerShell implementation",
            script.display()
        );
    }
}

#[test]
fn test_install_service_server_ps1_renders_instance_specific_systemd_unit() {
    let root = workspace_root();
    let temp_root = unique_temp_root("server_service_render");
    let config_dir = temp_root.join("config");
    let install_root = temp_root.join("install");
    fs::create_dir_all(&config_dir).expect("config dir should be created");
    fs::create_dir_all(&install_root).expect("install dir should be created");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/install-service-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-InstallRoot",
            install_root
                .to_str()
                .expect("install root should be a valid path"),
        ])
        .output()
        .expect("install-service-server.ps1 should execute");

    assert!(
        output.status.success(),
        "install-service-server.ps1 should render a generated service contract"
    );

    let generated_unit = config_dir
        .join("generated")
        .join("sdkwork-im-server.service");
    let unit_content = fs::read_to_string(&generated_unit).unwrap_or_else(|_| {
        panic!(
            "generated systemd unit should exist at {}",
            generated_unit.display()
        )
    });

    assert!(
        unit_content.contains(&format!(
            "EnvironmentFile={}",
            config_dir.join("server.env").display()
        )),
        "generated systemd unit must point at the selected instance config root"
    );
    assert!(
        unit_content.contains(&format!(
            "ExecStart={}/bin/sdkwork-im-server --config {}",
            install_root.display(),
            config_dir.join("server.yaml").display()
        )),
        "generated systemd unit must point at the selected install root and instance config"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_install_service_server_ps1_renders_instance_specific_launchd_plist() {
    let root = workspace_root();
    let temp_root = unique_temp_root("server_launchd_render");
    let config_dir = temp_root.join("config");
    let install_root = temp_root.join("install");
    let log_dir = temp_root.join("logs");
    fs::create_dir_all(&config_dir).expect("config dir should be created");
    fs::create_dir_all(&install_root).expect("install dir should be created");
    fs::create_dir_all(&log_dir).expect("log dir should be created");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/install-service-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-InstallRoot",
            install_root
                .to_str()
                .expect("install root should be a valid path"),
            "-LogDir",
            log_dir.to_str().expect("log dir should be a valid path"),
        ])
        .output()
        .expect("install-service-server.ps1 should execute");

    assert!(
        output.status.success(),
        "install-service-server.ps1 should render a generated launchd contract"
    );

    let generated_plist = config_dir
        .join("generated")
        .join("com.sdkwork.im.server.plist");
    let plist_content = fs::read_to_string(&generated_plist).unwrap_or_else(|_| {
        panic!(
            "generated launchd plist should exist at {}",
            generated_plist.display()
        )
    });

    assert!(
        plist_content.contains("<string>com.sdkwork.im.server</string>"),
        "generated launchd plist must preserve the canonical launchd label"
    );
    assert!(
        plist_content.contains(&format!(
            "<string>{}/bin/sdkwork-im-server</string>",
            install_root.display()
        )),
        "generated launchd plist must point at the selected install root"
    );
    assert!(
        plist_content.contains("<string>--config</string>")
            && plist_content.contains(&format!(
                "<string>{}</string>",
                config_dir.join("server.yaml").display()
            )),
        "generated launchd plist must pass the selected instance config"
    );
    assert!(
        plist_content.contains(&format!(
            "<string>{}</string>",
            log_dir.join("sdkwork-im-server.out.log").display()
        )) && plist_content.contains(&format!(
            "<string>{}</string>",
            log_dir.join("sdkwork-im-server.err.log").display()
        )),
        "generated launchd plist must render instance-specific stdout and stderr log targets"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_install_service_server_ps1_renders_instance_specific_windows_service_contract() {
    let root = workspace_root();
    let temp_root = unique_temp_root("server_windows_service_render");
    let config_dir = temp_root.join("config");
    let install_root = temp_root.join("install");
    let log_dir = temp_root.join("logs");
    fs::create_dir_all(&config_dir).expect("config dir should be created");
    fs::create_dir_all(&install_root).expect("install dir should be created");
    fs::create_dir_all(&log_dir).expect("log dir should be created");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/install-service-server.ps1",
            "-InstanceName",
            "demo",
            "-ConfigDir",
            config_dir
                .to_str()
                .expect("config dir should be a valid path"),
            "-InstallRoot",
            install_root
                .to_str()
                .expect("install root should be a valid path"),
            "-LogDir",
            log_dir.to_str().expect("log dir should be a valid path"),
        ])
        .output()
        .expect("install-service-server.ps1 should execute");

    assert!(
        output.status.success(),
        "install-service-server.ps1 should render a generated windows service contract"
    );

    let generated_xml = config_dir.join("generated").join("SdkworkImServer.xml");
    let generated_install_script = config_dir
        .join("generated")
        .join("install-SdkworkImServer.ps1");
    let generated_uninstall_script = config_dir
        .join("generated")
        .join("uninstall-SdkworkImServer.ps1");

    let xml_content = fs::read_to_string(&generated_xml).unwrap_or_else(|_| {
        panic!(
            "generated windows service wrapper config should exist at {}",
            generated_xml.display()
        )
    });
    let install_script_content =
        fs::read_to_string(&generated_install_script).unwrap_or_else(|_| {
            panic!(
                "generated windows service install script should exist at {}",
                generated_install_script.display()
            )
        });
    let uninstall_script_content =
        fs::read_to_string(&generated_uninstall_script).unwrap_or_else(|_| {
            panic!(
                "generated windows service uninstall script should exist at {}",
                generated_uninstall_script.display()
            )
        });

    assert!(
        xml_content.contains("<id>SdkworkImServer</id>")
            && xml_content.contains("<name>SdkworkImServer</name>"),
        "generated windows service wrapper config must preserve the canonical service identity"
    );
    assert!(
        xml_content.contains(&format!(
            "<executable>{}\\bin\\sdkwork-im-server.exe</executable>",
            install_root.display()
        )),
        "generated windows service wrapper config must point at the selected install root"
    );
    assert!(
        xml_content.contains(&format!(
            "<arguments>--config &quot;{}&quot;</arguments>",
            config_dir.join("server.yaml").display()
        )),
        "generated windows service wrapper config must pass the selected instance config"
    );
    assert!(
        xml_content.contains(&format!("<logpath>{}</logpath>", log_dir.display())),
        "generated windows service wrapper config must render the selected log directory"
    );
    assert!(
        install_script_content.contains("SdkworkImServer.exe")
            && install_script_content.contains(" install"),
        "generated windows service install script must invoke the service wrapper install entrypoint"
    );
    assert!(
        uninstall_script_content.contains("SdkworkImServer.exe")
            && uninstall_script_content.contains(" uninstall"),
        "generated windows service uninstall script must invoke the service wrapper uninstall entrypoint"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_server_runtime_uses_canonical_sdkwork_im_server_binary_contract() {
    let root = workspace_root();
    let gateway_cargo =
        fs::read_to_string(
            root.join("services")
                .join("sdkwork-im-gateway")
                .join("Cargo.toml"),
        )
            .expect("sdkwork-im-gateway Cargo.toml should exist");
    let start_ps1 = fs::read_to_string(root.join("bin").join("start-server.ps1"))
        .expect("start-server.ps1 should exist");
    let start_sh = fs::read_to_string(root.join("bin").join("start-server.sh"))
        .expect("start-server.sh should exist");

    assert!(
        gateway_cargo.contains("[[bin]]") && gateway_cargo.contains("name = \"sdkwork-im-server\""),
        "sdkwork-im-gateway package must publish the canonical sdkwork-im-server binary entrypoint"
    );
    assert!(
        start_ps1.contains("target\\release\\sdkwork-im-server.exe")
            && start_ps1.contains("target\\debug\\sdkwork-im-server.exe"),
        "start-server.ps1 must prefer the canonical sdkwork-im-server build artifacts"
    );
    assert!(
        start_sh.contains("target/release/sdkwork-im-server")
            && start_sh.contains("target/debug/sdkwork-im-server"),
        "start-server.sh must prefer the canonical sdkwork-im-server build artifacts"
    );
}

#[test]
fn test_server_docs_and_release_bundle_freeze_external_postgresql_install_contract() {
    let root = workspace_root();
    let deployment_readme_path = root.join("docs").join("部署").join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");
    let postgres_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本配置与PostgreSQL接入.md");
    let service_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本service托管标准.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");

    let deployment_readme = fs::read_to_string(&deployment_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing deployment README: {}",
            deployment_readme_path.display()
        )
    });
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing server install doc: {}", install_doc_path.display()));
    let postgres_doc = fs::read_to_string(&postgres_doc_path).unwrap_or_else(|_| {
        panic!(
            "missing server postgresql doc: {}",
            postgres_doc_path.display()
        )
    });
    let service_doc = fs::read_to_string(&service_doc_path)
        .unwrap_or_else(|_| panic!("missing server service doc: {}", service_doc_path.display()));
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });

    for content in [&deployment_readme, &install_doc] {
        for contract in [
            "sdkwork-im-server",
            "install-server",
            "init-config-server",
            "init-storage-server",
            "verify-server",
            "plan-release-server",
            "sdkwork-im-gateway",
        ] {
            assert!(
                content.contains(contract),
                "server deployment docs must freeze the install contract `{contract}`"
            );
        }
    }

    for contract in ["openapi/index.json", "openapi/services", "docs/services"] {
        assert!(
            install_doc.contains(contract),
            "server install doc must freeze the unified gateway contract `{contract}`"
        );
    }

    for contract in [
        "postgresql",
        "external",
        "configuration file",
        "verify-only",
        "bootstrap-schema",
        "create-db-and-schema",
        "passwordFile",
        "migrationMode",
    ] {
        assert!(
            postgres_doc
                .to_ascii_lowercase()
                .contains(&contract.to_ascii_lowercase()),
            "server PostgreSQL doc must freeze the external configuration contract `{contract}`"
        );
    }

    for contract in [
        "systemd",
        "launchd",
        "windows service",
        "SdkworkImServer",
        "openapi/index.json",
        "openapi/services",
        "docs/services",
    ] {
        assert!(
            service_doc
                .to_ascii_lowercase()
                .contains(&contract.to_ascii_lowercase()),
            "server service doc must freeze the cross-platform service target `{contract}`"
        );
    }

    for contract in [
        "canonical",
        "payload",
        "sdkwork-im-server",
        "plan-release-server",
        "tar.gz",
        "zip",
        "deb",
        "rpm",
        "pkg",
        "msi",
    ] {
        assert!(
            releases_readme
                .to_ascii_lowercase()
                .contains(&contract.to_ascii_lowercase()),
            "artifacts/releases/README.md must freeze the server bundle/install contract `{contract}`"
        );
    }
}

#[test]
fn test_server_release_plan_scripts_freeze_machine_readable_release_contract_surface() {
    let root = workspace_root();
    let plan_release_ps1_path = root.join("bin").join("plan-release-server.ps1");
    let plan_release_sh_path = root.join("bin").join("plan-release-server.sh");
    let plan_release_cmd_path = root.join("bin").join("plan-release-server.cmd");
    let plan_release_helper_path = root.join("bin").join("plan-release-server-contracts.mjs");

    let plan_release_ps1 = fs::read_to_string(&plan_release_ps1_path).unwrap_or_else(|_| {
        panic!(
            "missing plan-release-server.ps1: {}",
            plan_release_ps1_path.display()
        )
    });
    let plan_release_sh = fs::read_to_string(&plan_release_sh_path).unwrap_or_else(|_| {
        panic!(
            "missing plan-release-server.sh: {}",
            plan_release_sh_path.display()
        )
    });
    let plan_release_cmd = fs::read_to_string(&plan_release_cmd_path).unwrap_or_else(|_| {
        panic!(
            "missing plan-release-server.cmd: {}",
            plan_release_cmd_path.display()
        )
    });
    let plan_release_helper = fs::read_to_string(&plan_release_helper_path).unwrap_or_else(|_| {
        panic!(
            "missing plan-release-server-contracts.mjs: {}",
            plan_release_helper_path.display()
        )
    });

    for content in [&plan_release_ps1, &plan_release_sh] {
        for contract in [
            "release-gate",
            "package-catalog",
            "release-execution",
            "platform",
            "checksum",
            "artifact-file-list",
            "json",
            "text",
            "plan",
        ] {
            assert!(
                content.to_ascii_lowercase().contains(contract),
                "plan-release-server scripts must freeze the release planning contract `{contract}`"
            );
        }
    }

    assert!(
        plan_release_cmd.contains("_cmd-forward-powershell.cmd"),
        "plan-release-server.cmd must remain a thin forwarder to the PowerShell implementation"
    );
    assert!(
        plan_release_ps1.contains("plan-release-server-contracts.mjs"),
        "plan-release-server.ps1 must delegate release-plan resolution to the shared helper"
    );
    assert!(
        plan_release_sh.contains("plan-release-server-contracts.mjs"),
        "plan-release-server.sh must delegate release-plan resolution to the shared helper"
    );
    assert!(
        plan_release_helper.contains("stagingReadmePath"),
        "plan-release-server-contracts.mjs must keep stagingReadmePath in the emitted platform plan surface"
    );
    assert!(
        plan_release_helper.contains("checksumCommandExample"),
        "plan-release-server-contracts.mjs must keep checksumCommandExample in the emitted platform plan surface"
    );
    assert!(
        plan_release_helper.contains("status"),
        "plan-release-server-contracts.mjs must keep platform execution status in the emitted plan surface"
    );
}

#[test]
fn test_plan_release_server_contract_helper_can_emit_machine_readable_release_execution_plan() {
    let root = workspace_root();
    let helper_path = root.join("bin").join("plan-release-server-contracts.mjs");
    let release_gate_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");

    let output = Command::new("node")
        .current_dir(&root)
        .arg(helper_path)
        .args([
            "--release-gate-path",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
            "--platform",
            "linux",
            "--format",
            "json",
        ])
        .output()
        .expect("plan-release-server-contracts.mjs should execute");

    assert!(
        output.status.success(),
        "plan-release-server-contracts.mjs should emit a structured release plan"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "plan-release-server-contracts.mjs should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["product"], "sdkwork-im-server");
    assert_eq!(report["bundleId"], "wave-d-2026-04-08");
    assert_eq!(report["selectedPlatform"], "linux");
    assert_eq!(report["contractsValid"], true);
    assert_eq!(report["platformPlanCount"], 1);
    let linux_plan = &report["platformPlans"]
        .as_array()
        .expect("platformPlans should be an array")[0];
    assert_eq!(linux_plan["platform"], "linux");
    assert_eq!(
        linux_plan["stagingReadmePath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md"
    );
    assert_eq!(linux_plan["status"], "template_only_pending_execution");
    assert_eq!(
        linux_plan["checksumCommandExample"],
        "sha256sum -b <artifact> >> ../../SHA256SUMS"
    );
}

#[test]
fn test_plan_release_server_contract_helper_detects_semantic_release_contract_drift() {
    let root = workspace_root();
    let temp_root = unique_temp_root("plan_release_semantic_mismatch");

    for relative_path in [
        "artifacts/releases/wave-d-2026-04-08/server/release-gate.json",
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
        "docs/review/step-13-release-readiness-2026-04-08.md",
        "docs/review/step-13-go-no-go清单-2026-04-08.md",
    ] {
        let source_path = root.join(relative_path);
        let source_content = fs::read_to_string(&source_path)
            .unwrap_or_else(|_| panic!("missing source fixture: {}", source_path.display()));
        write_file_with_parents(&temp_root.join(relative_path), &source_content);
    }

    let release_gate_path = temp_root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");
    let mut gate_json: serde_json::Value =
        serde_json::from_str(&fs::read_to_string(&release_gate_path).unwrap_or_else(|_| {
            panic!(
                "missing temp release gate fixture: {}",
                release_gate_path.display()
            )
        }))
        .expect("temp release gate fixture should be valid json");
    gate_json["platformGateChecks"][0]["requiredPackageIds"] =
        serde_json::json!(["linux-tar-gz", "linux-snap"]);
    fs::write(
        &release_gate_path,
        serde_json::to_string_pretty(&gate_json)
            .expect("mutated release gate should serialize cleanly"),
    )
    .expect("mutated release gate should be written");

    let helper_path = root.join("bin").join("plan-release-server-contracts.mjs");
    let output = Command::new("node")
        .current_dir(&root)
        .arg(helper_path)
        .args([
            "--release-gate-path",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
            "--platform",
            "linux",
            "--format",
            "json",
        ])
        .output()
        .expect("plan-release-server-contracts.mjs should execute");

    assert!(
        output.status.success(),
        "plan-release-server-contracts.mjs should emit a structured report even when semantic release checks fail"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "plan-release-server-contracts.mjs should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["selectedPlatform"], "linux");
    assert_eq!(report["contractsValid"], false);
    assert!(
        report["semanticIssueCount"]
            .as_u64()
            .expect("semanticIssueCount should be numeric")
            > 0,
        "plan-release-server-contracts.mjs must surface semantic drift"
    );
    assert!(
        report["semanticIssues"]
            .as_array()
            .expect("semanticIssues should be an array")
            .iter()
            .any(|entry| entry.as_str() == Some("platform:linux:required-package-ids-mismatch")),
        "plan-release-server-contracts.mjs must surface the stable semantic mismatch code"
    );

    let _ = fs::remove_dir_all(&temp_root);
}

#[test]
fn test_plan_release_server_ps1_can_emit_machine_readable_release_execution_plan() {
    let root = workspace_root();
    let release_gate_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");

    let output = Command::new("powershell")
        .current_dir(&root)
        .args([
            "-NoProfile",
            "-ExecutionPolicy",
            "Bypass",
            "-File",
            "bin/plan-release-server.ps1",
            "-ReleaseGatePath",
            release_gate_path
                .to_str()
                .expect("release gate path should be a valid path"),
            "-Platform",
            "windows",
            "-OutputFormat",
            "json",
        ])
        .output()
        .expect("plan-release-server.ps1 should execute");

    assert!(
        output.status.success(),
        "plan-release-server.ps1 should accept release gate input and emit a structured release plan"
    );

    let report: serde_json::Value = serde_json::from_slice(&output.stdout).unwrap_or_else(|_| {
        panic!(
            "plan-release-server.ps1 should emit valid json, got: {}",
            String::from_utf8_lossy(&output.stdout)
        )
    });

    assert_eq!(report["product"], "sdkwork-im-server");
    assert_eq!(report["bundleId"], "wave-d-2026-04-08");
    assert_eq!(report["wave"], "wave-d");
    assert_eq!(report["selectedPlatform"], "windows");
    assert_eq!(
        report["canonicalBuildCommand"],
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline"
    );
    assert_eq!(
        report["canonicalStartupCommand"],
        "sdkwork-im-server --config <config-root>/server.yaml"
    );
    assert_eq!(report["contractsValid"], true);
    assert_eq!(report["packageArtifactCount"], 7);
    assert_eq!(report["gateCheckCount"], 6);
    assert_eq!(report["platformPlanCount"], 1);

    let platform_plans = report["platformPlans"]
        .as_array()
        .expect("platformPlans should be an array");
    assert_eq!(platform_plans.len(), 1);
    let windows_plan = &platform_plans[0];
    assert_eq!(windows_plan["platform"], "windows");
    assert_eq!(windows_plan["serviceManager"], "windows-service");
    assert_eq!(
        windows_plan["stagingRoot"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts"
    );
    assert_eq!(
        windows_plan["acceptanceManifestPath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json"
    );
    assert_eq!(
        windows_plan["checksumCommandExample"],
        "Get-FileHash -Algorithm SHA256 <artifact> | Format-Table -HideTableHeaders >> ../../SHA256SUMS"
    );

    let package_ids = windows_plan["packageIds"]
        .as_array()
        .expect("windows platform plan packageIds should be an array");
    for package_id in ["windows-zip", "windows-msi"] {
        assert!(
            package_ids
                .iter()
                .any(|entry| entry.as_str() == Some(package_id)),
            "windows platform plan must contain package id {package_id}"
        );
    }
}

#[test]
fn test_cli_and_scripts_doc_freezes_server_release_helper_contract() {
    let root = workspace_root();
    let cli_doc_path = root
        .join("docs")
        .join("sites")
        .join("reference")
        .join("cli-and-scripts.md");

    let cli_doc = fs::read_to_string(&cli_doc_path)
        .unwrap_or_else(|_| panic!("missing cli reference doc: {}", cli_doc_path.display()));

    for contract in [
        "verify-server.*",
        "status-server.*",
        "plan-release-server.*",
        "semantic",
        "release-gate",
        "contractsValid",
        "verify-server-release-contracts.mjs",
        "plan-release-server-contracts.mjs",
    ] {
        assert!(
            cli_doc.contains(contract),
            "cli-and-scripts.md must freeze the server release helper contract `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_windows_service_wrapper_payload_contract() {
    let root = workspace_root();
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let windows_payload_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("windows-service")
        .join("README.md");

    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let windows_payload_readme =
        fs::read_to_string(&windows_payload_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing windows payload placeholder readme: {}",
                windows_payload_readme_path.display()
            )
        });

    for contract in [
        "bin/SdkworkImServer.exe",
        "deployments/windows-service/SdkworkImServer.xml",
        "wrapper-required",
        "server.yaml",
        "install-service-server",
    ] {
        assert!(
            releases_readme.contains(contract),
            "artifacts/releases/README.md must freeze the Windows Service payload contract `{contract}`"
        );
    }

    assert!(
        bundle_manifest
            .contains("artifacts/releases/wave-d-2026-04-08/server/windows-service/README.md"),
        "Wave D bundle manifest must reference the Windows Service payload placeholder readme"
    );

    for contract in [
        "template_only_pending_payload",
        "bin/sdkwork-im-server.exe",
        "bin/SdkworkImServer.exe",
        "deployments/windows-service/SdkworkImServer.xml",
        "generated/SdkworkImServer.xml",
        "install-SdkworkImServer.ps1",
        "uninstall-SdkworkImServer.ps1",
        "server.yaml",
    ] {
        assert!(
            windows_payload_readme.contains(contract),
            "Windows Service payload placeholder readme must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_root_payload_index_contract() {
    let root = workspace_root();
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let server_payload_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");

    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let server_payload_index =
        fs::read_to_string(&server_payload_index_path).unwrap_or_else(|_| {
            panic!(
                "missing server payload index readme: {}",
                server_payload_index_path.display()
            )
        });

    for contract in [
        "server/README.md",
        "canonical payload layout",
        "bin/sdkwork-im-server",
        "deployments/templates/server.yaml.example",
        "deployments/systemd/sdkwork-im-server.service",
        "deployments/launchd/com.sdkwork.im.server.plist",
        "deployments/windows-service/SdkworkImServer.xml",
    ] {
        assert!(
            releases_readme.contains(contract),
            "artifacts/releases/README.md must freeze the root server payload index contract `{contract}`"
        );
    }

    assert!(
        bundle_manifest.contains("artifacts/releases/wave-d-2026-04-08/server/README.md"),
        "Wave D bundle manifest must reference the server payload index readme"
    );

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server",
        "bin/sdkwork-im-server",
        "bin/sdkwork-im-server.exe",
        "deployments/templates/server.yaml.example",
        "deployments/templates/server.env.example",
        "deployments/templates/postgresql.yaml.example",
        "deployments/systemd/sdkwork-im-server.service",
        "deployments/launchd/com.sdkwork.im.server.plist",
        "deployments/windows-service/SdkworkImServer.xml",
        "windows-service/README.md",
        "server.yaml",
        "install-service-server",
    ] {
        assert!(
            server_payload_index.contains(contract),
            "server payload index readme must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_bin_and_deployments_payload_group_contracts() {
    let root = workspace_root();
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let server_payload_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");
    let server_bin_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("bin")
        .join("README.md");
    let server_deployments_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("deployments")
        .join("README.md");

    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let server_payload_index =
        fs::read_to_string(&server_payload_index_path).unwrap_or_else(|_| {
            panic!(
                "missing server payload index readme: {}",
                server_payload_index_path.display()
            )
        });
    let server_bin_readme = fs::read_to_string(&server_bin_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing server bin payload readme: {}",
            server_bin_readme_path.display()
        )
    });
    let server_deployments_readme = fs::read_to_string(&server_deployments_readme_path)
        .unwrap_or_else(|_| {
            panic!(
                "missing server deployments payload readme: {}",
                server_deployments_readme_path.display()
            )
        });

    for contract in [
        "bin/README.md",
        "deployments/README.md",
        "windows-service/README.md",
    ] {
        assert!(
            server_payload_index.contains(contract),
            "server payload index readme must reference child payload contract `{contract}`"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/bin/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/deployments/README.md",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference the child payload placeholder `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/bin",
        "bin/sdkwork-im-server",
        "bin/sdkwork-im-server.exe",
        "bin/SdkworkImServer.exe",
    ] {
        assert!(
            server_bin_readme.contains(contract),
            "server bin payload readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/deployments",
        "deployments/templates/server.yaml.example",
        "deployments/templates/server.env.example",
        "deployments/templates/postgresql.yaml.example",
        "deployments/systemd/sdkwork-im-server.service",
        "deployments/launchd/com.sdkwork.im.server.plist",
        "deployments/windows-service/SdkworkImServer.xml",
    ] {
        assert!(
            server_deployments_readme.contains(contract),
            "server deployments payload readme must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_package_matrix_contract() {
    let root = workspace_root();
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let server_payload_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let linux_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("README.md");
    let macos_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("README.md");
    let windows_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("README.md");

    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let server_payload_index =
        fs::read_to_string(&server_payload_index_path).unwrap_or_else(|_| {
            panic!(
                "missing server payload index readme: {}",
                server_payload_index_path.display()
            )
        });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let linux_packages = fs::read_to_string(&linux_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing linux packages readme: {}",
            linux_packages_path.display()
        )
    });
    let macos_packages = fs::read_to_string(&macos_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS packages readme: {}",
            macos_packages_path.display()
        )
    });
    let windows_packages = fs::read_to_string(&windows_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing windows packages readme: {}",
            windows_packages_path.display()
        )
    });

    for contract in [
        "server/packages/README.md",
        "linux",
        "macOS",
        "windows",
        "tar.gz",
        "zip",
        "deb",
        "rpm",
        "pkg",
        "msi",
    ] {
        assert!(
            releases_readme.contains(contract),
            "artifacts/releases/README.md must freeze the package matrix contract `{contract}`"
        );
    }

    for contract in [
        "packages/README.md",
        "packages/linux/README.md",
        "packages/macos/README.md",
        "packages/windows/README.md",
    ] {
        assert!(
            server_payload_index.contains(contract),
            "server payload index readme must reference package matrix contract `{contract}`"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/README.md",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference the package matrix placeholder `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/packages",
        "linux/README.md",
        "macos/README.md",
        "windows/README.md",
        "tar.gz",
        "zip",
        "deb",
        "rpm",
        "pkg",
        "msi",
    ] {
        assert!(
            packages_index.contains(contract),
            "server package matrix readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "tar.gz",
        "deb",
        "rpm",
        "install-server.sh",
        "init-config-server.sh",
        "init-storage-server.sh",
        "install-service-server.sh",
    ] {
        assert!(
            linux_packages.contains(contract),
            "linux packages readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "tar.gz",
        "pkg",
        "install-server.sh",
        "init-config-server.sh",
        "init-storage-server.sh",
        "install-service-server.sh",
    ] {
        assert!(
            macos_packages.contains(contract),
            "macOS packages readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "zip",
        "msi",
        "install-server.ps1",
        "init-config-server.ps1",
        "init-storage-server.ps1",
        "install-service-server.ps1",
        "install-server.cmd",
        "init-config-server.cmd",
        "init-storage-server.cmd",
        "install-service-server.cmd",
    ] {
        assert!(
            windows_packages.contains(contract),
            "windows packages readme must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_package_artifact_naming_and_install_root_contract() {
    let root = workspace_root();
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let linux_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("README.md");
    let macos_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("README.md");
    let windows_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("README.md");
    let packages_sha256s_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("SHA256SUMS");
    let packages_artifact_list_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("artifact-file-list.txt");

    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let linux_packages = fs::read_to_string(&linux_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing linux packages readme: {}",
            linux_packages_path.display()
        )
    });
    let macos_packages = fs::read_to_string(&macos_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS packages readme: {}",
            macos_packages_path.display()
        )
    });
    let windows_packages = fs::read_to_string(&windows_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing windows packages readme: {}",
            windows_packages_path.display()
        )
    });
    let packages_sha256s = fs::read_to_string(&packages_sha256s_path).unwrap_or_else(|_| {
        panic!(
            "missing package checksum manifest: {}",
            packages_sha256s_path.display()
        )
    });
    let packages_artifact_list =
        fs::read_to_string(&packages_artifact_list_path).unwrap_or_else(|_| {
            panic!(
                "missing package artifact file list: {}",
                packages_artifact_list_path.display()
            )
        });

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference the package artifact contract `{contract}`"
        );
    }

    for contract in [
        "SHA256SUMS",
        "artifact-file-list.txt",
        "sdkwork-im-server-linux-x86_64.tar.gz",
        "sdkwork-im-server_<version>_amd64.deb",
        "sdkwork-im-server-<version>-1.x86_64.rpm",
        "sdkwork-im-server-darwin-universal.tar.gz",
        "sdkwork-im-server-<version>.pkg",
        "sdkwork-im-server-windows-x86_64.zip",
        "sdkwork-im-server-<version>-x64.msi",
    ] {
        assert!(
            packages_index.contains(contract),
            "server package matrix readme must freeze artifact naming contract `{contract}`"
        );
    }

    for contract in [
        "/opt/sdkwork-im",
        "/etc/sdkwork-im/default",
        "/var/lib/sdkwork-im/default",
        "/var/log/sdkwork-im/default",
        "/var/run/sdkwork-im/default",
    ] {
        assert!(
            linux_packages.contains(contract),
            "linux packages readme must freeze install-root mapping `{contract}`"
        );
        assert!(
            macos_packages.contains(contract),
            "macOS packages readme must freeze install-root mapping `{contract}`"
        );
    }

    for contract in [
        "ProgramFiles",
        "CommonApplicationData",
        "SdkworkIm",
        "config",
        "data",
        "logs",
        "run",
    ] {
        assert!(
            windows_packages.contains(contract),
            "windows packages readme must freeze install-root mapping `{contract}`"
        );
    }

    for contract in [
        "sdkwork-im-server-linux-x86_64.tar.gz",
        "sdkwork-im-server_<version>_amd64.deb",
        "sdkwork-im-server-<version>-1.x86_64.rpm",
        "sdkwork-im-server-darwin-universal.tar.gz",
        "sdkwork-im-server-<version>.pkg",
        "sdkwork-im-server-windows-x86_64.zip",
        "sdkwork-im-server-<version>-x64.msi",
    ] {
        assert!(
            packages_artifact_list.contains(contract),
            "package artifact file list must freeze `{contract}`"
        );
    }

    for contract in [
        "sha256:<pending>",
        "sdkwork-im-server-linux-x86_64.tar.gz",
        "sdkwork-im-server-windows-x86_64.zip",
    ] {
        assert!(
            packages_sha256s.contains(contract),
            "package checksum manifest must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_platform_artifact_staging_and_checksum_workflow_contract() {
    let root = workspace_root();
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let linux_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("README.md");
    let macos_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("README.md");
    let windows_packages_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("README.md");
    let linux_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("artifacts")
        .join("README.md");
    let macos_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("artifacts")
        .join("README.md");
    let windows_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("artifacts")
        .join("README.md");

    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let linux_packages = fs::read_to_string(&linux_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing linux packages readme: {}",
            linux_packages_path.display()
        )
    });
    let macos_packages = fs::read_to_string(&macos_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS packages readme: {}",
            macos_packages_path.display()
        )
    });
    let windows_packages = fs::read_to_string(&windows_packages_path).unwrap_or_else(|_| {
        panic!(
            "missing windows packages readme: {}",
            windows_packages_path.display()
        )
    });
    let linux_artifacts = fs::read_to_string(&linux_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing linux artifacts readme: {}",
            linux_artifacts_readme_path.display()
        )
    });
    let macos_artifacts = fs::read_to_string(&macos_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS artifacts readme: {}",
            macos_artifacts_readme_path.display()
        )
    });
    let windows_artifacts =
        fs::read_to_string(&windows_artifacts_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing windows artifacts readme: {}",
                windows_artifacts_readme_path.display()
            )
        });

    for (content, platform_name) in [
        (&linux_packages, "linux"),
        (&macos_packages, "macOS"),
        (&windows_packages, "windows"),
    ] {
        assert!(
            content.contains("artifacts/README.md"),
            "{platform_name} packages readme must reference its artifact staging contract"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference the platform artifact staging contract `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts",
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline",
        "sha256sum -b",
        "artifact-file-list.txt",
        "SHA256SUMS",
        "sdkwork-im-server-linux-x86_64.tar.gz",
    ] {
        assert!(
            linux_artifacts.contains(contract),
            "linux artifact staging readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts",
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline",
        "shasum -a 256",
        "artifact-file-list.txt",
        "SHA256SUMS",
        "sdkwork-im-server-darwin-universal.tar.gz",
    ] {
        assert!(
            macos_artifacts.contains(contract),
            "macOS artifact staging readme must freeze `{contract}`"
        );
    }

    for contract in [
        "template_only_pending_payload",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts",
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline",
        "Get-FileHash -Algorithm SHA256",
        "artifact-file-list.txt",
        "SHA256SUMS",
        "sdkwork-im-server-windows-x86_64.zip",
        "SdkworkImServer.exe",
    ] {
        assert!(
            windows_artifacts.contains(contract),
            "windows artifact staging readme must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_release_checklist_and_ordered_packaging_steps_contract() {
    let root = workspace_root();
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let release_checklist_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("release-checklist.md");
    let linux_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("artifacts")
        .join("README.md");
    let macos_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("artifacts")
        .join("README.md");
    let windows_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("artifacts")
        .join("README.md");

    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let release_checklist = fs::read_to_string(&release_checklist_path).unwrap_or_else(|_| {
        panic!(
            "missing server package release checklist: {}",
            release_checklist_path.display()
        )
    });
    let linux_artifacts = fs::read_to_string(&linux_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing linux artifacts readme: {}",
            linux_artifacts_readme_path.display()
        )
    });
    let macos_artifacts = fs::read_to_string(&macos_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS artifacts readme: {}",
            macos_artifacts_readme_path.display()
        )
    });
    let windows_artifacts =
        fs::read_to_string(&windows_artifacts_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing windows artifacts readme: {}",
                windows_artifacts_readme_path.display()
            )
        });

    assert!(
        packages_index.contains("release-checklist.md"),
        "server packages index readme must reference the release checklist contract"
    );
    assert!(
        bundle_manifest
            .contains("artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md"),
        "Wave D bundle manifest must reference the server package release checklist"
    );

    for contract in [
        "template_only_pending_execution",
        "Step 1",
        "Step 2",
        "Step 3",
        "Step 4",
        "Step 5",
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline",
        "artifact-file-list.txt",
        "SHA256SUMS",
        "go / no-go",
    ] {
        assert!(
            release_checklist.contains(contract),
            "server package release checklist must freeze `{contract}`"
        );
    }

    for (content, platform_name) in [
        (&linux_artifacts, "linux"),
        (&macos_artifacts, "macOS"),
        (&windows_artifacts, "windows"),
    ] {
        for contract in ["Step 1", "Step 2", "Step 3", "Step 4"] {
            assert!(
                content.contains(contract),
                "{platform_name} artifacts readme must freeze ordered packaging step `{contract}`"
            );
        }
    }
}

#[test]
fn test_server_release_bundle_freezes_machine_auditable_package_layout_tree_contract() {
    let root = workspace_root();
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let package_layout_tree_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("layout-tree.txt");
    let linux_layout_tree_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("artifacts")
        .join("layout-tree.txt");
    let macos_layout_tree_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("artifacts")
        .join("layout-tree.txt");
    let windows_layout_tree_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("artifacts")
        .join("layout-tree.txt");

    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let package_layout_tree = fs::read_to_string(&package_layout_tree_path).unwrap_or_else(|_| {
        panic!(
            "missing package layout tree: {}",
            package_layout_tree_path.display()
        )
    });
    let linux_layout_tree = fs::read_to_string(&linux_layout_tree_path).unwrap_or_else(|_| {
        panic!(
            "missing linux artifact layout tree: {}",
            linux_layout_tree_path.display()
        )
    });
    let macos_layout_tree = fs::read_to_string(&macos_layout_tree_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS artifact layout tree: {}",
            macos_layout_tree_path.display()
        )
    });
    let windows_layout_tree = fs::read_to_string(&windows_layout_tree_path).unwrap_or_else(|_| {
        panic!(
            "missing windows artifact layout tree: {}",
            windows_layout_tree_path.display()
        )
    });

    assert!(
        packages_index.contains("layout-tree.txt"),
        "server packages index readme must reference the package layout tree contract"
    );

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference layout tree contract `{contract}`"
        );
    }

    for contract in [
        "server/packages/",
        "layout-tree.txt",
        "SHA256SUMS",
        "artifact-file-list.txt",
        "release-checklist.md",
        "linux/",
        "macos/",
        "windows/",
    ] {
        assert!(
            package_layout_tree.contains(contract),
            "package layout tree must freeze `{contract}`"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/",
        "sdkwork-im-server-linux-x86_64.tar.gz",
        "sdkwork-im-server_<version>_amd64.deb",
        "sdkwork-im-server-<version>-1.x86_64.rpm",
    ] {
        assert!(
            linux_layout_tree.contains(contract),
            "linux artifact layout tree must freeze `{contract}`"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/",
        "sdkwork-im-server-darwin-universal.tar.gz",
        "sdkwork-im-server-<version>.pkg",
    ] {
        assert!(
            macos_layout_tree.contains(contract),
            "macOS artifact layout tree must freeze `{contract}`"
        );
    }

    for contract in [
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/",
        "sdkwork-im-server-windows-x86_64.zip",
        "sdkwork-im-server-<version>-x64.msi",
    ] {
        assert!(
            windows_layout_tree.contains(contract),
            "windows artifact layout tree must freeze `{contract}`"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_machine_readable_package_catalog_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-package-catalog.schema.json");
    let catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("package-catalog.json");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let catalog = fs::read_to_string(&catalog_path)
        .unwrap_or_else(|_| panic!("missing package catalog: {}", catalog_path.display()));
    let catalog_json: serde_json::Value = serde_json::from_str(&catalog)
        .unwrap_or_else(|_| panic!("invalid package catalog json: {}", catalog_path.display()));
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing install doc: {}", install_doc_path.display()));

    assert_eq!(
        catalog_json["$schema"],
        "../../schemas/server-package-catalog.schema.json"
    );
    assert_eq!(schema_json["title"], "sdkwork-im server package catalog");
    assert_eq!(schema_json["type"], "object");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "server-package-catalog"
    );

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "wave",
        "artifact",
        "state",
        "updatedAt",
        "packageArtifacts",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "schema required fields must contain {field}"
        );
    }

    let item_required = schema_json["properties"]["packageArtifacts"]["items"]["required"]
        .as_array()
        .expect("package artifact required fields should be an array");
    for field in [
        "id",
        "platform",
        "packageType",
        "fileNameTemplate",
        "artifactPath",
        "installModel",
        "defaultInstallRoot",
        "configRoot",
        "dataRoot",
        "logRoot",
        "runRoot",
        "serviceManager",
        "startupCommand",
        "layoutTreePath",
        "stagingReadmePath",
        "releaseChecklistPath",
        "checksumManifestPath",
        "state",
    ] {
        assert!(
            item_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "package artifact schema required fields must contain {field}"
        );
    }

    assert_eq!(catalog_json["version"], 1);
    assert_eq!(catalog_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(catalog_json["wave"], "wave-d");
    assert_eq!(catalog_json["artifact"], "server-package-catalog");
    assert_eq!(catalog_json["state"], "template_only_pending_build");

    let package_artifacts = catalog_json["packageArtifacts"]
        .as_array()
        .expect("packageArtifacts should be an array");
    assert_eq!(
        package_artifacts.len(),
        7,
        "packageArtifacts must freeze all current server package forms"
    );

    for (
        id,
        platform,
        package_type,
        file_name_template,
        install_model,
        default_install_root,
        config_root,
        data_root,
        log_root,
        run_root,
        service_manager,
        layout_tree_path,
        staging_readme_path,
    ) in [
        (
            "linux-tar-gz",
            "linux",
            "tar.gz",
            "sdkwork-im-server-linux-x86_64.tar.gz",
            "archive",
            "/opt/sdkwork-im",
            "/etc/sdkwork-im/default",
            "/var/lib/sdkwork-im/default",
            "/var/log/sdkwork-im/default",
            "/var/run/sdkwork-im/default",
            "systemd",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        ),
        (
            "linux-deb",
            "linux",
            "deb",
            "sdkwork-im-server_<version>_amd64.deb",
            "native-installer",
            "/opt/sdkwork-im",
            "/etc/sdkwork-im/default",
            "/var/lib/sdkwork-im/default",
            "/var/log/sdkwork-im/default",
            "/var/run/sdkwork-im/default",
            "systemd",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        ),
        (
            "linux-rpm",
            "linux",
            "rpm",
            "sdkwork-im-server-<version>-1.x86_64.rpm",
            "native-installer",
            "/opt/sdkwork-im",
            "/etc/sdkwork-im/default",
            "/var/lib/sdkwork-im/default",
            "/var/log/sdkwork-im/default",
            "/var/run/sdkwork-im/default",
            "systemd",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
        ),
        (
            "macos-tar-gz",
            "macos",
            "tar.gz",
            "sdkwork-im-server-darwin-universal.tar.gz",
            "archive",
            "/opt/sdkwork-im",
            "/etc/sdkwork-im/default",
            "/var/lib/sdkwork-im/default",
            "/var/log/sdkwork-im/default",
            "/var/run/sdkwork-im/default",
            "launchd",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        ),
        (
            "macos-pkg",
            "macos",
            "pkg",
            "sdkwork-im-server-<version>.pkg",
            "native-installer",
            "/opt/sdkwork-im",
            "/etc/sdkwork-im/default",
            "/var/lib/sdkwork-im/default",
            "/var/log/sdkwork-im/default",
            "/var/run/sdkwork-im/default",
            "launchd",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
        ),
        (
            "windows-zip",
            "windows",
            "zip",
            "sdkwork-im-server-windows-x86_64.zip",
            "archive",
            "%ProgramFiles%\\\\SdkworkIm",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\config",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\data",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\logs",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\run",
            "windows-service",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
        ),
        (
            "windows-msi",
            "windows",
            "msi",
            "sdkwork-im-server-<version>-x64.msi",
            "native-installer",
            "%ProgramFiles%\\\\SdkworkIm",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\config",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\data",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\logs",
            "%CommonApplicationData%\\\\SdkworkIm\\\\default\\\\run",
            "windows-service",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
        ),
    ] {
        let package_entry = package_artifacts
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("package catalog must contain entry {id}"));

        assert_eq!(package_entry["platform"], platform);
        assert_eq!(package_entry["packageType"], package_type);
        assert_eq!(package_entry["fileNameTemplate"], file_name_template);
        assert_eq!(package_entry["installModel"], install_model);
        assert_eq!(package_entry["defaultInstallRoot"], default_install_root);
        assert_eq!(package_entry["configRoot"], config_root);
        assert_eq!(package_entry["dataRoot"], data_root);
        assert_eq!(package_entry["logRoot"], log_root);
        assert_eq!(package_entry["runRoot"], run_root);
        assert_eq!(package_entry["serviceManager"], service_manager);
        assert_eq!(
            package_entry["startupCommand"],
            "sdkwork-im-server --config <config-root>/server.yaml"
        );
        assert_eq!(package_entry["layoutTreePath"], layout_tree_path);
        assert_eq!(package_entry["stagingReadmePath"], staging_readme_path);
        assert_eq!(
            package_entry["releaseChecklistPath"],
            "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md"
        );
        assert_eq!(
            package_entry["checksumManifestPath"],
            "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS"
        );
        assert_eq!(package_entry["state"], "template_only_pending_build");
    }

    for contract in [
        "artifacts/releases/schemas/server-package-catalog.schema.json",
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference package catalog contract `{contract}`"
        );
    }

    assert!(
        releases_readme.contains("artifacts/releases/schemas/server-package-catalog.schema.json"),
        "artifacts/releases/README.md must document the server package catalog schema path"
    );
    assert!(
        packages_index.contains("package-catalog.json"),
        "server packages index readme must reference the machine-readable package catalog"
    );
    assert!(
        install_doc.contains("server/package-catalog.json"),
        "server install doc must reference the machine-readable package catalog"
    );
}

#[test]
fn test_server_release_bundle_freezes_platform_package_acceptance_manifest_contract() {
    let root = workspace_root();
    let package_catalog_schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-package-catalog.schema.json");
    let package_catalog_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("package-catalog.json");
    let acceptance_schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-package-acceptance.schema.json");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");
    let linux_acceptance_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("artifacts")
        .join("acceptance-manifest.json");
    let macos_acceptance_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("artifacts")
        .join("acceptance-manifest.json");
    let windows_acceptance_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("artifacts")
        .join("acceptance-manifest.json");
    let linux_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("linux")
        .join("artifacts")
        .join("README.md");
    let macos_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("macos")
        .join("artifacts")
        .join("README.md");
    let windows_artifacts_readme_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("windows")
        .join("artifacts")
        .join("README.md");

    let package_catalog_schema =
        fs::read_to_string(&package_catalog_schema_path).unwrap_or_else(|_| {
            panic!(
                "missing release schema: {}",
                package_catalog_schema_path.display()
            )
        });
    let package_catalog_schema_json: serde_json::Value =
        serde_json::from_str(&package_catalog_schema).unwrap_or_else(|_| {
            panic!(
                "invalid release schema json: {}",
                package_catalog_schema_path.display()
            )
        });
    let package_catalog = fs::read_to_string(&package_catalog_path).unwrap_or_else(|_| {
        panic!(
            "missing package catalog: {}",
            package_catalog_path.display()
        )
    });
    let package_catalog_json: serde_json::Value = serde_json::from_str(&package_catalog)
        .unwrap_or_else(|_| {
            panic!(
                "invalid package catalog json: {}",
                package_catalog_path.display()
            )
        });
    let acceptance_schema = fs::read_to_string(&acceptance_schema_path).unwrap_or_else(|_| {
        panic!(
            "missing release schema: {}",
            acceptance_schema_path.display()
        )
    });
    let acceptance_schema_json: serde_json::Value = serde_json::from_str(&acceptance_schema)
        .unwrap_or_else(|_| {
            panic!(
                "invalid release schema json: {}",
                acceptance_schema_path.display()
            )
        });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing install doc: {}", install_doc_path.display()));
    let linux_acceptance = fs::read_to_string(&linux_acceptance_path).unwrap_or_else(|_| {
        panic!(
            "missing acceptance manifest: {}",
            linux_acceptance_path.display()
        )
    });
    let linux_acceptance_json: serde_json::Value = serde_json::from_str(&linux_acceptance)
        .unwrap_or_else(|_| {
            panic!(
                "invalid acceptance manifest json: {}",
                linux_acceptance_path.display()
            )
        });
    let macos_acceptance = fs::read_to_string(&macos_acceptance_path).unwrap_or_else(|_| {
        panic!(
            "missing acceptance manifest: {}",
            macos_acceptance_path.display()
        )
    });
    let macos_acceptance_json: serde_json::Value = serde_json::from_str(&macos_acceptance)
        .unwrap_or_else(|_| {
            panic!(
                "invalid acceptance manifest json: {}",
                macos_acceptance_path.display()
            )
        });
    let windows_acceptance = fs::read_to_string(&windows_acceptance_path).unwrap_or_else(|_| {
        panic!(
            "missing acceptance manifest: {}",
            windows_acceptance_path.display()
        )
    });
    let windows_acceptance_json: serde_json::Value = serde_json::from_str(&windows_acceptance)
        .unwrap_or_else(|_| {
            panic!(
                "invalid acceptance manifest json: {}",
                windows_acceptance_path.display()
            )
        });
    let linux_artifacts = fs::read_to_string(&linux_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing linux artifacts readme: {}",
            linux_artifacts_readme_path.display()
        )
    });
    let macos_artifacts = fs::read_to_string(&macos_artifacts_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing macOS artifacts readme: {}",
            macos_artifacts_readme_path.display()
        )
    });
    let windows_artifacts =
        fs::read_to_string(&windows_artifacts_readme_path).unwrap_or_else(|_| {
            panic!(
                "missing windows artifacts readme: {}",
                windows_artifacts_readme_path.display()
            )
        });

    let catalog_item_required =
        package_catalog_schema_json["properties"]["packageArtifacts"]["items"]["required"]
            .as_array()
            .expect("package artifact required fields should be an array");
    assert!(
        catalog_item_required
            .iter()
            .any(|entry| entry.as_str() == Some("acceptanceManifestPath")),
        "package catalog schema must require acceptanceManifestPath"
    );

    for (id, acceptance_manifest_path) in [
        (
            "linux-tar-gz",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        ),
        (
            "linux-deb",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        ),
        (
            "linux-rpm",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        ),
        (
            "macos-tar-gz",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        ),
        (
            "macos-pkg",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        ),
        (
            "windows-zip",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        ),
        (
            "windows-msi",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        ),
    ] {
        let package_entry = package_catalog_json["packageArtifacts"]
            .as_array()
            .expect("packageArtifacts should be an array")
            .iter()
            .find(|entry| entry["id"] == id)
            .unwrap_or_else(|| panic!("package catalog must contain entry {id}"));
        assert_eq!(
            package_entry["acceptanceManifestPath"],
            acceptance_manifest_path
        );
    }

    assert_eq!(
        acceptance_schema_json["title"],
        "sdkwork-im server package acceptance manifest"
    );
    assert_eq!(acceptance_schema_json["type"], "object");
    assert_eq!(
        acceptance_schema_json["properties"]["artifact"]["const"],
        "server-package-acceptance-manifest"
    );

    let required = acceptance_schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "platform",
        "artifact",
        "artifactRoot",
        "validationStatus",
        "updatedAt",
        "packageChecks",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "acceptance schema required fields must contain {field}"
        );
    }

    let package_check_required =
        acceptance_schema_json["properties"]["packageChecks"]["items"]["required"]
            .as_array()
            .expect("package check required fields should be an array");
    for field in [
        "packageId",
        "packageType",
        "artifactPath",
        "installModel",
        "serviceManager",
        "startupCommand",
        "requiredEntries",
        "validationEvidencePath",
        "status",
    ] {
        assert!(
            package_check_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "package check schema required fields must contain {field}"
        );
    }

    for (
        manifest_json,
        platform,
        artifact_root,
        expected_checks,
        expected_service_manager,
        expected_binary,
        expected_service_entry,
    ) in [
        (
            &linux_acceptance_json,
            "linux",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts",
            vec![
                ("linux-tar-gz", "tar.gz", "archive"),
                ("linux-deb", "deb", "native-installer"),
                ("linux-rpm", "rpm", "native-installer"),
            ],
            "systemd",
            "bin/sdkwork-im-server",
            "deployments/systemd/sdkwork-im-server.service",
        ),
        (
            &macos_acceptance_json,
            "macos",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts",
            vec![
                ("macos-tar-gz", "tar.gz", "archive"),
                ("macos-pkg", "pkg", "native-installer"),
            ],
            "launchd",
            "bin/sdkwork-im-server",
            "deployments/launchd/com.sdkwork.im.server.plist",
        ),
        (
            &windows_acceptance_json,
            "windows",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts",
            vec![
                ("windows-zip", "zip", "archive"),
                ("windows-msi", "msi", "native-installer"),
            ],
            "windows-service",
            "bin/sdkwork-im-server.exe",
            "deployments/windows-service/SdkworkImServer.xml",
        ),
    ] {
        assert_eq!(
            manifest_json["$schema"],
            "../../../../../schemas/server-package-acceptance.schema.json"
        );
        assert_eq!(manifest_json["bundleId"], "wave-d-2026-04-08");
        assert_eq!(manifest_json["platform"], platform);
        assert_eq!(
            manifest_json["artifact"],
            "server-package-acceptance-manifest"
        );
        assert_eq!(manifest_json["artifactRoot"], artifact_root);
        assert_eq!(
            manifest_json["validationStatus"],
            "template_only_pending_execution"
        );

        let package_checks = manifest_json["packageChecks"]
            .as_array()
            .expect("packageChecks should be an array");
        assert_eq!(
            package_checks.len(),
            expected_checks.len(),
            "{platform} acceptance manifest must freeze all package checks"
        );

        for (package_id, package_type, install_model) in expected_checks {
            let package_check = package_checks
                .iter()
                .find(|entry| entry["packageId"] == package_id)
                .unwrap_or_else(|| {
                    panic!("{platform} acceptance manifest must contain check {package_id}")
                });
            assert_eq!(package_check["packageType"], package_type);
            assert_eq!(package_check["installModel"], install_model);
            assert_eq!(package_check["serviceManager"], expected_service_manager);
            assert_eq!(
                package_check["startupCommand"],
                "sdkwork-im-server --config <config-root>/server.yaml"
            );
            assert_eq!(package_check["status"], "pending_validation");
            assert!(
                package_check["validationEvidencePath"].is_null(),
                "{platform} acceptance check {package_id} must keep validationEvidencePath null before execution"
            );

            let required_entries = package_check["requiredEntries"]
                .as_array()
                .expect("requiredEntries should be an array");
            assert!(
                required_entries
                    .iter()
                    .any(|entry| entry.as_str() == Some(expected_binary)),
                "{platform} acceptance check {package_id} must require binary entry {expected_binary}"
            );
            assert!(
                required_entries
                    .iter()
                    .any(|entry| entry.as_str() == Some(expected_service_entry)),
                "{platform} acceptance check {package_id} must require service entry {expected_service_entry}"
            );
            if platform == "windows" {
                assert!(
                    required_entries
                        .iter()
                        .any(|entry| entry.as_str() == Some("bin/SdkworkImServer.exe")),
                    "windows acceptance checks must require the dedicated service-host wrapper"
                );
            }
        }
    }

    for contract in [
        "artifacts/releases/schemas/server-package-acceptance.schema.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference acceptance contract `{contract}`"
        );
    }

    assert!(
        releases_readme
            .contains("artifacts/releases/schemas/server-package-acceptance.schema.json"),
        "artifacts/releases/README.md must document the server package acceptance schema path"
    );
    assert!(
        packages_index.contains("acceptance-manifest.json"),
        "server packages index readme must reference platform acceptance manifests"
    );
    assert!(
        install_doc.contains("acceptance-manifest.json"),
        "server install doc must reference platform acceptance manifests"
    );
    for (content, platform_name) in [
        (&linux_artifacts, "linux"),
        (&macos_artifacts, "macOS"),
        (&windows_artifacts, "windows"),
    ] {
        assert!(
            content.contains("acceptance-manifest.json"),
            "{platform_name} artifacts readme must reference the acceptance manifest"
        );
    }
}

#[test]
fn test_server_release_bundle_freezes_machine_readable_release_execution_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-release-execution.schema.json");
    let execution_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-execution.json");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let server_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let execution_manifest = fs::read_to_string(&execution_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release execution manifest: {}",
            execution_manifest_path.display()
        )
    });
    let execution_manifest_json: serde_json::Value = serde_json::from_str(&execution_manifest)
        .unwrap_or_else(|_| {
            panic!(
                "invalid release execution manifest json: {}",
                execution_manifest_path.display()
            )
        });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let server_index = fs::read_to_string(&server_index_path)
        .unwrap_or_else(|_| panic!("missing server index: {}", server_index_path.display()));
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing install doc: {}", install_doc_path.display()));

    assert_eq!(
        execution_manifest_json["$schema"],
        "../../schemas/server-release-execution.schema.json"
    );
    assert_eq!(
        schema_json["title"],
        "sdkwork-im server release execution manifest"
    );
    assert_eq!(schema_json["type"], "object");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "server-release-execution"
    );

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "wave",
        "artifact",
        "state",
        "updatedAt",
        "canonicalBuild",
        "canonicalStartupCommand",
        "packageCatalogPath",
        "releaseChecklistPath",
        "checksumManifestPath",
        "artifactFileListPath",
        "platformExecutions",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "release execution schema required fields must contain {field}"
        );
    }

    let build_required = schema_json["properties"]["canonicalBuild"]["required"]
        .as_array()
        .expect("canonicalBuild required should be an array");
    for field in ["command", "package", "binary", "profile"] {
        assert!(
            build_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "canonicalBuild schema required fields must contain {field}"
        );
    }

    let platform_required = schema_json["properties"]["platformExecutions"]["items"]["required"]
        .as_array()
        .expect("platformExecutions item required should be an array");
    for field in [
        "platform",
        "stagingRoot",
        "stagingReadmePath",
        "acceptanceManifestPath",
        "layoutTreePath",
        "packageIds",
        "checksumCommandExample",
        "serviceManager",
        "status",
    ] {
        assert!(
            platform_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "platformExecutions schema required fields must contain {field}"
        );
    }

    assert_eq!(execution_manifest_json["version"], 1);
    assert_eq!(execution_manifest_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(execution_manifest_json["wave"], "wave-d");
    assert_eq!(
        execution_manifest_json["artifact"],
        "server-release-execution"
    );
    assert_eq!(
        execution_manifest_json["state"],
        "template_only_pending_execution"
    );
    assert_eq!(
        execution_manifest_json["canonicalBuild"]["command"],
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline"
    );
    assert_eq!(
        execution_manifest_json["canonicalBuild"]["package"],
        "sdkwork-im-gateway"
    );
    assert_eq!(
        execution_manifest_json["canonicalBuild"]["binary"],
        "sdkwork-im-server"
    );
    assert_eq!(
        execution_manifest_json["canonicalBuild"]["profile"],
        "release"
    );
    assert_eq!(
        execution_manifest_json["canonicalStartupCommand"],
        "sdkwork-im-server --config <config-root>/server.yaml"
    );
    assert_eq!(
        execution_manifest_json["packageCatalogPath"],
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json"
    );
    assert_eq!(
        execution_manifest_json["releaseChecklistPath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md"
    );
    assert_eq!(
        execution_manifest_json["checksumManifestPath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS"
    );
    assert_eq!(
        execution_manifest_json["artifactFileListPath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt"
    );

    let platform_executions = execution_manifest_json["platformExecutions"]
        .as_array()
        .expect("platformExecutions should be an array");
    assert_eq!(
        platform_executions.len(),
        3,
        "platformExecutions must freeze linux, macOS, and windows execution surfaces"
    );

    for (
        platform,
        staging_root,
        staging_readme_path,
        acceptance_manifest_path,
        layout_tree_path,
        package_ids,
        checksum_command_example,
        service_manager,
    ) in [
        (
            "linux",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/README.md",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/layout-tree.txt",
            vec!["linux-tar-gz", "linux-deb", "linux-rpm"],
            "sha256sum -b <artifact> >> ../SHA256SUMS",
            "systemd",
        ),
        (
            "macos",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/README.md",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/layout-tree.txt",
            vec!["macos-tar-gz", "macos-pkg"],
            "shasum -a 256 <artifact> >> ../SHA256SUMS",
            "launchd",
        ),
        (
            "windows",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/README.md",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/layout-tree.txt",
            vec!["windows-zip", "windows-msi"],
            "Get-FileHash -Algorithm SHA256 <artifact> | Format-Table -HideTableHeaders >> ../SHA256SUMS",
            "windows-service",
        ),
    ] {
        let platform_entry = platform_executions
            .iter()
            .find(|entry| entry["platform"] == platform)
            .unwrap_or_else(|| panic!("release execution manifest must contain {platform}"));
        assert_eq!(platform_entry["stagingRoot"], staging_root);
        assert_eq!(platform_entry["stagingReadmePath"], staging_readme_path);
        assert_eq!(
            platform_entry["acceptanceManifestPath"],
            acceptance_manifest_path
        );
        assert_eq!(platform_entry["layoutTreePath"], layout_tree_path);
        assert_eq!(
            platform_entry["checksumCommandExample"],
            checksum_command_example
        );
        assert_eq!(platform_entry["serviceManager"], service_manager);
        assert_eq!(platform_entry["status"], "template_only_pending_execution");

        let package_ids_json = platform_entry["packageIds"]
            .as_array()
            .expect("packageIds should be an array");
        for package_id in package_ids {
            assert!(
                package_ids_json
                    .iter()
                    .any(|entry| entry.as_str() == Some(package_id)),
                "release execution manifest platform {platform} must contain package id {package_id}"
            );
        }
    }

    for contract in [
        "artifacts/releases/schemas/server-release-execution.schema.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference release execution contract `{contract}`"
        );
    }

    assert!(
        releases_readme.contains("artifacts/releases/schemas/server-release-execution.schema.json"),
        "artifacts/releases/README.md must document the server release execution schema path"
    );
    assert!(
        server_index.contains("release-execution.json"),
        "server payload index must reference the machine-readable release execution manifest"
    );
    assert!(
        packages_index.contains("../release-execution.json"),
        "server packages index readme must reference the bundle-level release execution manifest"
    );
    assert!(
        install_doc.contains("server/release-execution.json"),
        "server install doc must reference the machine-readable release execution manifest"
    );
}

#[test]
fn test_server_release_bundle_freezes_machine_readable_release_provenance_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-release-provenance.schema.json");
    let provenance_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-provenance.json");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let server_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let provenance = fs::read_to_string(&provenance_path)
        .unwrap_or_else(|_| panic!("missing release provenance: {}", provenance_path.display()));
    let provenance_json: serde_json::Value =
        serde_json::from_str(&provenance).unwrap_or_else(|_| {
            panic!(
                "invalid release provenance json: {}",
                provenance_path.display()
            )
        });
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let server_index = fs::read_to_string(&server_index_path)
        .unwrap_or_else(|_| panic!("missing server index: {}", server_index_path.display()));
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing install doc: {}", install_doc_path.display()));

    assert_eq!(
        provenance_json["$schema"],
        "../../schemas/server-release-provenance.schema.json"
    );
    assert_eq!(
        schema_json["title"],
        "sdkwork-im server release provenance manifest"
    );
    assert_eq!(schema_json["type"], "object");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "server-release-provenance"
    );

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "wave",
        "artifact",
        "state",
        "updatedAt",
        "canonicalBuildCommand",
        "contractPaths",
        "payloadSourcePaths",
        "platformArtifactRoots",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "release provenance schema required fields must contain {field}"
        );
    }

    let platform_required = schema_json["properties"]["platformArtifactRoots"]["items"]["required"]
        .as_array()
        .expect("platformArtifactRoots required should be an array");
    for field in ["platform", "artifactRoot", "acceptanceManifestPath"] {
        assert!(
            platform_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "platformArtifactRoots schema required fields must contain {field}"
        );
    }

    assert_eq!(provenance_json["version"], 1);
    assert_eq!(provenance_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(provenance_json["wave"], "wave-d");
    assert_eq!(provenance_json["artifact"], "server-release-provenance");
    assert_eq!(provenance_json["state"], "template_only_pending_capture");
    assert_eq!(
        provenance_json["canonicalBuildCommand"],
        "cargo build -p sdkwork-im-gateway --release --bin sdkwork-im-server --offline"
    );

    let contract_paths = provenance_json["contractPaths"]
        .as_array()
        .expect("contractPaths should be an array");
    for contract_path in [
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md",
        "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
        "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
    ] {
        assert!(
            contract_paths
                .iter()
                .any(|entry| entry.as_str() == Some(contract_path)),
            "release provenance contractPaths must contain {contract_path}"
        );
    }

    let payload_source_paths = provenance_json["payloadSourcePaths"]
        .as_array()
        .expect("payloadSourcePaths should be an array");
    for payload_source in [
        "services/sdkwork-im-gateway/Cargo.toml",
        "deployments/templates/server.yaml.example",
        "deployments/templates/server.env.example",
        "deployments/templates/postgresql.yaml.example",
        "deployments/systemd/sdkwork-im-server.service",
        "deployments/launchd/com.sdkwork.im.server.plist",
        "deployments/windows-service/SdkworkImServer.xml",
    ] {
        assert!(
            payload_source_paths
                .iter()
                .any(|entry| entry.as_str() == Some(payload_source)),
            "release provenance payloadSourcePaths must contain {payload_source}"
        );
    }

    let platform_roots = provenance_json["platformArtifactRoots"]
        .as_array()
        .expect("platformArtifactRoots should be an array");
    for (platform, artifact_root, acceptance_manifest_path) in [
        (
            "linux",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
        ),
        (
            "macos",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
        ),
        (
            "windows",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
        ),
    ] {
        let platform_entry = platform_roots
            .iter()
            .find(|entry| entry["platform"] == platform)
            .unwrap_or_else(|| panic!("release provenance must contain platform {platform}"));
        assert_eq!(platform_entry["artifactRoot"], artifact_root);
        assert_eq!(
            platform_entry["acceptanceManifestPath"],
            acceptance_manifest_path
        );
    }

    for contract in [
        "artifacts/releases/schemas/server-release-provenance.schema.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference release provenance contract `{contract}`"
        );
    }

    assert!(
        releases_readme
            .contains("artifacts/releases/schemas/server-release-provenance.schema.json"),
        "artifacts/releases/README.md must document the server release provenance schema path"
    );
    assert!(
        server_index.contains("release-provenance.json"),
        "server payload index must reference the machine-readable release provenance manifest"
    );
    assert!(
        install_doc.contains("server/release-provenance.json"),
        "server install doc must reference the machine-readable release provenance manifest"
    );
}

#[test]
fn test_server_release_bundle_freezes_machine_readable_release_gate_contract() {
    let root = workspace_root();
    let schema_path = root
        .join("artifacts")
        .join("releases")
        .join("schemas")
        .join("server-release-gate.schema.json");
    let gate_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("release-gate.json");
    let bundle_manifest_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("bundle-manifest.md");
    let releases_readme_path = root.join("artifacts").join("releases").join("README.md");
    let server_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("README.md");
    let packages_index_path = root
        .join("artifacts")
        .join("releases")
        .join("wave-d-2026-04-08")
        .join("server")
        .join("packages")
        .join("README.md");
    let install_doc_path = root
        .join("docs")
        .join("部署")
        .join("server版本安装与初始化.md");

    let schema = fs::read_to_string(&schema_path)
        .unwrap_or_else(|_| panic!("missing release schema: {}", schema_path.display()));
    let schema_json: serde_json::Value = serde_json::from_str(&schema)
        .unwrap_or_else(|_| panic!("invalid release schema json: {}", schema_path.display()));
    let gate = fs::read_to_string(&gate_path)
        .unwrap_or_else(|_| panic!("missing release gate: {}", gate_path.display()));
    let gate_json: serde_json::Value = serde_json::from_str(&gate)
        .unwrap_or_else(|_| panic!("invalid release gate json: {}", gate_path.display()));
    let bundle_manifest = fs::read_to_string(&bundle_manifest_path).unwrap_or_else(|_| {
        panic!(
            "missing release bundle manifest: {}",
            bundle_manifest_path.display()
        )
    });
    let releases_readme = fs::read_to_string(&releases_readme_path).unwrap_or_else(|_| {
        panic!(
            "missing releases README: {}",
            releases_readme_path.display()
        )
    });
    let server_index = fs::read_to_string(&server_index_path)
        .unwrap_or_else(|_| panic!("missing server index: {}", server_index_path.display()));
    let packages_index = fs::read_to_string(&packages_index_path).unwrap_or_else(|_| {
        panic!(
            "missing server packages index readme: {}",
            packages_index_path.display()
        )
    });
    let install_doc = fs::read_to_string(&install_doc_path)
        .unwrap_or_else(|_| panic!("missing install doc: {}", install_doc_path.display()));

    assert_eq!(
        gate_json["$schema"],
        "../../schemas/server-release-gate.schema.json"
    );
    assert_eq!(schema_json["title"], "sdkwork-im server release gate");
    assert_eq!(schema_json["type"], "object");
    assert_eq!(
        schema_json["properties"]["artifact"]["const"],
        "server-release-gate"
    );

    let required = schema_json["required"]
        .as_array()
        .expect("schema required should be an array");
    for field in [
        "bundleId",
        "wave",
        "artifact",
        "state",
        "decisionStatus",
        "updatedAt",
        "releaseChecklistPath",
        "packageCatalogPath",
        "releaseExecutionPath",
        "releaseProvenancePath",
        "reviewDocPaths",
        "gateChecks",
        "platformGateChecks",
    ] {
        assert!(
            required.iter().any(|entry| entry.as_str() == Some(field)),
            "release gate schema required fields must contain {field}"
        );
    }

    let gate_check_required = schema_json["properties"]["gateChecks"]["items"]["required"]
        .as_array()
        .expect("gateChecks required should be an array");
    for field in [
        "gateId",
        "description",
        "contractPath",
        "status",
        "blocking",
    ] {
        assert!(
            gate_check_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "gateChecks schema required fields must contain {field}"
        );
    }

    let platform_gate_required =
        schema_json["properties"]["platformGateChecks"]["items"]["required"]
            .as_array()
            .expect("platformGateChecks required should be an array");
    for field in [
        "platform",
        "acceptanceManifestPath",
        "requiredPackageIds",
        "status",
    ] {
        assert!(
            platform_gate_required
                .iter()
                .any(|entry| entry.as_str() == Some(field)),
            "platformGateChecks schema required fields must contain {field}"
        );
    }

    assert_eq!(gate_json["version"], 1);
    assert_eq!(gate_json["bundleId"], "wave-d-2026-04-08");
    assert_eq!(gate_json["wave"], "wave-d");
    assert_eq!(gate_json["artifact"], "server-release-gate");
    assert_eq!(gate_json["state"], "template_only_pending_evaluation");
    assert_eq!(gate_json["decisionStatus"], "pending_go_no_go");
    assert_eq!(
        gate_json["releaseChecklistPath"],
        "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md"
    );
    assert_eq!(
        gate_json["packageCatalogPath"],
        "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json"
    );
    assert_eq!(
        gate_json["releaseExecutionPath"],
        "artifacts/releases/wave-d-2026-04-08/server/release-execution.json"
    );
    assert_eq!(
        gate_json["releaseProvenancePath"],
        "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json"
    );

    let review_doc_paths = gate_json["reviewDocPaths"]
        .as_array()
        .expect("reviewDocPaths should be an array");
    assert!(
        review_doc_paths
            .iter()
            .any(|entry| entry.as_str()
                == Some("docs/review/step-13-release-readiness-2026-04-08.md")),
        "release gate reviewDocPaths must contain the release-readiness review doc"
    );
    assert!(
        review_doc_paths.iter().any(|entry| {
            entry
                .as_str()
                .map(|path| {
                    path.starts_with("docs/review/step-13-go-no-go")
                        && path.ends_with("-2026-04-08.md")
                })
                .unwrap_or(false)
        }),
        "release gate reviewDocPaths must contain the step-13 go/no-go review doc"
    );

    let gate_checks = gate_json["gateChecks"]
        .as_array()
        .expect("gateChecks should be an array");
    for (gate_id, contract_path) in [
        (
            "release_checklist_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/packages/release-checklist.md",
        ),
        (
            "package_catalog_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/package-catalog.json",
        ),
        (
            "release_execution_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/release-execution.json",
        ),
        (
            "release_provenance_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/release-provenance.json",
        ),
        (
            "checksum_manifest_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/packages/SHA256SUMS",
        ),
        (
            "artifact_file_list_frozen",
            "artifacts/releases/wave-d-2026-04-08/server/packages/artifact-file-list.txt",
        ),
    ] {
        let gate_entry = gate_checks
            .iter()
            .find(|entry| entry["gateId"] == gate_id)
            .unwrap_or_else(|| panic!("release gate must contain gate check {gate_id}"));
        assert_eq!(gate_entry["contractPath"], contract_path);
        assert_eq!(gate_entry["status"], "pending_validation");
        assert_eq!(gate_entry["blocking"], true);
        assert!(
            gate_entry["description"].is_string(),
            "release gate check {gate_id} must keep a human-readable description"
        );
    }

    let platform_gate_checks = gate_json["platformGateChecks"]
        .as_array()
        .expect("platformGateChecks should be an array");
    for (platform, acceptance_manifest_path, required_package_ids) in [
        (
            "linux",
            "artifacts/releases/wave-d-2026-04-08/server/packages/linux/artifacts/acceptance-manifest.json",
            vec!["linux-tar-gz", "linux-deb", "linux-rpm"],
        ),
        (
            "macos",
            "artifacts/releases/wave-d-2026-04-08/server/packages/macos/artifacts/acceptance-manifest.json",
            vec!["macos-tar-gz", "macos-pkg"],
        ),
        (
            "windows",
            "artifacts/releases/wave-d-2026-04-08/server/packages/windows/artifacts/acceptance-manifest.json",
            vec!["windows-zip", "windows-msi"],
        ),
    ] {
        let platform_entry = platform_gate_checks
            .iter()
            .find(|entry| entry["platform"] == platform)
            .unwrap_or_else(|| panic!("release gate must contain platform gate {platform}"));
        assert_eq!(
            platform_entry["acceptanceManifestPath"],
            acceptance_manifest_path
        );
        assert_eq!(platform_entry["status"], "pending_validation");

        let package_ids_json = platform_entry["requiredPackageIds"]
            .as_array()
            .expect("requiredPackageIds should be an array");
        for package_id in required_package_ids {
            assert!(
                package_ids_json
                    .iter()
                    .any(|entry| entry.as_str() == Some(package_id)),
                "release gate platform {platform} must contain required package id {package_id}"
            );
        }
    }

    for contract in [
        "artifacts/releases/schemas/server-release-gate.schema.json",
        "artifacts/releases/wave-d-2026-04-08/server/release-gate.json",
    ] {
        assert!(
            bundle_manifest.contains(contract),
            "Wave D bundle manifest must reference release gate contract `{contract}`"
        );
    }

    assert!(
        releases_readme.contains("artifacts/releases/schemas/server-release-gate.schema.json"),
        "artifacts/releases/README.md must document the server release gate schema path"
    );
    assert!(
        server_index.contains("release-gate.json"),
        "server payload index must reference the machine-readable release gate manifest"
    );
    assert!(
        packages_index.contains("../release-gate.json"),
        "server packages index readme must reference the bundle-level release gate manifest"
    );
    assert!(
        install_doc.contains("server/release-gate.json"),
        "server install doc must reference the machine-readable release gate manifest"
    );
}
