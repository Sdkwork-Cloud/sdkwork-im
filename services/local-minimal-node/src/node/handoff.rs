use super::*;

pub(super) async fn get_agent_handoff_state(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(
        state
            .conversation_runtime
            .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?,
    ))
}

pub(super) async fn accept_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state
        .conversation_runtime
        .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?;
    let result = state
        .conversation_runtime
        .accept_agent_handoff_from_auth_context(&auth, conversation_id)?;
    if result != previous_state {
        effects::publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}

pub(super) async fn resolve_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state
        .conversation_runtime
        .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?;
    let result = state
        .conversation_runtime
        .resolve_agent_handoff_from_auth_context(&auth, conversation_id)?;
    if result != previous_state {
        effects::publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}

pub(super) async fn close_agent_handoff(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<AgentHandoffStateView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let previous_state = state
        .conversation_runtime
        .get_agent_handoff_state_from_auth_context(&auth, conversation_id.as_str())?;
    let result = state
        .conversation_runtime
        .close_agent_handoff_from_auth_context(&auth, conversation_id)?;
    if result != previous_state {
        effects::publish_realtime_agent_handoff_status_changed_event(
            &state,
            &auth,
            &previous_state,
            &result,
        )?;
    }
    Ok(Json(result))
}
