use super::*;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct TimelineQuery {
    after_seq: Option<u64>,
    limit: Option<usize>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactsQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactTagsQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

fn parse_offset_cursor(cursor: Option<&str>) -> Result<usize, ApiError> {
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

fn validate_page_limit(
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

pub(super) async fn get_contacts(
    Query(query): Query<ContactsQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ContactsResponse>, ApiError> {
    const CONTACT_LIST_DEFAULT_LIMIT: usize = 100;
    const CONTACT_LIST_MAX_LIMIT: usize = 200;

    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_contact_user_actor(&auth)?;
    social::maybe_repair_pending_friend_request_acceptances(&state).await?;
    let limit = validate_page_limit(
        query.limit,
        CONTACT_LIST_DEFAULT_LIMIT,
        CONTACT_LIST_MAX_LIMIT,
    )?;
    let cursor_offset = parse_offset_cursor(query.cursor.as_deref())?;

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
    items.retain(|contact| {
        let key = contact_preferences_key(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            contact.target_user_id.as_str(),
        );
        !state
            .contact_preferences
            .lock()
            .expect("contact preferences mutex should not be poisoned")
            .get(key.as_str())
            .map(|preferences| preferences.is_blocked)
            .unwrap_or(false)
    });
    items.sort_by(|left, right| {
        right
            .last_interaction_at
            .cmp(&left.last_interaction_at)
            .then_with(|| left.target_user_id.cmp(&right.target_user_id))
    });
    let start = cursor_offset.min(items.len());
    let end = start.saturating_add(limit).min(items.len());
    let has_more = end < items.len();
    let window_items = items[start..end].to_vec();
    Ok(Json(ContactsResponse {
        items: window_items,
        next_cursor: has_more.then(|| end.to_string()),
        has_more,
    }))
}

pub(super) async fn list_contact_tags(
    Query(query): Query<ContactTagsQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ContactTagsResponse>, ApiError> {
    const CONTACT_TAGS_DEFAULT_LIMIT: usize = 100;
    const CONTACT_TAGS_MAX_LIMIT: usize = 200;

    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_contact_user_actor(&auth)?;
    let limit = validate_page_limit(
        query.limit,
        CONTACT_TAGS_DEFAULT_LIMIT,
        CONTACT_TAGS_MAX_LIMIT,
    )?;
    let cursor_offset = parse_offset_cursor(query.cursor.as_deref())?;

    let mut items = state
        .contact_tags
        .lock()
        .expect("contact tags mutex should not be poisoned")
        .values()
        .filter(|tag| tag.tenant_id == auth.tenant_id && tag.owner_user_id == auth.actor_id)
        .cloned()
        .collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.name
            .cmp(&right.name)
            .then_with(|| left.tag_id.cmp(&right.tag_id))
    });
    let start = cursor_offset.min(items.len());
    let end = start.saturating_add(limit).min(items.len());
    let has_more = end < items.len();
    Ok(Json(ContactTagsResponse {
        items: items[start..end].to_vec(),
        next_cursor: has_more.then(|| end.to_string()),
        has_more,
    }))
}

pub(super) async fn create_contact_tag(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateContactTagRequest>,
) -> Result<Json<ContactTagView>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_contact_user_actor(&auth)?;
    let name = normalize_required_contact_tag_field("name", request.name, 128)?;
    let color = normalize_required_contact_tag_field("color", request.color, 64)?;
    let bg = normalize_profile_field("bg", request.bg, 128)?.unwrap_or_default();
    let border = normalize_profile_field("border", request.border, 128)?.unwrap_or_default();
    let now = im_time::utc_now_rfc3339_millis();
    let tag_id = create_contact_tag_id(auth.actor_id.as_str(), now.as_str(), name.as_str());
    let view = ContactTagView {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.actor_id.clone(),
        tag_id: tag_id.clone(),
        name,
        color,
        count: request.count.unwrap_or(0),
        bg,
        border,
        created_at: now.clone(),
        updated_at: now,
    };
    state
        .contact_tags
        .lock()
        .expect("contact tags mutex should not be poisoned")
        .insert(
            contact_tag_key(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                tag_id.as_str(),
            ),
            view.clone(),
        );

    record_contact_tag_audit(&state, &auth, "social.contact_tag_created", &view);
    Ok(Json(view))
}

pub(super) async fn update_contact_tag(
    Path(tag_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateContactTagRequest>,
) -> Result<Json<ContactTagView>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_contact_user_actor(&auth)?;
    let tag_id = normalize_contact_tag_id(tag_id)?;
    let name = match request.name {
        Some(name) => Some(normalize_required_contact_tag_field("name", name, 128)?),
        None => None,
    };
    let color = match request.color {
        Some(color) => Some(normalize_required_contact_tag_field("color", color, 64)?),
        None => None,
    };
    let bg = normalize_profile_field("bg", request.bg, 128)?;
    let border = normalize_profile_field("border", request.border, 128)?;
    let key = contact_tag_key(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        tag_id.as_str(),
    );
    let view = {
        let mut tags = state
            .contact_tags
            .lock()
            .expect("contact tags mutex should not be poisoned");
        let mut view = tags.get(key.as_str()).cloned().ok_or_else(|| ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "contact_tag_not_found",
            message: format!("contact tag not found: {tag_id}"),
        })?;
        if let Some(name) = name {
            view.name = name;
        }
        if let Some(color) = color {
            view.color = color;
        }
        if let Some(count) = request.count {
            view.count = count;
        }
        if let Some(bg) = bg {
            view.bg = bg;
        }
        if let Some(border) = border {
            view.border = border;
        }
        view.updated_at = im_time::utc_now_rfc3339_millis();
        tags.insert(key, view.clone());
        view
    };

    record_contact_tag_audit(&state, &auth, "social.contact_tag_updated", &view);
    Ok(Json(view))
}

pub(super) async fn delete_contact_tag(
    Path(tag_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<DeleteContactTagResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_contact_user_actor(&auth)?;
    let tag_id = normalize_contact_tag_id(tag_id)?;
    let key = contact_tag_key(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        tag_id.as_str(),
    );
    let removed = state
        .contact_tags
        .lock()
        .expect("contact tags mutex should not be poisoned")
        .remove(key.as_str());
    if removed.is_none() {
        return Err(ApiError {
            status: axum::http::StatusCode::NOT_FOUND,
            code: "contact_tag_not_found",
            message: format!("contact tag not found: {tag_id}"),
        });
    }

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id("audit_contact_tag_deleted_", key.as_str()),
            aggregate_type: "contact_tag".into(),
            aggregate_id: key,
            action: "social.contact_tag_deleted".into(),
            payload: Some(
                serde_json::json!({
                    "ownerUserId": auth.actor_id.as_str(),
                    "tagId": tag_id.as_str(),
                })
                .to_string(),
            ),
        },
    );

    Ok(Json(DeleteContactTagResponse {
        tag_id,
        deleted: true,
    }))
}

