use super::*;

pub(super) fn post_message_with_side_effects(
    state: &AppState,
    auth: &AppContext,
    conversation_id: String,
    client_msg_id: Option<String>,
    message_type: MessageType,
    body: MessageBody,
) -> Result<PostMessageResult, ApiError> {
    access::ensure_registered_device(state, auth)?;
    access::ensure_conversation_member(state, auth, conversation_id.as_str())?;
    let summary = body.summary.clone();
    let message_type_name = match &message_type {
        MessageType::Standard => "standard",
        MessageType::Signal => "signal",
        MessageType::System => "system",
    };

    let result = state
        .conversation_runtime
        .post_message(PostMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            sender: principal_profile::resolve_sender_from_auth_context(state, auth)?,
            client_msg_id,
            message_type,
            body,
        })?;

    finalize_post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        message_type_name,
        summary,
        result,
    )
}

pub(super) fn publish_system_channel_message_with_side_effects(
    state: &AppState,
    auth: &AppContext,
    conversation_id: String,
    client_msg_id: Option<String>,
    body: MessageBody,
) -> Result<PostMessageResult, ApiError> {
    access::ensure_registered_device(state, auth)?;
    let summary = body.summary.clone();
    let result = state.conversation_runtime.publish_system_channel_message(
        PublishSystemChannelMessageCommand {
            tenant_id: auth.tenant_id.clone(),
            conversation_id: conversation_id.clone(),
            publisher: principal_profile::resolve_sender_from_auth_context(state, auth)?,
            client_msg_id,
            body,
        },
    )?;

    finalize_post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        "standard",
        summary,
        result,
    )
}

fn finalize_post_message_with_side_effects(
    state: &AppState,
    auth: &AppContext,
    conversation_id: String,
    message_type_name: &str,
    summary: Option<String>,
    result: PostMessageResult,
) -> Result<PostMessageResult, ApiError> {
    if !result.is_applied() {
        return Ok(result);
    }

    let conversation_scope_id = conversation_id.clone();

    if let Err(error) = side_effect_outbox::drain_pending_message_side_effect_outbox(state) {
        record_message_side_effect_store_failure(
            state,
            auth,
            conversation_scope_id.as_str(),
            result.message_id.as_str(),
            result.message_seq,
            "outbox_drain",
            &error,
        );
    }

    fanout_message_notifications(
        state,
        auth,
        conversation_id.as_str(),
        result.message_id.as_str(),
        result.message_seq,
        result.event_id.as_str(),
        message_type_name,
        summary.clone(),
    );

    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id("audit_", result.message_id.as_str()),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id,
            action: "message.posted".into(),
            payload: Some(
                serde_json::json!({
                    "messageId": result.message_id,
                    "messageSeq": result.message_seq,
                    "messageType": message_type_name,
                })
                .to_string(),
            ),
        },
    );

    let realtime_payload = serde_json::json!({
        "conversationId": conversation_scope_id,
        "messageId": result.message_id,
        "messageSeq": result.message_seq,
        "messageType": message_type_name,
        "summary": summary,
    })
    .to_string();
    let outbox_record = match side_effect_outbox::record_pending_message_realtime_side_effect(
        state,
        auth,
        conversation_scope_id.as_str(),
        result.message_id.as_str(),
        result.message_seq,
        "message.posted",
        realtime_payload.clone(),
    ) {
        Ok(record) => Some(record),
        Err(error) => {
            record_message_side_effect_store_failure(
                state,
                auth,
                conversation_scope_id.as_str(),
                result.message_id.as_str(),
                result.message_seq,
                "outbox_persist",
                &error,
            );
            None
        }
    };

    let delivery_result = publish_realtime_conversation_message_event(
        state,
        auth,
        conversation_scope_id.as_str(),
        "message.posted",
        realtime_payload,
    );
    match delivery_result {
        Ok(()) => {
            if let Some(record) = outbox_record.as_ref()
                && let Err(error) = side_effect_outbox::mark_message_side_effect_delivered(
                    state,
                    record.outbox_id.as_str(),
                )
            {
                record_message_side_effect_store_failure(
                    state,
                    auth,
                    conversation_scope_id.as_str(),
                    result.message_id.as_str(),
                    result.message_seq,
                    "outbox_delivered",
                    &error,
                );
            }
        }
        Err(error) => {
            if let Some(record) = outbox_record.as_ref()
                && let Err(store_error) = side_effect_outbox::mark_message_side_effect_failed(
                    state,
                    record.outbox_id.as_str(),
                    &error,
                )
            {
                record_message_side_effect_store_failure(
                    state,
                    auth,
                    conversation_scope_id.as_str(),
                    result.message_id.as_str(),
                    result.message_seq,
                    "outbox_failed",
                    &store_error,
                );
            }
            record_message_side_effect_failure(
                state,
                auth,
                conversation_scope_id.as_str(),
                result.message_id.as_str(),
                result.message_seq,
                "realtime_delivery",
                &error,
            );
        }
    }

    Ok(result)
}

