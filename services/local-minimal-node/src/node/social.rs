use super::*;
use axum::body::Body;
use fs4::fs_std::FileExt;
use http_body_util::BodyExt;
use im_domain_core::social::{FriendRequestStatus, FriendshipStatus, normalize_user_pair};
use im_time::format_unix_timestamp_millis;
use std::fs;
use std::io::Write;
use std::sync::atomic::{AtomicU64, Ordering};
use tower::ServiceExt;

const LOCAL_SOCIAL_SYSTEM_ACTOR_ID: &str = "svc_local_social";
const SOCIAL_USER_SEARCH_DEFAULT_LIMIT: usize = 20;
const SOCIAL_USER_SEARCH_MAX_LIMIT: usize = 50;
const SOCIAL_FRIEND_REQUEST_LIST_DEFAULT_LIMIT: usize = 100;
const SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT: usize = 200;
const SOCIAL_ACCEPT_REPAIR_FILE_NAME: &str = "social-friend-request-accept-repairs.json";
const SOCIAL_ACCEPT_REPAIR_STORE_LOCK_FILE_NAME: &str = "social-friend-request-accept-repairs.lock";
const SOCIAL_ACCEPT_REPAIR_RUN_LOCK_FILE_NAME: &str =
    "social-friend-request-accept-repairs.run.lock";
const SOCIAL_ACCEPT_REPAIR_LOCK_POLL_INTERVAL_MS: u64 = 10;
#[cfg(debug_assertions)]
const SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE_ENV: &str =
    "SDKWORK_IM_TEST_SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE";
static NEXT_SOCIAL_ID_SEQUENCE: AtomicU64 = AtomicU64::new(0);

struct PendingFriendRequestAcceptRepairLockGuard {
    file: fs::File,
}

impl Drop for PendingFriendRequestAcceptRepairLockGuard {
    fn drop(&mut self) {
        if let Err(error) = self.file.unlock() {
            tracing::warn!("failed to unlock pending friend request accept repair lock: {error}");
        }
    }
}

pub(super) async fn list_social_users(
    Query(query): Query<SocialUserSearchQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialUserSearchResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;

    let q = query.q.as_deref().map(str::trim).unwrap_or_default();
    let limit = query.limit.unwrap_or(SOCIAL_USER_SEARCH_DEFAULT_LIMIT);
    if limit == 0 || limit > SOCIAL_USER_SEARCH_MAX_LIMIT {
        return Err(ApiError::bad_request(
            "limit_invalid",
            format!("limit must be between 1 and {SOCIAL_USER_SEARCH_MAX_LIMIT}"),
        ));
    }
    let cursor_offset = parse_social_offset_cursor(query.cursor.as_deref())?;
    if q.is_empty() {
        return Ok(Json(SocialUserSearchResponse {
            items: Vec::new(),
            next_cursor: None,
            has_more: false,
        }));
    }

    let public_chat_id_query = q.to_ascii_lowercase();
    let mut by_user_id = BTreeMap::new();
    let current_user_chat_id =
        principal_profile::public_chat_id_for_user(auth.tenant_id.as_str(), auth.actor_id.as_str());
    if q == auth.actor_id || public_chat_id_query == current_user_chat_id {
        let profile = principal_profile::ensure_active_user(
            &state,
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
        )?;
        by_user_id.insert(
            auth.actor_id.clone(),
            social_user_search_result_from_profile(&state, &auth, profile),
        );
    }

    for profile in state
        .principal_profile_provider
        .search_profiles(auth.tenant_id.as_str(), "user", q)?
        .into_iter()
        .filter(|profile| !profile.inactive)
    {
        by_user_id.insert(
            profile.principal_id.clone(),
            social_user_search_result_from_profile(&state, &auth, profile),
        );
    }

    for contact in state
        .projection_service
        .contacts(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .filter(|contact| {
            contact_matches_social_user_query(contact.target_user_id.as_str(), q)
                || principal_profile::public_chat_id_for_user(
                    auth.tenant_id.as_str(),
                    contact.target_user_id.as_str(),
                ) == public_chat_id_query
        })
    {
        if contact.target_user_id == auth.actor_id {
            continue;
        }
        let profile = principal_profile::ensure_active_user(
            &state,
            auth.tenant_id.as_str(),
            contact.target_user_id.as_str(),
        )?;
        by_user_id.insert(
            contact.target_user_id.clone(),
            social_user_search_result_from_profile(&state, &auth, profile),
        );
    }

    if principal_profile::is_public_chat_id_query(q) {
        for profile in state
            .principal_profile_provider
            .search_profiles(auth.tenant_id.as_str(), "user", "")?
            .into_iter()
            .filter(|profile| !profile.inactive)
            .filter(|profile| {
                principal_profile::public_chat_id_for_profile(profile) == public_chat_id_query
            })
        {
            by_user_id.insert(
                profile.principal_id.clone(),
                social_user_search_result_from_profile(&state, &auth, profile),
            );
        }
    }

    let mut items = by_user_id.into_values().collect::<Vec<_>>();
    items.sort_by(|left, right| {
        left.display_name
            .cmp(&right.display_name)
            .then_with(|| left.user_id.cmp(&right.user_id))
    });
    let has_more = cursor_offset.saturating_add(limit) < items.len();
    let page_items = items
        .into_iter()
        .skip(cursor_offset)
        .take(limit)
        .collect::<Vec<_>>();
    let next_cursor = if has_more {
        Some((cursor_offset + page_items.len()).to_string())
    } else {
        None
    };

    Ok(Json(SocialUserSearchResponse {
        items: page_items,
        next_cursor,
        has_more,
    }))
}

pub(super) async fn list_friend_requests(
    Query(query): Query<ListFriendRequestsAppQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestListResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;
    let limit = query
        .limit
        .unwrap_or(SOCIAL_FRIEND_REQUEST_LIST_DEFAULT_LIMIT);
    if limit == 0 || limit > SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT {
        return Err(ApiError::bad_request(
            "limit_invalid",
            format!("limit must be between 1 and {SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT}"),
        ));
    }

    let response = dispatch_control_plane_json(
        &state,
        &auth,
        "GET",
        build_friend_request_inventory_control_plane_uri(
            auth.actor_id.as_str(),
            query.direction,
            query.status,
            limit,
            query.cursor.as_deref(),
        ),
        "control.read",
        None,
    )
    .await?;

    Ok(Json(SocialFriendRequestListResponse {
        items: parse_json_field(&response, "items")?,
        next_cursor: optional_json_string_field(&response, "nextCursor"),
    }))
}

pub(super) async fn submit_friend_request(
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<SubmitFriendRequestAppRequest>,
) -> Result<Json<SocialFriendRequestMutationResponse>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    if request.target_user_id == auth.actor_id {
        return Err(ApiError::bad_request(
            "friend_request_self_not_allowed",
            "cannot send a friend request to yourself",
        ));
    }
    maybe_repair_pending_friend_request_acceptances(&state).await?;
    ensure_social_friend_request_users_active(
        &state,
        auth.tenant_id.as_str(),
        auth.actor_id.as_str(),
        request.target_user_id.as_str(),
    )?;

    let requested_at = current_social_timestamp();
    let request_id = if let Some(existing_request_id) = existing_pending_friend_request_id_for_pair(
        &state,
        &auth,
        auth.actor_id.as_str(),
        request.target_user_id.as_str(),
    )
    .await?
    {
        existing_request_id
    } else {
        unique_friend_request_id_for_pair(
            auth.tenant_id.as_str(),
            auth.actor_id.as_str(),
            request.target_user_id.as_str(),
        )?
    };
    let event_id = deterministic_social_id(
        "evt_fr_submit_",
        format!(
            "{}:{}:{}",
            request_id,
            auth.actor_id,
            current_unix_epoch_nanos()
        )
        .as_str(),
    );
    let (response_status, response_value) = dispatch_control_plane_json_response(
        &state,
        &auth,
        "POST",
        "/backend/v3/api/control/social/friend_requests".into(),
        "control.write",
        Some(serde_json::json!({
            "requestId": request_id,
            "eventId": event_id,
            "requesterUserId": auth.actor_id,
            "targetUserId": request.target_user_id,
            "requestMessage": request.request_message,
            "requestedAt": requested_at,
        })),
    )
    .await?;
    if response_status.is_success() {
        let friend_request: FriendRequest = parse_json_field(&response_value, "friendRequest")?;
        publish_friend_request_realtime_event(
            &state,
            &auth,
            "friend_request.submitted",
            &friend_request,
        );
        return Ok(Json(SocialFriendRequestMutationResponse { friend_request }));
    }
    if response_status == axum::http::StatusCode::CONFLICT
        && let Some(existing_request_id) = control_plane_existing_request_id(&response_value)
    {
        let snapshot = dispatch_control_plane_json(
            &state,
            &auth,
            "GET",
            format!("/backend/v3/api/control/social/friend_requests/{existing_request_id}"),
            "control.read",
            None,
        )
        .await?;
        let friend_request: FriendRequest = parse_json_field(&snapshot, "friendRequest")?;
        publish_friend_request_realtime_event(
            &state,
            &auth,
            "friend_request.submitted",
            &friend_request,
        );
        return Ok(Json(SocialFriendRequestMutationResponse { friend_request }));
    }
    if response_status == axum::http::StatusCode::CONFLICT
        && let Some(existing_friendship_id) = control_plane_existing_friendship_id(&response_value)
    {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friendship_already_active",
            message: format!(
                "active friendship already exists for this pair: {existing_friendship_id}"
            ),
        });
    }

    Err(api_error_from_control_plane_response(
        response_status,
        &response_value,
    ))
}

