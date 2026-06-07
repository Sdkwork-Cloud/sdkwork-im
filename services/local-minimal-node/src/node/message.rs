use super::*;

pub(super) async fn post_message(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.reply_to,
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<PostMessageRequest>,
) -> Result<Json<PostMessageResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.reply_to,
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<EditMessageRequest>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let summary = request.summary.clone();
    let body = effects::build_message_body(
        request.summary,
        request.text,
        request.reply_to,
        request.parts,
        request.render_hints,
    )?;
    let mut command = EditMessageCommand::from_auth_context(&auth, message_id.clone(), body);
    command.editor = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<MessageMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let mut command = RecallMessageCommand::from_auth_context(&auth, message_id);
    command.recalled_by = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
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

pub(super) async fn delete_message_visibility(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<MessageVisibilityMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;

    let message_seq = state
        .projection_service
        .timeline(auth.tenant_id.as_str(), conversation_id.as_str())
        .into_iter()
        .find(|entry| entry.message_id == message_id)
        .map(|entry| entry.message_seq)
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "message_not_found",
            message: format!("message not found: {message_id}"),
        })?;
    let updated_at = im_time::utc_now_rfc3339_millis();
    let view = MessageVisibilityMutationResult {
        tenant_id: auth.tenant_id.clone(),
        conversation_id: conversation_id.clone(),
        message_id: message_id.clone(),
        message_seq,
        principal_kind: auth.actor_kind.clone(),
        principal_id: auth.actor_id.clone(),
        is_deleted: true,
        updated_at: updated_at.clone(),
    };
    let key = message_visibility_key(
        auth.tenant_id.as_str(),
        message_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    state
        .message_visibility
        .lock()
        .expect("message visibility mutex should not be poisoned")
        .insert(key.clone(), view.clone());

    let event_id = stable_local_audit_record_id("evt_message_visibility_deleted_", key.as_str());
    let payload = serde_json::json!({
        "conversationId": conversation_id,
        "messageId": message_id,
        "messageSeq": message_seq,
        "principalKind": auth.actor_kind,
        "principalId": auth.actor_id,
        "isDeleted": true,
    })
    .to_string();

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_message_visibility_deleted_",
                key.as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: view.conversation_id.clone(),
            action: "message.visibility_deleted".into(),
            payload: Some(payload.clone()),
        },
    );

    state
        .projection_service
        .append_principal_client_route_sync_event(
            auth.tenant_id.as_str(),
            view.principal_id.as_str(),
            view.principal_kind.as_str(),
            event_id.as_str(),
            "message.visibility_deleted",
            Some(view.conversation_id.clone()),
            Some(view.message_id.clone()),
            Some(view.message_seq),
            auth.device_id.clone(),
            None,
            Some("im.message.visibility.deleted.v1".into()),
            Some(payload.clone()),
            updated_at.clone(),
        );

    effects::publish_realtime_event_to_scope(
        &state,
        &auth,
        "message_visibility",
        view.conversation_id.as_str(),
        "message.visibility_deleted",
        payload,
    )?;

    Ok(Json(view))
}

pub(super) async fn list_message_favorites(
    Query(query): Query<MessageFavoritesQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<FavoriteMessagesResponse>, ApiError> {
    const MESSAGE_FAVORITES_DEFAULT_LIMIT: usize = 100;
    const MESSAGE_FAVORITES_MAX_LIMIT: usize = 200;

    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let limit = validate_favorite_page_limit(
        query.limit,
        MESSAGE_FAVORITES_DEFAULT_LIMIT,
        MESSAGE_FAVORITES_MAX_LIMIT,
    )?;
    let cursor_offset = parse_favorite_offset_cursor(query.cursor.as_deref())?;
    let favorite_type = query.favorite_type.map(MessageFavoriteType::as_str);
    let q = query
        .q
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_ascii_lowercase);

    let mut items = state
        .message_favorites
        .lock()
        .expect("message favorites mutex should not be poisoned")
        .values()
        .filter(|favorite| {
            favorite.tenant_id == auth.tenant_id
                && favorite.principal_kind == auth.actor_kind
                && favorite.principal_id == auth.actor_id
        })
        .filter(|favorite| {
            favorite_type
                .map(|expected| favorite.favorite_type == expected)
                .unwrap_or(true)
        })
        .filter(|favorite| {
            q.as_ref()
                .map(|query| favorite_matches_query(favorite, query.as_str()))
                .unwrap_or(true)
        })
        .cloned()
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        right
            .favorited_at
            .cmp(&left.favorited_at)
            .then_with(|| right.message_seq.cmp(&left.message_seq))
            .then_with(|| left.favorite_id.cmp(&right.favorite_id))
    });

    let start = cursor_offset.min(items.len());
    let end = start.saturating_add(limit).min(items.len());
    let has_more = end < items.len();

    Ok(Json(FavoriteMessagesResponse {
        items: items[start..end].to_vec(),
        next_cursor: has_more.then(|| end.to_string()),
        has_more,
    }))
}

