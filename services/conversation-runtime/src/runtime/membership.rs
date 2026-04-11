use super::*;
use std::collections::BTreeMap;

const SHARED_HISTORY_LINK_ATTRIBUTE_KEYS: [&str; 3] = [
    "sharedChannelPolicyId",
    "externalConnectionId",
    "externalMemberId",
];

fn has_non_empty_shared_history_link_value(
    attributes: &BTreeMap<String, String>,
    key: &str,
) -> bool {
    attributes
        .get(key)
        .is_some_and(|value| !value.trim().is_empty())
}

fn resolve_shared_history_linked_member(
    attributes: &BTreeMap<String, String>,
) -> Result<bool, RuntimeError> {
    let present_count = SHARED_HISTORY_LINK_ATTRIBUTE_KEYS
        .iter()
        .filter(|key| has_non_empty_shared_history_link_value(attributes, key))
        .count();
    if present_count == 0 {
        return Ok(false);
    }
    if present_count != SHARED_HISTORY_LINK_ATTRIBUTE_KEYS.len() {
        return Err(RuntimeError::InvalidInput(
            "shared history external-linked member requires sharedChannelPolicyId, externalConnectionId, and externalMemberId".into(),
        ));
    }

    Ok(true)
}

fn shared_history_link_attributes(
    shared_channel_policy_id: &str,
    external_connection_id: &str,
    external_member_id: &str,
) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "sharedChannelPolicyId".into(),
            shared_channel_policy_id.into(),
        ),
        ("externalConnectionId".into(), external_connection_id.into()),
        ("externalMemberId".into(), external_member_id.into()),
    ])
}