pub(super) async fn create_contact_recommendation(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<CreateContactRecommendationRequest>,
) -> Result<Json<ContactRecommendationView>, ApiError> {
    const TARGET_CONVERSATION_ID_MAX_BYTES: usize = 128;

    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let target_user_id = ensure_active_contact_access(&state, &auth, target_user_id).await?;
    let target_conversation_id = normalize_profile_field(
        "targetConversationId",
        request.target_conversation_id,
        TARGET_CONVERSATION_ID_MAX_BYTES,
    )?;
    let now = im_time::utc_now_rfc3339_millis();
    let recommendation_id = create_contact_recommendation_id(
        auth.actor_id.as_str(),
        target_user_id.as_str(),
        now.as_str(),
    );
    let key = contact_recommendation_key(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        target_user_id.as_str(),
        recommendation_id.as_str(),
    );
    let view = ContactRecommendationView {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.actor_id.clone(),
        target_user_id: target_user_id.clone(),
        recommendation_id,
        target_conversation_id,
        created_at: now,
    };
    state
        .contact_recommendations
        .lock()
        .expect("contact recommendations mutex should not be poisoned")
        .insert(key.clone(), view.clone());

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id("audit_contact_recommendation_", key.as_str()),
            aggregate_type: "contact".into(),
            aggregate_id: contact_preferences_key(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                target_user_id.as_str(),
            ),
            action: "social.contact_recommendation_created".into(),
            payload: Some(
                serde_json::json!({
                    "ownerUserId": view.owner_user_id.as_str(),
                    "targetUserId": view.target_user_id.as_str(),
                    "recommendationId": view.recommendation_id.as_str(),
                    "targetConversationId": view.target_conversation_id.as_deref(),
                })
                .to_string(),
            ),
        },
    );

    Ok(Json(view))
}