pub(super) async fn accept_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestAcceptanceResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;

    let snapshot = dispatch_control_plane_json(
        &state,
        &auth,
        "GET",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}"),
        "control.read",
        None,
    )
    .await?;
    let friend_request: FriendRequest = parse_json_field(&snapshot, "friendRequest")?;
    if friend_request.target_user_id != auth.actor_id {
        return Err(ApiError {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "friend_request_accept_forbidden",
            message: format!(
                "principal cannot accept friend request targeted to {}",
                friend_request.target_user_id
            ),
        });
    }
    if !matches!(
        friend_request.status,
        FriendRequestStatus::Pending | FriendRequestStatus::Accepted
    ) {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friend_request_not_pending",
            message: format!("friend request is not pending: {request_id}"),
        });
    }
    ensure_social_friend_request_users_active(
        &state,
        auth.tenant_id.as_str(),
        friend_request.requester_user_id.as_str(),
        friend_request.target_user_id.as_str(),
    )?;

    let mut accepted_at = if friend_request.status == FriendRequestStatus::Accepted {
        friend_request.updated_at.clone()
    } else {
        current_social_timestamp()
    };
    let acceptance_repair = PendingFriendRequestAcceptanceRepair {
        tenant_id: auth.tenant_id.clone(),
        request_id: request_id.clone(),
        requester_user_id: friend_request.requester_user_id.clone(),
        target_user_id: friend_request.target_user_id.clone(),
        accepted_at: accepted_at.clone(),
    };
    register_pending_friend_request_accept_repair(&state, &acceptance_repair).await?;
    let mut accepted_request_response = None;
    let accepted_friend_request = if friend_request.status == FriendRequestStatus::Accepted {
        friend_request.clone()
    } else {
        maybe_pause_friend_request_accept_before_request_commit().await;
        match dispatch_control_plane_json(
            &state,
            &auth,
            "POST",
            format!("/backend/v3/api/control/social/friend_requests/{request_id}/accept"),
            "control.write",
            Some(serde_json::json!({
                "eventId": deterministic_social_id("evt_fr_accept_", request_id.as_str()),
                "acceptedByUserId": auth.actor_id,
                "acceptedAt": accepted_at,
            })),
        )
        .await
        {
            Ok(response_value) => {
                apply_control_plane_latest_commit(&state, &response_value)?;
                apply_optional_control_plane_commit_field(
                    &state,
                    &response_value,
                    "friendshipLatestCommit",
                )?;
                apply_optional_control_plane_commit_field(
                    &state,
                    &response_value,
                    "directChatLatestCommit",
                )?;
                accepted_request_response = Some(response_value.clone());
                parse_json_field(&response_value, "friendRequest")?
            }
            Err(error) => {
                if is_converged_friend_request_mutation_conflict(error.code)
                    && let Some(latest_friend_request) =
                        reconcile_accept_friend_request_after_not_pending(
                            &state,
                            &auth,
                            request_id.as_str(),
                            &acceptance_repair,
                        )
                        .await?
                {
                    accepted_at = latest_friend_request.updated_at.clone();
                    if acceptance_repair.accepted_at != accepted_at {
                        let mut updated_repair = acceptance_repair.clone();
                        updated_repair.accepted_at = accepted_at.clone();
                        register_pending_friend_request_accept_repair(&state, &updated_repair)
                            .await?;
                    }
                    latest_friend_request
                } else {
                    return Err(error);
                }
            }
        }
    };

    maybe_pause_friend_request_accept_after_request_commit().await;

    let friendship_id = deterministic_social_id("fs_", request_id.as_str());
    let direct_chat_id = deterministic_social_id("dc_", request_id.as_str());
    let conversation_id = deterministic_social_id("c_direct_", request_id.as_str());

    let friendship = match accepted_request_response
        .as_ref()
        .map(|response| parse_optional_json_field(response, "friendship"))
        .transpose()?
        .flatten()
    {
        Some(friendship) => {
            ensure_friendship_matches_acceptance(&accepted_friend_request, &friendship)?;
            friendship
        }
        None => {
            ensure_friendship_for_acceptance(
                &state,
                &auth,
                &accepted_friend_request,
                request_id.as_str(),
                friendship_id.as_str(),
                direct_chat_id.as_str(),
                accepted_at.as_str(),
            )
            .await?
        }
    };

    let direct_chat = match accepted_request_response
        .as_ref()
        .map(|response| parse_optional_json_field(response, "directChat"))
        .transpose()?
        .flatten()
    {
        Some(direct_chat) => {
            ensure_direct_chat_matches_acceptance(&accepted_friend_request, &direct_chat)?;
            direct_chat
        }
        None => {
            ensure_direct_chat_for_acceptance(
                &state,
                &auth,
                &accepted_friend_request,
                request_id.as_str(),
                direct_chat_id.as_str(),
                conversation_id.as_str(),
                accepted_at.as_str(),
            )
            .await?
        }
    };
    let bound_conversation_id = direct_chat.conversation_id.clone().ok_or(ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_invalid_field",
        message: format!(
            "direct chat {} is missing a conversation binding after friend request acceptance",
            direct_chat.direct_chat_id
        ),
    })?;
    let conversation = state.conversation_runtime.bind_direct_chat_conversation(
        BindDirectChatConversationCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: bound_conversation_id,
            direct_chat_id: direct_chat.direct_chat_id.clone(),
            left_actor_id: accepted_friend_request.requester_user_id.clone(),
            left_actor_kind: "user".into(),
            right_actor_id: accepted_friend_request.target_user_id.clone(),
            right_actor_kind: "user".into(),
            bound_by: LOCAL_SOCIAL_SYSTEM_ACTOR_ID.into(),
        },
    )?;

    try_clear_pending_friend_request_accept_repair(&state, request_id.as_str()).await;
    publish_friend_request_realtime_event(
        &state,
        &auth,
        "friend_request.accepted",
        &accepted_friend_request,
    );

    Ok(Json(SocialFriendRequestAcceptanceResponse {
        friend_request: accepted_friend_request,
        friendship,
        direct_chat,
        conversation,
    }))
}

pub(super) async fn decline_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;

    let snapshot = dispatch_control_plane_json(
        &state,
        &auth,
        "GET",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}"),
        "control.read",
        None,
    )
    .await?;
    let friend_request: FriendRequest = parse_json_field(&snapshot, "friendRequest")?;
    if friend_request.target_user_id != auth.actor_id {
        return Err(ApiError {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "friend_request_decline_forbidden",
            message: format!(
                "principal cannot decline friend request targeted to {}",
                friend_request.target_user_id
            ),
        });
    }
    if friend_request.status == FriendRequestStatus::Declined {
        return Ok(Json(SocialFriendRequestMutationResponse { friend_request }));
    }
    if friend_request.status != FriendRequestStatus::Pending {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friend_request_not_pending",
            message: format!("friend request is not pending: {request_id}"),
        });
    }

    let declined_at = current_social_timestamp();
    maybe_pause_friend_request_decline_before_request_commit().await;
    let response = match dispatch_control_plane_json(
        &state,
        &auth,
        "POST",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}/decline"),
        "control.write",
        Some(serde_json::json!({
            "eventId": deterministic_social_id("evt_fr_decline_", request_id.as_str()),
            "declinedByUserId": auth.actor_id,
            "declinedAt": declined_at,
        })),
    )
    .await
    {
        Ok(response) => response,
        Err(error) => {
            if error.code == "friend_request_not_pending"
                && let Some(latest_friend_request) =
                    reconcile_friend_request_mutation_after_not_pending(
                        &state,
                        &auth,
                        request_id.as_str(),
                        FriendRequestStatus::Declined,
                    )
                    .await?
            {
                publish_friend_request_realtime_event(
                    &state,
                    &auth,
                    "friend_request.declined",
                    &latest_friend_request,
                );
                return Ok(Json(SocialFriendRequestMutationResponse {
                    friend_request: latest_friend_request,
                }));
            }
            return Err(error);
        }
    };
    let declined_friend_request: FriendRequest = parse_json_field(&response, "friendRequest")?;
    publish_friend_request_realtime_event(
        &state,
        &auth,
        "friend_request.declined",
        &declined_friend_request,
    );

    Ok(Json(SocialFriendRequestMutationResponse {
        friend_request: declined_friend_request,
    }))
}

pub(super) async fn cancel_friend_request(
    Path(request_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendRequestMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;

    let snapshot = dispatch_control_plane_json(
        &state,
        &auth,
        "GET",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}"),
        "control.read",
        None,
    )
    .await?;
    let friend_request: FriendRequest = parse_json_field(&snapshot, "friendRequest")?;
    if friend_request.requester_user_id != auth.actor_id {
        return Err(ApiError {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "friend_request_cancel_forbidden",
            message: format!(
                "principal cannot cancel friend request requested by {}",
                friend_request.requester_user_id
            ),
        });
    }
    if friend_request.status == FriendRequestStatus::Canceled {
        return Ok(Json(SocialFriendRequestMutationResponse { friend_request }));
    }
    if friend_request.status != FriendRequestStatus::Pending {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friend_request_not_pending",
            message: format!("friend request is not pending: {request_id}"),
        });
    }

    let canceled_at = current_social_timestamp();
    maybe_pause_friend_request_cancel_before_request_commit().await;
    let response = match dispatch_control_plane_json(
        &state,
        &auth,
        "POST",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}/cancel"),
        "control.write",
        Some(serde_json::json!({
            "eventId": deterministic_social_id("evt_fr_cancel_", request_id.as_str()),
            "canceledByUserId": auth.actor_id,
            "canceledAt": canceled_at,
        })),
    )
    .await
    {
        Ok(response) => response,
        Err(error) => {
            if error.code == "friend_request_not_pending"
                && let Some(latest_friend_request) =
                    reconcile_friend_request_mutation_after_not_pending(
                        &state,
                        &auth,
                        request_id.as_str(),
                        FriendRequestStatus::Canceled,
                    )
                    .await?
            {
                publish_friend_request_realtime_event(
                    &state,
                    &auth,
                    "friend_request.canceled",
                    &latest_friend_request,
                );
                return Ok(Json(SocialFriendRequestMutationResponse {
                    friend_request: latest_friend_request,
                }));
            }
            return Err(error);
        }
    };
    let canceled_friend_request: FriendRequest = parse_json_field(&response, "friendRequest")?;
    publish_friend_request_realtime_event(
        &state,
        &auth,
        "friend_request.canceled",
        &canceled_friend_request,
    );

    Ok(Json(SocialFriendRequestMutationResponse {
        friend_request: canceled_friend_request,
    }))
}

