use super::*;

pub(super) async fn create_rtc_session(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_create_access(&state, &auth, &request)?;
    let request_key = rtc_create_request_key(&auth, &request);
    let outcome = state
        .rtc_runtime
        .create_session_with_outcome(&auth, request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn invite_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.invite",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "invite");
    let outcome =
        state
            .rtc_runtime
            .invite_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        effects::emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.invite")?;
    }
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn accept_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.accept",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "accept");
    let outcome =
        state
            .rtc_runtime
            .accept_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        effects::emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.accept")?;
    }
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn reject_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.reject",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "reject");
    let outcome =
        state
            .rtc_runtime
            .reject_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        effects::emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.reject")?;
    }
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn end_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.end",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "end");
    let outcome =
        state
            .rtc_runtime
            .end_session_with_outcome(&auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        effects::emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.end")?;
    }
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn post_rtc_signal(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<im_domain_core::rtc::RtcSignalEvent>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.signal",
    )?;
    let signal = state
        .rtc_runtime
        .post_signal(&auth, rtc_session_id.as_str(), request)?;
    effects::emit_rtc_custom_signal_message(&state, &auth, &signal)?;
    Ok(Json(signal))
}

pub(super) async fn issue_rtc_participant_credential(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Result<Json<im_platform_contracts::RtcParticipantCredential>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.issue_credential",
    )?;
    Ok(Json(state.rtc_runtime.issue_participant_credential(
        &auth,
        rtc_session_id.as_str(),
        request.participant_id.as_str(),
    )?))
}

pub(super) async fn get_rtc_recording_artifact(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::RtcRecordingArtifact>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.artifact",
    )?;
    Ok(Json(
        state
            .rtc_runtime
            .recording_artifact(&auth, rtc_session_id.as_str())?,
    ))
}

pub(super) async fn map_rtc_provider_callback(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<im_platform_contracts::RtcCallbackRequest>,
) -> Result<Json<im_platform_contracts::RtcCallbackEvent>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    Ok(Json(
        state.rtc_runtime.map_provider_callback(&auth, request)?,
    ))
}

pub(super) async fn get_rtc_provider_health(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<im_platform_contracts::ProviderHealthSnapshot>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    Ok(Json(
        state
            .rtc_runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?,
    ))
}
