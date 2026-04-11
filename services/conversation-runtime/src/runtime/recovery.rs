use super::*;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationCreatedPayload {
    conversation_type: String,
    business_type: Option<String>,
    business_id: Option<String>,
    source: Option<ChangeAgentHandoffStatusView>,
    target: Option<ChangeAgentHandoffStatusView>,
    handoff: Option<RecoveredConversationHandoffPayload>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RecoveredConversationHandoffPayload {
    session_id: String,
    reason: Option<String>,
    status: String,
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn apply_recovered_envelope(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        match envelope.event_type.as_str() {
            "conversation.created" => self.apply_recovered_conversation_created(envelope),
            "conversation.policy_applied" => {
                self.apply_recovered_conversation_policy_applied(envelope)
            }
            "conversation.member_joined" => self.apply_recovered_member_joined(envelope),
            "conversation.member_removed" | "conversation.member_left" => {
                self.apply_recovered_member_deactivated(envelope)
            }
            "conversation.read_cursor_updated" => self.apply_recovered_read_cursor(envelope),
            "conversation.owner_transferred" => self.apply_recovered_owner_transfer(envelope),
            "conversation.member_role_changed" => {
                self.apply_recovered_member_role_changed(envelope)
            }
            "conversation.agent_handoff_status_changed" => {
                self.apply_recovered_handoff_status_changed(envelope)
            }
            "message.posted" => self.apply_recovered_message_posted(envelope),
            "message.edited" => self.apply_recovered_message_edited(envelope),
            "message.recalled" => self.apply_recovered_message_recalled(envelope),
            "message.reaction_added" => self.apply_recovered_message_reaction_added(envelope),
            "message.reaction_removed" => self.apply_recovered_message_reaction_removed(envelope),
            "message.pin_added" => self.apply_recovered_message_pinned(envelope),
            "message.pin_removed" => self.apply_recovered_message_unpinned(envelope),
            _ => Ok(()),
        }
    }

    fn apply_recovered_conversation_created(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: RecoveredConversationCreatedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.created {}: {error}",
                    envelope.event_id
                ))
            })?;
        let business_binding = match (payload.business_type, payload.business_id) {
            (Some(business_type), Some(business_id)) => Some(ConversationBusinessBinding {
                business_type,
                business_id,
            }),
            _ => None,
        };
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let business_scope_key = business_binding.as_ref().map(|binding| {
            conversation_business_scope_key(
                envelope.tenant_id.as_str(),
                binding.business_type.as_str(),
                binding.business_id.as_str(),
            )
        });
        if let (Some(binding), Some(business_scope_key)) =
            (business_binding.as_ref(), business_scope_key.as_ref())
        {
            if let Some(existing_conversation_id) =
                state.business_index.get(business_scope_key.as_str())
                && existing_conversation_id != envelope.scope_id.as_str()
            {
                return Err(RuntimeError::Conflict(format!(
                    "replayed business binding {}/{} already mapped to conversation {existing_conversation_id}",
                    binding.business_type, binding.business_id
                )));
            }
        }

        {
            let conversation = state.conversations.entry(scope_key).or_default();
            conversation.aggregate = ConversationAggregateState::new(payload.conversation_type);
            if let Some(binding) = business_binding.clone() {
                conversation
                    .aggregate
                    .replace_business_binding(Some(binding));
            }
            if let (Some(source), Some(target), Some(handoff)) =
                (payload.source, payload.target, payload.handoff)
            {
                conversation
                    .aggregate
                    .replace_handoff_state(Some(AgentHandoffStateView {
                        tenant_id: envelope.tenant_id.clone(),
                        conversation_id: envelope.scope_id.clone(),
                        status: handoff.status,
                        source,
                        target,
                        handoff_session_id: handoff.session_id,
                        handoff_reason: handoff.reason,
                        accepted_at: None,
                        accepted_by: None,
                        resolved_at: None,
                        resolved_by: None,
                        closed_at: None,
                        closed_by: None,
                    }));
            }
        }
        if let Some(business_scope_key) = business_scope_key {
            state
                .business_index
                .insert(business_scope_key, envelope.scope_id.clone());
        }
        Ok(())
    }

    fn apply_recovered_conversation_policy_applied(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: ConversationPolicyAppliedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.policy_applied {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        let policy = payload.into_policy().normalize().map_err(|error| {
            RuntimeError::Conflict(format!(
                "failed to normalize replayed conversation policy {}: {error}",
                envelope.event_id
            ))
        })?;
        conversation
            .aggregate
            .observe_policy_epoch(envelope.ordering_seq);
        conversation.aggregate.replace_policy(Some(policy));
        Ok(())
    }

    fn apply_recovered_member_joined(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_joined {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, member.clone());
        conversation.roster.ensure_default_read_cursor(&member);
        Ok(())
    }

    fn apply_recovered_member_deactivated(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let member: ConversationMember =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay {} {}: {error}",
                    envelope.event_type, envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_member_epoch(envelope.ordering_seq);
        conversation.roster.deactivate_member(member);
        Ok(())
    }

    fn apply_recovered_read_cursor(&self, envelope: &CommitEnvelope) -> Result<(), RuntimeError> {
        let cursor: ConversationReadCursor = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.read_cursor_updated {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        upsert_read_cursor(conversation, cursor);
        Ok(())
    }

    fn apply_recovered_owner_transfer(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: TransferConversationOwnerPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.owner_transferred {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, payload.previous_owner);
        upsert_member(conversation, payload.new_owner);
        Ok(())
    }

    fn apply_recovered_member_role_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: ChangeConversationMemberRolePayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.member_role_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_member_epoch(envelope.ordering_seq);
        upsert_member(conversation, payload.updated_member);
        Ok(())
    }

    fn apply_recovered_handoff_status_changed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let payload: AgentHandoffStatusChangedPayload =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay conversation.agent_handoff_status_changed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .aggregate
            .observe_handoff_status_epoch(envelope.ordering_seq);
        conversation
            .aggregate
            .replace_handoff_state(Some(payload.state));
        Ok(())
    }

    fn apply_recovered_message_posted(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let message: Message =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.posted {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        {
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::Conflict(format!(
                            "cannot replay message.posted without conversation {}",
                            envelope.scope_id
                        ))
                    })?;
            conversation.message_log.store_posted(message.clone());
        }
        state.message_locator.register_message(&message);
        Ok(())
    }

    fn apply_recovered_message_edited(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let edited: MessageEdited =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.edited {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .message_log
            .apply_edited(&edited)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.edited without message {}",
                    edited.message_id
                ))
            })?;
        Ok(())
    }

    fn apply_recovered_message_recalled(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let recalled: MessageRecalled =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.recalled {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .message_log
            .apply_recalled(&recalled)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.recalled without message {}",
                    recalled.message_id
                ))
            })?;
        Ok(())
    }

    fn apply_recovered_message_reaction_added(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let reaction: MessageReactionAdded = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.reaction_added {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .message_log
            .apply_reaction_added(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_added without message {}",
                    reaction.message_id
                ))
            })?;
        Ok(())
    }

    fn apply_recovered_message_reaction_removed(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let reaction: MessageReactionRemoved = serde_json::from_str(envelope.payload.as_str())
            .map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.reaction_removed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .message_log
            .apply_reaction_removed(&reaction)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.reaction_removed without message {}",
                    reaction.message_id
                ))
            })?;
        Ok(())
    }

    fn apply_recovered_message_pinned(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let pin: MessagePinned =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.pin_added {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation.message_log.apply_pinned(&pin).ok_or_else(|| {
            RuntimeError::Conflict(format!(
                "cannot replay message.pin_added without message {}",
                pin.message_id
            ))
        })?;
        Ok(())
    }

    fn apply_recovered_message_unpinned(
        &self,
        envelope: &CommitEnvelope,
    ) -> Result<(), RuntimeError> {
        let pin: MessageUnpinned =
            serde_json::from_str(envelope.payload.as_str()).map_err(|error| {
                RuntimeError::Conflict(format!(
                    "failed to replay message.pin_removed {}: {error}",
                    envelope.event_id
                ))
            })?;
        let scope_key =
            conversation_scope_key(envelope.tenant_id.as_str(), envelope.scope_id.as_str());
        let mut state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get_mut(scope_key.as_str())
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay event without conversation {}",
                    envelope.scope_id
                ))
            })?;
        conversation
            .message_log
            .apply_unpinned(&pin)
            .ok_or_else(|| {
                RuntimeError::Conflict(format!(
                    "cannot replay message.pin_removed without message {}",
                    pin.message_id
                ))
            })?;
        Ok(())
    }
}
