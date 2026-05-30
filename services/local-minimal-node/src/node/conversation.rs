use super::*;

pub(super) async fn create_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let (_, creator_attributes) = principal_profile::resolve_member_principal(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
    )?;
    Ok(Json(
        state
            .conversation_runtime
            .create_conversation_from_auth_context_with_creator_attributes(
                &auth,
                request.conversation_id,
                request.conversation_type,
                creator_attributes,
            )?,
    ))
}

pub(super) async fn create_agent_dialog(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentDialogRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_app_context(&headers)?;
    let (_, requester_attributes) = principal_profile::resolve_member_principal(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        auth.actor_kind.as_str(),
    )?;
    Ok(Json(
        state
            .conversation_runtime
            .create_agent_dialog_from_auth_context_with_requester_attributes(
                &auth,
                request.conversation_id,
                request.agent_id,
                requester_attributes,
            )?,
    ))
}

pub(super) async fn create_agent_handoff(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentHandoffRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    let (target_kind, target_attributes) = principal_profile::resolve_member_principal(
        &state,
        auth.tenant_id.as_str(),
        request.target_id.as_str(),
        request.target_kind.as_str(),
    )?;
    Ok(Json(
        state
            .conversation_runtime
            .create_agent_handoff_from_auth_context_with_target_attributes(
                &auth,
                request.conversation_id,
                request.target_id,
                target_kind,
                request.handoff_session_id,
                request.handoff_reason,
                target_attributes,
            )?,
    ))
}

pub(super) async fn create_system_channel(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateSystemChannelRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    let (_, subscriber_attributes) = principal_profile::resolve_member_principal(
        &state,
        auth.tenant_id.as_str(),
        request.subscriber_id.as_str(),
        "user",
    )?;
    Ok(Json(
        state
            .conversation_runtime
            .create_system_channel_from_auth_context_with_subscriber_attributes(
                &auth,
                request.conversation_id,
                request.subscriber_id,
                subscriber_attributes,
            )?,
    ))
}

pub(super) async fn create_thread_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateThreadConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .create_thread_conversation_from_auth_context(
                &auth,
                request.conversation_id,
                request.parent_conversation_id,
                request.root_message_id,
            )?,
    ))
}

pub(super) async fn bind_direct_chat_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    principal_profile::ensure_active_principal(
        &state,
        auth.tenant_id.as_str(),
        request.left_actor_id.as_str(),
        request.left_actor_kind.as_str(),
    )?;
    principal_profile::ensure_active_principal(
        &state,
        auth.tenant_id.as_str(),
        request.right_actor_id.as_str(),
        request.right_actor_kind.as_str(),
    )?;
    Ok(Json(
        state
            .conversation_runtime
            .bind_direct_chat_conversation_from_auth_context(
                &auth,
                request.conversation_id,
                request.direct_chat_id,
                request.left_actor_id,
                request.left_actor_kind,
                request.right_actor_id,
                request.right_actor_kind,
            )?,
    ))
}
