#[test]
fn test_projection_service_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/projection-service/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_projection_service_http_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "struct RegisterDeviceRequest {",
        "struct SyncFeedQuery {",
        "struct HealthResponse {",
        "struct TimelineResponse {",
        "struct InboxResponse {",
        "struct DeviceSyncFeedResponse {",
        "pub struct ProjectionApiError {",
        "pub fn build_default_app(",
        "pub fn build_public_app(",
        "pub fn build_public_app_with_service(",
        "pub fn build_app(",
        "async fn require_public_bearer_auth(",
        "async fn healthz(",
        "async fn readyz(",
        "async fn register_device(",
        "async fn get_device_sync_feed(",
        "async fn get_timeline(",
        "async fn get_inbox(",
        "async fn get_conversation_summary(",
        "async fn get_read_cursor(",
        "fn resolve_requested_device_id(",
        "fn validate_device_scope(",
        "fn ensure_conversation_member_access(",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/projection-service/src/lib.rs should not keep http surface symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_handoff_projection_surface_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");

    for forbidden_symbol in [
        "fn handoff_view_from_created_payload(",
        "fn handoff_view_from_state_payload(",
        "fn projection_actor_to_view(",
        "fn latest_summary_activity_at(",
        "pub enum ProjectionError {",
    ] {
        assert!(
            !lib_source.contains(forbidden_symbol),
            "services/projection-service/src/lib.rs should not keep projection helper symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_access_module_exposes_auth_context_entrypoints() {
    let access_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/access.rs"))
            .expect("services/projection-service/src/access.rs should exist");

    for required_symbol in [
        "pub struct ProjectionAccessError {",
        "pub struct DeviceSyncSessionState {",
        "pub fn ensure_active_member_from_auth_context(",
        "pub fn active_conversation_principal_ids_from_auth_context(",
        "pub fn message_posted_notification_principal_ids_from_auth_context(",
        "pub fn register_device_from_auth_context(",
        "pub fn registered_devices_from_auth_context(",
        "pub fn device_sync_session_state_from_auth_context(",
        "pub fn realtime_fanout_targets_from_auth_context(",
        "pub fn latest_device_sync_seq_from_auth_context(",
        "pub fn device_sync_feed_from_auth_context(",
        "pub fn inbox_from_auth_context(",
        "pub fn contacts_from_auth_context(",
        "pub fn timeline_from_auth_context(",
        "pub fn conversation_summary_from_auth_context(",
        "pub fn message_interaction_summary_from_auth_context(",
        "pub fn pinned_messages_from_auth_context(",
        "pub fn read_cursor_from_auth_context(",
    ] {
        assert!(
            access_source.contains(required_symbol),
            "projection-service access module should own auth-context authority wrapper: {required_symbol}"
        );
    }
}

#[test]
fn test_projection_service_exposes_realtime_fanout_target_owner_seam() {
    let lib_source = include_str!("../src/lib.rs");

    assert!(
        lib_source.contains("pub fn realtime_fanout_targets_for_principals("),
        "services/projection-service/src/lib.rs should expose a projection-owned realtime fanout target seam for principal-to-device resolution"
    );
}

#[test]
fn test_projection_service_exposes_conversation_device_sync_target_owner_seam() {
    let lib_source = include_str!("../src/lib.rs");

    assert!(
        lib_source.contains("pub fn device_sync_fanout_targets_for_conversation("),
        "services/projection-service/src/lib.rs should expose a projection-owned conversation device-sync target seam for active member plus fallback principal resolution"
    );
    assert!(
        lib_source
            .matches(".device_sync_fanout_targets_for_conversation(")
            .count()
            >= 4,
        "services/projection-service/src/lib.rs should route conversation-scoped device-sync fanout callers through the shared projection owner seam"
    );
    assert!(
        lib_source
            .matches("for device in self.registered_devices(")
            .count()
            <= 1,
        "services/projection-service/src/lib.rs should not duplicate raw principal-to-device device-sync fanout loops across multiple projection handlers"
    );
}

#[test]
fn test_projection_service_http_surface_uses_auth_context_entrypoints() {
    let http_source = include_str!("../src/http.rs");

    for required_symbol in [
        ".register_device_from_auth_context(",
        ".device_sync_feed_from_auth_context(",
        ".inbox_from_auth_context(",
        ".contacts_from_auth_context(",
        ".timeline_from_auth_context(",
        ".conversation_summary_from_auth_context(",
        ".message_interaction_summary_from_auth_context(",
        ".pinned_messages_from_auth_context(",
        ".read_cursor_from_auth_context(",
    ] {
        assert!(
            http_source.contains(required_symbol),
            "projection-service http surface should consume projection auth-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        ".register_device(\n        auth.tenant_id.as_str(),",
        ".device_sync_feed(\n            auth.tenant_id.as_str(),",
        ".inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        ".timeline(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".conversation_summary(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".read_cursor(\n            auth.tenant_id.as_str(),",
        "ensure_conversation_member_access(",
        "resolve_requested_device_id(",
        "validate_device_scope(",
    ] {
        assert!(
            !http_source.contains(forbidden_symbol),
            "projection-service http surface should not keep raw auth authority capture outside access module: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_projection_service_device_sync_entry_assembly_moves_out_of_lib_impl() {
    let lib_source = include_str!("../src/lib.rs");
    let device_sync_source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/device_sync.rs"))
            .expect("services/projection-service/src/device_sync.rs should exist");

    for required_symbol in [
        "pub(crate) struct DeviceSyncEntryDraft {",
        "impl DeviceSyncEntryDraft {",
        "pub(crate) fn build_for_target(",
    ] {
        assert!(
            device_sync_source.contains(required_symbol),
            "projection-service device-sync entry owner module should expose shared assembly symbol: {required_symbol}"
        );
    }

    assert_eq!(
        lib_source.matches("DeviceSyncFeedEntry {").count(),
        0,
        "services/projection-service/src/lib.rs should not keep inline device-sync entry assembly after the shared owner seam is extracted"
    );
    assert!(
        lib_source.matches(".append_device_sync_draft(").count() >= 5,
        "services/projection-service/src/lib.rs should route message, mutation, read-cursor, handoff, and member-governance sync fanout through the shared device-sync draft owner seam"
    );
}