fn shared_history_link_matches(
    member: &ConversationMember,
    command: &SyncSharedChannelLinkedMemberCommand,
) -> bool {
    member.role == MembershipRole::Guest
        && member.state == MembershipState::Linked
        && member.principal_kind == command.local_actor_kind
        && member
            .attributes
            .get("sharedChannelPolicyId")
            .map(String::as_str)
            == Some(command.shared_channel_policy_id.as_str())
        && member
            .attributes
            .get("externalConnectionId")
            .map(String::as_str)
            == Some(command.external_connection_id.as_str())
        && member
            .attributes
            .get("externalMemberId")
            .map(String::as_str)
            == Some(command.external_member_id.as_str())
}

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn list_members_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, RuntimeError> {
        self.require_active_member_from_auth_context(auth, conversation_id)?;
        self.list_members(auth.tenant_id.as_str(), conversation_id)
    }

    pub fn list_messages_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        self.list_messages(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
        )
    }

    pub fn read_cursor_view_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        self.read_cursor_view(
            auth.tenant_id.as_str(),
            conversation_id,
            auth.actor_id.as_str(),
        )
    }

    pub fn add_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        principal_id: String,
        principal_kind: String,
        role: MembershipRole,
        attributes: BTreeMap<String, String>,
    ) -> Result<ConversationMember, RuntimeError> {
        self.add_member_with_actor_kind_and_attributes(
            AddConversationMemberCommand::from_auth_context(
                auth,
                conversation_id,
                principal_id,
                principal_kind,
                role,
            ),
            auth.actor_kind.as_str(),
            attributes,
        )
    }

    pub fn sync_shared_channel_linked_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        shared_channel_policy_id: String,
        external_connection_id: String,
        local_actor_id: String,
        local_actor_kind: String,
        external_member_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.sync_shared_channel_linked_member_with_requester_kind(
            SyncSharedChannelLinkedMemberCommand::from_auth_context(
                auth,
                conversation_id,
                shared_channel_policy_id,
                external_connection_id,
                local_actor_id,
                local_actor_kind,
                external_member_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn sync_shared_channel_linked_member(
        &self,
        command: SyncSharedChannelLinkedMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        self.sync_shared_channel_linked_member_with_requester_kind(command, "system")
    }

    pub fn add_member(
        &self,
        command: AddConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.invited_by.as_str(),
            )?
            .principal_kind;
        self.add_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn add_member_with_actor_kind(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        self.add_member_with_actor_kind_and_attributes(command, actor_kind, BTreeMap::new())
    }

    pub fn sync_shared_channel_linked_member_with_requester_kind(
        &self,
        command: SyncSharedChannelLinkedMemberCommand,
        requester_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        policy::ensure_shared_channel_sync_requester_kind(requester_kind)?;

        if command.conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires conversation_id".into(),
            ));
        }
        if command.shared_channel_policy_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires shared_channel_policy_id".into(),
            ));
        }
        if command.external_connection_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires external_connection_id".into(),
            ));
        }
        if command.local_actor_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires local_actor_id".into(),
            ));
        }
        if command.local_actor_kind.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires local_actor_kind".into(),
            ));
        }
        if command.external_member_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires external_member_id".into(),
            ));
        }
        if command.synced_by.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "shared channel linked-member sync requires synced_by actor identity".into(),
            ));
        }

        let attributes = shared_history_link_attributes(
            command.shared_channel_policy_id.as_str(),
            command.external_connection_id.as_str(),
            command.external_member_id.as_str(),
        );
        resolve_shared_history_linked_member(&attributes)?;

        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, ordering_seq, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let history_visibility = conversation
                .aggregate
                .policy()
                .map(|policy| policy.history_visibility.as_str())
                .unwrap_or("joined");
            if history_visibility != "shared" {
                return Err(RuntimeError::InvalidInput(format!(
                    "shared channel linked-member sync requires history_visibility=shared, got {history_visibility}"
                )));
            }

            if let Some(current_member) = conversation
                .roster
                .resolve_current_member(command.local_actor_id.as_str())
            {
                if shared_history_link_matches(&current_member, &command) {
                    return Ok(current_member);
                }

                return Err(RuntimeError::Conflict(format!(
                    "principal {} is already materialized as conversation member {} with incompatible shared-channel link truth",
                    command.local_actor_id, current_member.member_id
                )));
            }

            let member_episode = next_member_episode(conversation, command.local_actor_id.as_str());
            let mut member = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_episode_id(
                    command.conversation_id.as_str(),
                    command.local_actor_id.as_str(),
                    member_episode,
                ),
                command.local_actor_id.as_str(),
                command.local_actor_kind.as_str(),
                MembershipRole::Guest,
                Some(command.synced_by.clone()),
                conversation_timestamp(),
                attributes,
            );
            member.state = MembershipState::Linked;

            let ordering_seq = conversation.aggregate.next_member_epoch();
            upsert_member(conversation, member.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&member));
            let retention_class = conversation_retention_class(conversation);
            (member, ordering_seq, retention_class)
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            member.clone(),
            ordering_seq,
            retention_class.as_str(),
            command.synced_by.as_str(),
            requester_kind,
        ))?;

        Ok(member)
    }

    fn add_member_with_actor_kind_and_attributes(
        &self,
        command: AddConversationMemberCommand,
        actor_kind: &str,
        attributes: BTreeMap<String, String>,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let invited_by_member =
                resolve_active_member(conversation, command.invited_by.as_str())?;
            policy::ensure_actor_kind_matches_member(&invited_by_member, actor_kind)?;
            policy::ensure_member_add_actor_allowed(conversation, &invited_by_member)?;
            let history_visibility = conversation
                .aggregate
                .policy()
                .map(|policy| policy.history_visibility.as_str())
                .unwrap_or("joined");

            if conversation
                .roster
                .resolve_current_member(command.principal_id.as_str())
                .is_some()
            {
                return Err(RuntimeError::MemberAlreadyExists(command.principal_id));
            }
            policy::ensure_member_add_request_allowed(
                conversation,
                &invited_by_member,
                &command.role,
            )?;
            let member_episode = next_member_episode(conversation, command.principal_id.as_str());
            let shared_history_linked = resolve_shared_history_linked_member(&attributes)?;

            let mut member = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_episode_id(
                    command.conversation_id.as_str(),
                    command.principal_id.as_str(),
                    member_episode,
                ),
                command.principal_id.as_str(),
                command.principal_kind.as_str(),
                command.role,
                Some(command.invited_by.clone()),
                conversation_timestamp(),
                attributes,
            );
            if history_visibility == "invited" {
                member.state = MembershipState::Invited;
            } else if history_visibility == "shared" && shared_history_linked {
                member.state = MembershipState::Linked;
            }

            let member_epoch = conversation.aggregate.next_member_epoch();
            upsert_member(conversation, member.clone());
            upsert_read_cursor(conversation, build_default_read_cursor(&member));
            let retention_class = conversation_retention_class(conversation);
            (
                member,
                member_epoch,
                invited_by_member.principal_kind.clone(),
                retention_class,
            )
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            member.clone(),
            member_epoch,
            retention_class.as_str(),
            command.invited_by.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn remove_member(
        &self,
        command: RemoveConversationMemberCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.removed_by.as_str(),
            )?
            .principal_kind;
        self.remove_member_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn remove_member_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        member_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.remove_member_with_actor_kind(
            RemoveConversationMemberCommand::from_auth_context(auth, conversation_id, member_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn remove_member_with_actor_kind(
        &self,
        command: RemoveConversationMemberCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let removed_by_member =
                resolve_active_member(conversation, command.removed_by.as_str())?;
            policy::ensure_actor_kind_matches_member(&removed_by_member, actor_kind)?;

            let mut member = conversation
                .roster
                .member(command.member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.member_id.clone()))?;
            policy::ensure_current_active_member_target(conversation, &member)?;
            policy::ensure_member_remove_allowed(conversation, &removed_by_member, &member)?;
            member.state = MembershipState::Removed;
            member.removed_at = Some(conversation_timestamp());

            let member_epoch = conversation.aggregate.next_member_epoch();
            conversation.roster.deactivate_member(member.clone());
            let retention_class = conversation_retention_class(conversation);
            (
                member,
                member_epoch,
                removed_by_member.principal_kind.clone(),
                retention_class,
            )
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_removed",
            member.clone(),
            member_epoch,
            retention_class.as_str(),
            command.removed_by.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn leave_conversation(
        &self,
        command: LeaveConversationCommand,
    ) -> Result<ConversationMember, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.leave_conversation_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn leave_conversation_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
    ) -> Result<ConversationMember, RuntimeError> {
        self.leave_conversation_with_actor_kind(
            LeaveConversationCommand::from_auth_context(auth, conversation_id),
            auth.actor_kind.as_str(),
        )
    }

    pub fn leave_conversation_with_actor_kind(
        &self,
        command: LeaveConversationCommand,
        actor_kind: &str,
    ) -> Result<ConversationMember, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (member, member_epoch, actor_kind, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let leaving_member =
                resolve_active_member(conversation, command.principal_id.as_str())?;
            policy::ensure_actor_kind_matches_member(&leaving_member, actor_kind)?;
            policy::ensure_member_leave_allowed(conversation, &leaving_member)?;

            let mut member = leaving_member.clone();
            member.state = MembershipState::Left;
            member.removed_at = Some(conversation_timestamp());

            let member_epoch = conversation.aggregate.next_member_epoch();
            conversation.roster.deactivate_member(member.clone());
            let retention_class = conversation_retention_class(conversation);
            (
                member,
                member_epoch,
                leaving_member.principal_kind.clone(),
                retention_class,
            )
        };

        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_left",
            member.clone(),
            member_epoch,
            retention_class.as_str(),
            command.principal_id.as_str(),
            actor_kind.as_str(),
        ))?;

        Ok(member)
    }

    pub fn transfer_conversation_owner(
        &self,
        command: TransferConversationOwnerCommand,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.transferred_by.as_str(),
            )?
            .principal_kind;
        self.transfer_conversation_owner_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn transfer_conversation_owner_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        target_member_id: String,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        self.transfer_conversation_owner_with_actor_kind(
            TransferConversationOwnerCommand::from_auth_context(
                auth,
                conversation_id,
                target_member_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn transfer_conversation_owner_with_actor_kind(
        &self,
        command: TransferConversationOwnerCommand,
        actor_kind: &str,
    ) -> Result<TransferConversationOwnerResult, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (payload, ordering_seq, actor_kind, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let owner_member =
                resolve_active_member(conversation, command.transferred_by.as_str())?;
            policy::ensure_actor_kind_matches_member(&owner_member, actor_kind)?;
            let target_member = conversation
                .roster
                .member(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            policy::ensure_owner_transfer_allowed(conversation, &owner_member, &target_member)?;

            let transferred_at = conversation_timestamp();
            let actor_kind = owner_member.principal_kind.clone();
            let previous_owner = ConversationMember {
                role: MembershipRole::Admin,
                ..owner_member
            };
            let new_owner = ConversationMember {
                role: MembershipRole::Owner,
                ..target_member
            };

            conversation.roster.upsert_member(previous_owner.clone());
            conversation.roster.upsert_member(new_owner.clone());
            let ordering_seq = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);

            (
                TransferConversationOwnerPayload {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    previous_owner,
                    new_owner,
                    transferred_at,
                },
                ordering_seq,
                actor_kind,
                retention_class,
            )
        };

        let event = build_owner_transfer_envelope(
            payload.clone(),
            ordering_seq,
            retention_class.as_str(),
            command.transferred_by.as_str(),
            actor_kind.as_str(),
        );
        self.journal.append(event.clone())?;

        Ok(TransferConversationOwnerResult {
            event_id: event.event_id,
            transferred_at: payload.transferred_at.clone(),
            previous_owner: payload.previous_owner,
            new_owner: payload.new_owner,
        })
    }

    pub fn change_conversation_member_role(
        &self,
        command: ChangeConversationMemberRoleCommand,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.changed_by.as_str(),
            )?
            .principal_kind;
        self.change_conversation_member_role_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn change_conversation_member_role_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        target_member_id: String,
        new_role: MembershipRole,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        self.change_conversation_member_role_with_actor_kind(
            ChangeConversationMemberRoleCommand::from_auth_context(
                auth,
                conversation_id,
                target_member_id,
                new_role,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn change_conversation_member_role_with_actor_kind(
        &self,
        command: ChangeConversationMemberRoleCommand,
        actor_kind: &str,
    ) -> Result<ChangeConversationMemberRoleResult, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let (payload, ordering_seq, actor_kind, retention_class) = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            let actor_member = resolve_active_member(conversation, command.changed_by.as_str())?;
            policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let target_member = conversation
                .roster
                .member(command.target_member_id.as_str())
                .cloned()
                .ok_or_else(|| RuntimeError::MemberNotFound(command.target_member_id.clone()))?;
            policy::ensure_current_active_member_target(conversation, &target_member)?;
            policy::ensure_member_role_change_allowed(
                conversation,
                &actor_member,
                &target_member,
                &command.new_role,
            )?;

            let changed_at = conversation_timestamp();
            let previous_member = target_member.clone();
            let updated_member = ConversationMember {
                role: command.new_role.clone(),
                ..target_member
            };
            conversation.roster.upsert_member(updated_member.clone());
            let ordering_seq = conversation.aggregate.next_member_epoch();
            let retention_class = conversation_retention_class(conversation);

            (
                ChangeConversationMemberRolePayload {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    previous_member,
                    updated_member,
                    changed_at,
                },
                ordering_seq,
                actor_member.principal_kind.clone(),
                retention_class,
            )
        };

        let event = build_member_role_changed_envelope(
            payload.clone(),
            ordering_seq,
            retention_class.as_str(),
            command.changed_by.as_str(),
            actor_kind.as_str(),
        );
        self.journal.append(event.clone())?;

        Ok(ChangeConversationMemberRoleResult {
            event_id: event.event_id,
            changed_at: payload.changed_at.clone(),
            previous_member: payload.previous_member,
            updated_member: payload.updated_member,
        })
    }

    pub fn list_members(
        &self,
        tenant_id: &str,
        conversation_id: &str,
    ) -> Result<Vec<ConversationMember>, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;

        Ok(conversation
            .roster
            .members()
            .values()
            .filter(|member| member.is_active())
            .cloned()
            .collect())
    }

    pub fn update_read_cursor(
        &self,
        command: UpdateReadCursorCommand,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let actor_kind = self
            .require_active_member(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                command.principal_id.as_str(),
            )?
            .principal_kind;
        self.update_read_cursor_with_actor_kind(command, actor_kind.as_str())
    }

    pub fn update_read_cursor_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        read_seq: u64,
        last_read_message_id: Option<String>,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        self.update_read_cursor_with_actor_kind(
            UpdateReadCursorCommand::from_auth_context(
                auth,
                conversation_id,
                read_seq,
                last_read_message_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn update_read_cursor_with_actor_kind(
        &self,
        command: UpdateReadCursorCommand,
        actor_kind: &str,
    ) -> Result<ConversationReadCursor, RuntimeError> {
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let mut changed_event: Option<(ConversationReadCursor, String, String)> = None;
        let cursor = {
            let mut state = self.state.lock().expect("runtime state should lock");
            let conversation =
                state
                    .conversations
                    .get_mut(scope_key.as_str())
                    .ok_or_else(|| {
                        RuntimeError::ConversationNotFound(command.conversation_id.clone())
                    })?;
            if command.read_seq > conversation.message_log.high_watermark() {
                return Err(RuntimeError::ReadCursorInvalid(format!(
                    "read cursor exceeds conversation high watermark: {} > {}",
                    command.read_seq,
                    conversation.message_log.high_watermark()
                )));
            }

            let actor_member = resolve_active_member(conversation, command.principal_id.as_str())?;
            policy::ensure_actor_kind_matches_member(&actor_member, actor_kind)?;
            let retention_class = conversation_retention_class(conversation);
            let member_id = actor_member.member_id.clone();
            let cursor = conversation
                .roster
                .read_cursors_mut()
                .entry(member_id.clone())
                .or_insert_with(|| ConversationReadCursor {
                    tenant_id: command.tenant_id.clone(),
                    conversation_id: command.conversation_id.clone(),
                    member_id: member_id.clone(),
                    principal_id: command.principal_id.clone(),
                    read_seq: 0,
                    last_read_message_id: None,
                    updated_at: conversation_timestamp(),
                });

            if command.read_seq > cursor.read_seq {
                cursor.read_seq = command.read_seq;
                cursor.last_read_message_id = command.last_read_message_id.clone();
                cursor.updated_at = conversation_timestamp();
                changed_event = Some((
                    cursor.clone(),
                    actor_member.principal_kind.clone(),
                    retention_class.clone(),
                ));
            }

            cursor.clone()
        };

        if let Some((changed_cursor, actor_kind, retention_class)) = changed_event {
            self.journal.append(build_read_cursor_envelope(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                changed_cursor.clone(),
                changed_cursor.read_seq,
                retention_class.as_str(),
                command.principal_id.as_str(),
                actor_kind.as_str(),
            ))?;
        }

        Ok(cursor)
    }

    pub fn read_cursor_view(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<ConversationReadCursorView, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        let member_id = resolve_active_member_id(conversation, principal_id)?;
        let cursor = conversation
            .roster
            .read_cursors()
            .get(member_id.as_str())
            .ok_or_else(|| {
                RuntimeError::PermissionDenied(format!(
                    "principal is not active conversation member: {principal_id}"
                ))
            })?;

        Ok(ConversationReadCursorView::from_cursor(
            cursor,
            conversation.message_log.unread_count_since(cursor.read_seq),
        ))
    }

    pub fn list_messages(
        &self,
        tenant_id: &str,
        conversation_id: &str,
        principal_id: &str,
    ) -> Result<MessageHistoryResult, RuntimeError> {
        let scope_key = conversation_scope_key(tenant_id, conversation_id);
        let state = self.state.lock().expect("runtime state should lock");
        let conversation = state
            .conversations
            .get(scope_key.as_str())
            .ok_or_else(|| RuntimeError::ConversationNotFound(conversation_id.into()))?;
        policy::ensure_history_read_allowed(conversation, principal_id)?;

        Ok(MessageHistoryResult {
            items: conversation.message_log.messages_in_order(),
            high_watermark: conversation.message_log.high_watermark(),
        })
    }
}