fn ensure_contact_user_actor(auth: &AppContext) -> Result<(), ApiError> {
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
    auth: &AppContext,
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

pub(super) async fn get_contact_preferences(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ContactPreferencesView>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let target_user_id = ensure_active_contact_access(&state, &auth, target_user_id).await?;
    let key = contact_preferences_key(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        target_user_id.as_str(),
    );
    let stored = state
        .contact_preferences
        .lock()
        .expect("contact preferences mutex should not be poisoned")
        .get(key.as_str())
        .cloned();
    Ok(Json(stored.unwrap_or_else(|| {
        default_contact_preferences(&auth, target_user_id)
    })))
}

pub(super) async fn update_contact_preferences(
    Path(target_user_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateContactPreferencesRequest>,
) -> Result<Json<ContactPreferencesView>, ApiError> {
    const CONTACT_REMARK_MAX_BYTES: usize = 256;

    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    let target_user_id = ensure_active_contact_access(&state, &auth, target_user_id).await?;
    let remark = normalize_profile_field("remark", request.remark, CONTACT_REMARK_MAX_BYTES)?;
    let key = contact_preferences_key(
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        target_user_id.as_str(),
    );
    let view = {
        let mut preferences = state
            .contact_preferences
            .lock()
            .expect("contact preferences mutex should not be poisoned");
        let mut view = preferences
            .get(key.as_str())
            .cloned()
            .unwrap_or_else(|| default_contact_preferences(&auth, target_user_id.clone()));
        if let Some(is_starred) = request.is_starred {
            view.is_starred = is_starred;
        }
        if let Some(remark) = remark {
            view.remark = remark;
        }
        if let Some(is_blocked) = request.is_blocked {
            view.is_blocked = is_blocked;
            if is_blocked {
                view.is_starred = false;
            }
        }
        view.updated_at = im_time::utc_now_rfc3339_millis();
        preferences.insert(key, view.clone());
        view
    };

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_contact_preferences_",
                contact_preferences_key(
                    auth.tenant_id.as_str(),
                    auth.actor_id.as_str(),
                    target_user_id.as_str(),
                )
                .as_str(),
            ),
            aggregate_type: "contact".into(),
            aggregate_id: contact_preferences_key(
                auth.tenant_id.as_str(),
                auth.actor_id.as_str(),
                target_user_id.as_str(),
            ),
            action: "social.contact_preferences_updated".into(),
            payload: Some(
                serde_json::json!({
                    "ownerUserId": view.owner_user_id.as_str(),
                    "targetUserId": view.target_user_id.as_str(),
                    "isStarred": view.is_starred,
                    "remark": view.remark.as_str(),
                    "isBlocked": view.is_blocked,
                })
                .to_string(),
            ),
        },
    );

    Ok(Json(view))
}

async fn ensure_active_contact_access(
    state: &AppState,
    auth: &AppContext,
    target_user_id: String,
) -> Result<String, ApiError> {
    ensure_contact_user_actor(auth)?;
    social::maybe_repair_pending_friend_request_acceptances(state).await?;
    let target_user_id = target_user_id.trim().to_owned();
    if target_user_id.is_empty() {
        return Err(ApiError::bad_request(
            "contact_target_user_id_required",
            "targetUserId is required",
        ));
    }
    if target_user_id == auth.actor_id {
        return Err(ApiError::forbidden(
            "contact_scope_forbidden",
            "contact preferences require a distinct active contact",
        ));
    }

    let active_friendship_exists = state
        .social_query
        .authoritative_active_friendships_for_user(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "social_query_unavailable",
            message: format!("failed to load authoritative contacts from control plane: {error}"),
        })?
        .into_iter()
        .any(|friendship| {
            friendship.user_low_id == target_user_id || friendship.user_high_id == target_user_id
        });
    if !active_friendship_exists {
        return Err(ApiError::forbidden(
            "contact_preferences_forbidden",
            "contact preferences require an active friendship with the target user",
        ));
    }
    if state
        .social_query
        .active_friendship_access_block_for_pair(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            target_user_id.as_str(),
        )
        .is_some()
    {
        return Err(ApiError::forbidden(
            "contact_preferences_forbidden",
            "contact preferences are not visible for blocked friendship access",
        ));
    }

    Ok(target_user_id)
}

