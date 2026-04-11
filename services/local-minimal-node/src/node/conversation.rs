use super::*;

pub(super) async fn create_conversation(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let (_, creator_attributes) = user_module::resolve_member_principal(
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
    let auth = resolve_auth_context(&headers)?;
    let (_, requester_attributes) = user_module::resolve_member_principal(
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
    let auth = resolve_auth_context(&headers)?;
    let (target_kind, target_attributes) = user_module::resolve_member_principal(
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
    let auth = resolve_auth_context(&headers)?;
    let (_, subscriber_attributes) = user_module::resolve_member_principal(
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