pub(super) async fn remove_friendship(
    Path(friendship_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<SocialFriendshipMutationResponse>, ApiError> {
    let auth = access::resolve_active_auth_context(&state, auth, &headers)?;
    ensure_social_user_actor(&auth)?;
    maybe_repair_pending_friend_request_acceptances(&state).await?;

    let snapshot = dispatch_control_plane_json(
        &state,
        &auth,
        "GET",
        format!("/backend/v3/api/control/social/friendships/{friendship_id}"),
        "control.read",
        None,
    )
    .await?;
    let friendship: Friendship = parse_json_field(&snapshot, "friendship")?;
    if friendship.user_low_id != auth.actor_id && friendship.user_high_id != auth.actor_id {
        return Err(ApiError {
            status: axum::http::StatusCode::FORBIDDEN,
            code: "friendship_remove_forbidden",
            message: format!("principal cannot remove friendship: {friendship_id}"),
        });
    }
    if friendship.status == FriendshipStatus::Removed {
        return Ok(Json(SocialFriendshipMutationResponse { friendship }));
    }

    let removed_at = current_social_timestamp();
    maybe_pause_friendship_remove_before_request_commit().await;
    let (removed_friendship, latest_commit_response) = match dispatch_control_plane_json(
        &state,
        &auth,
        "POST",
        format!("/backend/v3/api/control/social/friendships/{friendship_id}/remove"),
        "control.write",
        Some(serde_json::json!({
            "eventId": deterministic_social_id("evt_fs_remove_", friendship_id.as_str()),
            "removedByUserId": auth.actor_id,
            "removedAt": removed_at,
        })),
    )
    .await
    {
        Ok(response) => (parse_json_field(&response, "friendship")?, Some(response)),
        Err(error) => {
            if error.code == "friendship_not_active"
                && let Some(latest_friendship) = reconcile_friendship_remove_after_not_active(
                    &state,
                    &auth,
                    friendship_id.as_str(),
                )
                .await?
            {
                return Ok(Json(SocialFriendshipMutationResponse {
                    friendship: latest_friendship,
                }));
            }
            return Err(error);
        }
    };
    if let Some(response) = latest_commit_response.as_ref() {
        apply_control_plane_latest_commit(&state, response)?;
    }

    Ok(Json(SocialFriendshipMutationResponse {
        friendship: removed_friendship,
    }))
}

async fn ensure_friendship_for_acceptance(
    state: &AppState,
    auth: &AppContext,
    friend_request: &FriendRequest,
    request_id: &str,
    friendship_id: &str,
    direct_chat_id: &str,
    accepted_at: &str,
) -> Result<Friendship, ApiError> {
    let response = match dispatch_control_plane_json_response(
        state,
        auth,
        "POST",
        "/backend/v3/api/control/social/friendships".into(),
        "control.write",
        Some(serde_json::json!({
            "friendshipId": friendship_id,
            "eventId": deterministic_social_id("evt_fs_activate_", request_id),
            "initiatorUserId": friend_request.requester_user_id,
            "peerUserId": friend_request.target_user_id,
            "directChatId": direct_chat_id,
            "establishedAt": accepted_at,
        })),
    )
    .await
    {
        Ok((status, response)) if status.is_success() => {
            apply_control_plane_latest_commit(state, &response)?;
            response
        }
        Ok((status, response)) if status == axum::http::StatusCode::CONFLICT => {
            if matches!(
                control_plane_error_code(&response),
                Some("friendship_blocked")
            ) {
                return Err(api_error_from_control_plane_response(status, &response));
            }
            let snapshot_friendship_id =
                control_plane_existing_friendship_id(&response).unwrap_or(friendship_id);
            let snapshot = dispatch_control_plane_json(
                state,
                auth,
                "GET",
                format!("/backend/v3/api/control/social/friendships/{snapshot_friendship_id}"),
                "control.read",
                None,
            )
            .await?;
            apply_control_plane_snapshot_latest_commit(state, &snapshot)?;
            snapshot
        }
        Ok((status, response)) => {
            return Err(api_error_from_control_plane_response(status, &response));
        }
        Err(error) => return Err(error),
    };

    let friendship: Friendship = parse_json_field(&response, "friendship")?;
    ensure_friendship_matches_acceptance(friend_request, &friendship)?;
    Ok(friendship)
}

async fn ensure_direct_chat_for_acceptance(
    state: &AppState,
    auth: &AppContext,
    friend_request: &FriendRequest,
    request_id: &str,
    direct_chat_id: &str,
    conversation_id: &str,
    accepted_at: &str,
) -> Result<DirectChat, ApiError> {
    let response = match dispatch_control_plane_json_response(
        state,
        auth,
        "POST",
        "/backend/v3/api/control/social/direct_chats/bindings".into(),
        "control.write",
        Some(serde_json::json!({
            "directChatId": direct_chat_id,
            "eventId": deterministic_social_id("evt_dc_bind_", request_id),
            "leftActorId": friend_request.requester_user_id,
            "rightActorId": friend_request.target_user_id,
            "conversationId": conversation_id,
            "boundAt": accepted_at,
        })),
    )
    .await
    {
        Ok((status, response)) if status.is_success() => {
            apply_control_plane_latest_commit(state, &response)?;
            response
        }
        Ok((status, response)) if status == axum::http::StatusCode::CONFLICT => {
            let snapshot_direct_chat_id =
                control_plane_existing_direct_chat_id(&response).unwrap_or(direct_chat_id);
            let snapshot = dispatch_control_plane_json(
                state,
                auth,
                "GET",
                format!("/backend/v3/api/control/social/direct_chats/{snapshot_direct_chat_id}"),
                "control.read",
                None,
            )
            .await?;
            apply_control_plane_snapshot_latest_commit(state, &snapshot)?;
            snapshot
        }
        Ok((status, response)) => {
            return Err(api_error_from_control_plane_response(status, &response));
        }
        Err(error) => return Err(error),
    };

    let direct_chat: DirectChat = parse_json_field(&response, "directChat")?;
    ensure_direct_chat_matches_acceptance(friend_request, &direct_chat)?;
    Ok(direct_chat)
}

fn ensure_friendship_matches_acceptance(
    friend_request: &FriendRequest,
    friendship: &Friendship,
) -> Result<(), ApiError> {
    if !friendship.status.is_active() {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friendship_not_active",
            message: format!(
                "friendship {} is not active for request {}",
                friendship.friendship_id, friend_request.request_id
            ),
        });
    }
    let participants_match = (friendship.user_low_id == friend_request.requester_user_id
        && friendship.user_high_id == friend_request.target_user_id)
        || (friendship.user_low_id == friend_request.target_user_id
            && friendship.user_high_id == friend_request.requester_user_id);
    if !participants_match {
        return Err(ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!(
                "friendship {} does not match request pair {} -> {}",
                friendship.friendship_id,
                friend_request.requester_user_id,
                friend_request.target_user_id
            ),
        });
    }

    Ok(())
}

fn ensure_direct_chat_matches_acceptance(
    friend_request: &FriendRequest,
    direct_chat: &DirectChat,
) -> Result<(), ApiError> {
    if !direct_chat.status.is_active() {
        return Err(ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "direct_chat_not_active",
            message: format!(
                "direct chat {} is not active for request {}",
                direct_chat.direct_chat_id, friend_request.request_id
            ),
        });
    }
    let participants_match = (direct_chat.left_actor_id == friend_request.requester_user_id
        && direct_chat.right_actor_id == friend_request.target_user_id)
        || (direct_chat.left_actor_id == friend_request.target_user_id
            && direct_chat.right_actor_id == friend_request.requester_user_id);
    if !participants_match {
        return Err(ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!(
                "direct chat {} does not match request pair {} -> {}",
                direct_chat.direct_chat_id,
                friend_request.requester_user_id,
                friend_request.target_user_id
            ),
        });
    }
    if direct_chat
        .conversation_id
        .as_deref()
        .is_none_or(|conversation_id| conversation_id.trim().is_empty())
    {
        return Err(ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!(
                "direct chat {} is missing a bound conversation for request {}",
                direct_chat.direct_chat_id, friend_request.request_id
            ),
        });
    }

    Ok(())
}

fn publish_friend_request_realtime_event(
    state: &AppState,
    auth: &AppContext,
    event_type: &'static str,
    friend_request: &FriendRequest,
) {
    let recipients = BTreeSet::from([
        NotificationRecipientView {
            principal_id: friend_request.requester_user_id.clone(),
            principal_kind: "user".into(),
        },
        NotificationRecipientView {
            principal_id: friend_request.target_user_id.clone(),
            principal_kind: "user".into(),
        },
    ]);
    let payload = serde_json::json!({
        "friendRequest": friend_request,
        "requestId": friend_request.request_id,
        "requesterUserId": friend_request.requester_user_id,
        "targetUserId": friend_request.target_user_id,
        "status": friend_request.status,
        "occurredAt": im_time::utc_now_rfc3339_millis(),
    })
    .to_string();

    if let Err(error) = effects::publish_realtime_event_to_principals(
        state, auth, recipients, "user", event_type, payload,
    ) {
        record_friend_request_realtime_failure(state, auth, event_type, friend_request, &error);
    }
}

fn record_friend_request_realtime_failure(
    state: &AppState,
    auth: &AppContext,
    event_type: &str,
    friend_request: &FriendRequest,
    error: &ApiError,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_friend_request_realtime_failed_",
                format!("{}:{event_type}", friend_request.request_id).as_str(),
            ),
            aggregate_type: "friend_request".into(),
            aggregate_id: friend_request.request_id.clone(),
            action: "friend_request.realtime_failed".into(),
            payload: Some(
                serde_json::json!({
                    "eventType": event_type,
                    "requestId": friend_request.request_id,
                    "requesterUserId": friend_request.requester_user_id,
                    "targetUserId": friend_request.target_user_id,
                    "errorCode": error.code,
                    "errorMessage": error.message,
                })
                .to_string(),
            ),
        },
    );
}

pub(super) fn load_pending_friend_request_accept_repairs(
    runtime_dir: Option<&StdPath>,
) -> PendingFriendRequestAcceptanceRepairStore {
    if let Err(error) = recover_pending_friend_request_accept_repairs_temp_file(runtime_dir) {
        tracing::warn!(
            "failed to recover pending friend request accept repair store temp file: {}",
            error.message
        );
    }
    let Some(path) = pending_friend_request_accept_repairs_path(runtime_dir) else {
        return PendingFriendRequestAcceptanceRepairStore::default();
    };
    let Ok(content) = fs::read_to_string(path.as_path()) else {
        return PendingFriendRequestAcceptanceRepairStore::default();
    };
    maybe_pause_pending_friend_request_accept_repair_store_io();
    serde_json::from_str(&content).unwrap_or_else(|error| {
        tracing::warn!(
            "failed to parse pending friend request accept repair store {}: {error}. starting with empty repair store",
            path.display()
        );
        PendingFriendRequestAcceptanceRepairStore::default()
    })
}

