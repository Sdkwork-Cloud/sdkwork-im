use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn test_im_commercial_gate_covers_strict_lints_and_exactly_once_regressions() {
    let workflow_path = repo_root().join(".github/workflows/im-commercial-gates.yml");
    let workflow = fs::read_to_string(&workflow_path).unwrap_or_else(|_| {
        panic!(
            "commercial gate workflow should exist at {}",
            workflow_path.display()
        )
    });

    for required_command in [
        "cargo test -p session-gateway --tests",
        "cargo test -p session-gateway --test http_smoke_test test_presence_heartbeat_rejects_same_route_id_with_different_actor_kind_over_http -- --exact",
        "cargo test -p session-gateway --test http_smoke_test test_presence_heartbeat_rejects_same_route_id_with_different_principal_over_http -- --exact",
        "cargo test -p session-gateway --test http_smoke_test test_realtime_subscription_sync_rejects_oversized_event_types_payload_over_http -- --exact",
        "cargo test -p session-gateway --test http_smoke_test test_realtime_subscription_sync_rejects_too_many_items_over_http -- --exact",
        "cargo test -p session-gateway --test websocket_smoke_test test_realtime_websocket_rejects_oversized_request_id -- --exact",
        "cargo test -p session-gateway --test websocket_smoke_test test_realtime_websocket_rejects_oversized_frame_type -- --exact",
        "cargo test -p im-postgres-realtime-contracts --test postgres_realtime_contracts_test",
        "cargo test -p im-adapters-postgres-realtime --test postgres_realtime_checkpoint_store_test",
        "cargo test -p im-adapters-postgres-realtime --test postgres_realtime_live_integration_test -- --nocapture",
        "cargo test -p session-gateway --test postgres_realtime_live_runtime_test -- --nocapture",
        "cargo test -p session-gateway --test postgres_realtime_websocket_live_drill_test -- --nocapture",
        "cargo test -p local-minimal-node --test realtime_storage_provider_contract_test",
        "cargo clippy -p im-postgres-realtime-contracts -p im-adapters-postgres-realtime --tests -- -D warnings",
        "cargo clippy -p im-postgres-realtime-contracts -p im-adapters-postgres-realtime -p session-gateway --tests -- -D warnings",
        "cargo clippy -p im-postgres-realtime-contracts -p im-adapters-postgres-realtime -p local-minimal-node --tests -- -D warnings",
        "cargo test -p projection-service --tests",
        "cargo test -p projection-service --test http_smoke_test test_timeline_query_rejects_same_actor_id_with_different_actor_kind_over_http -- --exact",
        "cargo test -p projection-service --test http_smoke_test test_contacts_query_rejects_same_actor_id_with_different_actor_kind_over_http -- --exact",
        "cargo test -p media-service --tests",
        "cargo test -p streaming-service --tests",
        "cargo test -p audit-service --tests",
        "cargo clippy -p conversation-runtime -p control-plane-api --tests -- -D warnings",
        "cargo test -p conversation-runtime --test conversation_flow_test test_duplicate_bind_direct_chat_conversation_is_idempotent_and_conflicting_retry_is_rejected -- --exact",
        "cargo test -p conversation-runtime --test http_smoke_test test_duplicate_bind_direct_chat_conversation_request_is_idempotent_and_conflicting_retry_is_rejected_over_http -- --exact",
        "cargo test -p conversation-runtime --test conversation_flow_test test_create_conversation_rejects_oversized_creator_attributes -- --exact",
        "cargo test -p conversation-runtime --test http_smoke_test test_add_member_rejects_oversized_attributes_over_http -- --exact",
        "cargo test -p conversation-runtime --test conversation_flow_test test_post_message_rejects_oversized_sender_metadata -- --exact",
        "cargo test -p conversation-runtime --test conversation_flow_test test_post_message_rejects_oversized_render_hints -- --exact",
        "cargo test -p im-platform-contracts --test provider_registry_contract_test test_runtime_provider_registry_rejects_reversed_policy_diff_version_range -- --exact",
        "cargo test -p control-plane-api --test provider_registry_test test_control_plane_rejects_provider_policy_diff_with_reversed_version_range -- --exact",
        "cargo test -p im-platform-contracts --test provider_registry_contract_test test_runtime_provider_registry_rejects_empty_tenant_override_id -- --exact",
        "cargo test -p control-plane-api --test governance_loop_test test_control_plane_rejects_empty_tenant_provider_bindings_query_without_polluting_ops_runtime -- --exact",
        "cargo test -p control-plane-api --test governance_loop_test test_control_plane_rejects_empty_tenant_provider_policy_write_without_mutating_ops_or_audit -- --exact",
        "cargo test -p im-platform-contracts --test provider_registry_contract_test test_runtime_provider_registry_rejects_oversized_tenant_override_id -- --exact",
        "cargo test -p control-plane-api --test governance_loop_test test_control_plane_rejects_oversized_tenant_provider_bindings_query_without_polluting_ops_runtime -- --exact",
        "cargo test -p control-plane-api --test governance_loop_test test_control_plane_rejects_oversized_tenant_provider_policy_write_without_mutating_ops_or_audit -- --exact",
        "cargo test -p notification-service --test http_smoke_test test_duplicate_notification_id_is_idempotent_and_conflicting_retry_is_rejected_over_http -- --exact",
        "cargo test -p notification-service --test http_smoke_test test_notification_queries_reject_same_actor_id_with_different_actor_kind_over_http -- --exact",
        "cargo test -p automation-service --test http_smoke_test test_duplicate_execution_id_is_idempotent_and_conflicting_retry_is_rejected_over_http -- --exact",
        "cargo test -p local-minimal-node --test task10_capabilities_e2e_test test_local_minimal_profile_treats_duplicate_notification_request_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_notification_queries_from_different_actor_kind -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_same_actor_id_with_different_actor_kind_on_contacts_query -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_preserves_add_member_request_attributes_for_non_user_principal -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_oversized_render_hints_on_post_message -- --exact",
        "cargo test -p local-minimal-node --test commercial_gate_contract_test",
        "cargo test -p local-minimal-node --test commercial_readiness_gate_test",
        "cargo test -p local-minimal-node --test runtime_cli_error_handling_test",
        "cargo test -p local-minimal-node --test principal_profile_provider_mainline_test test_local_principal_profile_provider_rejects_oversized_creator_attributes_on_create_conversation -- --exact",
        "cargo test -p local-minimal-node --test principal_profile_provider_mainline_test test_local_principal_profile_provider_rejects_oversized_sender_metadata_on_post_message -- --exact",
        "cargo test -p local-minimal-node --test principal_profile_provider_mainline_test test_local_principal_profile_provider_merges_add_member_request_attributes -- --exact",
        "cargo test -p local-minimal-node --test task10_capabilities_e2e_test test_local_minimal_profile_treats_duplicate_automation_request_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test task10_capabilities_e2e_test test_local_minimal_profile_isolates_automation_notifications_by_actor_kind -- --exact",
        "cargo test -p local-minimal-node --test task10_capabilities_e2e_test test_local_minimal_profile_records_automation_audit_per_actor_kind -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_runs_end_to_end_flow -- --exact",
        "cargo test -p local-minimal-node --test openapi_schema_export_test",
        "cargo test -p local-minimal-node --test openapi_im_v3_contract_test",
        "cargo test -p local-minimal-node --test chat_runtime_session_namespace_test",
        "cargo test -p local-minimal-node --test runtime_config_test",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_treats_duplicate_direct_chat_binding_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_does_not_refanout_duplicate_message_post_retry -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_does_not_refanout_duplicate_stream_frame_retry -- --exact",
        "node scripts/auth-appbase-ui-contract.test.mjs",
        "node scripts/notary-app-sdk-integration-contract.test.mjs",
        "node ./scripts/docs-verify.mjs",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_treats_duplicate_rtc_session_create_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_duplicate_rtc_create_from_different_actor_kind -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_suppresses_duplicate_rtc_state_side_effects -- --exact",
        "cargo test -p local-minimal-node --test media_provider_http_test",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_does_not_accept_legacy_media_upload_lifecycle_requests -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_realtime_limit_above_guardrail_over_http -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_open_stream_is_idempotent_and_conflicting_retry_is_rejected -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_open_stream_with_different_actor_is_conflict -- --exact",
        "cargo test -p streaming-service --test http_smoke_test test_open_stream_rejects_oversized_durability_class_over_http -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_stream_list_rejects_oversized_stream_id_over_http -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_request_scoped_stream_append_rejects_different_actor_over_http -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_treats_duplicate_open_stream_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_duplicate_open_stream_from_different_actor -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_request_stream_list_from_different_actor -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_rejects_oversized_stream_id_on_list_frames_over_http -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_complete_stream_request_is_idempotent_and_conflicting_retry_is_rejected -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_complete_stream_request_with_different_actor_is_not_found -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_stream_complete_rejects_oversized_result_message_id_over_http -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_does_not_refanout_duplicate_stream_complete_retry -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_abort_stream_request_is_idempotent_and_conflicting_retry_is_rejected -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_abort_stream_request_with_different_actor_is_not_found -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_stream_abort_rejects_oversized_reason_over_http -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_does_not_refanout_duplicate_stream_abort_retry -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_checkpoint_stream_request_replays_after_stream_completes -- --exact",
        "cargo test -p streaming-service --test stream_lifecycle_test test_duplicate_checkpoint_stream_request_with_different_actor_is_not_found -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_replays_duplicate_checkpoint_retry_after_complete -- --exact",
        "cargo test -p audit-service --test http_smoke_test test_duplicate_record_anchor_request_is_idempotent_and_conflicting_retry_is_rejected -- --exact",
        "cargo test -p audit-service --test http_smoke_test test_duplicate_record_anchor_request_replays_after_session_rotation -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_treats_duplicate_audit_anchor_as_idempotent -- --exact",
        "cargo test -p local-minimal-node --test http_e2e_test test_local_minimal_profile_replays_duplicate_audit_anchor_after_session_rotation -- --exact",
        "cargo clippy -p notification-service -p automation-service -p local-minimal-node --tests -- -D warnings",
        "cargo clippy --no-deps -p conversation-runtime -p local-minimal-node --tests -- -D warnings",
        "cargo clippy --no-deps -p media-service -p streaming-service -p audit-service -p local-minimal-node --tests -- -D warnings",
    ] {
        assert!(
            workflow.contains(required_command),
            "commercial gate workflow must include `{required_command}`"
        );
    }

    assert!(
        !workflow.contains("apps/sdkwork-im-admin")
            && !workflow.contains("apps/sdkwork-im-portal")
            && !workflow.contains("tests/admin-architecture.test.mjs")
            && !workflow.contains("tests/portal-build-smoke.test.mjs")
            && !workflow.contains("tests/portal-real-auth.test.mjs"),
        "commercial gate workflow must not reference retired admin or portal app paths"
    );
    assert!(
        !workflow.contains(
            "test_duplicate_complete_upload_retry_uses_existing_asset_without_reinvoking_provider"
        ) && !workflow.contains("test_get_media_download_url_rejects_zero_ttl_over_http")
            && !workflow.contains(
                "test_local_minimal_profile_treats_duplicate_media_upload_requests_as_idempotent"
            )
            && !workflow.contains("sdkwork-rtc-signaling-service"),
        "commercial gate workflow must not preserve removed IM-owned media upload/download lifecycle tests"
    );
    assert!(
        workflow.contains("- 'adapters/**'"),
        "commercial gate workflow must trigger when adapter crates change"
    );
    assert!(
        workflow.contains("postgres:"),
        "commercial gate workflow must provision a PostgreSQL service for live realtime storage integration"
    );
    assert!(
        workflow.contains("SDKWORK_IM_POSTGRES_TEST_DATABASE_URL:"),
        "commercial gate workflow must set SDKWORK_IM_POSTGRES_TEST_DATABASE_URL so the live PostgreSQL realtime test cannot silently skip in CI"
    );
    assert!(
        workflow
            .matches("SDKWORK_IM_POSTGRES_TEST_DATABASE_URL:")
            .count()
            >= 3,
        "commercial gate workflow must wire the PostgreSQL database URL into adapter-level, session-gateway runtime, and websocket live tests"
    );
    assert!(
        workflow.contains("postgres://sdkwork_im_test:sdkwork_im_test@localhost:5432/sdkwork_im_test"),
        "commercial gate workflow must wire the live PostgreSQL realtime test to the provisioned test database"
    );
}

#[test]
fn test_as_built_alignment_no_longer_claims_local_minimal_lacks_direct_chat_binding_route() {
    let as_built_path =
        repo_root().join("docs/架构/152CJ-current-architecture-as-built-alignment-2026-04-09.md");
    let as_built = fs::read_to_string(&as_built_path).unwrap_or_else(|_| {
        panic!(
            "as-built alignment doc should exist at {}",
            as_built_path.display()
        )
    });

    assert!(
        !as_built.contains(
            "local-minimal-node` still does not expose a dedicated `/im/v3/api/chat/conversations/direct_chats/bindings` route"
        ),
        "as-built alignment doc must not preserve the stale direct-chat binding route gap after the route exists in code"
    );
}
