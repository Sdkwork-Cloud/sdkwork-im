use super::*;

pub(super) async fn open_stream(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<OpenStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_stream_open_access(&state, &auth, &request)?;
    Ok(Json(state.streaming_runtime.open_stream(&auth, request)?))
}

pub(super) async fn checkpoint_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<CheckpointStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_stream_session_write_access(
        &state,
        &auth,
        stream_id.as_str(),
        "stream.checkpoint",
    )?;
    Ok(Json(state.streaming_runtime.checkpoint_stream(
        &auth,
        stream_id.as_str(),
        request,
    )?))
}

pub(super) async fn append_stream_frame(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AppendStreamFrameRequest>,
) -> Result<Json<im_domain_core::stream::StreamFrame>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.append")?;
    let frame = state
        .streaming_runtime
        .append_frame(&auth, stream_id.as_str(), request)?;
    effects::publish_realtime_stream_frame_event(&state, &auth, &frame)?;
    Ok(Json(frame))
}

pub(super) async fn list_stream_frames(
    Path(stream_id): Path<String>,
    Query(query): Query<ListStreamFramesQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<StreamFrameWindow>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
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
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_stream_session_write_access(
        &state,
        &auth,
        stream_id.as_str(),
        "stream.complete",
    )?;
    let session = state
        .streaming_runtime
        .complete_stream(&auth, stream_id.as_str(), request)?;
    effects::publish_realtime_stream_lifecycle_event(
        &state,
        &auth,
        &session,
        "stream.completed",
        None,
    )?;
    Ok(Json(session))
}

pub(super) async fn abort_stream(
    Path(stream_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<AbortStreamRequest>,
) -> Result<Json<im_domain_core::stream::StreamSession>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_stream_session_write_access(&state, &auth, stream_id.as_str(), "stream.abort")?;
    let abort_reason = request.reason.clone();
    let session = state
        .streaming_runtime
        .abort_stream(&auth, stream_id.as_str(), request)?;
    effects::publish_realtime_stream_lifecycle_event(
        &state,
        &auth,
        &session,
        "stream.aborted",
        abort_reason,
    )?;
    Ok(Json(session))
}