pub(super) fn spawn_pending_friend_request_accept_repair(state: AppState) {
    if pending_friend_request_accept_repairs_snapshot(&state).is_empty() {
        return;
    }
    let Ok(handle) = tokio::runtime::Handle::try_current() else {
        return;
    };
    handle.spawn(async move {
        if let Err(error) = maybe_repair_pending_friend_request_acceptances(&state).await {
            tracing::warn!(
                "failed to repair pending friend request acceptances during startup: {}",
                error.message
            );
        }
    });
}

pub(super) async fn maybe_repair_pending_friend_request_acceptances(
    state: &AppState,
) -> Result<(), ApiError> {
    let _local_gate = state.friend_request_accept_repair_gate.lock().await;
    let _run_lock =
        acquire_pending_friend_request_accept_repair_run_lock(state.runtime_dir.as_deref()).await?;
    let repairs = refresh_pending_friend_request_accept_repairs_from_authority(state).await?;

    for repair in repairs.into_values() {
        repair_pending_friend_request_acceptance(state, &repair).await?;
    }

    Ok(())
}

fn pending_friend_request_accept_repairs_snapshot(
    state: &AppState,
) -> PendingFriendRequestAcceptanceRepairStore {
    state
        .pending_friend_request_accept_repairs
        .lock()
        .unwrap_or_else(|poisoned| {
            tracing::warn!("recovering poisoned pending friend request accept repair store lock");
            poisoned.into_inner()
        })
        .clone()
}

async fn register_pending_friend_request_accept_repair(
    state: &AppState,
    repair: &PendingFriendRequestAcceptanceRepair,
) -> Result<(), ApiError> {
    let repair = repair.clone();
    update_pending_friend_request_accept_repairs(state, move |next| {
        next.insert(repair.request_id.clone(), repair.clone());
    })
    .await
}

async fn clear_pending_friend_request_accept_repair(
    state: &AppState,
    request_id: &str,
) -> Result<(), ApiError> {
    let request_id = request_id.to_owned();
    update_pending_friend_request_accept_repairs(state, move |next| {
        next.remove(request_id.as_str());
    })
    .await
}

async fn try_clear_pending_friend_request_accept_repair(state: &AppState, request_id: &str) {
    if let Err(error) = clear_pending_friend_request_accept_repair(state, request_id).await {
        tracing::warn!(
            "failed to clear pending friend request accept repair entry {request_id}: {}",
            error.message
        );
    }
}

async fn update_pending_friend_request_accept_repairs(
    state: &AppState,
    apply: impl FnOnce(&mut PendingFriendRequestAcceptanceRepairStore) + Send + 'static,
) -> Result<(), ApiError> {
    let mut next = if let Some(runtime_dir) = state.runtime_dir.as_deref() {
        let runtime_dir = runtime_dir.to_path_buf();
        let write_lock = acquire_pending_friend_request_accept_repairs_lock(
            pending_friend_request_accept_repairs_lock_path(Some(runtime_dir.as_path())),
            false,
            "write",
        )
        .await?;
        run_pending_friend_request_accept_repair_blocking_io("update", move || {
            let _write_lock = write_lock;
            let mut next = load_pending_friend_request_accept_repairs(Some(runtime_dir.as_path()));
            apply(&mut next);
            persist_pending_friend_request_accept_repairs(Some(runtime_dir.as_path()), &next)?;
            Ok(next)
        })
        .await?
    } else {
        let mut next = pending_friend_request_accept_repairs_snapshot(state);
        apply(&mut next);
        persist_pending_friend_request_accept_repairs(None, &next)?;
        next
    };
    let mut guard = state
        .pending_friend_request_accept_repairs
        .lock()
        .unwrap_or_else(|poisoned| {
            tracing::warn!("recovering poisoned pending friend request accept repair store lock");
            poisoned.into_inner()
        });
    if state.runtime_dir.is_none() {
        *guard = next;
        return Ok(());
    }

    *guard = std::mem::take(&mut next);
    Ok(())
}

async fn refresh_pending_friend_request_accept_repairs_from_authority(
    state: &AppState,
) -> Result<PendingFriendRequestAcceptanceRepairStore, ApiError> {
    let authoritative = if let Some(runtime_dir) = state.runtime_dir.as_deref() {
        let runtime_dir = runtime_dir.to_path_buf();
        let read_lock = acquire_pending_friend_request_accept_repairs_lock(
            pending_friend_request_accept_repairs_lock_path(Some(runtime_dir.as_path())),
            true,
            "read",
        )
        .await?;
        run_pending_friend_request_accept_repair_blocking_io("refresh", move || {
            let _read_lock = read_lock;
            Ok(load_pending_friend_request_accept_repairs(Some(
                runtime_dir.as_path(),
            )))
        })
        .await?
    } else {
        pending_friend_request_accept_repairs_snapshot(state)
    };
    *state
        .pending_friend_request_accept_repairs
        .lock()
        .unwrap_or_else(|poisoned| {
            tracing::warn!("recovering poisoned pending friend request accept repair store lock");
            poisoned.into_inner()
        }) = authoritative.clone();
    Ok(authoritative)
}

async fn acquire_pending_friend_request_accept_repair_run_lock(
    runtime_dir: Option<&StdPath>,
) -> Result<Option<PendingFriendRequestAcceptRepairLockGuard>, ApiError> {
    acquire_pending_friend_request_accept_repairs_lock(
        pending_friend_request_accept_repair_run_lock_path(runtime_dir),
        false,
        "run",
    )
    .await
}

async fn acquire_pending_friend_request_accept_repairs_lock(
    path: Option<PathBuf>,
    shared: bool,
    lock_kind: &str,
) -> Result<Option<PendingFriendRequestAcceptRepairLockGuard>, ApiError> {
    let Some(path) = path else {
        return Ok(None);
    };
    loop {
        let attempt_path = path.clone();
        let lock_kind = lock_kind.to_owned();
        if let Some(guard) =
            run_pending_friend_request_accept_repair_blocking_io("lock", move || {
                try_acquire_pending_friend_request_accept_repairs_lock_once(
                    attempt_path,
                    shared,
                    lock_kind.as_str(),
                )
            })
            .await?
        {
            return Ok(Some(guard));
        }
        tokio::time::sleep(std::time::Duration::from_millis(
            SOCIAL_ACCEPT_REPAIR_LOCK_POLL_INTERVAL_MS,
        ))
        .await;
    }
}

async fn run_pending_friend_request_accept_repair_blocking_io<T>(
    operation_kind: &'static str,
    operation: impl FnOnce() -> Result<T, ApiError> + Send + 'static,
) -> Result<T, ApiError>
where
    T: Send + 'static,
{
    tokio::task::spawn_blocking(operation)
        .await
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: format!(
                "pending friend request accept repair {operation_kind} task failed to join: {error}"
            ),
        })?
}

fn try_acquire_pending_friend_request_accept_repairs_lock_once(
    path: PathBuf,
    shared: bool,
    lock_kind: &str,
) -> Result<Option<PendingFriendRequestAcceptRepairLockGuard>, ApiError> {
    let Some(parent) = path.parent() else {
        return Ok(None);
    };
    fs::create_dir_all(parent).map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to create pending friend request accept repair lock dir {}: {error}",
            parent.display()
        ),
    })?;
    let file = fs::OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(path.as_path())
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: format!(
                "failed to open pending friend request accept repair {lock_kind} lock {}: {error}",
                path.display()
            ),
        })?;
    let acquired = if shared {
        FileExt::try_lock_shared(&file)
    } else {
        FileExt::try_lock_exclusive(&file)
    }
    .map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to acquire pending friend request accept repair {lock_kind} lock {}: {error}",
            path.display()
        ),
    })?;
    if !acquired {
        return Ok(None);
    }
    Ok(Some(PendingFriendRequestAcceptRepairLockGuard { file }))
}

fn pending_friend_request_accept_repairs_lock_path(
    runtime_dir: Option<&StdPath>,
) -> Option<PathBuf> {
    runtime_dir.map(|runtime_dir| {
        runtime_dir
            .join("state")
            .join(SOCIAL_ACCEPT_REPAIR_STORE_LOCK_FILE_NAME)
    })
}

fn pending_friend_request_accept_repair_run_lock_path(
    runtime_dir: Option<&StdPath>,
) -> Option<PathBuf> {
    runtime_dir.map(|runtime_dir| {
        runtime_dir
            .join("state")
            .join(SOCIAL_ACCEPT_REPAIR_RUN_LOCK_FILE_NAME)
    })
}

fn pending_friend_request_accept_repairs_path(runtime_dir: Option<&StdPath>) -> Option<PathBuf> {
    runtime_dir.map(|runtime_dir| {
        runtime_dir
            .join("state")
            .join(SOCIAL_ACCEPT_REPAIR_FILE_NAME)
    })
}

fn pending_friend_request_accept_repairs_temp_path(
    runtime_dir: Option<&StdPath>,
) -> Option<PathBuf> {
    pending_friend_request_accept_repairs_path(runtime_dir)
        .map(|path| path.with_extension("json.tmp"))
}

fn recover_pending_friend_request_accept_repairs_temp_file(
    runtime_dir: Option<&StdPath>,
) -> Result<(), ApiError> {
    let Some(path) = pending_friend_request_accept_repairs_path(runtime_dir) else {
        return Ok(());
    };
    let Some(temp_path) = pending_friend_request_accept_repairs_temp_path(runtime_dir) else {
        return Ok(());
    };
    if !temp_path.exists() {
        return Ok(());
    }
    if path.exists() {
        return fs::remove_file(temp_path.as_path()).map_err(|error| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: format!(
                "failed to discard stale pending friend request accept repair temp file {}: {error}",
                temp_path.display()
            ),
        });
    }
    fs::rename(temp_path.as_path(), path.as_path()).map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to recover pending friend request accept repair store from temp file {} to {}: {error}",
            temp_path.display(),
            path.display()
        ),
    })
}