fn record_message_side_effect_store_failure(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    side_effect: &str,
    error: &ContractError,
) {
    let api_error = ApiError::from(error.clone());
    record_message_side_effect_failure(
        state,
        auth,
        conversation_id,
        message_id,
        message_seq,
        side_effect,
        &api_error,
    );
}

fn record_message_side_effect_failure(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    side_effect: &str,
    error: &ApiError,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: stable_local_audit_record_id(
                "audit_message_side_effect_failed_",
                format!("{message_id}:{side_effect}").as_str(),
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: stable_local_audit_aggregate_id("conversation", conversation_id),
            action: "message.side_effect_failed".into(),
            payload: Some(
                serde_json::json!({
                    "conversationId": conversation_id,
                    "messageId": message_id,
                    "messageSeq": message_seq,
                    "sideEffect": side_effect,
                    "errorCode": error.code,
                    "errorMessage": error.message,
                })
                .to_string(),
            ),
        },
    );
}

// Notification fanout mirrors the message publication boundary and keeps the
// addressed metadata explicit for downstream notification routing.
#[allow(clippy::too_many_arguments)]
fn fanout_message_notifications(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    message_id: &str,
    message_seq: u64,
    source_event_id: &str,
    message_type_name: &str,
    summary: Option<String>,
) {
    let notification_request = notification_service::RequestMessagePostedNotifications {
        source_event_id: source_event_id.into(),
        conversation_id: conversation_id.into(),
        message_id: message_id.into(),
        message_seq,
        message_type: message_type_name.into(),
        summary,
    };
    let outbox_record = match side_effect_outbox::record_pending_message_notification_side_effect(
        state,
        auth,
        notification_request.clone(),
    ) {
        Ok(record) => Some(record),
        Err(error) => {
            record_message_side_effect_store_failure(
                state,
                auth,
                conversation_id,
                message_id,
                message_seq,
                "notification_outbox_persist",
                &error,
            );
            None
        }
    };

    match state
        .notification_runtime
        .request_message_posted_notifications(auth, notification_request)
    {
        Ok(_) => {
            if let Some(record) = outbox_record.as_ref()
                && let Err(error) = side_effect_outbox::mark_message_side_effect_delivered(
                    state,
                    record.outbox_id.as_str(),
                )
            {
                record_message_side_effect_store_failure(
                    state,
                    auth,
                    conversation_id,
                    message_id,
                    message_seq,
                    "notification_outbox_delivered",
                    &error,
                );
            }
        }
        Err(error) => {
            if let Some(record) = outbox_record.as_ref()
                && let Err(store_error) =
                    side_effect_outbox::mark_message_notification_side_effect_failed(
                        state,
                        record.outbox_id.as_str(),
                        &error,
                    )
            {
                record_message_side_effect_store_failure(
                    state,
                    auth,
                    conversation_id,
                    message_id,
                    message_seq,
                    "notification_outbox_failed",
                    &store_error,
                );
            }
            record_message_side_effect_failure(
                state,
                auth,
                conversation_id,
                message_id,
                message_seq,
                "notification_delivery",
                &ApiError::service_unavailable(
                    error.code(),
                    format!(
                        "message notification side-effect failed: {}",
                        error.message()
                    ),
                ),
            );
        }
    }
}

