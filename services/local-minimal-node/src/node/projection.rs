use super::*;

pub(super) async fn get_inbox(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<InboxResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(InboxResponse {
        items: state.projection_service.inbox_from_auth_context(&auth),
    }))
}

pub(super) async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let cursor = state
        .projection_service
        .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(cursor))
}

pub(super) async fn update_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<UpdateReadCursorRequest>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_registered_device(&state, &auth)?;
    let cursor = state
        .conversation_runtime
        .update_read_cursor_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.read_seq,
            request.last_read_message_id,
        )?;

    state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: format!("audit_read_cursor_{}", cursor.member_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.clone(),
            action: "conversation.read_cursor_updated".into(),
            payload: Some(
                serde_json::json!({
                    "memberId": cursor.member_id,
                    "principalId": cursor.principal_id,
                    "readSeq": cursor.read_seq,
                    "lastReadMessageId": cursor.last_read_message_id,
                })
                .to_string(),
            ),
        },
    );

    let view = state
        .projection_service
        .read_cursor_from_auth_context(&auth, conversation_id.as_str())?
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_read_cursor_not_found",
            message: format!("conversation read cursor not found: {conversation_id}"),
        })?;
    Ok(Json(view))
}

pub(super) async fn get_timeline(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    Ok(Json(serde_json::json!({
        "items": state
            .projection_service
            .timeline_from_auth_context(&auth, conversation_id.as_str())?
    })))
}

pub(super) async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<projection_service::ConversationSummaryView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let summary = state
        .projection_service
        .conversation_summary_from_auth_context(&auth, conversation_id.as_str())?
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "conversation_summary_not_found",
            message: format!("conversation summary not found: {conversation_id}"),
        })?;
    Ok(Json(summary))
}