pub(super) async fn create_message_favorite(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<FavoriteMessageRequest>,
) -> Result<Json<MessageFavoriteView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    if request.conversation_id != conversation_id {
        return Err(ApiError::bad_request(
            "message_favorite_conversation_mismatch",
            "conversationId does not match the target message",
        ));
    }
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;

    let timeline_entry = state
        .projection_service
        .timeline(auth.tenant_id.as_str(), conversation_id.as_str())
        .into_iter()
        .find(|entry| entry.message_id == message_id)
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "message_not_found",
            message: format!("message not found: {message_id}"),
        })?;
    let title = normalize_favorite_required_text("title", request.title, 256)?;
    let content_preview =
        normalize_favorite_required_text("contentPreview", request.content_preview, 1024)?;
    let source_display_name =
        normalize_favorite_required_text("sourceDisplayName", request.source_display_name, 128)?;
    let favorited_at = im_time::utc_now_rfc3339_millis();
    let favorite_id = message_favorite_id(
        auth.tenant_id.as_str(),
        message_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    let view = MessageFavoriteView {
        tenant_id: auth.tenant_id.clone(),
        principal_kind: auth.actor_kind.clone(),
        principal_id: auth.actor_id.clone(),
        favorite_id: favorite_id.clone(),
        favorite_type: request.favorite_type.as_str().into(),
        conversation_id: conversation_id.clone(),
        message_id: message_id.clone(),
        message_seq: timeline_entry.message_seq,
        title,
        content_preview,
        source_display_name,
        favorited_at: favorited_at.clone(),
    };
    let key = message_favorite_key(
        auth.tenant_id.as_str(),
        favorite_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    state
        .message_favorites
        .lock()
        .expect("message favorites mutex should not be poisoned")
        .insert(key.clone(), view.clone());

    publish_message_favorite_event(
        &state,
        &auth,
        "message.favorite_created",
        "im.message.favorite.created.v1",
        key.as_str(),
        &view,
    )?;

    Ok(Json(view))
}

pub(super) async fn delete_message_favorite(
    Path(favorite_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<DeleteMessageFavoriteResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let favorite_id = normalize_favorite_id(favorite_id)?;
    let key = message_favorite_key(
        auth.tenant_id.as_str(),
        favorite_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    let removed = state
        .message_favorites
        .lock()
        .expect("message favorites mutex should not be poisoned")
        .remove(key.as_str())
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "message_favorite_not_found",
            message: format!("message favorite not found: {favorite_id}"),
        })?;

    publish_message_favorite_event(
        &state,
        &auth,
        "message.favorite_deleted",
        "im.message.favorite.deleted.v1",
        key.as_str(),
        &removed,
    )?;

    Ok(Json(DeleteMessageFavoriteResponse {
        favorite_id,
        deleted: true,
    }))
}

pub(super) async fn add_message_reaction(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<MessageReactionRequest>,
) -> Result<Json<MessageReactionMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    if request.reaction_key.trim().is_empty() {
        return Err(ApiError::bad_request(
            "reaction_key_invalid",
            "reaction key must not be empty",
        ));
    }
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let mut command = AddMessageReactionCommand::from_auth_context(
        &auth,
        message_id,
        request.reaction_key.clone(),
    );
    command.reacted_by = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state.conversation_runtime.add_message_reaction(command)?;
    publish_message_reaction_mutation_event(&state, &auth, "message.reaction_added", &result)?;

    Ok(Json(result))
}

fn parse_favorite_offset_cursor(cursor: Option<&str>) -> Result<usize, ApiError> {
    match cursor {
        Some(cursor) if cursor.trim().is_empty() => Err(ApiError::bad_request(
            "cursor_invalid",
            "cursor must be a non-negative item offset",
        )),
        Some(cursor) => cursor.parse::<usize>().map_err(|_| {
            ApiError::bad_request(
                "cursor_invalid",
                "cursor must be a non-negative item offset",
            )
        }),
        None => Ok(0),
    }
}

fn validate_favorite_page_limit(
    limit: Option<usize>,
    default_limit: usize,
    max_limit: usize,
) -> Result<usize, ApiError> {
    let limit = limit.unwrap_or(default_limit);
    if limit == 0 || limit > max_limit {
        return Err(ApiError::bad_request(
            "limit_invalid",
            format!("limit must be between 1 and {max_limit}"),
        ));
    }
    Ok(limit)
}

fn favorite_matches_query(favorite: &MessageFavoriteView, query: &str) -> bool {
    favorite.title.to_ascii_lowercase().contains(query)
        || favorite
            .content_preview
            .to_ascii_lowercase()
            .contains(query)
        || favorite
            .source_display_name
            .to_ascii_lowercase()
            .contains(query)
}