fn persist_pending_friend_request_accept_repairs(
    runtime_dir: Option<&StdPath>,
    store: &PendingFriendRequestAcceptanceRepairStore,
) -> Result<(), ApiError> {
    let Some(path) = pending_friend_request_accept_repairs_path(runtime_dir) else {
        return Ok(());
    };
    let Some(parent) = path.parent() else {
        return Ok(());
    };
    fs::create_dir_all(parent).map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to create pending friend request accept repair dir {}: {error}",
            parent.display()
        ),
    })?;
    if store.is_empty() {
        if path.exists() {
            fs::remove_file(path.as_path()).map_err(|error| ApiError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "friend_request_accept_repair_store_unavailable",
                message: format!(
                    "failed to clear pending friend request accept repair store {}: {error}",
                    path.display()
                ),
            })?;
        }
        if let Some(temp_path) = pending_friend_request_accept_repairs_temp_path(runtime_dir)
            && temp_path.exists()
        {
            fs::remove_file(temp_path.as_path()).map_err(|error| ApiError {
                status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
                code: "friend_request_accept_repair_store_unavailable",
                message: format!(
                    "failed to clear pending friend request accept repair temp store {}: {error}",
                    temp_path.display()
                ),
            })?;
        }
        return Ok(());
    }
    maybe_pause_pending_friend_request_accept_repair_store_io();
    let payload = serde_json::to_vec(store).map_err(|error| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "friend_request_accept_repair_store_invalid",
        message: format!("failed to serialize pending friend request accept repairs: {error}"),
    })?;
    let temp_path =
        pending_friend_request_accept_repairs_temp_path(runtime_dir).ok_or(ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: "pending friend request accept repair temp store path is unavailable".into(),
        })?;
    if temp_path.exists() {
        fs::remove_file(temp_path.as_path()).map_err(|error| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: format!(
                "failed to clear stale pending friend request accept repair temp store {}: {error}",
                temp_path.display()
            ),
        })?;
    }
    let mut temp_file = fs::File::create(temp_path.as_path()).map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to create pending friend request accept repair temp store {}: {error}",
            temp_path.display()
        ),
    })?;
    temp_file
        .write_all(payload.as_slice())
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
            code: "friend_request_accept_repair_store_unavailable",
            message: format!(
                "failed to write pending friend request accept repair temp store {}: {error}",
                temp_path.display()
            ),
        })?;
    temp_file.sync_all().map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to sync pending friend request accept repair temp store {}: {error}",
            temp_path.display()
        ),
    })?;
    drop(temp_file);
    maybe_fail_pending_friend_request_accept_repair_store_after_temp_write(temp_path.as_path())?;
    fs::rename(temp_path.as_path(), path.as_path()).map_err(|error| ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "failed to persist pending friend request accept repair store {} from temp file {}: {error}",
            path.display(),
            temp_path.display()
        ),
    })
}

#[cfg(debug_assertions)]
fn maybe_fail_pending_friend_request_accept_repair_store_after_temp_write(
    temp_path: &StdPath,
) -> Result<(), ApiError> {
    let Ok(raw_value) = std::env::var(SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE_ENV) else {
        return Ok(());
    };
    if raw_value != "1" {
        return Ok(());
    }

    Err(ApiError {
        status: axum::http::StatusCode::SERVICE_UNAVAILABLE,
        code: "friend_request_accept_repair_store_unavailable",
        message: format!(
            "debug failpoint forced pending friend request accept repair store finalize failure after temp write {}",
            temp_path.display()
        ),
    })
}

#[cfg(not(debug_assertions))]
fn maybe_fail_pending_friend_request_accept_repair_store_after_temp_write(
    _temp_path: &StdPath,
) -> Result<(), ApiError> {
    Ok(())
}

async fn repair_pending_friend_request_acceptance(
    state: &AppState,
    repair: &PendingFriendRequestAcceptanceRepair,
) -> Result<(), ApiError> {
    let auth = AppContext {
        tenant_id: repair.tenant_id.clone(),
        organization_id: None,
        user_id: repair.target_user_id.clone(),
        actor_id: repair.target_user_id.clone(),
        actor_kind: "user".into(),
        session_id: None,
        app_id: Some("sdkwork-im".into()),
        environment: None,
        deployment_mode: None,
        auth_level: None,
        data_scope: BTreeSet::new(),
        permission_scope: BTreeSet::new(),
        device_id: None,
    };
    let snapshot = match dispatch_control_plane_json(
        state,
        &auth,
        "GET",
        format!(
            "/backend/v3/api/control/social/friend_requests/{}",
            repair.request_id
        ),
        "control.read",
        None,
    )
    .await
    {
        Ok(snapshot) => snapshot,
        Err(error) if error.status == axum::http::StatusCode::NOT_FOUND => {
            try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
            return Ok(());
        }
        Err(error) => return Err(error),
    };
    let friend_request: FriendRequest = parse_json_field(&snapshot, "friendRequest")?;
    if let Err(error) = ensure_social_friend_request_users_active(
        state,
        repair.tenant_id.as_str(),
        friend_request.requester_user_id.as_str(),
        friend_request.target_user_id.as_str(),
    ) {
        if is_terminal_social_user_resolution_error(&error) {
            tracing::warn!(
                "clearing pending friend request accept repair {} because participant resolution failed: {}",
                repair.request_id,
                error.message
            );
            try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
            return Ok(());
        }
        return Err(error);
    }
    maybe_pause_friend_request_accept_repair_after_snapshot().await;
    match friend_request.status {
        FriendRequestStatus::Pending => {
            let accepted_response = match dispatch_control_plane_json(
                state,
                &auth,
                "POST",
                format!(
                    "/backend/v3/api/control/social/friend_requests/{}/accept",
                    repair.request_id
                ),
                "control.write",
                Some(serde_json::json!({
                    "eventId": deterministic_social_id("evt_fr_accept_", repair.request_id.as_str()),
                    "acceptedByUserId": repair.target_user_id,
                    "acceptedAt": repair.accepted_at,
                })),
            )
            .await
            {
                Ok(response) => response,
                Err(error) => {
                    if is_converged_friend_request_mutation_conflict(error.code)
                        && recover_pending_friend_request_accept_repair_after_not_pending(
                            state, &auth, repair,
                        )
                        .await?
                    {
                        return Ok(());
                    }
                    if maybe_clear_terminal_friend_request_accept_repair_error(
                        state, repair, &error,
                    )
                    .await
                    {
                        return Ok(());
                    }
                    return Err(error);
                }
            };
            let accepted_friend_request: FriendRequest =
                parse_json_field(&accepted_response, "friendRequest")?;
            if let Err(error) = finalize_friend_request_acceptance_repair(
                state,
                &auth,
                &accepted_friend_request,
                repair,
            )
            .await
            {
                if maybe_clear_terminal_friend_request_accept_repair_error(state, repair, &error)
                    .await
                {
                    return Ok(());
                }
                return Err(error);
            }
        }
        FriendRequestStatus::Accepted => {
            if let Err(error) =
                finalize_friend_request_acceptance_repair(state, &auth, &friend_request, repair)
                    .await
            {
                if maybe_clear_terminal_friend_request_accept_repair_error(state, repair, &error)
                    .await
                {
                    return Ok(());
                }
                return Err(error);
            }
        }
        _ => {
            try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
        }
    }

    Ok(())
}

async fn recover_pending_friend_request_accept_repair_after_not_pending(
    state: &AppState,
    auth: &AppContext,
    repair: &PendingFriendRequestAcceptanceRepair,
) -> Result<bool, ApiError> {
    let Some(latest_friend_request) =
        load_latest_friend_request_for_acceptance(state, auth, repair.request_id.as_str()).await?
    else {
        try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
        return Ok(true);
    };
    match latest_friend_request.status {
        FriendRequestStatus::Accepted => {
            if let Err(error) = finalize_friend_request_acceptance_repair(
                state,
                auth,
                &latest_friend_request,
                repair,
            )
            .await
            {
                if maybe_clear_terminal_friend_request_accept_repair_error(state, repair, &error)
                    .await
                {
                    return Ok(true);
                }
                return Err(error);
            }
            Ok(true)
        }
        FriendRequestStatus::Pending => Ok(false),
        _ => {
            try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
            Ok(true)
        }
    }
}

async fn reconcile_accept_friend_request_after_not_pending(
    state: &AppState,
    auth: &AppContext,
    request_id: &str,
    repair: &PendingFriendRequestAcceptanceRepair,
) -> Result<Option<FriendRequest>, ApiError> {
    let Some(latest_friend_request) =
        load_latest_friend_request_for_acceptance(state, auth, request_id).await?
    else {
        try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
        return Ok(None);
    };
    if latest_friend_request.status == FriendRequestStatus::Accepted {
        return Ok(Some(latest_friend_request));
    }
    if latest_friend_request.status != FriendRequestStatus::Pending {
        try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
    }
    Ok(None)
}

async fn reconcile_friend_request_mutation_after_not_pending(
    state: &AppState,
    auth: &AppContext,
    request_id: &str,
    expected_status: FriendRequestStatus,
) -> Result<Option<FriendRequest>, ApiError> {
    let Some(latest_friend_request) =
        load_latest_friend_request_for_acceptance(state, auth, request_id).await?
    else {
        return Ok(None);
    };
    if latest_friend_request.status == expected_status {
        return Ok(Some(latest_friend_request));
    }
    Ok(None)
}

async fn reconcile_friendship_remove_after_not_active(
    state: &AppState,
    auth: &AppContext,
    friendship_id: &str,
) -> Result<Option<Friendship>, ApiError> {
    let Some(latest_friendship) =
        load_latest_friendship_for_mutation(state, auth, friendship_id).await?
    else {
        return Ok(None);
    };
    if latest_friendship.status == FriendshipStatus::Removed {
        return Ok(Some(latest_friendship));
    }
    Ok(None)
}

