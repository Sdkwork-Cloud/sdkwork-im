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
        "fn bind_client_route_key(",
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
        "async fn resume_device_session(",
        "async fn get_presence_me(",
        "async fn heartbeat_presence(",
        "async fn disconnect_device_session(",
        "async fn register_client_route(",
        "async fn sync_realtime_subscriptions(",
        "async fn list_realtime_events(",
        "async fn ack_realtime_events(",
        "async fn realtime_websocket(",
        "async fn get_client_route_sync_feed(",
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
        "local-minimal-node message paths should construct message mutation commands from AppContext in one place"
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
            "local-minimal-node non-message paths should consume runtime principal-context entrypoint in one place: {required_symbol}"
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
            "local-minimal-node non-message paths should not keep non-message authority capture outside runtime principal-context entrypoint: {forbidden_symbol}"
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
            "local-minimal-node non-message paths should consume runtime principal-context entrypoint: {required_symbol}"
        );
    }

    assert!(
        !combined.contains("with_actor_kind(")
            && !combined.contains("with_creator_kind(")
            && !combined.contains("with_requester_kind(")
            && !combined.contains("with_source_kind("),
        "local-minimal-node non-message write paths should not use the old *with_*kind entrypoints once conversation-runtime owns that principal-context boundary"
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
        ".list_members_window_from_auth_context(",
        ".get_agent_handoff_state_from_auth_context(",
        ".require_active_member_from_auth_context(",
    ] {
        assert!(
            combined.contains(required_symbol),
            "local-minimal-node read query paths should consume runtime principal-context entrypoint: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        ".list_members(auth.tenant_id.as_str(), conversation_id.as_str())",
        ".get_agent_handoff_state(\n        auth.tenant_id.as_str(),",
        ".require_active_member(\n        auth.tenant_id.as_str(),",
    ] {
        assert!(
            !combined.contains(forbidden_symbol),
            "local-minimal-node read query paths should not keep raw auth field capture outside conversation-runtime principal-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_access_paths_use_runtime_write_access_auth_context_entrypoint() {
    let access_source = include_str!("../src/node/access.rs");

    assert!(
        access_source.contains(".ensure_conversation_bound_write_allowed_from_auth_context("),
        "services/local-minimal-node/src/node/access.rs should consume conversation-runtime principal-context write-access guard"
    );

    assert!(
        !access_source.contains(".ensure_conversation_bound_write_allowed_with_actor_kind("),
        "services/local-minimal-node/src/node/access.rs should not keep raw actor_kind threading for conversation-bound write access"
    );
}

#[test]
fn test_local_minimal_node_client_route_registration_owner_moves_out_of_access_impl() {
    let access_source = include_str!("../src/node/access.rs");
    let node_source = include_str!("../src/node.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/client_route_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/client_route_registration.rs should exist");

    for forbidden_symbol in [
        "fn ensure_route_session_current(",
        "pub(super) fn bind_registered_device(",
        "pub(super) fn bind_client_route_key(",
    ] {
        assert!(
            !access_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/access.rs should not keep legacy route registration owner symbol: {forbidden_symbol}"
        );
    }

    for required_symbol in [
        "mod client_route_registration;",
        "client_route_registration: LocalNodeClientRouteRegistration,",
        "self.client_route_registration",
    ] {
        assert!(
            node_source.contains(required_symbol),
            "services/local-minimal-node/src/node.rs should delegate client route registration ownership through LocalNodeClientRouteRegistration: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) struct LocalNodeClientRouteRegistration",
        "pub(crate) fn new(",
        "pub(crate) fn bind_client_route_key(",
        "pub(crate) fn ensure_client_route_key(",
        "fn ensure_route_session_current(",
        "self.presence_runtime",
        "self.realtime_runtime",
        "self.projection_service",
        ".ensure_client_route_registration_allowed_from_auth_context(",
        ".register_client_route_from_auth_context(",
        "self.realtime_cluster.bind_client_route_for_principal_kind(",
        "platform::refresh_node_operational_view(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/local-minimal-node/src/node/client_route_registration.rs should host client route registration owner implementation: {required_symbol}"
        );
    }

    assert!(
        !owner_source.contains("projection_service.register_client_route("),
        "services/local-minimal-node/src/node/client_route_registration.rs should not keep projection-service legacy route registration once actor-kind-aware route registration owns that seam"
    );
}

#[test]
fn test_local_minimal_node_manifest_avoids_rand_0_8_direct_dependency() {
    let manifest = include_str!("../Cargo.toml");
    assert!(
        !manifest.contains("rand = \"0.8.5\""),
        "services/local-minimal-node/Cargo.toml should not keep a direct rand 0.8.5 dependency once token entropy moves to getrandom"
    );
}

#[test]
fn test_local_minimal_node_route_preflight_owner_moves_out_of_presence_entrypoints() {
    let node_source = include_str!("../src/node.rs");
    let presence_source = include_str!("../src/node/presence_routes.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/client_route_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/client_route_registration.rs should exist");

    assert!(
        !presence_source.contains("fn bind_device("),
        "services/local-minimal-node/src/node/presence_routes.rs should not keep local route preflight glue helper once client_route_registration owns that seam"
    );

    for required_symbol in [
        "fn prepare_active_client_route(",
        "self.client_route_registration",
    ] {
        assert!(
            node_source.contains(required_symbol),
            "services/local-minimal-node/src/node.rs should expose the route preflight owner seam: {required_symbol}"
        );
    }

    {
        let required_symbol = "state.prepare_active_client_route(";
        assert!(
            presence_source.contains(required_symbol),
            "services/local-minimal-node/src/node/presence_routes.rs should consume the shared route preflight owner seam: {required_symbol}"
        );
    }

    for required_symbol in [
        "pub(crate) fn prepare_active_client_route(",
        "self.bind_client_route_key(",
        "fn ensure_route_session_current(",
    ] {
        assert!(
            owner_source.contains(required_symbol),
            "services/local-minimal-node/src/node/client_route_registration.rs should host route preflight owner detail: {required_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_disconnect_lifecycle_is_not_exposed_by_presence_entrypoints() {
    let presence_source = include_str!("../src/node/presence_routes.rs");
    let owner_source = std::fs::read_to_string(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node/client_route_registration.rs"),
    )
    .expect("services/local-minimal-node/src/node/client_route_registration.rs should exist");

    for forbidden_symbol in [
        "state.realtime_cluster.disconnect_fence_matches_client_route_session(",
        "state.realtime_runtime.clear_client_route_subscriptions(",
        "state.realtime_cluster.release_client_route(",
        "state.realtime_cluster.mark_client_route_disconnected(",
        "platform::refresh_node_operational_view(&state)",
    ] {
        assert!(
            !presence_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/presence_routes.rs should not keep raw disconnect lifecycle glue once client_route_registration owns that seam: {forbidden_symbol}"
        );
    }

    assert!(
        !presence_source.contains("state.disconnect_active_client_route("),
        "services/local-minimal-node/src/node/presence_routes.rs must not expose retired HTTP disconnect behavior"
    );

    for forbidden_symbol in [
        "pub(crate) enum DisconnectActiveClientRouteOutcome",
        "pub(crate) fn disconnect_active_client_route(",
        "disconnect_fence_matches_client_route_session_for_principal_kind(",
        "clear_client_route_subscriptions_for_principal_kind(",
        "mark_client_route_disconnected_for_principal_kind(",
    ] {
        assert!(
            !owner_source.contains(forbidden_symbol),
            "services/local-minimal-node/src/node/client_route_registration.rs should not keep retired HTTP disconnect lifecycle owner detail: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_effects_member_fanout_uses_projection_auth_context_entrypoints() {
    let effects_source = include_str!("../src/node/effects.rs");

    assert!(
        effects_source.contains(".active_conversation_principal_recipients_from_auth_context("),
        "services/local-minimal-node/src/node/effects.rs should consume projection-service principal-context active-recipient seam for notification/realtime recipient resolution"
    );

    for forbidden_symbol in [
        ".active_conversation_principal_ids_from_auth_context(",
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
        effects_source.contains(".realtime_fanout_targets_for_recipients_from_auth_context("),
        "services/local-minimal-node/src/node/effects.rs should consume projection-service's typed principal-context realtime fanout target seam for principal-to-device resolution"
    );

    for forbidden_symbol in [
        ".realtime_fanout_targets_from_auth_context(",
        ".registered_client_routes(tenant_id, principal_id.as_str())",
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
fn test_local_minimal_node_effects_do_not_swallow_realtime_delivery_failures() {
    let effects_source = include_str!("../src/node/effects.rs").replace("\r\n", "\n");

    assert!(
        effects_source.contains(".delivery_error_code"),
        "services/local-minimal-node/src/node/effects.rs should inspect cluster realtime delivery failures instead of treating delivered=0 as success"
    );
    assert!(
        effects_source.contains("realtime_delivery_failed"),
        "services/local-minimal-node/src/node/effects.rs should surface realtime fanout delivery failures with a stable API error code"
    );
    assert!(
        !effects_source.contains("let _ = state\n            .realtime_cluster\n            .publish_client_route_event_for_principal_kind("),
        "services/local-minimal-node/src/node/effects.rs must not silently discard realtime publish results"
    );
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
        "services/local-minimal-node/src/node/effects.rs should not keep threading message-posted recipient_ids once notification-service owns recipient resolution through projection principal-context"
    );
}

#[test]
fn test_local_minimal_node_projection_paths_use_projection_service_auth_context_entrypoints() {
    let projection_source = include_str!("../src/node/projection.rs");

    for required_symbol in [
        ".inbox_from_auth_context(",
        ".read_cursor_from_auth_context(",
        ".timeline_window_from_auth_context(",
        ".conversation_summary_from_auth_context(",
    ] {
        assert!(
            projection_source.contains(required_symbol),
            "local-minimal-node projection paths should consume projection-service principal-context entrypoint: {required_symbol}"
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
            "local-minimal-node projection paths should not keep raw projection auth field capture outside projection-service principal-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_read_cursor_write_path_uses_strict_domain_membership_gate() {
    let projection_source = include_str!("../src/node/projection.rs").replace("\r\n", "\n");
    let update_read_cursor = projection_source
        .split("pub(super) async fn update_read_cursor(")
        .nth(1)
        .and_then(|source| source.split("pub(super) async fn get_timeline(").next())
        .expect("projection.rs should keep update_read_cursor before get_timeline");

    assert!(
        update_read_cursor.contains(
            "access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?"
        ),
        "POST /read_cursor mutates conversation-runtime state and must use the strict domain membership gate"
    );
    assert!(
        !update_read_cursor.contains("access::ensure_conversation_read_access("),
        "POST /read_cursor must not use projection snapshot fallback because that fallback is read-only recovery access"
    );
}

#[test]
fn test_local_minimal_node_presence_paths_use_projection_service_auth_context_entrypoints() {
    let presence_source = include_str!("../src/node/presence_routes.rs");

    let required_symbol = ".client_route_sync_state_snapshot_from_auth_context(";
    assert!(
        presence_source.contains(required_symbol),
        "local-minimal-node presence paths should consume projection-service principal-context entrypoint: {required_symbol}"
    );

    for forbidden_symbol in [
        "fn registered_client_routes(state: &AppState, auth: &AppContext) -> Vec<String>",
        ".registered_client_routes_from_auth_context(",
        ".latest_client_route_sync_seq_from_auth_context(",
        ".registered_client_routes(auth.tenant_id.as_str(), auth.actor_id.as_str())",
        ".latest_client_route_sync_seq(\n        auth.tenant_id.as_str(),",
        ".latest_client_route_sync_seq(\n            auth.tenant_id.as_str(),",
        ".client_route_sync_feed(\n            auth.tenant_id.as_str(),",
    ] {
        assert!(
            !presence_source.contains(forbidden_symbol),
            "local-minimal-node presence paths should not keep raw projection auth field capture outside projection-service principal-context entrypoint: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_platform_does_not_keep_appbase_app_side_notification_and_automation_seams()
 {
    let platform_source = include_str!("../src/node/platform.rs");

    for forbidden_symbol in [
        ".request_notification_from_app_context(",
        "access::ensure_notification_request_access(",
        ".request_notification_with_outcome(&auth, request)",
        ".request_automation_result_notification(",
        "let _ = state.notification_runtime.request_notification(",
    ] {
        assert!(
            !platform_source.contains(forbidden_symbol),
            "local-minimal-node platform.rs must not keep appbase-owned app-side notification/automation seam: {forbidden_symbol}"
        );
    }
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
            "services/local-minimal-node/src/node.rs should not keep runtime_dir lifecycle symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_does_not_keep_appbase_owned_local_api_modules() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let node_source = include_str!("../src/node.rs");
    let build_source = include_str!("../src/node/build.rs");
    let runtime_dir_source = include_str!("../src/node/runtime_dir.rs");
    let access_source = include_str!("../src/node/access.rs");
    let platform_source = include_str!("../src/node/platform.rs");
    let principal_profile_source = include_str!("../src/node/principal_profile.rs");

    for removed_module in ["iam.rs", "iot.rs", "portal.rs", "twin.rs"] {
        let module_path = manifest_dir.join("src").join("node").join(removed_module);
        assert!(
            !module_path.exists(),
            "local-minimal-node must not keep appbase-owned local API module {}",
            module_path.display()
        );
    }

    for forbidden_symbol in [
        "mod iam;",
        "mod iot;",
        "mod portal;",
        "mod twin;",
        "DeviceTwinStore",
        "device_twin_store",
        "MediaRuntime",
        "media_runtime",
        "fn iot_access_provider_health(",
        "fn iot_protocol_provider_health(",
        "fn principal_profile_provider_health(",
        "fn get_media_provider_health(",
        "impl From<media_service::MediaError> for ApiError",
    ] {
        assert!(
            !node_source.contains(forbidden_symbol),
            "local-minimal-node node.rs must not keep appbase-owned local API residue: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "FileDeviceTwinStore",
        "MemoryDeviceTwinStore",
        "device-twin-state.json",
        "device_twin_store",
        "MediaRuntime::new()",
        "media_runtime",
    ] {
        assert!(
            !build_source.contains(forbidden_symbol),
            "local-minimal-node build.rs must not assemble appbase-owned local API state: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in ["device-twin-state.json", "validate_device_twin_store_file"] {
        assert!(
            !runtime_dir_source.contains(forbidden_symbol),
            "local-minimal-node runtime_dir.rs must not manage appbase-owned local API state: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "fn ensure_portal_access(",
        "fn ensure_iot_protocol_uplink_access(",
        "fn ensure_iot_protocol_downlink_access(",
        "fn ensure_iot_protocol_uplink_actor_preflight(",
        "fn ensure_iot_protocol_uplink_decoded_device_matches_preflight(",
        "fn ensure_device_twin_read_access(",
        "fn ensure_device_twin_desired_write_access(",
        "fn ensure_device_twin_reported_write_access(",
    ] {
        assert!(
            !access_source.contains(forbidden_symbol),
            "local-minimal-node access.rs must not keep appbase-owned local API guards: {forbidden_symbol}"
        );
    }

    for forbidden_symbol in [
        "async fn request_notification(",
        "async fn list_notifications(",
        "async fn get_notification(",
        "async fn request_automation_execution(",
        "async fn get_automation_execution(",
        "async fn start_agent_response(",
        "async fn append_agent_response_delta(",
        "async fn complete_agent_response(",
        "async fn request_agent_tool_call(",
        "async fn complete_agent_tool_call(",
        "fn record_automation_audit_anchor(",
        "fn automation_audit_aggregate_id(",
        "fn automation_audit_record_id(",
    ] {
        assert!(
            !platform_source.contains(forbidden_symbol),
            "local-minimal-node platform.rs must not keep appbase-owned app-side handlers: {forbidden_symbol}"
        );
    }

    assert!(
        !principal_profile_source.contains("async fn get_principal_profile_provider_health("),
        "local-minimal-node principal_profile.rs must not expose appbase-owned provider-health route handlers"
    );
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
            "services/local-minimal-node/src/node/runtime_dir.rs should not keep runtime_dir preview symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_runtime_dir_preview_keys_use_segment_safe_encoding() {
    let preview_diff_source = include_str!("../src/node/runtime_dir/preview/diff.rs");

    for required_symbol in [
        "fn encode_runtime_dir_preview_key_segments",
        "encode_runtime_dir_preview_key_segments([scope_type, scope_id])",
        "encode_runtime_dir_preview_key_segments([record_key, scope_key])",
        "encode_runtime_dir_preview_key_segments([record_key, \"frame\", frame_seq.to_string().as_str()])",
        "fn qualified_rtc_signal_key(record_key: &str, signal_index: usize) -> String",
        "\"signal\"",
        "signal_index.to_string().as_str()",
    ] {
        assert!(
            preview_diff_source.contains(required_symbol),
            "runtime_dir restore preview keys should use segment-safe encoding: {required_symbol}"
        );
    }

    for forbidden_symbol in [
        "format!(\"{scope_type}:{scope_id}\")",
        "format!(\"{record_key}#{scope_key}\")",
        "format!(\"{record_key}#frame:{frame_seq}\")",
        "format!(\"{record_key}#signal:{signal_index}\")",
    ] {
        assert!(
            !preview_diff_source.contains(forbidden_symbol),
            "runtime_dir restore preview keys must not use ambiguous delimiter concatenation: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_local_minimal_node_source_does_not_keep_legacy_device_rust_symbols() {
    let source_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("src/node");
    let forbidden_symbols = [
        "DeviceSync",
        "device_sync",
        "RegisteredDevice",
        "registered_devices",
        "register_device",
        "DeviceTwin",
        "device-twin-state",
        "device_route",
        "bind_device",
        "release_device",
        "ensure_device_registration",
    ];

    for entry in std::fs::read_dir(&source_dir)
        .expect("services/local-minimal-node/src/node should be readable")
    {
        let entry = entry.expect("local-minimal-node src/node entry should be readable");
        let path = entry.path();
        if path.extension().and_then(|value| value.to_str()) != Some("rs") {
            continue;
        }
        let source =
            std::fs::read_to_string(&path).expect("local-minimal-node Rust source should read");
        for forbidden_symbol in forbidden_symbols {
            assert!(
                !source.contains(forbidden_symbol),
                "{} must use sdkwork-aiot or client_route naming instead of legacy device symbol: {forbidden_symbol}",
                path.display()
            );
        }
    }
}