fn normalize_favorite_required_text(
    field: &'static str,
    value: String,
    max_bytes: usize,
) -> Result<String, ApiError> {
    let normalized = value.trim().to_owned();
    if normalized.is_empty() {
        return Err(ApiError::bad_request(
            "message_favorite_field_required",
            format!("{field} is required"),
        ));
    }
    let actual_bytes = normalized.len();
    if actual_bytes > max_bytes {
        return Err(ApiError::payload_too_large(field, max_bytes, actual_bytes));
    }
    Ok(normalized)
}

fn normalize_favorite_id(favorite_id: String) -> Result<String, ApiError> {
    let favorite_id = favorite_id.trim().to_owned();
    if favorite_id.is_empty() {
        return Err(ApiError::bad_request(
            "message_favorite_id_required",
            "favoriteId is required",
        ));
    }
    Ok(favorite_id)
}

fn publish_message_favorite_event(
    state: &AppState,
    auth: &AppContext,
    action: &'static str,
    payload_schema: &'static str,
    key: &str,
    favorite: &MessageFavoriteView,
) -> Result<(), ApiError> {
    let payload = serde_json::to_string(favorite).map_err(|error| {
        ApiError::service_unavailable(
            "message_favorite_event_invalid",
            format!("failed to encode message favorite event payload: {error}"),
        )
    })?;
    let event_id = stable_local_audit_record_id(format!("evt_{action}_").as_str(), key);
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(format!("audit_{action}_").as_str(), key),
            aggregate_type: "message_favorite".into(),
            aggregate_id: favorite.favorite_id.clone(),
            action: action.into(),
            payload: Some(payload.clone()),
        },
    );

    state
        .projection_service
        .append_principal_client_route_sync_event(
            favorite.tenant_id.as_str(),
            favorite.principal_id.as_str(),
            favorite.principal_kind.as_str(),
            event_id.as_str(),
            action,
            Some(favorite.conversation_id.clone()),
            Some(favorite.message_id.clone()),
            Some(favorite.message_seq),
            auth.device_id.clone(),
            None,
            Some(payload_schema.into()),
            Some(payload.clone()),
            favorite.favorited_at.clone(),
        );

    effects::publish_realtime_event_to_scope(
        state,
        auth,
        "message_favorite",
        favorite.favorite_id.as_str(),
        action,
        payload,
    )
}

pub(super) async fn remove_message_reaction(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<MessageReactionRequest>,
) -> Result<Json<MessageReactionMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    if request.reaction_key.trim().is_empty() {
        return Err(ApiError::bad_request(
            "reaction_key_invalid",
            "reaction key must not be empty",
        ));
    }
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let mut command = RemoveMessageReactionCommand::from_auth_context(
        &auth,
        message_id,
        request.reaction_key.clone(),
    );
    command.removed_by = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state
        .conversation_runtime
        .remove_message_reaction(command)?;
    publish_message_reaction_mutation_event(&state, &auth, "message.reaction_removed", &result)?;

    Ok(Json(result))
}

pub(super) async fn pin_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<MessagePinMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let mut command = PinMessageCommand::from_auth_context(&auth, message_id);
    command.pinned_by = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state.conversation_runtime.pin_message(command)?;
    publish_message_pin_mutation_event(&state, &auth, "message.pin_added", &result)?;

    Ok(Json(result))
}

pub(super) async fn unpin_message(
    Path(message_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<MessagePinMutationResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_client_route_key(&state, &auth)?;
    let conversation_id = state
        .conversation_runtime
        .conversation_id_for_message_from_auth_context(&auth, message_id.as_str())?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let mut command = UnpinMessageCommand::from_auth_context(&auth, message_id);
    command.unpinned_by = principal_profile::resolve_sender_from_auth_context(&state, &auth)?;
    let result = state.conversation_runtime.unpin_message(command)?;
    publish_message_pin_mutation_event(&state, &auth, "message.pin_removed", &result)?;

    Ok(Json(result))
}

fn publish_message_reaction_mutation_event(
    state: &AppState,
    auth: &AppContext,
    action: &str,
    result: &MessageReactionMutationResult,
) -> Result<(), ApiError> {
    if !result.changed {
        return Ok(());
    }

    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                format!("audit_{action}_").as_str(),
                result.message_id.as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: action.into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                    "reactionKey": result.reaction_key,
                })
                .to_string(),
            ),
        },
    );

    effects::publish_realtime_conversation_message_event(
        state,
        auth,
        result.conversation_id.as_str(),
        action,
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
            "reactionKey": result.reaction_key,
        })
        .to_string(),
    )
}

fn publish_message_pin_mutation_event(
    state: &AppState,
    auth: &AppContext,
    action: &str,
    result: &MessagePinMutationResult,
) -> Result<(), ApiError> {
    if !result.changed {
        return Ok(());
    }

    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                format!("audit_{action}_").as_str(),
                result.message_id.as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: result.conversation_id.clone(),
            action: action.into(),
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
        state,
        auth,
        result.conversation_id.as_str(),
        action,
        serde_json::json!({
            "conversationId": result.conversation_id,
            "messageId": result.message_id,
            "messageSeq": result.message_seq,
        })
        .to_string(),
    )
}
