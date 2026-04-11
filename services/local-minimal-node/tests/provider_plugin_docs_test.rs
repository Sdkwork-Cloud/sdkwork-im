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

#[test]
fn test_provider_plugin_architecture_docs_cover_current_matrix_and_runtime_closures() {
    let plugin_doc = read_repo_file("docs/架构/150-插件化提供商体系与设备接入设计-2026-04-08.md");
    let object_storage_doc =
        read_repo_file("docs/架构/150A-对象存储插件运行时与媒体下载面设计-2026-04-08.md");
    let rtc_recording_doc =
        read_repo_file("docs/架构/150B-RTC录制对象存储运行时与播放面设计-2026-04-08.md");
    let provider_binding_doc =
        read_repo_file("docs/架构/150C-控制面provider绑定求值与租户治理设计-2026-04-08.md");
    let ops_provider_binding_doc =
        read_repo_file("docs/架构/150D-ops-provider-binding运行态消费与漂移视图设计-2026-04-08.md");
    let ops_provider_binding_drift_doc =
        read_repo_file("docs/架构/150E-ops-provider-binding漂移检测与运维视图设计-2026-04-08.md");
    let provider_policy_write_doc = read_repo_file(
        "docs/架构/150F-control-plane-provider-policy写接口与审计设计-2026-04-08.md",
    );
    let provider_policy_version_doc =
        read_repo_file("docs/架构/150G-control-plane-provider-policy版本与回滚设计-2026-04-08.md");
    let provider_policy_diff_doc =
        read_repo_file("docs/架构/150H-control-plane-provider-policy差异查询设计-2026-04-08.md");
    let provider_policy_preview_doc =
        read_repo_file("docs/架构/150I-control-plane-provider-policy预览设计-2026-04-08.md");
    let provider_policy_preview_confirmation_doc =
        read_repo_file("docs/架构/150J-control-plane-provider-policy预览确认设计-2026-04-08.md");
    let provider_policy_commit_response_doc =
        read_repo_file("docs/架构/150K-control-plane-provider-policy提交结果设计-2026-04-08.md");
    let provider_policy_noop_doc =
        read_repo_file("docs/架构/150L-control-plane-provider-policy-noop设计-2026-04-08.md");
    let provider_policy_status_doc =
        read_repo_file("docs/架构/150M-control-plane-provider-policy结果状态设计-2026-04-08.md");
    let provider_policy_read_status_doc = read_repo_file(
        "docs/架构/150N-control-plane-provider-policy-history-diff-rollback-status设计-2026-04-08.md",
    );
    let provider_policy_error_status_doc = read_repo_file(
        "docs/架构/150O-control-plane-provider-policy-error-status设计-2026-04-08.md",
    );
    let provider_surface_status_doc =
        read_repo_file("docs/架构/150P-control-plane-provider-snapshot-status设计-2026-04-08.md");
    let object_storage_plan_doc =
        read_repo_file("docs/架构/09A-实施计划-对象存储插件补充-2026-04-08.md");
    let rtc_recording_plan_doc =
        read_repo_file("docs/架构/09B-实施计划-RTC录制对象存储补充-2026-04-08.md");
    let provider_binding_plan_doc =
        read_repo_file("docs/架构/09C-实施计划-控制面provider绑定治理补充-2026-04-08.md");
    let ops_provider_binding_plan_doc =
        read_repo_file("docs/架构/09D-实施计划-ops-provider-binding消费补充-2026-04-08.md");
    let ops_provider_binding_drift_plan_doc =
        read_repo_file("docs/架构/09E-实施计划-ops-provider-binding漂移补充-2026-04-08.md");
    let provider_policy_write_plan_doc =
        read_repo_file("docs/架构/09F-实施计划-provider-policy写接口与审计补充-2026-04-08.md");
    let provider_policy_version_plan_doc =
        read_repo_file("docs/架构/09G-实施计划-provider-policy版本与回滚补充-2026-04-08.md");
    let provider_policy_diff_plan_doc =
        read_repo_file("docs/架构/09H-实施计划-provider-policy差异查询补充-2026-04-08.md");
    let provider_policy_preview_plan_doc =
        read_repo_file("docs/架构/09I-实施计划-provider-policy预览补充-2026-04-08.md");
    let provider_policy_preview_confirmation_plan_doc =
        read_repo_file("docs/架构/09J-实施计划-provider-policy预览确认补充-2026-04-08.md");
    let provider_policy_commit_response_plan_doc =
        read_repo_file("docs/架构/09K-实施计划-provider-policy提交结果补充-2026-04-08.md");
    let provider_policy_noop_plan_doc =
        read_repo_file("docs/架构/09L-实施计划-provider-policy-noop补充-2026-04-08.md");
    let provider_policy_status_plan_doc =
        read_repo_file("docs/架构/09M-实施计划-provider-policy结果状态补充-2026-04-08.md");
    let provider_policy_read_status_plan_doc =
        read_repo_file("docs/架构/09N-实施计划-provider-policy历史diff回滚状态补充-2026-04-08.md");
    let provider_policy_error_status_plan_doc =
        read_repo_file("docs/架构/09O-实施计划-provider-policy错误状态补充-2026-04-08.md");
    let provider_surface_status_plan_doc =
        read_repo_file("docs/架构/09P-实施计划-provider快照状态补充-2026-04-08.md");
    let provider_policy_version_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-version-and-rollback-2026-04-08.md",
    );
    let provider_policy_diff_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-diff-2026-04-08.md",
    );
    let provider_policy_preview_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-preview-2026-04-08.md",
    );
    let provider_policy_preview_confirmation_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-preview-confirmation-2026-04-08.md",
    );
    let provider_policy_commit_response_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-commit-response-2026-04-08.md",
    );
    let provider_policy_noop_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-noop-2026-04-08.md",
    );
    let provider_policy_status_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-status-2026-04-08.md",
    );
    let provider_policy_read_status_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-history-diff-rollback-status-2026-04-08.md",
    );
    let provider_policy_error_status_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-error-status-2026-04-08.md",
    );
    let provider_surface_status_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-snapshot-status-2026-04-08.md",
    );
    let adapter_doc = read_repo_file("adapters/object-storage-s3/README.md");

    for required in [
        "火山引擎",
        "阿里云",
        "腾讯云",
        "AWS",
        "Google",
        "Microsoft",
        "user-module-local",
        "user-module-external",
        "MQTT",
        "小智",
        "ObjectStorageProvider",
        "object-storage-s3",
    ] {
        assert!(
            plugin_doc.contains(required)
                || object_storage_doc.contains(required)
                || rtc_recording_doc.contains(required),
            "provider/plugin architecture docs must cover {required}"
        );
    }

    assert!(
        plugin_doc.contains("默认 RTC provider")
            && plugin_doc.contains("火山引擎")
            && plugin_doc.contains("阿里云")
            && plugin_doc.contains("腾讯云"),
        "architecture baseline must freeze the RTC provider matrix and default"
    );
    assert!(
        object_storage_doc.contains("deployment_profile")
            && object_storage_doc.contains("object-storage-volcengine"),
        "object storage runtime doc must freeze deployment-profile selection"
    );
    assert!(
        object_storage_doc.contains("GET /api/v1/media/provider-health")
            && object_storage_doc.contains("GET /api/v1/media/{media_asset_id}/download-url"),
        "object storage runtime doc must describe the media provider HTTP surface"
    );
    assert!(
        rtc_recording_doc.contains("GET /api/v1/rtc/sessions/{rtc_session_id}/artifacts/recording")
            && rtc_recording_doc.contains("RtcRecordingArtifact")
            && rtc_recording_doc.contains("bucket")
            && rtc_recording_doc.contains("storage_provider")
            && rtc_recording_doc.contains("ObjectStorageProvider"),
        "rtc recording runtime doc must capture the normalized artifact contract and HTTP surface"
    );
    assert!(
        object_storage_plan_doc.contains("object-storage-volcengine"),
        "object storage implementation supplement must capture the deployment default"
    );
    assert!(
        rtc_recording_plan_doc.contains("RTC recording artifact")
            && rtc_recording_plan_doc.contains("object-storage-volcengine"),
        "rtc recording implementation supplement must capture runtime rebinding and deployment default"
    );
    assert!(
        provider_binding_doc.contains("GET /api/v1/control/provider-bindings")
            && provider_binding_doc.contains("tenantId")
            && provider_binding_doc.contains("tenant_override")
            && provider_binding_doc.contains("deployment_profile"),
        "provider binding architecture doc must capture the dynamic control-plane binding view"
    );
    assert!(
        provider_binding_plan_doc.contains("object-storage-volcengine")
            && provider_binding_plan_doc.contains("rtc-aliyun")
            && provider_binding_plan_doc.contains("effectiveBindings"),
        "provider binding implementation supplement must freeze the evaluated binding examples"
    );
    assert!(
        ops_provider_binding_doc.contains("GET /api/v1/ops/provider-bindings")
            && ops_provider_binding_doc.contains("providerBindings")
            && ops_provider_binding_doc.contains("control-plane-api")
            && ops_provider_binding_doc.contains("OpsRuntime"),
        "ops provider binding architecture doc must capture the runtime consumption chain"
    );
    assert!(
        ops_provider_binding_drift_doc.contains("GET /api/v1/ops/provider-bindings/drift")
            && ops_provider_binding_drift_doc.contains("providerBindingDrift")
            && ops_provider_binding_drift_doc.contains("plugin_and_selection_source_changed"),
        "ops provider binding drift architecture doc must capture the drift surface and semantics"
    );
    assert!(
        ops_provider_binding_plan_doc.contains("GET /api/v1/ops/provider-bindings")
            && ops_provider_binding_plan_doc.contains("diagnostic bundle")
            && ops_provider_binding_plan_doc.contains("tenant override"),
        "ops provider binding implementation supplement must freeze the ops-facing closure"
    );
    assert!(
        ops_provider_binding_drift_plan_doc.contains("GET /api/v1/ops/provider-bindings/drift")
            && ops_provider_binding_drift_plan_doc.contains("providerBindingDrift")
            && ops_provider_binding_drift_plan_doc.contains("plugin_and_selection_source_changed"),
        "ops provider binding drift implementation supplement must freeze the drift contract"
    );
    assert!(
        provider_policy_write_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_write_doc.contains("RuntimeProviderRegistry")
            && provider_policy_write_doc.contains("control.provider_tenant_override_updated"),
        "provider policy write architecture doc must capture the write surface and audit actions"
    );
    assert!(
        provider_policy_write_plan_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_write_plan_doc
                .contains("control.provider_deployment_profile_updated")
            && provider_policy_write_plan_doc.contains("cross-domain plugin id"),
        "provider policy write implementation supplement must freeze the write contract"
    );
    assert!(
        provider_policy_version_doc.contains("GET /api/v1/control/provider-policies")
            && provider_policy_version_doc
                .contains("POST /api/v1/control/provider-policies/rollback")
            && provider_policy_version_doc.contains("rollbackFromVersion")
            && provider_policy_version_doc.contains("control.provider_policy_rolled_back")
            && provider_policy_version_doc.contains("replace_provider_binding_snapshots"),
        "provider policy version architecture doc must capture history, rollback, audit and ops refresh"
    );
    assert!(
        provider_policy_version_plan_doc.contains("07-C4")
            && provider_policy_version_plan_doc.contains("09G")
            && provider_policy_version_plan_doc.contains("150G")
            && provider_policy_version_plan_doc.contains("targetVersion")
            && provider_policy_version_plan_doc.contains("rollbackFromVersion"),
        "provider policy version implementation supplement must freeze the 07-C4 rollback plan"
    );
    assert!(
        provider_policy_diff_doc.contains("GET /api/v1/control/provider-policies/diff")
            && provider_policy_diff_doc.contains("fromVersion")
            && provider_policy_diff_doc.contains("toVersion")
            && provider_policy_diff_doc.contains("deploymentProfileChanges")
            && provider_policy_diff_doc.contains("tenantOverrideChanges")
            && provider_policy_diff_doc.contains("changeKind"),
        "provider policy diff architecture doc must capture the diff read surface and response model"
    );
    assert!(
        provider_policy_diff_plan_doc.contains("07-C5")
            && provider_policy_diff_plan_doc.contains("09H")
            && provider_policy_diff_plan_doc.contains("150H")
            && provider_policy_diff_plan_doc.contains("fromVersion")
            && provider_policy_diff_plan_doc.contains("toVersion")
            && provider_policy_diff_plan_doc.contains("changeKind"),
        "provider policy diff implementation supplement must freeze the 07-C5 diff contract"
    );
    assert!(
        provider_policy_preview_doc.contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_preview_doc.contains("previewBinding")
            && provider_policy_preview_doc.contains("baseVersion")
            && provider_policy_preview_doc.contains("previewVersion")
            && provider_policy_preview_doc.contains("control.write"),
        "provider policy preview architecture doc must capture the preview read-before-write surface"
    );
    assert!(
        provider_policy_preview_confirmation_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_preview_confirmation_doc.contains("expectedBaseVersion")
            && provider_policy_preview_confirmation_doc.contains("provider policy version drift")
            && provider_policy_preview_confirmation_doc.contains("provider_policy_conflict"),
        "provider policy preview confirmation architecture doc must capture stale-write rejection"
    );
    assert!(
        provider_policy_commit_response_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_commit_response_doc.contains("currentVersion")
            && provider_policy_commit_response_doc.contains("committedBinding")
            && provider_policy_commit_response_doc.contains("diff"),
        "provider policy commit response architecture doc must capture committed write echo"
    );
    assert!(
        provider_policy_noop_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_noop_doc.contains("applied")
            && provider_policy_noop_doc.contains("no-op")
            && provider_policy_noop_doc.contains("currentVersion"),
        "provider policy noop architecture doc must capture noop suppression semantics"
    );
    assert!(
        provider_policy_status_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_status_doc
                .contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_status_doc.contains("ProviderPolicyResultStatus")
            && provider_policy_status_doc.contains("status")
            && provider_policy_status_doc.contains("preview")
            && provider_policy_status_doc.contains("applied")
            && provider_policy_status_doc.contains("noop"),
        "provider policy status architecture doc must capture the normalized preview/applied/noop status contract"
    );
    assert!(
        provider_policy_read_status_doc.contains("GET /api/v1/control/provider-policies")
            && provider_policy_read_status_doc
                .contains("GET /api/v1/control/provider-policies/diff")
            && provider_policy_read_status_doc
                .contains("POST /api/v1/control/provider-policies/rollback")
            && provider_policy_read_status_doc.contains("status")
            && provider_policy_read_status_doc.contains("history")
            && provider_policy_read_status_doc.contains("diff")
            && provider_policy_read_status_doc.contains("rolled_back"),
        "provider policy history/diff/rollback status architecture doc must capture the normalized read and rollback status contract"
    );
    assert!(
        provider_policy_error_status_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_error_status_doc
                .contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_error_status_doc.contains("status")
            && provider_policy_error_status_doc.contains("code")
            && provider_policy_error_status_doc.contains("invalid")
            && provider_policy_error_status_doc.contains("conflict")
            && provider_policy_error_status_doc.contains("unavailable")
            && provider_policy_error_status_doc.contains("forbidden")
            && provider_policy_error_status_doc.contains("unauthorized"),
        "provider policy error status architecture doc must capture the normalized error status contract"
    );
    assert!(
        provider_policy_error_status_doc.contains("unknown provider policy version")
            && provider_policy_error_status_doc.contains("provider_policy_conflict")
            && provider_policy_error_status_doc
                .contains("provider-policy routes do not emit `status=not_found`"),
        "provider policy error status architecture doc must clarify that unknown policy versions stay in the conflict vocabulary"
    );
    assert!(
        provider_surface_status_doc.contains("GET /api/v1/control/provider-registry")
            && provider_surface_status_doc.contains("GET /api/v1/control/provider-bindings")
            && provider_surface_status_doc.contains("status")
            && provider_surface_status_doc.contains("registry")
            && provider_surface_status_doc.contains("bindings"),
        "provider snapshot status architecture doc must capture the normalized registry/bindings read status contract"
    );
    let provider_policy_error_vocabulary_doc = read_repo_file(
        "docs/架构/150Q-control-plane-provider-policy-error-vocabulary设计-2026-04-08.md",
    );
    assert!(
        provider_policy_error_vocabulary_doc.contains("GET /api/v1/control/provider-policies/diff")
            && provider_policy_error_vocabulary_doc
                .contains("POST /api/v1/control/provider-policies/rollback")
            && provider_policy_error_vocabulary_doc.contains("unknown provider policy version")
            && provider_policy_error_vocabulary_doc.contains("status=conflict")
            && provider_policy_error_vocabulary_doc.contains("07-C14"),
        "provider policy error vocabulary architecture doc must freeze the unknown-version conflict closure"
    );
    let provider_status_matrix_doc =
        read_repo_file("docs/架构/150R-control-plane-provider-status-matrix设计-2026-04-08.md");
    assert!(
        provider_status_matrix_doc.contains("07-C15")
            && provider_status_matrix_doc.contains("registry")
            && provider_status_matrix_doc.contains("bindings")
            && provider_status_matrix_doc.contains("preview")
            && provider_status_matrix_doc.contains("applied")
            && provider_status_matrix_doc.contains("noop")
            && provider_status_matrix_doc.contains("history")
            && provider_status_matrix_doc.contains("diff")
            && provider_status_matrix_doc.contains("rolled_back")
            && provider_status_matrix_doc.contains("invalid")
            && provider_status_matrix_doc.contains("conflict")
            && provider_status_matrix_doc.contains("unavailable")
            && provider_status_matrix_doc.contains("forbidden")
            && provider_status_matrix_doc.contains("unauthorized"),
        "provider status matrix architecture doc must freeze the consolidated provider control-plane status vocabulary"
    );
    assert!(
        provider_policy_preview_plan_doc.contains("07-C6")
            && provider_policy_preview_plan_doc.contains("09I")
            && provider_policy_preview_plan_doc.contains("150I")
            && provider_policy_preview_plan_doc.contains("previewBinding")
            && provider_policy_preview_plan_doc.contains("baseVersion")
            && provider_policy_preview_plan_doc.contains("previewVersion"),
        "provider policy preview implementation supplement must freeze the 07-C6 preview contract"
    );
    assert!(
        provider_policy_preview_confirmation_plan_doc.contains("07-C7")
            && provider_policy_preview_confirmation_plan_doc.contains("09J")
            && provider_policy_preview_confirmation_plan_doc.contains("150J")
            && provider_policy_preview_confirmation_plan_doc.contains("expectedBaseVersion")
            && provider_policy_preview_confirmation_plan_doc.contains("provider_policy_conflict"),
        "provider policy preview confirmation implementation supplement must freeze the 07-C7 confirmation contract"
    );
    assert!(
        provider_policy_commit_response_plan_doc.contains("07-C8")
            && provider_policy_commit_response_plan_doc.contains("09K")
            && provider_policy_commit_response_plan_doc.contains("150K")
            && provider_policy_commit_response_plan_doc.contains("currentVersion")
            && provider_policy_commit_response_plan_doc.contains("committedBinding")
            && provider_policy_commit_response_plan_doc.contains("diff"),
        "provider policy commit response implementation supplement must freeze the 07-C8 committed echo contract"
    );
    assert!(
        provider_policy_noop_plan_doc.contains("07-C9")
            && provider_policy_noop_plan_doc.contains("09L")
            && provider_policy_noop_plan_doc.contains("150L")
            && provider_policy_noop_plan_doc.contains("applied")
            && provider_policy_noop_plan_doc.contains("no-op"),
        "provider policy noop implementation supplement must freeze the 07-C9 noop contract"
    );
    assert!(
        provider_policy_status_plan_doc.contains("07-C10")
            && provider_policy_status_plan_doc.contains("09M")
            && provider_policy_status_plan_doc.contains("150M")
            && provider_policy_status_plan_doc.contains("ProviderPolicyResultStatus")
            && provider_policy_status_plan_doc.contains("status")
            && provider_policy_status_plan_doc.contains("preview")
            && provider_policy_status_plan_doc.contains("applied")
            && provider_policy_status_plan_doc.contains("noop"),
        "provider policy status implementation supplement must freeze the 07-C10 status contract"
    );
    assert!(
        provider_policy_read_status_plan_doc.contains("07-C11")
            && provider_policy_read_status_plan_doc.contains("09N")
            && provider_policy_read_status_plan_doc.contains("150N")
            && provider_policy_read_status_plan_doc.contains("status")
            && provider_policy_read_status_plan_doc.contains("history")
            && provider_policy_read_status_plan_doc.contains("diff")
            && provider_policy_read_status_plan_doc.contains("rolled_back"),
        "provider policy history/diff/rollback status implementation supplement must freeze the 07-C11 status contract"
    );
    assert!(
        provider_policy_error_status_plan_doc.contains("07-C12")
            && provider_policy_error_status_plan_doc.contains("09O")
            && provider_policy_error_status_plan_doc.contains("150O")
            && provider_policy_error_status_plan_doc.contains("status")
            && provider_policy_error_status_plan_doc.contains("code")
            && provider_policy_error_status_plan_doc.contains("invalid")
            && provider_policy_error_status_plan_doc.contains("conflict")
            && provider_policy_error_status_plan_doc.contains("unavailable")
            && provider_policy_error_status_plan_doc.contains("forbidden")
            && provider_policy_error_status_plan_doc.contains("unauthorized"),
        "provider policy error status implementation supplement must freeze the 07-C12 error contract"
    );
    assert!(
        provider_policy_error_status_plan_doc.contains("unknown provider policy version")
            && provider_policy_error_status_plan_doc.contains("provider_policy_conflict")
            && provider_policy_error_status_plan_doc
                .contains("provider-policy routes do not emit `status=not_found`"),
        "provider policy error status implementation supplement must correct the provider-policy error vocabulary"
    );
    assert!(
        provider_surface_status_plan_doc.contains("07-C13")
            && provider_surface_status_plan_doc.contains("09P")
            && provider_surface_status_plan_doc.contains("150P")
            && provider_surface_status_plan_doc.contains("status")
            && provider_surface_status_plan_doc.contains("registry")
            && provider_surface_status_plan_doc.contains("bindings"),
        "provider snapshot status implementation supplement must freeze the 07-C13 snapshot status contract"
    );
    let provider_policy_error_vocabulary_plan_doc =
        read_repo_file("docs/架构/09Q-实施计划-provider-policy错误词汇收敛-2026-04-08.md");
    assert!(
        provider_policy_error_vocabulary_plan_doc.contains("07-C14")
            && provider_policy_error_vocabulary_plan_doc.contains("09Q")
            && provider_policy_error_vocabulary_plan_doc.contains("150Q")
            && provider_policy_error_vocabulary_plan_doc
                .contains("unknown provider policy version")
            && provider_policy_error_vocabulary_plan_doc.contains("status=conflict"),
        "provider policy error vocabulary implementation supplement must freeze the 07-C14 conflict vocabulary closure"
    );
    let provider_status_matrix_plan_doc =
        read_repo_file("docs/架构/09R-实施计划-provider状态矩阵收敛-2026-04-08.md");
    assert!(
        provider_status_matrix_plan_doc.contains("07-C15")
            && provider_status_matrix_plan_doc.contains("09R")
            && provider_status_matrix_plan_doc.contains("150R")
            && provider_status_matrix_plan_doc.contains("registry")
            && provider_status_matrix_plan_doc.contains("applied")
            && provider_status_matrix_plan_doc.contains("rolled_back")
            && provider_status_matrix_plan_doc.contains("unauthorized"),
        "provider status matrix implementation supplement must freeze the 07-C15 status vocabulary closure"
    );
    assert!(
        provider_policy_version_review_doc.contains("GET /api/v1/control/provider-policies")
            && provider_policy_version_review_doc
                .contains("POST /api/v1/control/provider-policies/rollback")
            && provider_policy_version_review_doc.contains("control.provider_policy_rolled_back"),
        "provider policy version review doc must capture the delivered rollback closure"
    );
    assert!(
        provider_policy_diff_review_doc.contains("GET /api/v1/control/provider-policies/diff")
            && provider_policy_diff_review_doc.contains("deploymentProfileChanges")
            && provider_policy_diff_review_doc.contains("tenantOverrideChanges")
            && provider_policy_diff_review_doc.contains("changeKind"),
        "provider policy diff review doc must capture the delivered diff closure"
    );
    assert!(
        provider_policy_preview_review_doc
            .contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_preview_review_doc.contains("previewBinding")
            && provider_policy_preview_review_doc.contains("baseVersion")
            && provider_policy_preview_review_doc.contains("control.write"),
        "provider policy preview review doc must capture the delivered preview closure"
    );
    assert!(
        provider_policy_preview_confirmation_review_doc
            .contains("POST /api/v1/control/provider-bindings")
            && provider_policy_preview_confirmation_review_doc.contains("expectedBaseVersion")
            && provider_policy_preview_confirmation_review_doc.contains("provider_policy_conflict"),
        "provider policy preview confirmation review doc must capture the delivered stale-write closure"
    );
    assert!(
        provider_policy_commit_response_review_doc
            .contains("POST /api/v1/control/provider-bindings")
            && provider_policy_commit_response_review_doc.contains("currentVersion")
            && provider_policy_commit_response_review_doc.contains("committedBinding")
            && provider_policy_commit_response_review_doc.contains("diff"),
        "provider policy commit response review doc must capture the delivered committed echo closure"
    );
    assert!(
        provider_policy_noop_review_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_noop_review_doc.contains("applied")
            && provider_policy_noop_review_doc.contains("no-op"),
        "provider policy noop review doc must capture the delivered noop closure"
    );
    assert!(
        provider_policy_status_review_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_status_review_doc
                .contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_status_review_doc.contains("status")
            && provider_policy_status_review_doc.contains("preview")
            && provider_policy_status_review_doc.contains("applied")
            && provider_policy_status_review_doc.contains("noop"),
        "provider policy status review doc must capture the delivered normalized status closure"
    );
    assert!(
        provider_policy_read_status_review_doc.contains("GET /api/v1/control/provider-policies")
            && provider_policy_read_status_review_doc
                .contains("GET /api/v1/control/provider-policies/diff")
            && provider_policy_read_status_review_doc
                .contains("POST /api/v1/control/provider-policies/rollback")
            && provider_policy_read_status_review_doc.contains("status")
            && provider_policy_read_status_review_doc.contains("history")
            && provider_policy_read_status_review_doc.contains("diff")
            && provider_policy_read_status_review_doc.contains("rolled_back"),
        "provider policy history/diff/rollback status review doc must capture the delivered normalized read and rollback closure"
    );
    assert!(
        provider_policy_error_status_review_doc.contains("POST /api/v1/control/provider-bindings")
            && provider_policy_error_status_review_doc
                .contains("POST /api/v1/control/provider-policies/preview")
            && provider_policy_error_status_review_doc.contains("status")
            && provider_policy_error_status_review_doc.contains("code")
            && provider_policy_error_status_review_doc.contains("invalid")
            && provider_policy_error_status_review_doc.contains("conflict")
            && provider_policy_error_status_review_doc.contains("unavailable")
            && provider_policy_error_status_review_doc.contains("forbidden")
            && provider_policy_error_status_review_doc.contains("unauthorized"),
        "provider policy error status review doc must capture the delivered normalized error closure"
    );
    assert!(
        provider_policy_error_status_review_doc.contains("unknown provider policy version")
            && provider_policy_error_status_review_doc.contains("provider_policy_conflict")
            && provider_policy_error_status_review_doc
                .contains("provider-policy routes do not emit `status=not_found`"),
        "provider policy error status review doc must correct the provider-policy error vocabulary"
    );
    assert!(
        provider_surface_status_review_doc.contains("GET /api/v1/control/provider-registry")
            && provider_surface_status_review_doc.contains("GET /api/v1/control/provider-bindings")
            && provider_surface_status_review_doc.contains("status")
            && provider_surface_status_review_doc.contains("registry")
            && provider_surface_status_review_doc.contains("bindings"),
        "provider snapshot status review doc must capture the delivered normalized registry/bindings status closure"
    );
    let provider_policy_error_vocabulary_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-policy-error-vocabulary-2026-04-08.md",
    );
    assert!(
        provider_policy_error_vocabulary_review_doc.contains("unknown provider policy version")
            && provider_policy_error_vocabulary_review_doc.contains("status=conflict")
            && provider_policy_error_vocabulary_review_doc.contains("07-C14")
            && provider_policy_error_vocabulary_review_doc.contains("09Q")
            && provider_policy_error_vocabulary_review_doc.contains("150Q"),
        "provider policy error vocabulary review doc must capture the delivered unknown-version conflict closure"
    );
    let provider_status_matrix_review_doc = read_repo_file(
        "docs/review/continuous-optimization-control-plane-provider-status-matrix-2026-04-08.md",
    );
    assert!(
        provider_status_matrix_review_doc.contains("07-C15")
            && provider_status_matrix_review_doc.contains("09R")
            && provider_status_matrix_review_doc.contains("150R")
            && provider_status_matrix_review_doc.contains("registry")
            && provider_status_matrix_review_doc.contains("conflict")
            && provider_status_matrix_review_doc.contains("unauthorized"),
        "provider status matrix review doc must capture the delivered consolidated status vocabulary"
    );
    assert!(
        adapter_doc.contains("signed_download_url") && adapter_doc.contains("ProviderRegistry"),
        "adapter readme must document runtime binding and signed URL rules"
    );
}