fn default_contact_preferences(
    auth: &AppContext,
    target_user_id: String,
) -> ContactPreferencesView {
    ContactPreferencesView {
        tenant_id: auth.tenant_id.clone(),
        owner_user_id: auth.actor_id.clone(),
        target_user_id,
        is_starred: false,
        remark: String::new(),
        is_blocked: false,
        updated_at: im_time::utc_now_rfc3339_millis(),
    }
}

pub(super) async fn get_inbox(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<InboxResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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

pub(super) async fn get_conversation_profile(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ConversationProfileView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let key = conversation_profile_key(auth.tenant_id.as_str(), conversation_id.as_str());
    let stored = state
        .conversation_profiles
        .lock()
        .expect("conversation profiles mutex should not be poisoned")
        .get(key.as_str())
        .cloned();
    Ok(Json(stored.unwrap_or_else(|| {
        default_conversation_profile(&auth, conversation_id)
    })))
}

pub(super) async fn update_conversation_profile(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateConversationProfileRequest>,
) -> Result<Json<ConversationProfileView>, ApiError> {
    const DISPLAY_NAME_MAX_BYTES: usize = 256;
    const AVATAR_URL_MAX_BYTES: usize = 2048;
    const NOTICE_MAX_BYTES: usize = 4096;

    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let actor_member = state
        .conversation_runtime
        .require_active_member_from_auth_context(&auth, conversation_id.as_str())?;
    if !matches!(
        actor_member.role,
        MembershipRole::Owner | MembershipRole::Admin
    ) {
        return Err(ApiError::forbidden(
            "conversation_profile_permission_denied",
            "conversation profile updates require owner or admin role",
        ));
    }

    let display_name =
        normalize_profile_field("displayName", request.display_name, DISPLAY_NAME_MAX_BYTES)?;
    if matches!(display_name.as_deref(), Some("")) {
        return Err(ApiError::bad_request(
            "conversation_profile_display_name_invalid",
            "displayName must not be empty",
        ));
    }
    let avatar_url =
        normalize_profile_field("avatarUrl", request.avatar_url, AVATAR_URL_MAX_BYTES)?;
    let notice = normalize_profile_field("notice", request.notice, NOTICE_MAX_BYTES)?;
    let key = conversation_profile_key(auth.tenant_id.as_str(), conversation_id.as_str());
    let view = {
        let mut profiles = state
            .conversation_profiles
            .lock()
            .expect("conversation profiles mutex should not be poisoned");
        let mut view = profiles
            .get(key.as_str())
            .cloned()
            .unwrap_or_else(|| default_conversation_profile(&auth, conversation_id.clone()));
        if let Some(display_name) = display_name {
            view.display_name = display_name;
        }
        if let Some(avatar_url) = avatar_url {
            view.avatar_url = avatar_url;
        }
        if let Some(notice) = notice {
            view.notice = notice;
        }
        view.updated_at = im_time::utc_now_rfc3339_millis();
        view.updated_by_principal_kind = Some(actor_member.principal_kind.clone());
        view.updated_by_principal_id = Some(actor_member.principal_id.clone());
        profiles.insert(key, view.clone());
        view
    };

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_conversation_profile_",
                conversation_profile_key(auth.tenant_id.as_str(), conversation_id.as_str())
                    .as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id,
            action: "conversation.profile_updated".into(),
            payload: Some(
                serde_json::json!({
                    "displayName": view.display_name.as_str(),
                    "avatarUrl": view.avatar_url.as_str(),
                    "notice": view.notice.as_str(),
                    "updatedByPrincipalKind": view.updated_by_principal_kind.as_deref(),
                    "updatedByPrincipalId": view.updated_by_principal_id.as_deref(),
                })
                .to_string(),
            ),
        },
    );

    Ok(Json(view))
}

fn normalize_profile_field(
    field: &'static str,
    value: Option<String>,
    max_bytes: usize,
) -> Result<Option<String>, ApiError> {
    let Some(value) = value else {
        return Ok(None);
    };
    let normalized = value.trim().to_owned();
    let actual_bytes = normalized.len();
    if actual_bytes > max_bytes {
        return Err(ApiError::payload_too_large(field, max_bytes, actual_bytes));
    }
    Ok(Some(normalized))
}

