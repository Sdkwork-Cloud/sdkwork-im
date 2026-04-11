use std::path::PathBuf;

#[test]
fn test_local_minimal_node_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/local-minimal-node/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_local_minimal_node_effects_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "fn post_message_with_side_effects(",
        "fn publish_system_channel_message_with_side_effects(",
        "fn finalize_post_message_with_side_effects(",
        "fn fanout_message_notifications(",
        "fn publish_realtime_conversation_message_event(",
        "fn publish_realtime_membership_event(",
        "fn publish_realtime_agent_handoff_status_changed_event(",
        "fn publish_realtime_stream_frame_event(",
        "fn publish_realtime_stream_lifecycle_event(",
        "fn stream_target_principal_ids(",
        "fn conversation_member_principal_ids(",
        "fn publish_realtime_event_to_principals(",
        "fn handoff_lifecycle_changed_at(",
        "fn emit_rtc_signal_message(",
        "fn emit_rtc_custom_signal_message(",
        "fn record_membership_audit(",
        "fn record_owner_transfer_audit(",
        "fn record_member_role_change_audit(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep effects/realtime symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_access_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "fn ensure_audit_read_access(",
        "fn ensure_audit_write_access(",
        "fn ensure_ops_read_access(",
        "fn ensure_notification_request_access(",
        "fn ensure_registered_device(",
        "fn ensure_route_session_current(",
        "fn bind_registered_device(",
        "fn resolve_requested_device_id(",
        "fn validate_device_scope(",
        "fn ensure_conversation_member(",
        "fn resolve_conversation_actor_auth_context(",
        "fn ensure_conversation_bound_write_access(",
        "fn ensure_rtc_create_access(",
        "fn ensure_rtc_session_conversation_write_access(",
        "fn ensure_stream_open_access(",
        "fn ensure_stream_session_conversation_member(",
        "fn ensure_stream_session_write_access(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep access/auth symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_platform_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn request_notification(",
        "async fn list_notifications(",
        "async fn get_notification(",
        "async fn request_automation_execution(",
        "async fn get_automation_execution(",
        "async fn record_audit_anchor(",
        "async fn list_audit_records(",
        "async fn export_audit_bundle(",
        "async fn get_ops_health(",
        "async fn get_ops_cluster(",
        "async fn get_ops_lag(",
        "async fn get_ops_replay_status(",
        "async fn get_ops_runtime_dir(",
        "async fn get_ops_diagnostics(",
        "fn refresh_node_operational_view(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep platform symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_session_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn resume_session(",
        "async fn get_presence_me(",
        "async fn heartbeat_presence(",
        "async fn disconnect_session(",
        "async fn register_device(",
        "async fn sync_realtime_subscriptions(",
        "async fn list_realtime_events(",
        "async fn ack_realtime_events(",
        "async fn realtime_websocket(",
        "async fn get_device_sync_feed(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep session/realtime/device symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_stream_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn open_stream(",
        "async fn checkpoint_stream(",
        "async fn append_stream_frame(",
        "async fn list_stream_frames(",
        "async fn complete_stream(",
        "async fn abort_stream(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep stream symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_rtc_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn create_rtc_session(",
        "async fn invite_rtc_session(",
        "async fn accept_rtc_session(",
        "async fn reject_rtc_session(",
        "async fn end_rtc_session(",
        "async fn post_rtc_signal(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep rtc symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_media_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn create_media_upload(",
        "async fn complete_media_upload(",
        "async fn get_media(",
        "async fn attach_media(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep media symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_projection_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn get_inbox(",
        "async fn get_read_cursor(",
        "async fn update_read_cursor(",
        "async fn get_timeline(",
        "async fn get_conversation_summary(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep projection symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_membership_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn list_members(",
        "async fn add_member(",
        "async fn remove_member(",
        "async fn transfer_conversation_owner(",
        "async fn change_conversation_member_role(",
        "async fn leave_conversation(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep membership/governance symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_message_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn post_message(",
        "async fn publish_system_channel_message(",
        "async fn edit_message(",
        "async fn recall_message(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep message mutation symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_message_paths_use_auth_context_command_constructors() {
    let node_source = include_str!("../src/node.rs");
    let effects_source = include_str!("../src/node/effects.rs");
    let message_source = include_str!("../src/node/message.rs");

    assert!(
        !node_source.contains("fn build_sender("),
        "services/local-minimal-node/src/node.rs should not keep a local sender snapshot builder once message mutation commands own the shared from_auth_context constructor boundary"
    );

    let combined = format!("{effects_source}\n{message_source}");
    assert!(
        combined.contains("from_auth_context("),
        "local-minimal-node message paths should construct message mutation commands from AuthContext in one place"
    );

    for forbidden_symbol in [
        "sender: build_sender(",
        "publisher: build_sender(",
        "editor: build_sender(",
        "recalled_by: build_sender(",
    ] {
        assert!(
            !combined.contains(forbidden_symbol),
            "local-minimal-node message paths should not hand-build message sender snapshots: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_non_message_paths_use_auth_context_command_constructors() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        include_str!("../src/node/conversation.rs"),
        include_str!("../src/node/membership.rs"),
        include_str!("../src/node/projection.rs"),
        include_str!("../src/node/handoff.rs"),
    );

    for required_symbol in [
        ".create_conversation_from_auth_context_with_creator_attributes(",
        ".create_agent_dialog_from_auth_context_with_requester_attributes(",
        ".create_agent_handoff_from_auth_context_with_target_attributes(",
        ".create_system_channel_from_auth_context_with_subscriber_attributes(",
        ".accept_agent_handoff_from_auth_context(",
        ".resolve_agent_handoff_from_auth_context(",
        ".close_agent_handoff_from_auth_context(",
        ".add_member_from_auth_context(",
        ".remove_member_from_auth_context(",
        ".transfer_conversation_owner_from_auth_context(",
        ".change_conversation_member_role_from_auth_context(",
        ".leave_conversation_from_auth_context(",
        ".update_read_cursor_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "local-minimal-node non-message paths should consume runtime auth-context entrypoint in one place: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "CreateConversationCommand::from_auth_context(",
        "CreateAgentDialogCommand::from_auth_context(",
        "CreateAgentHandoffCommand::from_auth_context(",
        "CreateSystemChannelCommand::from_auth_context(",
        "AcceptAgentHandoffCommand::from_auth_context(",
        "ResolveAgentHandoffCommand::from_auth_context(",
        "CloseAgentHandoffCommand::from_auth_context(",
        "AddConversationMemberCommand::from_auth_context(",
        "RemoveConversationMemberCommand::from_auth_context(",
        "TransferConversationOwnerCommand::from_auth_context(",
        "ChangeConversationMemberRoleCommand::from_auth_context(",
        "LeaveConversationCommand::from_auth_context(",
        "UpdateReadCursorCommand::from_auth_context(",
        "create_conversation_with_creator_kind(",
        "create_agent_dialog_with_requester_kind(",
        "create_agent_handoff_with_source_kind(",
        "create_system_channel_with_requester_kind(",
        "accept_agent_handoff_with_actor_kind(",
        "resolve_agent_handoff_with_actor_kind(",
        "close_agent_handoff_with_actor_kind(",
        "add_member_with_actor_kind(",
        "remove_member_with_actor_kind(",
        "transfer_conversation_owner_with_actor_kind(",
        "change_conversation_member_role_with_actor_kind(",
        "leave_conversation_with_actor_kind(",
        "update_read_cursor_with_actor_kind(",
    ] {
        assert!(
            !combined.contains(forbidden_symbol),
            "local-minimal-node non-message paths should not keep non-message authority capture outside runtime auth-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_non_message_paths_use_runtime_auth_context_entrypoints() {
    let combined = format!(
        "{}\n{}\n{}\n{}",
        include_str!("../src/node/conversation.rs"),
        include_str!("../src/node/membership.rs"),
        include_str!("../src/node/projection.rs"),
        include_str!("../src/node/handoff.rs"),
    );

    for required_symbol in [
        ".create_conversation_from_auth_context_with_creator_attributes(",
        ".create_agent_dialog_from_auth_context_with_requester_attributes(",
        ".create_agent_handoff_from_auth_context_with_target_attributes(",
        ".create_system_channel_from_auth_context_with_subscriber_attributes(",
        ".accept_agent_handoff_from_auth_context(",
        ".resolve_agent_handoff_from_auth_context(",
        ".close_agent_handoff_from_auth_context(",
        ".add_member_from_auth_context(",
        ".remove_member_from_auth_context(",
        ".transfer_conversation_owner_from_auth_context(",
        ".change_conversation_member_role_from_auth_context(",
        ".leave_conversation_from_auth_context(",
        ".update_read_cursor_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "local-minimal-node non-message paths should consume runtime auth-context entrypoint: {required_symbol}"
        );
    }

    assert!(
        !combined.contains("with_actor_kind(")
            && !combined.contains("with_creator_kind(")
            && !combined.contains("with_requester_kind(")
            && !combined.contains("with_source_kind("),
        "local-minimal-node non-message write paths should not use the old *with_*kind entrypoints once conversation-runtime owns that auth-context boundary"
    );
}

#[test]
fn test_local_minimal_node_read_query_paths_use_runtime_auth_context_entrypoints() {
    let combined = format!(
        "{}\n{}\n{}",
        include_str!("../src/node/membership.rs"),
        include_str!("../src/node/handoff.rs"),
        include_str!("../src/node/access.rs"),
    );

    for required_symbol in [
        ".list_members_from_auth_context(",
        ".get_agent_handoff_state_from_auth_context(",
        ".require_active_member_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "local-minimal-node read query paths should consume runtime auth-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        ".list_members(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".get_agent_handoff_state(\n        auth.tenant_id.as_str(),",
        ".require_active_member(\n        auth.tenant_id.as_str(),",
    ] {
        assert!(
            !combined.contains(forbidden_symbol),
            "local-minimal-node read query paths should not keep raw auth field capture outside conversation-runtime auth-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_access_paths_use_runtime_write_access_auth_context_entrypoint() {
    let access_source = include_str!("../src/node/access.rs");

    assert!(
        access_source.contains(".ensure_conversation_bound_write_allowed_from_auth_context("),
        "services/local-minimal-node/src/node/access.rs should consume conversation-runtime auth-context write-access guard"
    );

    assert!(
        !access_source.contains(".ensure_conversation_bound_write_allowed_with_actor_kind("),
        "services/local-minimal-node/src/node/access.rs should not keep raw actor_kind threading for conversation-bound write access"
    );
}

#[test]
fn test_local_minimal_node_device_registration_owner_moves_out_of_access_impl() {
    let access_source = include_str!("../src/node/access.rs");
    let node_source = include_str!("../src/node.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/device_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/device_registration.rs should exist");

    for forbidden_symbol in [
        "fn ensure_route_session_current(",
        "pub(super) fn bind_registered_device(",
    ] {
        assert!(
            !access_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/access.rs should not keep device registration owner symbol: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod device_registration;",
        "device_registration: LocalNodeDeviceRegistration,",
        "self.device_registration.bind_registered_device(",
        "self.device_registration.ensure_registered_device(",
    ] {
        assert!(
            node_source.contains(required_symbol),
            "services/local-minimal-node/src/node.rs should delegate device registration ownership through LocalNodeDeviceRegistration: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct LocalNodeDeviceRegistration",
        "pub(crate) fn new(",
        "pub(crate) fn bind_registered_device(",
        "pub(crate) fn ensure_registered_device(",
        "fn ensure_route_session_current(",
        "self.session_presence_runtime",
        "self.realtime_runtime",
        "self.projection_service",
        "self.realtime_cluster.bind_device_route(",
        "platform::refresh_node_operational_view(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/local-minimal-node/src/node/device_registration.rs should host device registration owner implementation: {required_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_route_preflight_owner_moves_out_of_session_entrypoints() {
    let node_source = include_str!("../src/node.rs");
    let session_source = include_str!("../src/node/session.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/device_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/device_registration.rs should exist");

    assert!(
        !session_source.contains("fn bind_device("),
        "services/local-minimal-node/src/node/session.rs should not keep local route preflight glue helper once device_registration owns that seam"
    );

    for required_symbol in [
        "fn prepare_active_device_route(",
        "self.device_registration.prepare_active_device_route(",
    ] {
        assert!(
            node_source.contains(required_symbol),
            "services/local-minimal-node/src/node.rs should expose the route preflight owner seam: {required_symbol}"
        );
    }

    {
        let required_symbol = "state.prepare_active_device_route(";
        assert!(
            session_source.contains(required_symbol),
            "services/local-minimal-node/src/node/session.rs should consume the shared route preflight owner seam: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) fn prepare_active_device_route(",
        "self.bind_registered_device(",
        "fn ensure_route_session_current(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/local-minimal-node/src/node/device_registration.rs should host route preflight owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_disconnect_lifecycle_owner_moves_out_of_session_entrypoints() {
    let node_source = include_str!("../src/node.rs");
    let session_source = include_str!("../src/node/session.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/device_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/device_registration.rs should exist");

    for forbidden_symbol in [
        "state.realtime_cluster.disconnect_fence_matches_session(",
        "state.realtime_runtime.clear_device_subscriptions(",
        "state.realtime_cluster.release_device_route(",
        "state.realtime_cluster.mark_device_disconnected(",
        "platform::refresh_node_operational_view(&state)",
    ] {
        assert!(
            !session_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/session.rs should not keep raw disconnect lifecycle glue once device_registration owns that seam: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "fn disconnect_active_device_route(",
        "self.device_registration.disconnect_active_device_route(",
    ] {
        assert!(
            node_source.contains(required_symbol),
            "services/local-minimal-node/src/node.rs should expose the disconnect lifecycle owner seam: {required_symbol}"
        );
    }

    assert!(
        session_source.contains("state.disconnect_active_device_route("),
        "services/local-minimal-node/src/node/session.rs should consume the shared disconnect lifecycle owner seam"
    );

    for required_symbol in [
        "pub(crate) enum DisconnectActiveDeviceRouteOutcome",
        "pub(crate) fn disconnect_active_device_route(",
        "disconnect_fence_matches_session(",
        "clear_device_subscriptions(",
        "release_device_route(",
        "mark_device_disconnected(",
        "platform::refresh_node_operational_view(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/local-minimal-node/src/node/device_registration.rs should host disconnect lifecycle owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_effects_member_fanout_uses_projection_auth_context_entrypoints() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        effects_source.contains(".active_conversation_principal_ids_from_auth_context("),
        "services/local-minimal-node/src/node/effects.rs should consume projection-service auth-context active-principal seam for notification/realtime recipient resolution"
    );

    for forbidden_symbol in [
        ".list_members_from_auth_context(",
        ".list_members(auth.tenant_id.as_str(), conversation_id)",
        ".list_members(tenant_id, conversation_id)",
    ] {
        assert!(
            !effects_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/effects.rs should not keep runtime-owned member roster reads in effects path once projection-service owns active principal mapping: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_effects_use_projection_owned_realtime_fanout_target_seam() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        effects_source.contains(".realtime_fanout_targets_from_auth_context("),
        "services/local-minimal-node/src/node/effects.rs should consume projection-service's auth-context realtime fanout target seam for principal-to-device resolution"
    );

    for forbidden_symbol in [
        ".registered_devices(tenant_id, principal_id.as_str())",
        ".map(|item| item.device_id)",
        ".realtime_fanout_targets_for_principals(tenant_id, principal_ids)",
    ] {
        assert!(
            !effects_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/effects.rs should not rebuild or raw-thread realtime fanout targets once projection-service owns that seam: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_effects_use_notification_service_fanout_owner_seam() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        effects_source.contains(".request_notification_fanout(")
            || effects_source.contains(".request_message_posted_notifications("),
        "services/local-minimal-node/src/node/effects.rs should consume notification-service's runtime-owned notification side-effect seam"
    );

    assert!(
        !effects_source.contains("state.notification_runtime.request_notification("),
        "services/local-minimal-node/src/node/effects.rs should not keep local per-recipient notification request orchestration once notification-service owns the fanout seam"
    );
}

#[test]
fn test_local_minimal_node_effects_use_notification_service_message_posted_owner_seam() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        effects_source.contains(".request_message_posted_notifications("),
        "services/local-minimal-node/src/node/effects.rs should delegate message-posted notification request assembly to notification-service's owner seam"
    );

    for forbidden_symbol in [
        "notification_service::RequestNotificationFanout {",
        "notification_id_seed: message_id.into()",
        "source_event_type: \"message.posted\".into()",
    ] {
        assert!(
            !effects_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/effects.rs should not keep inline message-posted notification assembly once notification-service owns that seam: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_effects_do_not_thread_message_posted_recipient_ids() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        !effects_source.contains("recipient_ids:"),
        "services/local-minimal-node/src/node/effects.rs should not keep threading message-posted recipient_ids once notification-service owns recipient resolution through projection auth-context"
    );
}

#[test]
fn test_local_minimal_node_projection_paths_use_projection_service_auth_context_entrypoints() {
    let projection_source = include_str!("../src/node/projection.rs");

    for required_symbol in [
        ".inbox_from_auth_context(",
        ".read_cursor_from_auth_context(",
        ".timeline_from_auth_context(",
        ".conversation_summary_from_auth_context(",
    ] {
        assert!(
            projection_source.contains(required_symbol),
            "local-minimal-node projection paths should consume projection-service auth-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        ".inbox(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        ".read_cursor(\n            auth.tenant_id.as_str(),",
        ".timeline(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".conversation_summary(auth.tenant_id.as_str(), conversation_id.as_str())",
    ] {
        assert!(
            !projection_source.contains(forbidden_symbol),
            "local-minimal-node projection paths should not keep raw projection auth field capture outside projection-service auth-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_session_projection_paths_use_projection_service_auth_context_entrypoints()
 {
    let session_source = include_str!("../src/node/session.rs");

    for required_symbol in [
        ".device_sync_session_state_from_auth_context(",
        ".device_sync_feed_from_auth_context(",
    ] {
        assert!(
            session_source.contains(required_symbol),
            "local-minimal-node session paths should consume projection-service auth-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "fn registered_devices(state: &AppState, auth: &AuthContext) -> Vec<String>",
        ".registered_devices_from_auth_context(",
        ".latest_device_sync_seq_from_auth_context(",
        ".registered_devices(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        ".latest_device_sync_seq(\n        auth.tenant_id.as_str(),",
        ".latest_device_sync_seq(\n            auth.tenant_id.as_str(),",
        ".device_sync_feed(\n            auth.tenant_id.as_str(),",
    ] {
        assert!(
            !session_source.contains(forbidden_symbol),
            "local-minimal-node session paths should not keep raw projection auth field capture outside projection-service auth-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_notification_request_path_uses_notification_runtime_public_access_owner()
{
    let platform_source = include_str!("../src/node/platform.rs");

    assert!(
        platform_source.contains(".request_notification_from_public_api("),
        "services/local-minimal-node/src/node/platform.rs should consume notification-service's runtime-owned public notification request seam"
    );

    for forbidden_symbol in [
        "access::ensure_notification_request_access(",
        ".request_notification_with_outcome(&auth, request)",
    ] {
        assert!(
            !platform_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/platform.rs should not keep local notification public-access enforcement once notification-service owns that boundary: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_automation_path_uses_notification_runtime_automation_result_owner() {
    let platform_source = include_str!("../src/node/platform.rs");

    assert!(
        platform_source.contains(".request_automation_result_notification("),
        "services/local-minimal-node/src/node/platform.rs should consume notification-service's runtime-owned automation result notification seam"
    );

    assert!(
        !platform_source.contains("let _ = state.notification_runtime.request_notification("),
        "services/local-minimal-node/src/node/platform.rs should not hand-assemble automation result notifications once notification-service owns that seam"
    );
}

#[test]
fn test_local_minimal_node_conversation_create_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn create_conversation(",
        "async fn create_agent_dialog(",
        "async fn create_agent_handoff(",
        "async fn create_system_channel(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep conversation-create symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_handoff_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "async fn get_agent_handoff_state(",
        "async fn accept_agent_handoff(",
        "async fn resolve_agent_handoff(",
        "async fn close_agent_handoff(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep handoff lifecycle symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_build_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "fn configured_runtime_dir(",
        "fn build_default_app_with_bind_addr(",
        "fn build_public_app_with_bind_addr(",
        "fn build_default_app_with_bind_addr_and_runtime_dir(",
        "fn build_public_app_with_bind_addr_and_runtime_dir(",
        "fn build_local_minimal_realtime_cluster(",
        "fn build_local_minimal_realtime_runtime(",
        "fn build_local_minimal_presence_runtime(",
        "fn build_local_minimal_streaming_runtime(",
        "fn build_local_minimal_rtc_runtime(",
        "fn build_local_minimal_notification_runtime(",
        "fn build_local_minimal_automation_runtime(",
        "fn build_app_with_dependencies_and_runtime_and_journal(",
        "fn replay_projection_journal(",
        "fn build_app(state: AppState) -> Router",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep build/runtime assembly symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_build_surface_assembles_realtime_plane_via_session_gateway_bundle() {
    let build_source = include_str!("../src/node/build.rs");

    for required_symbol in [
        "RealtimePlaneAssembly",
        "fn build_local_minimal_realtime_plane(",
    ] {
        assert!(
            build_source.contains(required_symbol),
            "services/local-minimal-node/src/node/build.rs should assemble realtime plane via session-gateway bundle: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "fn build_local_minimal_realtime_cluster(",
        "fn build_local_minimal_realtime_runtime(",
        "fn build_local_minimal_presence_runtime(",
    ] {
        assert!(
            !build_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/build.rs should not keep split realtime assembly helper: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_runtime_dir_surface_moves_out_of_node_impl() {
    let node_source = include_str!("../src/node.rs");

    for forbidden_symbol in [
        "struct RuntimeDirRestorePreviewFingerprintMaterial",
        "struct RuntimeBackupSnapshotSummary",
        "fn contract_error_message(",
        "fn runtime_state_parse_failure(",
        "fn apply_projection_journal_envelopes(",
        "fn validate_projection_journal_file(",
        "fn validate_runtime_state_file(",
        "fn runtime_dir_operation_backup_dir(",
        "fn runtime_dir_repair_backup_dir(",
        "fn runtime_dir_restore_backup_dir(",
        "fn runtime_backup_operation(",
        "fn runtime_backup_snapshot_quality(",
        "fn runtime_backup_report_preview(",
        "fn stable_runtime_dir_restore_preview_fingerprint(",
        "fn summarize_runtime_restore_preview_change(",
        "fn summarize_disconnect_fence_restore_preview_change(",
        "fn summarize_realtime_checkpoint_restore_preview_change(",
        "fn summarize_realtime_subscription_restore_preview_change(",
        "fn summarize_stream_state_restore_preview_change(",
        "fn summarize_rtc_state_restore_preview_change(",
        "fn describe_runtime_backup_snapshot(",
        "fn validate_runtime_backup_source(",
        "fn snapshot_runtime_state_files(",
        "fn write_runtime_dir_repair_report(",
        "fn write_runtime_dir_restore_report(",
        "pub fn repair_runtime_dir(",
        "pub fn restore_runtime_dir(",
        "pub fn restore_runtime_dir_with_expected_preview_fingerprint(",
        "pub fn preview_restore_runtime_dir(",
        "pub fn list_runtime_backups(",
        "pub fn inspect_runtime_dir(",
        "pub fn format_runtime_dir_repair(",
        "pub fn format_runtime_dir_restore(",
        "pub fn format_runtime_backup_catalog(",
        "pub fn format_runtime_dir_restore_preview(",
        "pub fn format_runtime_dir_inspection(",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node.rs should not keep runtime-dir lifecycle symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_runtime_dir_preview_surface_moves_out_of_runtime_dir_impl() {
    let runtime_dir_source = include_str!("../src/node/runtime_dir.rs");

    for forbidden_symbol in [
        "pub struct RuntimeDirRestorePreviewChangeSummaryView",
        "pub struct RuntimeDirRestorePreviewDomainSummaryView",
        "pub struct RuntimeDirRestorePreviewActionView",
        "pub struct RuntimeDirRestorePreviewView",
        "struct RuntimeDirRestorePreviewFingerprintMaterial",
        "fn stable_runtime_dir_restore_preview_fingerprint(",
        "fn summarize_runtime_restore_preview_change(",
        "fn summarize_disconnect_fence_restore_preview_change(",
        "fn summarize_realtime_checkpoint_restore_preview_change(",
        "fn summarize_realtime_subscription_restore_preview_change(",
        "fn summarize_stream_state_restore_preview_change(",
        "fn summarize_rtc_state_restore_preview_change(",
        "pub fn preview_restore_runtime_dir(",
        "pub fn format_runtime_dir_restore_preview(",
    ] {
        assert!(
            !runtime_dir_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/runtime_dir.rs should not keep runtime-dir preview symbol: {forbidden_symbol}"
        );
    }
}