async fn load_latest_friend_request_for_acceptance(
    state: &AppState,
    auth: &AppContext,
    request_id: &str,
) -> Result<Option<FriendRequest>, ApiError> {
    let snapshot = match dispatch_control_plane_json(
        state,
        auth,
        "GET",
        format!("/backend/v3/api/control/social/friend_requests/{request_id}"),
        "control.read",
        None,
    )
    .await
    {
        Ok(snapshot) => snapshot,
        Err(error) if error.status == axum::http::StatusCode::NOT_FOUND => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(Some(parse_json_field(&snapshot, "friendRequest")?))
}

async fn load_latest_friendship_for_mutation(
    state: &AppState,
    auth: &AppContext,
    friendship_id: &str,
) -> Result<Option<Friendship>, ApiError> {
    let snapshot = match dispatch_control_plane_json(
        state,
        auth,
        "GET",
        format!("/backend/v3/api/control/social/friendships/{friendship_id}"),
        "control.read",
        None,
    )
    .await
    {
        Ok(snapshot) => snapshot,
        Err(error) if error.status == axum::http::StatusCode::NOT_FOUND => return Ok(None),
        Err(error) => return Err(error),
    };
    Ok(Some(parse_json_field(&snapshot, "friendship")?))
}

async fn finalize_friend_request_acceptance_repair(
    state: &AppState,
    auth: &AppContext,
    friend_request: &FriendRequest,
    repair: &PendingFriendRequestAcceptanceRepair,
) -> Result<(), ApiError> {
    let friendship_id = deterministic_social_id("fs_", repair.request_id.as_str());
    let direct_chat_id = deterministic_social_id("dc_", repair.request_id.as_str());
    let conversation_id = deterministic_social_id("c_direct_", repair.request_id.as_str());
    ensure_friendship_for_acceptance(
        state,
        auth,
        friend_request,
        repair.request_id.as_str(),
        friendship_id.as_str(),
        direct_chat_id.as_str(),
        repair.accepted_at.as_str(),
    )
    .await?;
    let direct_chat = ensure_direct_chat_for_acceptance(
        state,
        auth,
        friend_request,
        repair.request_id.as_str(),
        direct_chat_id.as_str(),
        conversation_id.as_str(),
        repair.accepted_at.as_str(),
    )
    .await?;
    let bound_conversation_id = direct_chat.conversation_id.clone().ok_or(ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_invalid_field",
        message: format!(
            "direct chat {} is missing a conversation binding during friend request acceptance repair",
            direct_chat.direct_chat_id
        ),
    })?;
    state.conversation_runtime.bind_direct_chat_conversation(
        BindDirectChatConversationCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: bound_conversation_id,
            direct_chat_id: direct_chat.direct_chat_id.clone(),
            left_actor_id: friend_request.requester_user_id.clone(),
            left_actor_kind: "user".into(),
            right_actor_id: friend_request.target_user_id.clone(),
            right_actor_kind: "user".into(),
            bound_by: LOCAL_SOCIAL_SYSTEM_ACTOR_ID.into(),
        },
    )?;
    try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
    Ok(())
}

#[cfg(debug_assertions)]
async fn maybe_pause_friend_request_accept_after_request_commit() {
    const SOCIAL_ACCEPT_TEST_POST_COMMIT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_ACCEPT_POST_COMMIT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_ACCEPT_TEST_POST_COMMIT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friend_request_accept_after_request_commit() {}

#[cfg(debug_assertions)]
async fn maybe_pause_friend_request_accept_before_request_commit() {
    const SOCIAL_ACCEPT_TEST_PRE_COMMIT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_ACCEPT_PRE_COMMIT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_ACCEPT_TEST_PRE_COMMIT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friend_request_accept_before_request_commit() {}

async fn existing_pending_friend_request_id_for_pair(
    state: &AppState,
    auth: &AppContext,
    requester_user_id: &str,
    target_user_id: &str,
) -> Result<Option<String>, ApiError> {
    for (user_id, direction) in [
        (
            requester_user_id,
            SocialFriendRequestListDirectionQuery::Outgoing,
        ),
        (
            requester_user_id,
            SocialFriendRequestListDirectionQuery::Incoming,
        ),
        (
            target_user_id,
            SocialFriendRequestListDirectionQuery::Outgoing,
        ),
        (
            target_user_id,
            SocialFriendRequestListDirectionQuery::Incoming,
        ),
    ] {
        let response = dispatch_control_plane_json(
            state,
            auth,
            "GET",
            build_friend_request_inventory_control_plane_uri(
                user_id,
                direction,
                SocialFriendRequestListStatusQuery::Pending,
                SOCIAL_FRIEND_REQUEST_LIST_MAX_LIMIT,
                None,
            ),
            "control.read",
            None,
        )
        .await?;
        let items: Vec<FriendRequest> = parse_json_field(&response, "items")?;
        if let Some(existing) = items.into_iter().find(|friend_request| {
            same_friend_request_pair(friend_request, requester_user_id, target_user_id)
        }) {
            return Ok(Some(existing.request_id));
        }
    }

    Ok(None)
}

fn same_friend_request_pair(
    friend_request: &FriendRequest,
    requester_user_id: &str,
    target_user_id: &str,
) -> bool {
    (friend_request.requester_user_id == requester_user_id
        && friend_request.target_user_id == target_user_id)
        || (friend_request.requester_user_id == target_user_id
            && friend_request.target_user_id == requester_user_id)
}

#[cfg(debug_assertions)]
async fn maybe_pause_friend_request_decline_before_request_commit() {
    const SOCIAL_DECLINE_TEST_PRE_COMMIT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_DECLINE_PRE_COMMIT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_DECLINE_TEST_PRE_COMMIT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friend_request_decline_before_request_commit() {}

#[cfg(debug_assertions)]
async fn maybe_pause_friend_request_cancel_before_request_commit() {
    const SOCIAL_CANCEL_TEST_PRE_COMMIT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_CANCEL_PRE_COMMIT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_CANCEL_TEST_PRE_COMMIT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friend_request_cancel_before_request_commit() {}

#[cfg(debug_assertions)]
async fn maybe_pause_friendship_remove_before_request_commit() {
    const SOCIAL_REMOVE_TEST_PRE_COMMIT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_REMOVE_PRE_COMMIT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_REMOVE_TEST_PRE_COMMIT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friendship_remove_before_request_commit() {}

#[cfg(debug_assertions)]
async fn maybe_pause_friend_request_accept_repair_after_snapshot() {
    const SOCIAL_ACCEPT_REPAIR_TEST_POST_SNAPSHOT_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_ACCEPT_REPAIR_POST_SNAPSHOT_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_ACCEPT_REPAIR_TEST_POST_SNAPSHOT_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
}

#[cfg(not(debug_assertions))]
async fn maybe_pause_friend_request_accept_repair_after_snapshot() {}

#[cfg(debug_assertions)]
fn maybe_pause_pending_friend_request_accept_repair_store_io() {
    const SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_MS";
    let Ok(raw_delay_ms) = std::env::var(SOCIAL_ACCEPT_REPAIR_STORE_IO_DELAY_ENV) else {
        return;
    };
    let Ok(delay_ms) = raw_delay_ms.parse::<u64>() else {
        return;
    };
    if delay_ms == 0 {
        return;
    }

    std::thread::sleep(std::time::Duration::from_millis(delay_ms));
}

#[cfg(not(debug_assertions))]
fn maybe_pause_pending_friend_request_accept_repair_store_io() {}

fn ensure_social_user_actor(auth: &AppContext) -> Result<(), ApiError> {
    if auth.actor_kind == "user" {
        return Ok(());
    }

    Err(ApiError {
        status: axum::http::StatusCode::FORBIDDEN,
        code: "social_user_required",
        message: format!(
            "social IM open-platform routes require user actor kind, got {}",
            auth.actor_kind
        ),
    })
}

fn ensure_social_friend_request_users_active(
    state: &AppState,
    tenant_id: &str,
    requester_user_id: &str,
    target_user_id: &str,
) -> Result<(), ApiError> {
    principal_profile::ensure_active_user(state, tenant_id, requester_user_id)?;
    principal_profile::ensure_active_user(state, tenant_id, target_user_id)?;
    Ok(())
}

fn is_terminal_social_user_resolution_error(error: &ApiError) -> bool {
    matches!(
        error.code,
        "principal_profile_not_found" | "principal_profile_inactive"
    )
}

fn is_terminal_friend_request_accept_repair_error(error: &ApiError) -> bool {
    is_terminal_social_user_resolution_error(error)
        || matches!(
            error.code,
            "friend_request_blocked"
                | "friendship_blocked"
                | "friendship_not_active"
                | "direct_chat_not_active"
        )
}

fn is_converged_friend_request_mutation_conflict(code: &str) -> bool {
    matches!(
        code,
        "friend_request_not_pending" | "social_event_id_conflict"
    )
}

async fn maybe_clear_terminal_friend_request_accept_repair_error(
    state: &AppState,
    repair: &PendingFriendRequestAcceptanceRepair,
    error: &ApiError,
) -> bool {
    if !is_terminal_friend_request_accept_repair_error(error) {
        return false;
    }
    tracing::warn!(
        "clearing pending friend request accept repair {} because repair reached terminal state: {}",
        repair.request_id,
        error.message
    );
    try_clear_pending_friend_request_accept_repair(state, repair.request_id.as_str()).await;
    true
}

async fn dispatch_control_plane_json(
    state: &AppState,
    auth: &AppContext,
    method: &str,
    uri: String,
    permissions: &str,
    body: Option<serde_json::Value>,
) -> Result<serde_json::Value, ApiError> {
    let (status, value) =
        dispatch_control_plane_json_response(state, auth, method, uri, permissions, body).await?;
    if status.is_success() {
        return Ok(value);
    }

    Err(api_error_from_control_plane_response(status, &value))
}

async fn dispatch_control_plane_json_response(
    state: &AppState,
    auth: &AppContext,
    method: &str,
    uri: String,
    permissions: &str,
    body: Option<serde_json::Value>,
) -> Result<(axum::http::StatusCode, serde_json::Value), ApiError> {
    let auth_headers = im_app_context::build_dual_token_headers_for_context(auth, [permissions]);
    let mut builder = Request::builder().method(method).uri(uri);
    for (name, value) in auth_headers.iter() {
        builder = builder.header(name, value);
    }
    let body = match body {
        Some(value) => {
            builder = builder.header(CONTENT_TYPE, "application/json");
            Body::from(value.to_string())
        }
        None => Body::empty(),
    };

    let response = state
        .control_plane_app
        .clone()
        .oneshot(builder.body(body).map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_request_build_failed",
            message: format!("failed to build internal control-plane request: {error}"),
        })?)
        .await
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_dispatch_failed",
            message: format!("failed to dispatch internal control-plane request: {error}"),
        })?;
    let status = response.status();
    let body = response
        .into_body()
        .collect()
        .await
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_read_failed",
            message: format!("failed to read internal control-plane response: {error}"),
        })?
        .to_bytes();
    let value = if body.is_empty() {
        serde_json::json!({})
    } else {
        serde_json::from_slice(&body).map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid",
            message: format!("failed to decode internal control-plane response: {error}"),
        })?
    };
    Ok((status, value))
}