pub(super) fn publish_realtime_conversation_message_event(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    event_type: &str,
    payload: String,
) -> Result<(), ApiError> {
    let recipients =
        conversation_member_principal_recipients_from_auth_context(state, auth, conversation_id)?;
    publish_realtime_event_to_recipients(
        state,
        auth,
        recipients,
        "conversation",
        conversation_id,
        event_type,
        payload,
    )
}

pub(super) fn publish_realtime_event_to_scope(
    state: &AppState,
    auth: &AppContext,
    scope_type: &str,
    scope_id: &str,
    event_type: &str,
    payload: String,
) -> Result<(), ApiError> {
    let recipients = if scope_type == "conversation" {
        conversation_member_principal_recipients_from_auth_context(state, auth, scope_id)?
    } else {
        BTreeSet::from([NotificationRecipientView {
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
        }])
    };
    publish_realtime_event_to_recipients(
        state, auth, recipients, scope_type, scope_id, event_type, payload,
    )
}

pub(super) fn publish_realtime_membership_event(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    event_type: &str,
    payload: String,
    base_recipients: BTreeSet<NotificationRecipientView>,
    additional_recipients: BTreeSet<NotificationRecipientView>,
) -> Result<(), ApiError> {
    let mut recipients = base_recipients;
    recipients.extend(additional_recipients);
    publish_realtime_event_to_recipients(
        state,
        auth,
        recipients,
        "conversation",
        conversation_id,
        event_type,
        payload,
    )
}

pub(super) fn publish_realtime_agent_handoff_status_changed_event(
    state: &AppState,
    auth: &AppContext,
    previous_state: &AgentHandoffStateView,
    current_state: &AgentHandoffStateView,
) -> Result<(), ApiError> {
    let changed_at = handoff_lifecycle_changed_at(current_state)
        .expect("agent handoff lifecycle state should expose a changed timestamp");
    let recipients = conversation_member_principal_recipients_from_auth_context(
        state,
        auth,
        current_state.conversation_id.as_str(),
    )?;

    publish_realtime_event_to_recipients(
        state,
        auth,
        recipients,
        "conversation",
        current_state.conversation_id.as_str(),
        "conversation.agent_handoff_status_changed",
        serde_json::json!({
            "tenantId": auth.tenant_id.as_str(),
            "conversationId": current_state.conversation_id.as_str(),
            "previousStatus": previous_state.status.as_str(),
            "currentStatus": current_state.status.as_str(),
            "changedBy": {
                "id": auth.actor_id.as_str(),
                "kind": auth.actor_kind.as_str(),
            },
            "changedAt": changed_at,
            "state": current_state,
        })
        .to_string(),
    )
}

pub(super) fn publish_realtime_stream_frame_event(
    state: &AppState,
    auth: &AppContext,
    frame: &im_domain_core::stream::StreamFrame,
) -> Result<(), ApiError> {
    let recipients = stream_target_principal_recipients(
        state,
        auth,
        frame.scope_kind.as_str(),
        frame.scope_id.as_str(),
    )?;

    publish_realtime_event_to_recipients(
        state,
        auth,
        recipients,
        "stream",
        frame.stream_id.as_str(),
        "stream.frame.appended",
        serde_json::json!({
            "streamId": frame.stream_id,
            "streamType": frame.stream_type,
            "scopeKind": frame.scope_kind,
            "scopeId": frame.scope_id,
            "frameSeq": frame.frame_seq,
            "frameType": frame.frame_type,
        })
        .to_string(),
    )
}

pub(super) fn publish_realtime_stream_lifecycle_event(
    state: &AppState,
    auth: &AppContext,
    session: &im_domain_core::stream::StreamSession,
    event_type: &str,
    reason: Option<String>,
) -> Result<(), ApiError> {
    let recipients = stream_target_principal_recipients(
        state,
        auth,
        session.scope_kind.as_str(),
        session.scope_id.as_str(),
    )?;

    publish_realtime_event_to_recipients(
        state,
        auth,
        recipients,
        "stream",
        session.stream_id.as_str(),
        event_type,
        serde_json::json!({
            "streamId": session.stream_id,
            "streamType": session.stream_type,
            "scopeKind": session.scope_kind,
            "scopeId": session.scope_id,
            "state": session.state.as_wire_value(),
            "lastFrameSeq": session.last_frame_seq,
            "lastCheckpointSeq": session.last_checkpoint_seq,
            "resultMessageId": session.result_message_id,
            "closedAt": session.closed_at,
            "reason": reason,
        })
        .to_string(),
    )
}

