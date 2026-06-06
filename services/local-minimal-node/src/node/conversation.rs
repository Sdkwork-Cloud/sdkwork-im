use super::*;

pub(super) async fn create_conversation(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentDialogRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    require_standard_agent_id(request.agent_id.as_str())?;
    let auth = resolve_request_app_context(auth, &headers)?;
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

fn require_standard_agent_id(agent_id: &str) -> Result<(), ApiError> {
    if agent_id.trim().is_empty() {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId is required",
        ));
    }
    if agent_id.trim() != agent_id {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId must not contain leading or trailing whitespace",
        ));
    }
    if agent_id.chars().count() > 128 {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId must be at most 128 characters",
        ));
    }
    if !agent_id.chars().all(is_standard_agent_id_character) {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId must use lowercase standard id characters",
        ));
    }
    if !agent_id.split('.').all(|segment| !segment.is_empty()) {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId must use non-empty dot-delimited segments",
        ));
    }
    if !agent_id.starts_with("agent.") {
        return Err(ApiError::bad_request(
            "agent_id_invalid",
            "agentId must start with agent.",
        ));
    }
    Ok(())
}

fn is_standard_agent_id_character(ch: char) -> bool {
    ch.is_ascii_lowercase() || ch.is_ascii_digit() || matches!(ch, '.' | '_' | '-')
}

pub(super) async fn create_agent_handoff(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateAgentHandoffRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateSystemChannelRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateThreadConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<BindDirectChatConversationRequest>,
) -> Result<Json<CreateConversationResult>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
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
