use im_domain_core::conversation::ConversationScenario;
use im_domain_core::social::normalize_actor_pair;

use super::*;

impl<J> ConversationRuntime<J>
where
    J: CommitJournal,
{
    pub fn create_conversation_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        conversation_type: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_from_auth_context_with_creator_attributes(
            auth,
            conversation_id,
            conversation_type,
            BTreeMap::new(),
        )
    }

    pub fn create_conversation_from_auth_context_with_creator_attributes(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        conversation_type: String,
        creator_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind_and_attributes(
            CreateConversationCommand::from_auth_context(auth, conversation_id, conversation_type),
            auth.actor_kind.as_str(),
            creator_attributes,
        )
    }

    pub fn create_conversation(
        &self,
        command: CreateConversationCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind(command, "user")
    }

    pub fn create_conversation_with_creator_kind(
        &self,
        command: CreateConversationCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_conversation_with_creator_kind_and_attributes(
            command,
            creator_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_conversation_with_creator_kind_and_attributes(
        &self,
        command: CreateConversationCommand,
        creator_kind: &str,
        creator_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        policy::ensure_generic_creatable_conversation_type(command.conversation_type.as_str())?;
        let creator_id = command.creator_id.clone();
        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let creator_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), creator_id.as_str()),
            creator_id.as_str(),
            creator_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            creator_attributes,
        );
        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }
        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new(command.conversation_type.clone()),
            ..ConversationState::default()
        };
        let creator_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, creator_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&creator_member),
        );
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: creator_id.clone(),
                actor_kind: creator_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": command.conversation_type
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            creator_member.clone(),
            creator_ordering_seq,
            "standard",
            creator_id.as_str(),
            creator_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_agent_dialog(
        &self,
        command: CreateAgentDialogCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind(command, "user")
    }

    pub fn create_thread_conversation_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        parent_conversation_id: String,
        root_message_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_thread_conversation_with_creator_kind(
            CreateThreadConversationCommand::from_auth_context(
                auth,
                conversation_id,
                parent_conversation_id,
                root_message_id,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn create_thread_conversation_with_creator_kind(
        &self,
        command: CreateThreadConversationCommand,
        creator_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        if command.conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires conversation_id".into(),
            ));
        }
        if command.parent_conversation_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires parent_conversation_id".into(),
            ));
        }
        if command.root_message_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires root_message_id".into(),
            ));
        }
        if command.creator_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "thread conversation requires creator identity".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let parent_scope_key = conversation_scope_key(
            command.tenant_id.as_str(),
            command.parent_conversation_id.as_str(),
        );
        let business_binding = ConversationBusinessBinding {
            business_type: "thread".into(),
            business_id: command.root_message_id.clone(),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );

        let thread_members = {
            let mut state = self.state.lock().expect("runtime state should lock");
            if state.conversations.contains_key(scope_key.as_str()) {
                return Err(RuntimeError::ConversationAlreadyExists(
                    command.conversation_id.clone(),
                ));
            }
            if let Some(existing_conversation_id) =
                state.business_index.get(business_scope_key.as_str())
            {
                return Err(RuntimeError::Conflict(format!(
                    "thread root message {} already mapped to conversation {existing_conversation_id}",
                    command.root_message_id
                )));
            }

            let located_parent_id = state
                .message_locator
                .conversation_id(command.tenant_id.as_str(), command.root_message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.root_message_id.clone()))?
                .to_owned();
            if located_parent_id != command.parent_conversation_id {
                return Err(RuntimeError::InvalidInput(format!(
                    "thread root message {} does not belong to parent conversation {}",
                    command.root_message_id, command.parent_conversation_id
                )));
            }

            let parent_conversation = state
                .conversations
                .get(parent_scope_key.as_str())
                .ok_or_else(|| {
                    RuntimeError::ConversationNotFound(command.parent_conversation_id.clone())
                })?;
            if parent_conversation.aggregate.scenario() != ConversationScenario::Group {
                return Err(RuntimeError::ConversationTypeInvalid(format!(
                    "thread parent conversation {} must be group, got {}",
                    command.parent_conversation_id,
                    parent_conversation.aggregate.conversation_type()
                )));
            }
            let parent_creator =
                resolve_active_member(parent_conversation, command.creator_id.as_str())?;
            policy::ensure_actor_kind_matches_member(&parent_creator, creator_kind)?;
            let root_message = parent_conversation
                .message_log
                .message(command.root_message_id.as_str())
                .ok_or_else(|| RuntimeError::MessageNotFound(command.root_message_id.clone()))?;
            if root_message.recalled {
                return Err(RuntimeError::InvalidInput(format!(
                    "thread root message {} is already recalled",
                    command.root_message_id
                )));
            }
            let auto_subscribed_root_author = parent_conversation
                .roster
                .resolve_active_member(root_message.message.sender.id.as_str())
                .filter(|member| member.principal_id != command.creator_id);

            let thread_owner = build_conversation_member_with_attributes(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                member_id(
                    command.conversation_id.as_str(),
                    command.creator_id.as_str(),
                ),
                command.creator_id.as_str(),
                creator_kind,
                MembershipRole::Owner,
                Some(command.creator_id.clone()),
                created_at.clone(),
                BTreeMap::from([
                    ("threadRole".into(), "owner".into()),
                    (
                        "parentConversationId".into(),
                        command.parent_conversation_id.clone(),
                    ),
                    ("rootMessageId".into(), command.root_message_id.clone()),
                ]),
            );

            let mut thread_conversation = ConversationState {
                aggregate: ConversationAggregateState::new("thread"),
                ..ConversationState::default()
            };
            thread_conversation
                .aggregate
                .replace_business_binding(Some(business_binding.clone()));
            let mut thread_members = Vec::new();
            let owner_ordering_seq = thread_conversation.aggregate.next_member_epoch();
            upsert_member(&mut thread_conversation, thread_owner.clone());
            upsert_read_cursor(
                &mut thread_conversation,
                build_default_read_cursor(&thread_owner),
            );
            thread_members.push((thread_owner, owner_ordering_seq));

            if let Some(root_author) = auto_subscribed_root_author {
                let root_author_member = build_conversation_member_with_attributes(
                    command.tenant_id.as_str(),
                    command.conversation_id.as_str(),
                    member_id(
                        command.conversation_id.as_str(),
                        root_author.principal_id.as_str(),
                    ),
                    root_author.principal_id.as_str(),
                    root_author.principal_kind.as_str(),
                    MembershipRole::Member,
                    Some(command.creator_id.clone()),
                    created_at.clone(),
                    BTreeMap::from([
                        ("threadRole".into(), "root_author".into()),
                        (
                            "parentConversationId".into(),
                            command.parent_conversation_id.clone(),
                        ),
                        ("rootMessageId".into(), command.root_message_id.clone()),
                    ]),
                );
                let root_author_ordering_seq = thread_conversation.aggregate.next_member_epoch();
                upsert_member(&mut thread_conversation, root_author_member.clone());
                upsert_read_cursor(
                    &mut thread_conversation,
                    build_default_read_cursor(&root_author_member),
                );
                thread_members.push((root_author_member, root_author_ordering_seq));
            }

            state
                .conversations
                .insert(scope_key.clone(), thread_conversation);
            state
                .business_index
                .insert(business_scope_key, command.conversation_id.clone());
            debug_assert_eq!(owner_ordering_seq, 1);
            thread_members
        };

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.creator_id.clone(),
                actor_kind: creator_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "thread",
                "businessType": business_binding.business_type,
                "businessId": business_binding.business_id,
                "parentConversationId": command.parent_conversation_id,
                "rootMessageId": command.root_message_id
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        for (thread_member, ordering_seq) in thread_members {
            self.journal.append(build_member_envelope(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
                "conversation.member_joined",
                thread_member,
                ordering_seq,
                "standard",
                command.creator_id.as_str(),
                creator_kind,
            ))?;
        }

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn bind_direct_chat_conversation(
        &self,
        command: BindDirectChatConversationCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.bind_direct_chat_conversation_with_binder_kind(command, "system")
    }

    pub fn bind_direct_chat_conversation_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        direct_chat_id: String,
        left_actor_id: String,
        left_actor_kind: String,
        right_actor_id: String,
        right_actor_kind: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.bind_direct_chat_conversation_with_binder_kind(
            BindDirectChatConversationCommand::from_auth_context(
                auth,
                conversation_id,
                direct_chat_id,
                left_actor_id,
                left_actor_kind,
                right_actor_id,
                right_actor_kind,
            ),
            auth.actor_kind.as_str(),
        )
    }

    pub fn bind_direct_chat_conversation_with_binder_kind(
        &self,
        command: BindDirectChatConversationCommand,
        binder_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        policy::ensure_direct_chat_binding_requester_kind(binder_kind)?;

        if command.direct_chat_id.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "direct chat binding requires direct_chat_id".into(),
            ));
        }
        if command.bound_by.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "direct chat binding requires binder identity".into(),
            ));
        }
        if command.left_actor_kind.trim().is_empty() || command.right_actor_kind.trim().is_empty() {
            return Err(RuntimeError::InvalidInput(
                "direct chat binding requires actor kinds for both participants".into(),
            ));
        }

        let pair = normalize_actor_pair(
            command.left_actor_id.as_str(),
            command.right_actor_id.as_str(),
        )
        .map_err(|error| RuntimeError::InvalidInput(error.to_string()))?;
        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let business_binding = ConversationBusinessBinding {
            business_type: "direct_chat".into(),
            business_id: command.direct_chat_id.clone(),
        };
        let business_scope_key = conversation_business_scope_key(
            command.tenant_id.as_str(),
            business_binding.business_type.as_str(),
            business_binding.business_id.as_str(),
        );

        let anchor_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                pair.left_actor_id.as_str(),
            ),
            pair.left_actor_id.as_str(),
            command.left_actor_kind.as_str(),
            MembershipRole::Owner,
            None,
            created_at.clone(),
            BTreeMap::from([
                (
                    "businessType".into(),
                    business_binding.business_type.clone(),
                ),
                ("businessId".into(), business_binding.business_id.clone()),
                ("directChatId".into(), command.direct_chat_id.clone()),
                ("pairHash".into(), pair.pair_hash.clone()),
                ("directChatRole".into(), "anchor".into()),
                ("peerActorId".into(), pair.right_actor_id.clone()),
                ("peerActorKind".into(), command.right_actor_kind.clone()),
            ]),
        );
        let peer_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                pair.right_actor_id.as_str(),
            ),
            pair.right_actor_id.as_str(),
            command.right_actor_kind.as_str(),
            MembershipRole::Member,
            Some(pair.left_actor_id.clone()),
            created_at.clone(),
            BTreeMap::from([
                (
                    "businessType".into(),
                    business_binding.business_type.clone(),
                ),
                ("businessId".into(), business_binding.business_id.clone()),
                ("directChatId".into(), command.direct_chat_id.clone()),
                ("pairHash".into(), pair.pair_hash.clone()),
                ("directChatRole".into(), "peer".into()),
                ("peerActorId".into(), pair.left_actor_id.clone()),
                ("peerActorKind".into(), command.left_actor_kind.clone()),
            ]),
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }
        if let Some(existing_conversation_id) =
            state.business_index.get(business_scope_key.as_str())
        {
            return Err(RuntimeError::Conflict(format!(
                "business binding {}/{} already mapped to conversation {}",
                business_binding.business_type,
                business_binding.business_id,
                existing_conversation_id
            )));
        }

        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("direct"),
            ..ConversationState::default()
        };
        conversation
            .aggregate
            .replace_business_binding(Some(business_binding.clone()));
        let anchor_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, anchor_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&anchor_member));
        let peer_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, peer_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&peer_member));

        state.conversations.insert(scope_key, conversation);
        state
            .business_index
            .insert(business_scope_key, command.conversation_id.clone());
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.bound_by.clone(),
                actor_kind: binder_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "direct",
                "businessType": business_binding.business_type,
                "businessId": business_binding.business_id
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            anchor_member,
            anchor_ordering_seq,
            "standard",
            command.bound_by.as_str(),
            binder_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            peer_member,
            peer_ordering_seq,
            "standard",
            command.bound_by.as_str(),
            binder_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_agent_dialog_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        agent_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_from_auth_context_with_requester_attributes(
            auth,
            conversation_id,
            agent_id,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_dialog_from_auth_context_with_requester_attributes(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        agent_id: String,
        requester_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind_and_attributes(
            CreateAgentDialogCommand::from_auth_context(auth, conversation_id, agent_id),
            auth.actor_kind.as_str(),
            requester_attributes,
        )
    }

    pub fn create_agent_dialog_with_requester_kind(
        &self,
        command: CreateAgentDialogCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_dialog_with_requester_kind_and_attributes(
            command,
            requester_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_dialog_with_requester_kind_and_attributes(
        &self,
        command: CreateAgentDialogCommand,
        requester_kind: &str,
        requester_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        policy::ensure_agent_dialog_requester_kind(requester_kind)?;

        if command.agent_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "agent dialog requires target agent id".into(),
            ));
        }
        if command.requester_id == command.agent_id {
            return Err(RuntimeError::PermissionDenied(
                "agent dialog agent must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let mut requester_member_attributes =
            BTreeMap::from([("dialogRole".into(), "requester".into())]);
        requester_member_attributes.extend(requester_attributes);
        let requester_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            requester_member_attributes,
        );
        let agent_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.agent_id.as_str()),
            command.agent_id.as_str(),
            "agent",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            BTreeMap::from([
                ("agentId".into(), command.agent_id.clone()),
                ("dialogRole".into(), "assistant".into()),
            ]),
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("agent_dialog"),
            ..ConversationState::default()
        };
        let requester_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, requester_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&requester_member),
        );
        let agent_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, agent_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&agent_member));
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "agent_dialog"
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            requester_member,
            requester_ordering_seq,
            "standard",
            command.requester_id.as_str(),
            requester_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            agent_member,
            agent_ordering_seq,
            "standard",
            command.requester_id.as_str(),
            requester_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_system_channel(
        &self,
        command: CreateSystemChannelCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind(command, "system")
    }

    pub fn create_system_channel_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        subscriber_id: String,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_from_auth_context_with_subscriber_attributes(
            auth,
            conversation_id,
            subscriber_id,
            BTreeMap::new(),
        )
    }

    pub fn create_system_channel_from_auth_context_with_subscriber_attributes(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        subscriber_id: String,
        subscriber_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind_and_subscriber_attributes(
            CreateSystemChannelCommand::from_auth_context(auth, conversation_id, subscriber_id),
            auth.actor_kind.as_str(),
            subscriber_attributes,
        )
    }

    pub fn create_system_channel_with_requester_kind(
        &self,
        command: CreateSystemChannelCommand,
        requester_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_system_channel_with_requester_kind_and_subscriber_attributes(
            command,
            requester_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_system_channel_with_requester_kind_and_subscriber_attributes(
        &self,
        command: CreateSystemChannelCommand,
        requester_kind: &str,
        subscriber_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        policy::ensure_system_channel_requester_kind(requester_kind)?;

        if command.subscriber_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "system channel requires subscriber id".into(),
            ));
        }
        if command.requester_id == command.subscriber_id {
            return Err(RuntimeError::PermissionDenied(
                "system channel subscriber must differ from requester".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());
        let publisher_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.requester_id.as_str(),
            ),
            command.requester_id.as_str(),
            requester_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            BTreeMap::from([("channelRole".into(), "publisher".into())]),
        );
        let mut subscriber_member_attributes =
            BTreeMap::from([("channelRole".into(), "subscriber".into())]);
        subscriber_member_attributes.extend(subscriber_attributes);
        let subscriber_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(
                command.conversation_id.as_str(),
                command.subscriber_id.as_str(),
            ),
            command.subscriber_id.as_str(),
            "user",
            MembershipRole::Member,
            Some(command.requester_id.clone()),
            created_at.clone(),
            subscriber_member_attributes,
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new("system_channel"),
            ..ConversationState::default()
        };
        let publisher_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, publisher_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&publisher_member),
        );
        let subscriber_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, subscriber_member.clone());
        upsert_read_cursor(
            &mut conversation,
            build_default_read_cursor(&subscriber_member),
        );
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.requester_id.clone(),
                actor_kind: requester_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "system_channel"
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            publisher_member,
            publisher_ordering_seq,
            "standard",
            command.requester_id.as_str(),
            requester_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            subscriber_member,
            subscriber_ordering_seq,
            "standard",
            command.requester_id.as_str(),
            requester_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }

    pub fn create_agent_handoff(
        &self,
        command: CreateAgentHandoffCommand,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind(command, "agent")
    }

    pub fn create_agent_handoff_from_auth_context(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_from_auth_context_with_target_attributes(
            auth,
            conversation_id,
            target_id,
            target_kind,
            handoff_session_id,
            handoff_reason,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_handoff_from_auth_context_with_target_attributes(
        &self,
        auth: &AuthContext,
        conversation_id: String,
        target_id: String,
        target_kind: String,
        handoff_session_id: String,
        handoff_reason: Option<String>,
        target_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind_and_target_attributes(
            CreateAgentHandoffCommand::from_auth_context(
                auth,
                conversation_id,
                target_id,
                target_kind,
                handoff_session_id,
                handoff_reason,
            ),
            auth.actor_kind.as_str(),
            target_attributes,
        )
    }

    pub fn create_agent_handoff_with_source_kind(
        &self,
        command: CreateAgentHandoffCommand,
        source_kind: &str,
    ) -> Result<CreateConversationResult, RuntimeError> {
        self.create_agent_handoff_with_source_kind_and_target_attributes(
            command,
            source_kind,
            BTreeMap::new(),
        )
    }

    pub fn create_agent_handoff_with_source_kind_and_target_attributes(
        &self,
        command: CreateAgentHandoffCommand,
        source_kind: &str,
        target_attributes: BTreeMap<String, String>,
    ) -> Result<CreateConversationResult, RuntimeError> {
        policy::ensure_agent_handoff_source_kind(source_kind)?;
        policy::ensure_agent_handoff_target_kind(command.target_kind.as_str())?;

        if command.handoff_session_id.trim().is_empty() {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff requires handoff session id".into(),
            ));
        }
        if command.source_id == command.target_id {
            return Err(RuntimeError::PermissionDenied(
                "agent handoff target must differ from source".into(),
            ));
        }

        let created_at = conversation_timestamp();
        let scope_key =
            conversation_scope_key(command.tenant_id.as_str(), command.conversation_id.as_str());

        let mut source_attributes = BTreeMap::from([
            ("handoffRole".into(), "source".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("targetId".into(), command.target_id.clone()),
            ("targetKind".into(), command.target_kind.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            source_attributes.insert("handoffReason".into(), reason);
        }
        let source_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.source_id.as_str()),
            command.source_id.as_str(),
            source_kind,
            MembershipRole::Owner,
            None,
            created_at.clone(),
            source_attributes,
        );

        let mut target_member_attributes = BTreeMap::from([
            ("handoffRole".into(), "target".into()),
            (
                "handoffSessionId".into(),
                command.handoff_session_id.clone(),
            ),
            ("sourceAgentId".into(), command.source_id.clone()),
        ]);
        if let Some(reason) = command.handoff_reason.clone() {
            target_member_attributes.insert("handoffReason".into(), reason);
        }
        target_member_attributes.extend(target_attributes);
        let target_member = build_conversation_member_with_attributes(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            member_id(command.conversation_id.as_str(), command.target_id.as_str()),
            command.target_id.as_str(),
            command.target_kind.as_str(),
            MembershipRole::Member,
            Some(command.source_id.clone()),
            created_at.clone(),
            target_member_attributes,
        );

        let mut state = self.state.lock().expect("runtime state should lock");
        if state.conversations.contains_key(scope_key.as_str()) {
            return Err(RuntimeError::ConversationAlreadyExists(
                command.conversation_id,
            ));
        }

        let mut conversation = ConversationState {
            aggregate: ConversationAggregateState::new_agent_handoff(AgentHandoffStateView {
                tenant_id: command.tenant_id.clone(),
                conversation_id: command.conversation_id.clone(),
                status: "open".into(),
                source: ChangeAgentHandoffStatusView {
                    id: command.source_id.clone(),
                    kind: source_kind.into(),
                },
                target: ChangeAgentHandoffStatusView {
                    id: command.target_id.clone(),
                    kind: command.target_kind.clone(),
                },
                handoff_session_id: command.handoff_session_id.clone(),
                handoff_reason: command.handoff_reason.clone(),
                accepted_at: None,
                accepted_by: None,
                resolved_at: None,
                resolved_by: None,
                closed_at: None,
                closed_by: None,
            }),
            ..ConversationState::default()
        };
        let source_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, source_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&source_member));
        let target_ordering_seq = conversation.aggregate.next_member_epoch();
        upsert_member(&mut conversation, target_member.clone());
        upsert_read_cursor(&mut conversation, build_default_read_cursor(&target_member));
        state.conversations.insert(scope_key, conversation);
        drop(state);

        let event_id = format!("evt_{}_created", command.conversation_id);
        let envelope = CommitEnvelope {
            event_id: event_id.clone(),
            tenant_id: command.tenant_id.clone(),
            event_type: "conversation.created".into(),
            event_version: 1,
            aggregate_type: AggregateType::Conversation,
            aggregate_id: command.conversation_id.clone(),
            scope_type: "conversation".into(),
            scope_id: command.conversation_id.clone(),
            ordering_key: CommitEnvelope::ordering_key(
                command.tenant_id.as_str(),
                command.conversation_id.as_str(),
            ),
            ordering_seq: 0,
            causation_id: None,
            correlation_id: None,
            idempotency_key: None,
            actor: EventActor {
                actor_id: command.source_id.clone(),
                actor_kind: source_kind.into(),
                actor_session_id: None,
            },
            occurred_at: created_at.clone(),
            committed_at: created_at.clone(),
            payload_schema: Some("conversation.created.v1".into()),
            payload: json!({
                "conversationId": command.conversation_id,
                "conversationType": "agent_handoff",
                "source": {
                    "id": command.source_id,
                    "kind": source_kind
                },
                "target": {
                    "id": command.target_id,
                    "kind": command.target_kind
                },
                "handoff": {
                    "sessionId": command.handoff_session_id,
                    "reason": command.handoff_reason,
                    "status": "open"
                }
            })
            .to_string(),
            retention_class: "standard".into(),
            audit_class: "default".into(),
        };

        self.journal.append(envelope)?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            source_member,
            source_ordering_seq,
            "standard",
            command.source_id.as_str(),
            source_kind,
        ))?;
        self.journal.append(build_member_envelope(
            command.tenant_id.as_str(),
            command.conversation_id.as_str(),
            "conversation.member_joined",
            target_member,
            target_ordering_seq,
            "standard",
            command.source_id.as_str(),
            source_kind,
        ))?;

        Ok(CreateConversationResult {
            conversation_id: command.conversation_id,
            event_id,
        })
    }
}