fn stream_target_principal_recipients(
    state: &AppState,
    auth: &AppContext,
    scope_kind: &str,
    scope_id: &str,
) -> Result<BTreeSet<NotificationRecipientView>, ApiError> {
    if scope_kind == "conversation" {
        conversation_member_principal_recipients_from_auth_context(state, auth, scope_id)
    } else {
        Ok(BTreeSet::from([NotificationRecipientView {
            principal_id: auth.actor_id.clone(),
            principal_kind: auth.actor_kind.clone(),
        }]))
    }
}

pub(super) fn conversation_member_principal_recipients_from_auth_context(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
) -> Result<BTreeSet<NotificationRecipientView>, ApiError> {
    Ok(state
        .projection_service
        .active_conversation_principal_recipients_from_auth_context(auth, conversation_id)?
        .into_iter()
        .collect::<BTreeSet<_>>())
}

fn publish_realtime_event_to_recipients(
    state: &AppState,
    auth: &AppContext,
    recipients: BTreeSet<NotificationRecipientView>,
    scope_type: &str,
    scope_id: &str,
    event_type: &str,
    payload: String,
) -> Result<(), ApiError> {
    let mut delivery_errors = Vec::new();
    for target in state
        .projection_service
        .realtime_fanout_targets_for_recipients_from_auth_context(auth, recipients)
    {
        let delivery = state
            .realtime_cluster
            .publish_device_event_for_principal_kind(
                state.node_id.as_str(),
                auth.tenant_id.as_str(),
                target.principal_id.as_str(),
                target.principal_kind.as_str(),
                target.device_id.as_str(),
                scope_type,
                scope_id,
                event_type,
                payload.clone(),
            );
        if let Some(error_code) = delivery.delivery_error_code.as_deref() {
            delivery_errors.push(format!(
                "principal_kind={} principal_id={} device_id={} target_node_id={} route_state={} error_code={} error_message={}",
                    target.principal_kind,
                    target.principal_id,
                    target.device_id,
                    delivery.target_node_id,
                    delivery.route_state,
                    error_code,
                    delivery.delivery_error_message.unwrap_or_default()
            ));
        }
    }
    if !delivery_errors.is_empty() {
        return Err(ApiError::service_unavailable(
            "realtime_delivery_failed",
            format!(
                "realtime delivery failed for {} target(s): {}",
                delivery_errors.len(),
                delivery_errors.join("; ")
            ),
        ));
    }
    Ok(())
}

fn handoff_lifecycle_changed_at(state: &AgentHandoffStateView) -> Option<String> {
    match state.status.as_str() {
        "accepted" => state.accepted_at.clone(),
        "resolved" => state.resolved_at.clone(),
        "closed" => state.closed_at.clone(),
        _ => None,
    }
}

pub(super) fn build_message_body(
    summary: Option<String>,
    text: Option<String>,
    reply_to: Option<im_domain_core::message::MessageReplyReference>,
    parts: Vec<ContentPart>,
    render_hints: BTreeMap<String, String>,
) -> Result<MessageBody, ApiError> {
    let mut resolved_parts = Vec::new();
    if let Some(text) = text
        && !text.trim().is_empty()
    {
        resolved_parts.push(ContentPart::text(text));
    }
    resolved_parts.extend(parts);

    if resolved_parts.is_empty() {
        return Err(ApiError::bad_request(
            "message_body_empty",
            "message body must contain text or parts",
        ));
    }

    Ok(MessageBody {
        summary,
        parts: resolved_parts,
        render_hints,
        reply_to,
    }
    .with_derived_summary())
}