#[test]
fn test_step_docs_and_prompt_capture_repeated_iteration_requirements() {
    let step_five = read_repo_file("docs/step/05-消息与会话主链路重构.md");
    let step_six = read_repo_file("docs/step/06-流式与RTC实时能力重构.md");
    let step_six_b = read_repo_file("docs/step/06-B-对象存储插件与媒体运行时闭环-2026-04-08.md");
    let step_six_c = read_repo_file("docs/step/06-C-RTC录制对象存储运行时闭环-2026-04-08.md");
    let step_seven = read_repo_file("docs/step/07-控制面与协议治理落地.md");
    let step_seven_a =
        read_repo_file("docs/step/07-A-控制面provider绑定求值可见性闭环-2026-04-08.md");
    let step_seven_c = read_repo_file("docs/step/07-C-控制面provider绑定ops消费闭环-2026-04-08.md");
    let step_seven_c2 =
        read_repo_file("docs/step/07-C2-控制面provider绑定漂移视图闭环-2026-04-08.md");
    let step_seven_c3 =
        read_repo_file("docs/step/07-C3-控制面provider-policy写接口与审计闭环-2026-04-08.md");
    let step_seven_c4 =
        read_repo_file("docs/step/07-C4-控制面provider-policy版本与回滚快照闭环-2026-04-08.md");
    let step_seven_c5 =
        read_repo_file("docs/step/07-C5-控制面provider-policy差异查询闭环-2026-04-08.md");
    let step_seven_c6 =
        read_repo_file("docs/step/07-C6-控制面provider-policy写前预览闭环-2026-04-08.md");
    let step_seven_c7 =
        read_repo_file("docs/step/07-C7-控制面provider-policy预览确认写入闭环-2026-04-08.md");
    let step_seven_c8 =
        read_repo_file("docs/step/07-C8-控制面provider-policy提交结果回显闭环-2026-04-08.md");
    let step_seven_c9 =
        read_repo_file("docs/step/07-C9-控制面provider-policy-noop抑制闭环-2026-04-08.md");
    let step_seven_c10 =
        read_repo_file("docs/step/07-C10-控制面provider-policy结果状态统一闭环-2026-04-08.md");
    let step_seven_c11 =
        read_repo_file("docs/step/07-C11-控制面provider-policy历史diff回滚状态闭环-2026-04-08.md");
    let step_seven_c12 =
        read_repo_file("docs/step/07-C12-控制面provider-policy错误状态闭环-2026-04-08.md");
    let step_seven_c13 =
        read_repo_file("docs/step/07-C13-控制面provider快照状态闭环-2026-04-08.md");
    let step_eight = read_repo_file("docs/step/08-AI-Agent-IoT统一扩展层落地.md");
    let step_prompt = read_repo_file("docs/prompts/反复执行Step指令.md");

    assert!(
        step_five.contains("user-module-local / user-module-external"),
        "step 05 must cover the two user-module plugin modes"
    );
    assert!(
        step_six.contains("默认 RTC provider") && step_six.contains("火山引擎 / 阿里云 / 腾讯云"),
        "step 06 must freeze the RTC provider matrix"
    );
    assert!(
        step_six_b.contains("object-storage-s3")
            && step_six_b.contains("GET /api/v1/media/provider-health")
            && step_six_b.contains("GET /api/v1/media/{media_asset_id}/download-url"),
        "step 06-B must capture the object storage closure"
    );
    assert!(
        step_six_c.contains("RtcRecordingArtifact")
            && step_six_c.contains("GET /api/v1/rtc/sessions/{rtc_session_id}/artifacts/recording")
            && step_six_c.contains("object-storage-volcengine"),
        "step 06-C must capture the rtc recording artifact closure"
    );
    assert!(
        step_seven.contains("provider registry / effective bindings")
            && step_seven.contains("deployment_profile")
            && step_seven.contains("provider drift")
            && step_seven.contains("provider policy 写接口"),
        "step 07 must freeze provider binding governance in the control-plane baseline"
    );
    assert!(
        step_seven_a.contains("GET /api/v1/control/provider-bindings")
            && step_seven_a.contains("tenantId")
            && step_seven_a.contains("object-storage-volcengine"),
        "step 07-A must capture the provider binding visibility closure"
    );
    assert!(
        step_seven_c.contains("GET /api/v1/ops/provider-bindings")
            && step_seven_c.contains("OpsRuntime")
            && step_seven_c.contains("providerBindings"),
        "step 07-C must capture the ops-facing provider binding consumption closure"
    );
    assert!(
        step_seven_c2.contains("GET /api/v1/ops/provider-bindings/drift")
            && step_seven_c2.contains("providerBindingDrift")
            && step_seven_c2.contains("plugin_and_selection_source_changed"),
        "step 07-C2 must capture the provider binding drift closure"
    );
    assert!(
        step_seven_c3.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c3.contains("RuntimeProviderRegistry")
            && step_seven_c3.contains("control.provider_tenant_override_updated"),
        "step 07-C3 must capture the provider policy write and audit closure"
    );
    assert!(
        step_seven_c4.contains("GET /api/v1/control/provider-policies")
            && step_seven_c4.contains("POST /api/v1/control/provider-policies/rollback")
            && step_seven_c4.contains("rollbackFromVersion")
            && step_seven_c4.contains("control.provider_policy_rolled_back"),
        "step 07-C4 must capture the provider policy version and rollback closure"
    );
    assert!(
        step_seven_c5.contains("GET /api/v1/control/provider-policies/diff")
            && step_seven_c5.contains("fromVersion")
            && step_seven_c5.contains("toVersion")
            && step_seven_c5.contains("deploymentProfileChanges")
            && step_seven_c5.contains("tenantOverrideChanges")
            && step_seven_c5.contains("changeKind"),
        "step 07-C5 must capture the provider policy diff closure"
    );
    assert!(
        step_seven_c6.contains("POST /api/v1/control/provider-policies/preview")
            && step_seven_c6.contains("previewBinding")
            && step_seven_c6.contains("baseVersion")
            && step_seven_c6.contains("previewVersion")
            && step_seven_c6.contains("无副作用"),
        "step 07-C6 must capture the provider policy preview closure"
    );
    assert!(
        step_seven_c7.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c7.contains("expectedBaseVersion")
            && step_seven_c7.contains("provider_policy_conflict")
            && step_seven_c7.contains("409"),
        "step 07-C7 must capture the preview confirmation stale-write closure"
    );
    assert!(
        step_seven_c8.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c8.contains("currentVersion")
            && step_seven_c8.contains("committedBinding")
            && step_seven_c8.contains("diff"),
        "step 07-C8 must capture the committed result echo closure"
    );
    assert!(
        step_seven_c9.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c9.contains("applied")
            && step_seven_c9.contains("no-op")
            && step_seven_c9.contains("currentVersion"),
        "step 07-C9 must capture the noop suppression closure"
    );
    assert!(
        step_seven_c10.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c10.contains("POST /api/v1/control/provider-policies/preview")
            && step_seven_c10.contains("ProviderPolicyResultStatus")
            && step_seven_c10.contains("status")
            && step_seven_c10.contains("preview")
            && step_seven_c10.contains("applied")
            && step_seven_c10.contains("noop"),
        "step 07-C10 must capture the normalized provider policy status closure"
    );
    assert!(
        step_seven_c11.contains("GET /api/v1/control/provider-policies")
            && step_seven_c11.contains("GET /api/v1/control/provider-policies/diff")
            && step_seven_c11.contains("POST /api/v1/control/provider-policies/rollback")
            && step_seven_c11.contains("status")
            && step_seven_c11.contains("history")
            && step_seven_c11.contains("diff")
            && step_seven_c11.contains("rolled_back"),
        "step 07-C11 must capture the normalized provider policy history/diff/rollback status closure"
    );
    assert!(
        step_seven_c12.contains("POST /api/v1/control/provider-bindings")
            && step_seven_c12.contains("POST /api/v1/control/provider-policies/preview")
            && step_seven_c12.contains("status")
            && step_seven_c12.contains("code")
            && step_seven_c12.contains("invalid")
            && step_seven_c12.contains("conflict")
            && step_seven_c12.contains("unavailable")
            && step_seven_c12.contains("forbidden")
            && step_seven_c12.contains("unauthorized"),
        "step 07-C12 must capture the normalized provider policy error status closure"
    );
    assert!(
        step_seven_c12.contains("unknown provider policy version")
            && step_seven_c12.contains("provider_policy_conflict")
            && step_seven_c12.contains("provider-policy routes do not emit `status=not_found`"),
        "step 07-C12 must reflect the corrected provider-policy error vocabulary"
    );
    assert!(
        step_seven_c13.contains("GET /api/v1/control/provider-registry")
            && step_seven_c13.contains("GET /api/v1/control/provider-bindings")
            && step_seven_c13.contains("status")
            && step_seven_c13.contains("registry")
            && step_seven_c13.contains("bindings"),
        "step 07-C13 must capture the normalized provider snapshot status closure"
    );
    let step_seven_c14 =
        read_repo_file("docs/step/07-C14-控制面provider-policy错误词汇收敛-2026-04-08.md");
    assert!(
        step_seven_c14.contains("GET /api/v1/control/provider-policies/diff")
            && step_seven_c14.contains("POST /api/v1/control/provider-policies/rollback")
            && step_seven_c14.contains("unknown provider policy version")
            && step_seven_c14.contains("status=conflict")
            && step_seven_c14.contains("07-C14 / 09Q / 150Q"),
        "step 07-C14 must capture the provider-policy unknown-version conflict closure"
    );
    let step_seven_c15 =
        read_repo_file("docs/step/07-C15-控制面provider状态矩阵收敛-2026-04-08.md");
    assert!(
        step_seven_c15.contains("07-C15 / 09R / 150R")
            && step_seven_c15.contains("registry")
            && step_seven_c15.contains("bindings")
            && step_seven_c15.contains("preview")
            && step_seven_c15.contains("applied")
            && step_seven_c15.contains("noop")
            && step_seven_c15.contains("history")
            && step_seven_c15.contains("diff")
            && step_seven_c15.contains("rolled_back")
            && step_seven_c15.contains("invalid")
            && step_seven_c15.contains("conflict")
            && step_seven_c15.contains("unavailable")
            && step_seven_c15.contains("forbidden")
            && step_seven_c15.contains("unauthorized"),
        "step 07-C15 must capture the consolidated provider control-plane status vocabulary"
    );
    assert!(
        step_eight.contains("MQTT") && step_eight.contains("小智"),
        "step 08 must keep the IoT plugin matrix visible"
    );
    assert!(
        step_prompt.lines().count() <= 12,
        "the repeated-step prompt should stay concise"
    );

    for required in [
        "step / 波次 / 是否闭环",
        "docs/review",
        "docs/step",
        "docs/架构",
        "deployment_profile",
        "下一轮动作",
    ] {
        assert!(
            step_prompt.contains(required),
            "the repeated-step prompt must still cover {required}"
        );
    }
}