fn normalize_required_contact_tag_field(
    field: &'static str,
    value: String,
    max_bytes: usize,
) -> Result<String, ApiError> {
    let normalized = normalize_profile_field(field, Some(value), max_bytes)?.unwrap_or_default();
    if normalized.is_empty() {
        return Err(ApiError::bad_request(
            "contact_tag_field_required",
            format!("{field} is required"),
        ));
    }
    Ok(normalized)
}

fn normalize_contact_tag_id(tag_id: String) -> Result<String, ApiError> {
    let tag_id = tag_id.trim().to_owned();
    if tag_id.is_empty() {
        return Err(ApiError::bad_request(
            "contact_tag_id_required",
            "tagId is required",
        ));
    }
    Ok(tag_id)
}

fn create_contact_tag_id(owner_user_id: &str, created_at: &str, name: &str) -> String {
    let digest = Sha256::digest(format!("{owner_user_id}:{created_at}:{name}").as_bytes());
    format!("tag_{digest:x}").chars().take(36).collect()
}

fn create_contact_recommendation_id(
    owner_user_id: &str,
    target_user_id: &str,
    created_at: &str,
) -> String {
    let digest =
        Sha256::digest(format!("{owner_user_id}:{target_user_id}:{created_at}").as_bytes());
    format!("rec_{digest:x}").chars().take(36).collect()
}

fn record_contact_tag_audit(
    state: &AppState,
    auth: &AppContext,
    action: &'static str,
    tag: &ContactTagView,
) {
    let aggregate_id = contact_tag_key(
        tag.tenant_id.as_str(),
        tag.owner_user_id.as_str(),
        tag.tag_id.as_str(),
    );
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id("audit_contact_tag_", aggregate_id.as_str()),
            aggregate_type: "contact_tag".into(),
            aggregate_id,
            action: action.into(),
            payload: Some(
                serde_json::json!({
                    "ownerUserId": tag.owner_user_id.as_str(),
                    "tagId": tag.tag_id.as_str(),
                    "name": tag.name.as_str(),
                    "color": tag.color.as_str(),
                    "count": tag.count,
                    "bg": tag.bg.as_str(),
                    "border": tag.border.as_str(),
                })
                .to_string(),
            ),
        },
    );
}

fn default_conversation_profile(
    auth: &AppContext,
    conversation_id: String,
) -> ConversationProfileView {
    ConversationProfileView {
        tenant_id: auth.tenant_id.clone(),
        conversation_id,
        display_name: String::new(),
        avatar_url: String::new(),
        notice: String::new(),
        updated_at: im_time::utc_now_rfc3339_millis(),
        updated_by_principal_kind: None,
        updated_by_principal_id: None,
    }
}

pub(super) async fn get_conversation_preferences(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ConversationPreferencesView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let key = conversation_preferences_key(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    let stored = state
        .conversation_preferences
        .lock()
        .expect("conversation preferences mutex should not be poisoned")
        .get(key.as_str())
        .cloned();
    Ok(Json(stored.unwrap_or_else(|| {
        default_conversation_preferences(&auth, conversation_id)
    })))
}

pub(super) async fn update_conversation_preferences(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateConversationPreferencesRequest>,
) -> Result<Json<ConversationPreferencesView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let key = conversation_preferences_key(
        auth.tenant_id.as_str(),
        conversation_id.as_str(),
        auth.actor_kind.as_str(),
        auth.actor_id.as_str(),
    );
    let view = {
        let mut preferences = state
            .conversation_preferences
            .lock()
            .expect("conversation preferences mutex should not be poisoned");
        let mut view = preferences
            .get(key.as_str())
            .cloned()
            .unwrap_or_else(|| default_conversation_preferences(&auth, conversation_id.clone()));
        if let Some(is_pinned) = request.is_pinned {
            view.is_pinned = is_pinned;
        }
        if let Some(is_muted) = request.is_muted {
            view.is_muted = is_muted;
        }
        if let Some(is_marked_unread) = request.is_marked_unread {
            view.is_marked_unread = is_marked_unread;
        }
        if let Some(is_hidden) = request.is_hidden {
            view.is_hidden = is_hidden;
        }
        view.updated_at = im_time::utc_now_rfc3339_millis();
        preferences.insert(key, view.clone());
        view
    };

    let _ = state.audit_runtime.record_anchor(
        &auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_conversation_preferences_",
                conversation_preferences_key(
                    auth.tenant_id.as_str(),
                    conversation_id.as_str(),
                    auth.actor_kind.as_str(),
                    auth.actor_id.as_str(),
                )
                .as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id,
            action: "conversation.preferences_updated".into(),
            payload: Some(
                serde_json::json!({
                    "principalKind": view.principal_kind.as_str(),
                    "principalId": view.principal_id.as_str(),
                    "isPinned": view.is_pinned,
                    "isMuted": view.is_muted,
                    "isMarkedUnread": view.is_marked_unread,
                    "isHidden": view.is_hidden,
                })
                .to_string(),
            ),
        },
    );

    Ok(Json(view))
}