fn api_error_from_control_plane_response(
    status: axum::http::StatusCode,
    value: &serde_json::Value,
) -> ApiError {
    let control_plane_code = value
        .get("code")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("control_plane_request_failed");
    let control_plane_message = value
        .get("message")
        .and_then(serde_json::Value::as_str)
        .unwrap_or("control-plane request failed");
    if let Some(code) = passthrough_social_control_plane_error_code(control_plane_code) {
        return ApiError {
            status,
            code,
            message: control_plane_message.to_owned(),
        };
    }
    ApiError {
        status,
        code: "control_plane_request_failed",
        message: format!(
            "control-plane request failed with code {control_plane_code}: {control_plane_message}"
        ),
    }
}

fn passthrough_social_control_plane_error_code(code: &str) -> Option<&'static str> {
    match code {
        "cursor_invalid" => Some("cursor_invalid"),
        "limit_invalid" => Some("limit_invalid"),
        "invalid_friend_request_query" => Some("invalid_friend_request_query"),
        "invalid_friend_request" => Some("invalid_friend_request"),
        "friend_request_not_found" => Some("friend_request_not_found"),
        "friend_request_not_pending" => Some("friend_request_not_pending"),
        "friend_request_pair_conflict" => Some("friend_request_pair_conflict"),
        "friend_request_conflict" => Some("friend_request_conflict"),
        "friend_request_blocked" => Some("friend_request_blocked"),
        "social_event_id_conflict" => Some("social_event_id_conflict"),
        "invalid_friendship" => Some("invalid_friendship"),
        "friendship_not_found" => Some("friendship_not_found"),
        "friendship_not_active" => Some("friendship_not_active"),
        "friendship_pair_conflict" => Some("friendship_pair_conflict"),
        "friendship_conflict" => Some("friendship_conflict"),
        "friendship_blocked" => Some("friendship_blocked"),
        _ => None,
    }
}

fn control_plane_existing_request_id(value: &serde_json::Value) -> Option<&str> {
    value
        .get("details")
        .and_then(|details| details.get("existingRequestId"))
        .and_then(serde_json::Value::as_str)
}

fn control_plane_existing_friendship_id(value: &serde_json::Value) -> Option<&str> {
    value
        .get("details")
        .and_then(|details| details.get("existingFriendshipId"))
        .and_then(serde_json::Value::as_str)
}

fn control_plane_existing_direct_chat_id(value: &serde_json::Value) -> Option<&str> {
    value
        .get("details")
        .and_then(|details| details.get("existingDirectChatId"))
        .and_then(serde_json::Value::as_str)
}

fn control_plane_error_code(value: &serde_json::Value) -> Option<&str> {
    value.get("code").and_then(serde_json::Value::as_str)
}

fn social_friend_request_direction_wire(
    direction: SocialFriendRequestListDirectionQuery,
) -> &'static str {
    match direction {
        SocialFriendRequestListDirectionQuery::Incoming => "incoming",
        SocialFriendRequestListDirectionQuery::Outgoing => "outgoing",
    }
}

fn social_friend_request_status_wire(status: SocialFriendRequestListStatusQuery) -> &'static str {
    match status {
        SocialFriendRequestListStatusQuery::Pending => "pending",
        SocialFriendRequestListStatusQuery::Accepted => "accepted",
        SocialFriendRequestListStatusQuery::Declined => "declined",
        SocialFriendRequestListStatusQuery::Canceled => "canceled",
        SocialFriendRequestListStatusQuery::Expired => "expired",
        SocialFriendRequestListStatusQuery::All => "all",
    }
}

fn build_friend_request_inventory_control_plane_uri(
    user_id: &str,
    direction: SocialFriendRequestListDirectionQuery,
    status: SocialFriendRequestListStatusQuery,
    limit: usize,
    cursor: Option<&str>,
) -> String {
    let mut uri = format!(
        "/backend/v3/api/control/social/friend_requests?userId={}&direction={}&status={}&limit={}",
        encode_query_component(user_id),
        encode_query_component(social_friend_request_direction_wire(direction)),
        encode_query_component(social_friend_request_status_wire(status)),
        limit
    );
    if let Some(cursor) = cursor {
        uri.push_str("&cursor=");
        uri.push_str(encode_query_component(cursor).as_str());
    }
    uri
}

fn parse_social_offset_cursor(cursor: Option<&str>) -> Result<usize, ApiError> {
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

fn contact_matches_social_user_query(target_user_id: &str, q: &str) -> bool {
    target_user_id
        .to_ascii_lowercase()
        .contains(q.to_ascii_lowercase().as_str())
}

fn social_user_search_result_from_profile(
    state: &AppState,
    auth: &AppContext,
    profile: im_platform_contracts::PrincipalProfile,
) -> SocialUserSearchResult {
    let relationship_state = state
        .projection_service
        .contacts(auth.tenant_id.as_str(), auth.actor_id.as_str())
        .into_iter()
        .find(|contact| contact.target_user_id == profile.principal_id)
        .map(|contact| contact.relationship_state)
        .unwrap_or_else(|| {
            if profile.principal_id == auth.actor_id {
                "self".into()
            } else {
                "none".into()
            }
        });
    let avatar_url = profile
        .attributes
        .get("avatarUrl")
        .or_else(|| profile.attributes.get("avatar"))
        .cloned();
    let email = profile.attributes.get("email").cloned();
    let phone = profile
        .attributes
        .get("phone")
        .or_else(|| profile.attributes.get("phoneNumber"))
        .cloned();

    SocialUserSearchResult {
        chat_id: principal_profile::public_chat_id_for_profile(&profile),
        tenant_id: profile.tenant_id,
        user_id: profile.principal_id,
        display_name: profile.display_name,
        relationship_state,
        avatar_url,
        email,
        phone,
        metadata: profile.attributes,
    }
}

fn encode_query_component(value: &str) -> String {
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(char::from(byte));
        } else {
            encoded.push('%');
            encoded.push_str(format!("{byte:02X}").as_str());
        }
    }
    encoded
}

fn optional_json_string_field(value: &serde_json::Value, field: &str) -> Option<String> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
}

fn parse_json_field<T>(value: &serde_json::Value, field: &str) -> Result<T, ApiError>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(value.get(field).cloned().ok_or_else(|| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_missing_field",
        message: format!("internal control-plane response missing field: {field}"),
    })?)
    .map_err(|error| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_invalid_field",
        message: format!("failed to decode field {field} from control-plane response: {error}"),
    })
}

fn parse_optional_json_field<T>(
    value: &serde_json::Value,
    field: &str,
) -> Result<Option<T>, ApiError>
where
    T: serde::de::DeserializeOwned,
{
    let Some(field_value) = value.get(field).cloned() else {
        return Ok(None);
    };
    if field_value.is_null() {
        return Ok(None);
    }
    serde_json::from_value(field_value)
        .map(Some)
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!("failed to decode field {field} from control-plane response: {error}"),
        })
}

fn apply_control_plane_latest_commit(
    state: &AppState,
    response: &serde_json::Value,
) -> Result<(), ApiError> {
    let commit = commit_envelope_from_latest_commit_response(response)?;
    apply_control_plane_commit(state, commit)
}

fn apply_optional_control_plane_commit_field(
    state: &AppState,
    response: &serde_json::Value,
    field: &str,
) -> Result<(), ApiError> {
    let Some(commit_value) = response.get(field) else {
        return Ok(());
    };
    if commit_value.is_null() {
        return Ok(());
    }
    let commit = commit_envelope_from_value(commit_value)?;
    apply_control_plane_commit(state, commit)
}

fn apply_control_plane_snapshot_latest_commit(
    state: &AppState,
    response: &serde_json::Value,
) -> Result<(), ApiError> {
    let commit = commit_envelope_from_snapshot_response(response)?;
    apply_control_plane_commit(state, commit)
}

fn apply_control_plane_commit(state: &AppState, commit: CommitEnvelope) -> Result<(), ApiError> {
    state
        .projection_service
        .apply(&commit)
        .map_err(|error| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "projection_apply_failed",
            message: format!("failed to apply social projection commit: {error:?}"),
        })
}

fn commit_envelope_from_latest_commit_response(
    response: &serde_json::Value,
) -> Result<CommitEnvelope, ApiError> {
    let latest_commit = response.get("latestCommit").ok_or_else(|| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_missing_field",
        message: "internal control-plane response missing field: latestCommit".into(),
    })?;
    commit_envelope_from_value(latest_commit)
}

fn commit_envelope_from_snapshot_response(
    response: &serde_json::Value,
) -> Result<CommitEnvelope, ApiError> {
    let latest_commit = response
        .get("commits")
        .and_then(serde_json::Value::as_array)
        .and_then(|commits| commits.last())
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_missing_field",
            message: "internal control-plane snapshot response missing field: commits".into(),
        })?;
    commit_envelope_from_value(latest_commit)
}

