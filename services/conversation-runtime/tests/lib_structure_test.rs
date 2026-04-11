#[test]
fn test_conversation_runtime_lib_rs_stays_below_step02_redline() {
    let line_count = include_str!("../src/lib.rs").lines().count();

    assert!(
        line_count <= 1000,
        "services/conversation-runtime/src/lib.rs must stay below 1000 lines for Step 02, found {line_count}"
    );
}

#[test]
fn test_conversation_runtime_http_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "pub fn build_default_app() -> Router",
        "pub fn build_public_app() -> Router",
        "fn build_app(state: AppState) -> Router",
        "async fn create_conversation(",
        "async fn recall_message(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep HTTP surface symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_policy_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "enum MessagePostPolicy {",
        "fn ensure_generic_creatable_conversation_type(",
        "fn ensure_agent_dialog_requester_kind(",
        "fn ensure_agent_handoff_source_kind(",
        "fn ensure_agent_handoff_target_kind(",
        "fn ensure_system_channel_requester_kind(",
        "fn ensure_agent_handoff_conversation(",
        "fn ensure_actor_kind_matches_member(",
        "fn ensure_target_handoff_actor(",
        "fn ensure_source_or_target_handoff_actor(",
        "fn is_closed_agent_handoff(",
        "fn ensure_member_add_actor_allowed(",
        "fn ensure_member_add_request_allowed(",
        "fn ensure_member_remove_allowed(",
        "fn ensure_current_active_member_target(",
        "fn ensure_member_leave_allowed(",
        "fn ensure_owner_transfer_allowed(",
        "fn ensure_member_role_change_allowed(",
        "fn ensure_message_post_allowed(",
        "fn ensure_system_channel_publish_command_allowed(",
        "fn ensure_message_edit_allowed(",
        "fn ensure_message_recall_allowed(",
        "fn ensure_system_channel_publisher_write_allowed(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep conversation policy symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_recovery_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "struct RecoveredConversationCreatedPayload {",
        "struct RecoveredConversationHandoffPayload {",
        "fn apply_recovered_conversation_created(",
        "fn apply_recovered_member_joined(",
        "fn apply_recovered_member_deactivated(",
        "fn apply_recovered_read_cursor(",
        "fn apply_recovered_owner_transfer(",
        "fn apply_recovered_member_role_changed(",
        "fn apply_recovered_handoff_status_changed(",
        "fn apply_recovered_message_posted(",
        "fn apply_recovered_message_edited(",
        "fn apply_recovered_message_recalled(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep recovery/replay symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_helper_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "fn conversation_scope_key(",
        "fn build_conversation_member(",
        "fn build_conversation_member_with_attributes(",
        "fn upsert_member(",
        "fn next_member_episode(",
        "fn resolve_active_member_id(",
        "fn resolve_active_member(",
        "fn upsert_read_cursor(",
        "fn build_member_envelope(",
        "fn build_default_read_cursor(",
        "fn build_read_cursor_envelope(",
        "fn build_owner_transfer_envelope(",
        "fn build_member_role_changed_envelope(",
        "fn build_agent_handoff_status_changed_envelope(",
        "fn build_message_edited_envelope(",
        "fn build_message_recalled_envelope(",
        "fn message_scope_key(",
        "fn member_id(",
        "fn member_episode_id(",
        "fn conversation_timestamp(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep helper/envelope symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_creation_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "pub fn create_conversation(",
        "pub fn create_conversation_with_creator_kind(",
        "pub fn create_agent_dialog(",
        "pub fn create_agent_dialog_with_requester_kind(",
        "pub fn create_system_channel(",
        "pub fn create_system_channel_with_requester_kind(",
        "pub fn create_agent_handoff(",
        "pub fn create_agent_handoff_with_source_kind(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep creation surface symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_handoff_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "pub fn get_agent_handoff_state(",
        "pub fn accept_agent_handoff_with_actor_kind(",
        "pub fn resolve_agent_handoff_with_actor_kind(",
        "pub fn close_agent_handoff_with_actor_kind(",
        "fn transition_agent_handoff_status(",
        "fn finish_agent_handoff_transition(",
        "fn build_handoff_actor_view(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep handoff surface symbol: {forbidden_symbol}"
        );
    }
}

#[test]
fn test_conversation_runtime_membership_surface_moves_out_of_runtime_impl() {
    let runtime_source = include_str!("../src/runtime.rs");

    for forbidden_symbol in [
        "pub fn add_member(",
        "pub fn add_member_with_actor_kind(",
        "pub fn remove_member(",
        "pub fn remove_member_with_actor_kind(",
        "pub fn leave_conversation(",
        "pub fn leave_conversation_with_actor_kind(",
        "pub fn transfer_conversation_owner(",
        "pub fn transfer_conversation_owner_with_actor_kind(",
        "pub fn change_conversation_member_role(",
        "pub fn change_conversation_member_role_with_actor_kind(",
        "pub fn list_members(",
        "pub fn update_read_cursor(",
        "pub fn update_read_cursor_with_actor_kind(",
        "pub fn read_cursor_view(",
    ] {
        assert!(
            !runtime_source.contains(forbidden_symbol),
            "services/conversation-runtime/src/runtime.rs should not keep membership surface symbol: {forbidden_symbol}"
        );
    }
}
