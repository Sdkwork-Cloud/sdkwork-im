use super::*;

pub(super) async fn list_members(
    Path(conversation_id): Path<String>,
    Query(query): Query<MemberListQuery>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ListMembersResponse>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    access::ensure_conversation_member(&state, &auth, conversation_id.as_str())?;
    let window = state
        .conversation_runtime
        .list_members_window_from_auth_context(
            &auth,
            conversation_id.as_str(),
            query.limit,
            query.cursor.as_deref(),
        )
        .map_err(map_member_list_window_error)?;
    Ok(Json(ListMembersResponse {
        items: window.items,
        next_cursor: window.next_cursor,
        has_more: window.has_more,
    }))
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub(super) struct MemberListQuery {
    limit: Option<usize>,
    cursor: Option<String>,
}

fn map_member_list_window_error(error: conversation_runtime::RuntimeError) -> ApiError {
    match error {
        conversation_runtime::RuntimeError::InvalidInput(message)
            if message.contains("member list limit") || message.contains("member list cursor") =>
        {
            ApiError {
                status: axum::http::StatusCode::BAD_REQUEST,
                code: if message.contains("cursor") {
                    "cursor_invalid"
                } else {
                    "limit_invalid"
                },
                message,
            }
        }
        other => ApiError::from(other),
    }
}

pub(super) async fn add_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<AddConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let AddConversationMemberRequest {
        principal_id,
        principal_kind,
        role,
        attributes: request_attributes,
    } = request;
    let actor_auth =
        access::resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let base_recipients = effects::conversation_member_principal_recipients_from_auth_context(
        &state,
        &auth,
        conversation_id.as_str(),
    )?;
    let (principal_kind, resolved_attributes) = principal_profile::resolve_member_principal(
        &state,
        auth.tenant_id.as_str(),
        principal_id.as_str(),
        principal_kind.as_str(),
    )?;
    let mut attributes = request_attributes;
    attributes.extend(resolved_attributes);
    let member = state.conversation_runtime.add_member_from_auth_context(
        &auth,
        conversation_id.clone(),
        principal_id,
        principal_kind,
        role,
        attributes,
    )?;

    effects::record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_joined",
        &member,
    );

    effects::publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_joined",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        base_recipients,
        BTreeSet::from([NotificationRecipientView {
            principal_id: member.principal_id.clone(),
            principal_kind: member.principal_kind.clone(),
        }]),
    )?;

    Ok(Json(member))
}

pub(super) async fn remove_member(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<RemoveConversationMemberRequest>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let actor_auth =
        access::resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let base_recipients = effects::conversation_member_principal_recipients_from_auth_context(
        &state,
        &auth,
        conversation_id.as_str(),
    )?;
    let member = state.conversation_runtime.remove_member_from_auth_context(
        &auth,
        conversation_id.clone(),
        request.member_id,
    )?;

    effects::record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_removed",
        &member,
    );

    effects::publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_removed",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        base_recipients,
        BTreeSet::from([NotificationRecipientView {
            principal_id: member.principal_id.clone(),
            principal_kind: member.principal_kind.clone(),
        }]),
    )?;

    Ok(Json(member))
}

pub(super) async fn transfer_conversation_owner(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<TransferConversationOwnerRequest>,
) -> Result<Json<TransferConversationOwnerResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let actor_auth =
        access::resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let base_recipients = effects::conversation_member_principal_recipients_from_auth_context(
        &state,
        &auth,
        conversation_id.as_str(),
    )?;
    let transfer = state
        .conversation_runtime
        .transfer_conversation_owner_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.member_id,
        )?;

    effects::record_owner_transfer_audit(&state, &actor_auth, conversation_id.as_str(), &transfer);

    effects::publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.owner_transferred",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "transferredAt": transfer.transferred_at.as_str(),
            "previousOwner": &transfer.previous_owner,
            "newOwner": &transfer.new_owner,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        base_recipients,
        BTreeSet::from([
            NotificationRecipientView {
                principal_id: transfer.previous_owner.principal_id.clone(),
                principal_kind: transfer.previous_owner.principal_kind.clone(),
            },
            NotificationRecipientView {
                principal_id: transfer.new_owner.principal_id.clone(),
                principal_kind: transfer.new_owner.principal_kind.clone(),
            },
        ]),
    )?;

    Ok(Json(transfer))
}

pub(super) async fn change_conversation_member_role(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
    Json(request): Json<ChangeConversationMemberRoleRequest>,
) -> Result<Json<ChangeConversationMemberRoleResult>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let actor_auth =
        access::resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let base_recipients = effects::conversation_member_principal_recipients_from_auth_context(
        &state,
        &auth,
        conversation_id.as_str(),
    )?;
    let change = state
        .conversation_runtime
        .change_conversation_member_role_from_auth_context(
            &auth,
            conversation_id.clone(),
            request.member_id,
            request.role,
        )?;

    effects::record_member_role_change_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        &change,
    );

    effects::publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_role_changed",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "changedAt": change.changed_at.as_str(),
            "previousMember": &change.previous_member,
            "updatedMember": &change.updated_member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        base_recipients,
        BTreeSet::from([NotificationRecipientView {
            principal_id: change.updated_member.principal_id.clone(),
            principal_kind: change.updated_member.principal_kind.clone(),
        }]),
    )?;

    Ok(Json(change))
}

pub(super) async fn leave_conversation(
    Path(conversation_id): Path<String>,
    headers: HeaderMap,
    auth: Option<Extension<AppContext>>,
    State(state): State<AppState>,
) -> Result<Json<ConversationMember>, ApiError> {
    let auth = resolve_request_app_context(auth, &headers)?;
    let actor_auth =
        access::resolve_conversation_actor_auth_context(&state, &auth, conversation_id.as_str())?;
    let base_recipients = effects::conversation_member_principal_recipients_from_auth_context(
        &state,
        &auth,
        conversation_id.as_str(),
    )?;
    let member = state
        .conversation_runtime
        .leave_conversation_from_auth_context(&auth, conversation_id.clone())?;

    effects::record_membership_audit(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_left",
        &member,
    );

    effects::publish_realtime_membership_event(
        &state,
        &actor_auth,
        conversation_id.as_str(),
        "conversation.member_left",
        serde_json::json!({
            "conversationId": conversation_id.as_str(),
            "member": &member,
            "actor": {
                "id": actor_auth.actor_id.as_str(),
                "kind": actor_auth.actor_kind.as_str(),
            }
        })
        .to_string(),
        base_recipients,
        BTreeSet::from([NotificationRecipientView {
            principal_id: member.principal_id.clone(),
            principal_kind: member.principal_kind.clone(),
        }]),
    )?;

    Ok(Json(member))
}