fn default_conversation_preferences(
    auth: &AppContext,
    conversation_id: String,
) -> ConversationPreferencesView {
    ConversationPreferencesView {
        tenant_id: auth.tenant_id.clone(),
        conversation_id,
        principal_kind: auth.actor_kind.clone(),
        principal_id: auth.actor_id.clone(),
        is_pinned: false,
        is_muted: false,
        is_marked_unread: false,
        is_hidden: false,
        updated_at: im_time::utc_now_rfc3339_millis(),
    }
}

pub(super) async fn update_read_cursor(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<UpdateReadCursorRequest>,
) -> Result<Json<ConversationReadCursorView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    access::ensure_client_route_key(&state, &auth)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<projection_service::TimelineWindowView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_read_access(&state, &auth, conversation_id.as_str())?;
    Ok(Json(timeline_window_visible_to_message_visibility(
        &state,
        &auth,
        conversation_id.as_str(),
        query.after_seq,
        query.limit,
    )?))
}

fn timeline_window_visible_to_message_visibility(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    after_seq: Option<u64>,
    limit: Option<usize>,
) -> Result<projection_service::TimelineWindowView, ApiError> {
    let hidden_message_ids = hidden_message_ids_for_principal(state, auth, conversation_id);
    if hidden_message_ids.is_empty() {
        return Ok(state.projection_service.timeline_window_from_auth_context(
            auth,
            conversation_id,
            after_seq,
            limit,
        )?);
    }

    let requested_limit = limit.unwrap_or(projection_service::PROJECTION_TIMELINE_DEFAULT_LIMIT);
    let mut scan_after_seq = after_seq;
    let mut last_scanned_seq = None;
    let mut visible_items = Vec::with_capacity(
        requested_limit
            .saturating_add(1)
            .min(projection_service::PROJECTION_TIMELINE_MAX_LIMIT + 1),
    );
    let mut has_more_visible = false;

    loop {
        let window = state.projection_service.timeline_window_from_auth_context(
            auth,
            conversation_id,
            scan_after_seq,
            limit,
        )?;

        let mut scanned_any = false;
        for item in window.items {
            scanned_any = true;
            scan_after_seq = Some(item.message_seq);
            last_scanned_seq = Some(item.message_seq);
            if !hidden_message_ids.contains(item.message_id.as_str()) {
                visible_items.push(item);
                if visible_items.len() > requested_limit {
                    has_more_visible = true;
                    break;
                }
            }
        }

        if has_more_visible || !window.has_more || !scanned_any {
            break;
        }
    }

    if has_more_visible {
        visible_items.truncate(requested_limit);
    }

    let next_after_seq = if has_more_visible {
        visible_items.last().map(|item| item.message_seq)
    } else {
        last_scanned_seq
    };

    Ok(projection_service::TimelineWindowView {
        items: visible_items,
        next_after_seq,
        has_more: has_more_visible,
    })
}

fn hidden_message_ids_for_principal(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> BTreeSet<String> {
    state
        .message_visibility
        .lock()
        .expect("message visibility mutex should not be poisoned")
        .values()
        .filter(|view| {
            view.tenant_id == auth.tenant_id
                && view.conversation_id == conversation_id
                && view.principal_kind == auth.actor_kind
                && view.principal_id == auth.actor_id
                && view.is_deleted
        })
        .map(|view| view.message_id.clone())
        .collect()
}

pub(super) async fn get_conversation_summary(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<projection_service::ConversationSummaryView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<MemberDirectoryResponse>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<PinnedMessagesResponse>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<projection_service::MessageInteractionSummaryView>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
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
