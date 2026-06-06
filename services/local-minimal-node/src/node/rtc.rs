use super::*;

pub(super) async fn create_rtc_session(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_create_access(&state, &auth, &request)?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let request_key = rtc_create_request_key(&rtc_auth, &request);
    let outcome = state
        .rtc_runtime
        .create_session_with_outcome(&rtc_auth, request)?;
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn invite_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<InviteRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.invite",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "invite");
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let outcome =
        state
            .rtc_runtime
            .invite_session_with_outcome(&rtc_auth, rtc_session_id.as_str(), request)?;
    if outcome.applied {
        effects::emit_rtc_signal_message(&state, &auth, &outcome.session, "rtc.invite")?;
    }
    Ok(Json(RtcSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn get_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<sdkwork_rtc_core::RtcSession>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let session = state.rtc_runtime.session(&rtc_auth, rtc_session_id.as_str())?;
    if let Some(conversation_id) = session.conversation_id.as_deref() {
        access::ensure_conversation_read_access(&state, &auth, conversation_id)?;
    }
    Ok(Json(session))
}

pub(super) async fn accept_rtc_session(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.accept",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "accept");
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let outcome =
        state
            .rtc_runtime
            .accept_session_with_outcome(&rtc_auth, rtc_session_id.as_str(), request)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.reject",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "reject");
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let outcome =
        state
            .rtc_runtime
            .reject_session_with_outcome(&rtc_auth, rtc_session_id.as_str(), request)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateRtcSessionRequest>,
) -> Result<Json<RtcSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.end",
    )?;
    let request_key =
        rtc_session_action_request_key(auth.tenant_id.as_str(), rtc_session_id.as_str(), "end");
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let outcome =
        state
            .rtc_runtime
            .end_session_with_outcome(&rtc_auth, rtc_session_id.as_str(), request)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<PostRtcSignalRequest>,
) -> Result<Json<sdkwork_rtc_core::RtcSignalEvent>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.signal",
    )?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    let signal = state
        .rtc_runtime
        .post_signal(&rtc_auth, rtc_session_id.as_str(), request)?;
    effects::emit_rtc_custom_signal_message(&state, &auth, &signal)?;
    Ok(Json(signal))
}

pub(super) async fn issue_rtc_participant_credential(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<IssueRtcParticipantCredentialRequest>,
) -> Result<Json<sdkwork_rtc_core::RtcParticipantCredential>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.issue_credential",
    )?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    Ok(Json(state.rtc_runtime.issue_participant_credential(
        &rtc_auth,
        rtc_session_id.as_str(),
        request.participant_id.as_str(),
    )?))
}

pub(super) async fn get_rtc_recording_artifact(
    Path(rtc_session_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<sdkwork_rtc_core::RtcRecordingArtifact>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_rtc_session_conversation_write_access(
        &state,
        &auth,
        rtc_session_id.as_str(),
        "rtc.artifact",
    )?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    Ok(Json(
        state
            .rtc_runtime
            .recording_artifact(&rtc_auth, rtc_session_id.as_str())?,
    ))
}

pub(super) async fn map_rtc_provider_callback(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<RtcCallbackRequest>,
) -> Result<Json<RtcCallbackEvent>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let rtc_auth = rtc_app_context_from_auth(&auth);
    Ok(Json(state.rtc_runtime.map_provider_callback(&rtc_auth, request)?))
}

pub(super) async fn get_rtc_provider_health(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ProviderHealthSnapshot>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    Ok(Json(
        state
            .rtc_runtime
            .provider_health_snapshot(auth.tenant_id.as_str())?,
    ))
}