pub(super) fn emit_rtc_signal_message(
    state: &AppState,
    auth: &AppContext,
    session: &sdkwork_rtc_core::RtcSession,
    signal_type: &'static str,
) -> Result<(), ApiError> {
    let Some(conversation_id) = session.conversation_id.clone() else {
        return Ok(());
    };

    let payload = serde_json::json!({
        "rtcSessionId": session.rtc_session_id,
        "conversationId": session.conversation_id,
        "rtcMode": session.rtc_mode,
        "state": session.state,
        "signalingStreamId": session.signaling_stream_id,
        "artifactMessageId": session.artifact_message_id,
    })
    .to_string();

    post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        None,
        MessageType::Signal,
        MessageBody {
            summary: Some(signal_type.into()),
            parts: vec![ContentPart::Signal(SignalPart {
                signal_type: signal_type.into(),
                schema_ref: Some("rtc.signal.v1".into()),
                payload,
            })],
            render_hints: BTreeMap::from([("channel".into(), "rtc".into())]),
            reply_to: None,
        },
    )
    .map(|_| ())
}

pub(super) fn emit_rtc_custom_signal_message(
    state: &AppState,
    auth: &AppContext,
    signal: &sdkwork_rtc_core::RtcSignalEvent,
) -> Result<(), ApiError> {
    let Some(conversation_id) = signal.conversation_id.clone() else {
        return Ok(());
    };

    let signal_payload = serde_json::from_str::<serde_json::Value>(signal.payload.as_str())
        .unwrap_or_else(|_| serde_json::Value::String(signal.payload.clone()));
    let payload = serde_json::json!({
        "rtcSessionId": signal.rtc_session_id,
        "conversationId": signal.conversation_id,
        "rtcMode": signal.rtc_mode,
        "signalingStreamId": signal.signaling_stream_id,
        "signalType": signal.signal_type,
        "signalPayload": signal_payload,
    })
    .to_string();

    post_message_with_side_effects(
        state,
        auth,
        conversation_id,
        None,
        MessageType::Signal,
        MessageBody {
            summary: Some(signal.signal_type.clone()),
            parts: vec![ContentPart::Signal(SignalPart {
                signal_type: signal.signal_type.clone(),
                schema_ref: signal
                    .schema_ref
                    .clone()
                    .or_else(|| Some("rtc.signal.v1".into())),
                payload,
            })],
            render_hints: BTreeMap::from([("channel".into(), "rtc".into())]),
            reply_to: None,
        },
    )
    .map(|_| ())
}

pub(super) fn record_membership_audit(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    action: &str,
    member: &ConversationMember,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!("audit_{}_{}", action.replace('.', "_"), member.member_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: action.into(),
            payload: Some(
                serde_json::json!({
                    "memberId": member.member_id,
                    "principalId": member.principal_id,
                    "principalKind": member.principal_kind,
                    "role": member.role,
                    "state": member.state,
                })
                .to_string(),
            ),
        },
    );
}

pub(super) fn record_owner_transfer_audit(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    transfer: &TransferConversationOwnerResult,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!(
                "audit_conversation_owner_transferred_{}",
                transfer.new_owner.member_id
            ),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: "conversation.owner_transferred".into(),
            payload: Some(
                serde_json::json!({
                    "previousOwnerMemberId": transfer.previous_owner.member_id,
                    "previousOwnerPrincipalId": transfer.previous_owner.principal_id,
                    "previousOwnerRole": transfer.previous_owner.role,
                    "newOwnerMemberId": transfer.new_owner.member_id,
                    "newOwnerPrincipalId": transfer.new_owner.principal_id,
                    "newOwnerRole": transfer.new_owner.role,
                    "transferredAt": transfer.transferred_at,
                })
                .to_string(),
            ),
        },
    );
}

pub(super) fn record_member_role_change_audit(
    state: &AppState,
    auth: &AppContext,
    conversation_id: &str,
    change: &ChangeConversationMemberRoleResult,
) {
    let _ = state.audit_runtime.record_anchor(
        auth,
        RecordAuditAnchor {
            record_id: format!("audit_{}", change.event_id),
            aggregate_type: "conversation".into(),
            aggregate_id: conversation_id.into(),
            action: "conversation.member_role_changed".into(),
            payload: Some(
                serde_json::json!({
                    "previousMemberId": change.previous_member.member_id,
                    "previousPrincipalId": change.previous_member.principal_id,
                    "previousRole": change.previous_member.role,
                    "updatedMemberId": change.updated_member.member_id,
                    "updatedPrincipalId": change.updated_member.principal_id,
                    "updatedRole": change.updated_member.role,
                    "changedAt": change.changed_at,
                })
                .to_string(),
            ),
        },
    );
}
