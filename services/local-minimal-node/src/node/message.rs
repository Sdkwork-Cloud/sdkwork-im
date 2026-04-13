use super::*;

pub(super) async fn post_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    let result = effects::post_message_with_side_effects(
        &state,
        &auth,
        conversation_id,
        request.client_msg_id,
        MessageType::Standard,
        body,
    )?;

    Ok(Json(result))
}

pub(super) async fn publish_system_channel_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    let result = effects::publish_system_channel_message_with_side_effects(
        &state,
        &auth,
        conversation_id,
        request.client_msg_id,
        body,
    )?;

    Ok(Json(result))
}

pub(super) async fn edit_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<EditMessageRequest>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_registered_device(&state, &auth)?;
    let summary = request.summary.clone();
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.parts,
        request.render_hints,
    )?;
    let mut command = EditMessageCommand::from_auth_context(&auth, message_id.clone(), body);
    command.editor = user_module::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state.conversation_runtime.edit_message(command)?;

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_message_edited_",
                result.message_id.as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: "message.edited".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                })
                .to_string(),
            ),
        },
    );

    effects::publish_realtime_conversation_message_event(
        &state,
        &auth,
        result.conversation_id.as_str(),
        "message.edited",
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "summary": summary,
        })
        .to_string(),
    )?;

    Ok(Json(result))
}

pub(super) async fn recall_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_registered_device(&state, &auth)?;
    let mut command = RecallMessageCommand::from_auth_context(&auth, message_id);
    command.recalled_by = user_module::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state.conversation_runtime.recall_message(command)?;

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_message_recalled_",
                result.message_id.as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: "message.recalled".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                })
                .to_string(),
            ),
        },
    );

    effects::publish_realtime_conversation_message_event(
        &state,
        &auth,
        result.conversation_id.as_str(),
        "message.recalled",
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "summary": "[recalled]",
        })
        .to_string(),
    )?;

    Ok(Json(result))
}
