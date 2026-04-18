use super::*;

pub(super) async fn open_stream(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_open_access(&state, &auth, &request)?;
    let request_key = stream_open_request_key(&auth, request.stream_id.as_str());
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state
            .streaming_runtime
            .open_stream_with_outcome(&auth, request)?,
        request_key,
    )))
}

pub(super) async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_session_write_access(
        &state,
        &auth,
        stream_id.as_str(),
        "stream.checkpoint",
    )?;
    let request_key = stream_checkpoint_request_key(&auth, stream_id.as_str(), request.frame_seq);
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        state.streaming_runtime.checkpoint_stream_with_outcome(
            &auth,
            stream_id.as_str(),
            request,
        )?,
        request_key,
    )))
}

pub(super) async fn append_stream_frame(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<StreamFrameMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.append")?;
    let request_key = stream_append_request_key(&auth, stream_id.as_str(), request.frame_seq);
    let outcome =
        state
            .streaming_runtime
            .append_frame_with_outcome(&auth, stream_id.as_str(), request)?;
    if outcome.applied {
        effects::publish_realtime_stream_frame_event(&state, &auth, &outcome.frame)?;
    }
    Ok(Json(StreamFrameMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_session_conversation_member(&state, &auth, stream_id.as_str())?;
    Ok(Json(state.streaming_runtime.list_frames(
        &auth,
        stream_id.as_str(),
        query,
    )?))
}

pub(super) async fn complete_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CompleteStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_session_write_access(
        &state,
        &auth,
        stream_id.as_str(),
        "stream.complete",
    )?;
    let request_key = stream_complete_request_key(&auth, stream_id.as_str());
    let outcome =
        state
            .streaming_runtime
            .complete_stream_with_outcome(&auth, stream_id.as_str(), request)?;
    if outcome.applied {
        effects::publish_realtime_stream_lifecycle_event(
            &state,
            &auth,
            &outcome.session,
            "stream.completed",
            None,
        )?;
    }
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}

pub(super) async fn abort_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<StreamSessionMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    access::ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.abort")?;
    let request_key = stream_abort_request_key(&auth, stream_id.as_str());
    let outcome =
        state
            .streaming_runtime
            .abort_stream_with_outcome(&auth, stream_id.as_str(), request)?;
    if outcome.applied {
        effects::publish_realtime_stream_lifecycle_event(
            &state,
            &auth,
            &outcome.session,
            "stream.aborted",
            outcome.session.abort_reason.clone(),
        )?;
    }
    Ok(Json(StreamSessionMutationResponse::from_outcome(
        outcome,
        request_key,
    )))
}