fn commit_envelope_from_value(
    latest_commit: &serde_json::Value,
) -> Result<CommitEnvelope, ApiError> {
    let actor = latest_commit.get("actor").ok_or_else(|| ApiError {
        status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        code: "control_plane_response_missing_field",
        message: "internal control-plane response missing field: latestCommit.actor".into(),
    })?;

    Ok(CommitEnvelope {
        event_id: json_string_field(latest_commit, "eventId")?.into(),
        tenant_id: json_string_field(latest_commit, "tenantId")?.into(),
        event_type: json_string_field(latest_commit, "eventType")?.into(),
        event_version: latest_commit
            .get("eventVersion")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| ApiError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "control_plane_response_invalid_field",
                message: "internal control-plane response missing eventVersion".into(),
            })? as u16,
        aggregate_type: aggregate_type_from_wire(json_string_field(
            latest_commit,
            "aggregateType",
        )?)?,
        aggregate_id: json_string_field(latest_commit, "aggregateId")?.into(),
        scope_type: json_string_field(latest_commit, "scopeType")?.into(),
        scope_id: json_string_field(latest_commit, "scopeId")?.into(),
        ordering_key: json_string_field(latest_commit, "orderingKey")?.into(),
        ordering_seq: latest_commit
            .get("orderingSeq")
            .and_then(serde_json::Value::as_u64)
            .ok_or_else(|| ApiError {
                status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                code: "control_plane_response_invalid_field",
                message: "internal control-plane response missing orderingSeq".into(),
            })?,
        causation_id: latest_commit
            .get("causationId")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        correlation_id: latest_commit
            .get("correlationId")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        idempotency_key: latest_commit
            .get("idempotencyKey")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        actor: EventActor {
            actor_id: json_string_field(actor, "actorId")?.into(),
            actor_kind: json_string_field(actor, "actorKind")?.into(),
            actor_session_id: actor
                .get("actorSessionId")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
        },
        occurred_at: json_string_field(latest_commit, "occurredAt")?.into(),
        committed_at: json_string_field(latest_commit, "committedAt")?.into(),
        payload_schema: latest_commit
            .get("payloadSchema")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned),
        payload: json_string_field(latest_commit, "payload")?.into(),
        retention_class: json_string_field(latest_commit, "retentionClass")?.into(),
        audit_class: json_string_field(latest_commit, "auditClass")?.into(),
    })
}

fn json_string_field<'a>(value: &'a serde_json::Value, field: &str) -> Result<&'a str, ApiError> {
    value
        .get(field)
        .and_then(serde_json::Value::as_str)
        .ok_or_else(|| ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!("internal control-plane response missing string field: {field}"),
        })
}

fn aggregate_type_from_wire(value: &str) -> Result<AggregateType, ApiError> {
    match value {
        "conversation" => Ok(AggregateType::Conversation),
        "friend_request" => Ok(AggregateType::FriendRequest),
        "friendship" => Ok(AggregateType::Friendship),
        "external_connection" => Ok(AggregateType::ExternalConnection),
        "external_member_link" => Ok(AggregateType::ExternalMemberLink),
        "shared_channel_policy" => Ok(AggregateType::SharedChannelPolicy),
        "stream_session" => Ok(AggregateType::StreamSession),
        "rtc_session" => Ok(AggregateType::RtcSession),
        "tenant_policy" => Ok(AggregateType::TenantPolicy),
        "direct_chat" => Ok(AggregateType::DirectChat),
        "notification" => Ok(AggregateType::Notification),
        "automation_execution" => Ok(AggregateType::AutomationExecution),
        "user_block" => Ok(AggregateType::UserBlock),
        _ => Err(ApiError {
            status: axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            code: "control_plane_response_invalid_field",
            message: format!("unknown aggregate type from control-plane response: {value}"),
        }),
    }
}

fn current_social_timestamp() -> String {
    format_unix_timestamp_millis(current_unix_epoch_millis())
}

fn current_unix_epoch_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}

fn current_unix_epoch_nanos() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_nanos()
}

fn unique_friend_request_id_for_pair(
    tenant_id: &str,
    requester_user_id: &str,
    target_user_id: &str,
) -> Result<String, ApiError> {
    let pair =
        normalize_user_pair(requester_user_id, target_user_id).map_err(|error| ApiError {
            status: axum::http::StatusCode::BAD_REQUEST,
            code: "invalid_friend_request_pair",
            message: format!("invalid friend request pair: {error:?}"),
        })?;
    Ok(deterministic_social_id(
        "fr_",
        format!(
            "{}:{}:{}:{}:{}",
            tenant_id,
            pair.user_low_id,
            pair.user_high_id,
            current_unix_epoch_nanos(),
            NEXT_SOCIAL_ID_SEQUENCE.fetch_add(1, Ordering::Relaxed)
        )
        .as_str(),
    ))
}

fn deterministic_social_id(prefix: &str, seed: &str) -> String {
    let digest = Sha256::digest(seed.as_bytes());
    let digest = format!("{digest:x}");
    format!("{prefix}{}", &digest[..24])
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    static NEXT_SOCIAL_TEST_RUNTIME_DIR_SEQUENCE: AtomicU64 = AtomicU64::new(0);
    const SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE_ENV: &str =
        "SDKWORK_IM_TEST_SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE";

    struct ScopedEnvVar {
        name: &'static str,
        previous: Option<String>,
    }

    impl ScopedEnvVar {
        fn set(name: &'static str, value: &str) -> Self {
            let previous = std::env::var(name).ok();
            unsafe {
                std::env::set_var(name, value);
            }
            Self { name, previous }
        }
    }

    impl Drop for ScopedEnvVar {
        fn drop(&mut self) {
            if let Some(previous) = &self.previous {
                unsafe {
                    std::env::set_var(self.name, previous);
                }
                return;
            }

            unsafe {
                std::env::remove_var(self.name);
            }
        }
    }

    fn social_accept_repair_store_env_guard() -> std::sync::MutexGuard<'static, ()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD
            .get_or_init(|| Mutex::new(()))
            .lock()
            .expect("social accept repair store env guard should not be poisoned")
    }

    fn unique_social_test_runtime_dir(prefix: &str) -> PathBuf {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        let sequence = NEXT_SOCIAL_TEST_RUNTIME_DIR_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        std::env::temp_dir().join(format!("sdkwork_im_social_{prefix}_{unique}_{sequence}"))
    }

    #[test]
    fn test_pending_friend_request_accept_repair_store_recovers_temp_file_when_primary_missing() {
        let runtime_dir = unique_social_test_runtime_dir("repair_store_temp_recovery");
        let primary_path = pending_friend_request_accept_repairs_path(Some(runtime_dir.as_path()))
            .expect("repair store path should resolve");
        let temp_path = primary_path.with_extension("json.tmp");
        fs::create_dir_all(
            primary_path
                .parent()
                .expect("repair store path should include state dir"),
        )
        .expect("repair store state dir should be created");
        fs::write(
            temp_path.as_path(),
            serde_json::json!({
                "fr_temp_only": {
                    "tenant_id": "t_demo",
                    "request_id": "fr_temp_only",
                    "requester_user_id": "u_alice",
                    "target_user_id": "u_bob",
                    "accepted_at": "2026-04-16T12:00:00Z"
                }
            })
            .to_string(),
        )
        .expect("repair store temp file should be writable");

        let loaded = load_pending_friend_request_accept_repairs(Some(runtime_dir.as_path()));
        let repair = loaded
            .get("fr_temp_only")
            .expect("repair store should recover pending temp file when primary file is missing");
        assert_eq!(repair.request_id, "fr_temp_only");
        assert_eq!(repair.requester_user_id, "u_alice");
        assert_eq!(repair.target_user_id, "u_bob");
        assert!(
            primary_path.exists(),
            "recovered primary repair store file should be materialized"
        );
        assert!(
            !temp_path.exists(),
            "pending repair temp file should be consumed after recovery"
        );

        let _ = fs::remove_dir_all(runtime_dir);
    }

    #[test]
    fn test_pending_friend_request_accept_repair_store_preserves_primary_file_when_finalize_fails()
    {
        let _env_guard = social_accept_repair_store_env_guard();
        let runtime_dir = unique_social_test_runtime_dir("repair_store_atomic_replace");
        let primary_path = pending_friend_request_accept_repairs_path(Some(runtime_dir.as_path()))
            .expect("repair store path should resolve");
        fs::create_dir_all(
            primary_path
                .parent()
                .expect("repair store path should include state dir"),
        )
        .expect("repair store state dir should be created");

        let mut original_store = PendingFriendRequestAcceptanceRepairStore::default();
        original_store.insert(
            "fr_existing".into(),
            PendingFriendRequestAcceptanceRepair {
                tenant_id: "t_demo".into(),
                request_id: "fr_existing".into(),
                requester_user_id: "u_alice".into(),
                target_user_id: "u_bob".into(),
                accepted_at: "2026-04-16T12:10:00Z".into(),
            },
        );
        persist_pending_friend_request_accept_repairs(Some(runtime_dir.as_path()), &original_store)
            .expect("original repair store should persist");

        let mut updated_store = original_store.clone();
        updated_store.insert(
            "fr_new".into(),
            PendingFriendRequestAcceptanceRepair {
                tenant_id: "t_demo".into(),
                request_id: "fr_new".into(),
                requester_user_id: "u_carol".into(),
                target_user_id: "u_dave".into(),
                accepted_at: "2026-04-16T12:11:00Z".into(),
            },
        );

        let _failpoint =
            ScopedEnvVar::set(SOCIAL_ACCEPT_REPAIR_STORE_FAIL_AFTER_TEMP_WRITE_ENV, "1");
        let error = persist_pending_friend_request_accept_repairs(
            Some(runtime_dir.as_path()),
            &updated_store,
        )
        .expect_err("repair store finalize failure should surface as an error");
        assert_eq!(error.code, "friend_request_accept_repair_store_unavailable");

        let loaded = load_pending_friend_request_accept_repairs(Some(runtime_dir.as_path()));
        assert_eq!(
            loaded.len(),
            1,
            "failed atomic replace must preserve the last committed repair store snapshot"
        );
        assert!(
            loaded.contains_key("fr_existing"),
            "existing repair entry must survive failed finalize"
        );
        assert!(
            !loaded.contains_key("fr_new"),
            "new repair entry must not appear when finalize fails"
        );

        let _ = fs::remove_dir_all(runtime_dir);
    }

    #[test]
    fn test_blocked_friend_request_accept_repairs_are_terminal() {
        assert!(is_terminal_friend_request_accept_repair_error(&ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friend_request_blocked",
            message: "friend request is blocked".into(),
        }));
        assert!(is_terminal_friend_request_accept_repair_error(&ApiError {
            status: axum::http::StatusCode::CONFLICT,
            code: "friendship_blocked",
            message: "friendship is blocked".into(),
        }));
    }

    #[test]
    fn test_unique_friend_request_id_allows_new_lifecycle_after_terminal_request() {
        let first = unique_friend_request_id_for_pair("t_demo", "u_alice", "u_bob")
            .expect("pair should generate first unique request id");
        let second = unique_friend_request_id_for_pair("t_demo", "u_bob", "u_alice")
            .expect("reversed pair should generate second unique request id");

        assert_ne!(
            first, second,
            "new lifecycle request after accepted/removed terminal state must not reuse pending id"
        );
    }
}
