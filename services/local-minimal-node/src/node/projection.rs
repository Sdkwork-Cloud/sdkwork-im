use super::*;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct TimelineQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
}

pub(super) async fn get_contacts(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ContactsResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    ensure_contact_user_actor(&auth)?;
    social::maybe_repair_pending_friend_request_acceptances(&state).await?;
    let mut items = state
        .social_query
        .authoritative_active_friendships_for_user(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_query_unavailable",
            message: format!("failed to load authoritative contacts from control plane: {error}"),
        })?
        .into_iter()
        .filter(|friendship| {
            state
                .social_query
                .active_friendship_access_block_for_pair(
                    auth.tenant_id.as_str(),
                    friendship.user_low_id.as_str(),
                    friendship.user_high_id.as_str(),
                )
                .is_none()
        })
        .map(|friendship| contact_view_from_authoritative_friendship(&state, &auth, friendship))
        .collect::<Result<Vec<_>, _>>()?;
    items.sort_by(|left, right| {
        right
            .last_interaction_at
            .cmp(&left.last_interaction_at)
            .then_with(|| left.target_user_id.cmp(&right.target_user_id))
    });
    Ok(Json(ContactsResponse { items }))
}

fn ensure_contact_user_actor(auth: &AuthContext) -> Result<(), ApiError> {
    if auth.actor_kind == "user" {
        return Ok(());
    }

    Err(ApiError::forbidden(
        "contact_scope_forbidden",
        format!("contacts require user actor kind, got {}", auth.actor_kind),
    ))
}

fn contact_view_from_authoritative_friendship(
    state: &AppState,
    auth: &AuthContext,
    friendship: Friendship,
) -> Result<ContactView, ApiError> {
    let target_user_id = if friendship.user_low_id == auth.actor_id {
        friendship.user_high_id.clone()
    } else if friendship.user_high_id == auth.actor_id {
        friendship.user_low_id.clone()
    } else {
        return Err(ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_query_invalid_state",
            message: format!(
                "authoritative friendship {} does not include contact owner {}",
                friendship.friendship_id, auth.actor_id
            ),
        });
    };

    let direct_chat = state
        .social_query
        .authoritative_active_direct_chat_for_pair(
            auth.tenant_id.as_str(),
            friendship.user_low_id.as_str(),
            friendship.user_high_id.as_str(),
        )
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_query_unavailable",
            message: format!(
                "failed to load authoritative direct chat for friendship {}: {error}",
                friendship.friendship_id
            ),
        })?;
    let established_at = friendship
        .established_at
        .clone()
        .unwrap_or_else(|| friendship.updated_at.clone());
    let last_interaction_at = direct_chat
        .as_ref()
        .map(|direct_chat| std::cmp::max(established_at.clone(), direct_chat.updated_at.clone()))
        .unwrap_or_else(|| established_at.clone());

    Ok(ContactView {
        tenant_id: friendship.tenant_id,
        owner_user_id: auth.actor_id.clone(),
        target_user_id,
        contact_type: "friendship".into(),
        relationship_state: "active".into(),
        friendship_id: friendship.friendship_id,
        direct_chat_id: direct_chat
            .as_ref()
            .map(|direct_chat| direct_chat.direct_chat_id.clone()),
        conversation_id: direct_chat.and_then(|direct_chat| direct_chat.conversation_id),
        established_at,
        last_interaction_at,
    })
}

pub(super) async fn get_inbox(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<InboxResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, &headers)?;
    let items = state
        .projection_service
        .inbox_from_auth_context(&auth)
        .into_iter()
        .filter(|item| {
            access::direct_chat_access_block_for_conversation(
                &state,
                auth.tenant_id.as_str(),
                item.conversation_id.as_str(),
            )
            .is_none()
        })
        .collect();
    Ok(Json(InboxResponse { items }))
}

pub(super) async fn get_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
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
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    access::ensure_registered_device(&state, &auth)?;
    let cursor = state
        .conversation_runtime
        .update_read_cursor_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.read_seq,
            request.last_read_message_id,
        )?;

    let _ = state.audit_runtime.record_anchor(
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
    Query(query): Query<TimelineQuery>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<projection_service::TimelineWindowView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
    Ok(Json(
        state.projection_service.timeline_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            query.after_seq,
            query.limit,
        )?,
    ))
}

pub(super) async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<projection_service::ConversationSummaryView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
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

pub(super) async fn get_member_directory(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<MemberDirectoryResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
    Ok(Json(MemberDirectoryResponse {
        items: state
            .projection_service
            .member_directory_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

pub(super) async fn get_pinned_messages(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<PinnedMessagesResponse>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
    Ok(Json(PinnedMessagesResponse {
        items: state
            .projection_service
            .pinned_messages_from_auth_context(&auth, conversation_id.as_str())?,
    }))
}

pub(super) async fn get_message_interaction_summary(
    Path((conversation_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Result<Json<projection_service::MessageInteractionSummaryView>, ApiError> {
    let auth = resolve_auth_context(&headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
    let summary = state
        .projection_service
        .message_interaction_summary_from_auth_context(
            &auth,
            conversation_id.as_str(),
            message_id.as_str(),
        )?
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "message_interaction_summary_not_found",
            message: format!(
                "message interaction summary not found: {conversation_id}/{message_id}"
            ),
        })?;
    Ok(Json(summary))
}
