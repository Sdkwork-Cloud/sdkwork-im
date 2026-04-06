use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use conversation_runtime::{
    AcceptAgentHandoffCommand, AddConversationMemberCommand, ChangeAgentHandoffStatusView,
    ChangeConversationMemberRoleCommand, CloseAgentHandoffCommand, ConversationRuntime,
    CreateAgentDialogCommand, CreateAgentHandoffCommand, CreateConversationCommand,
    CreateSystemChannelCommand, EditMessageCommand, LeaveConversationCommand, PostMessageCommand,
    PublishSystemChannelMessageCommand, RecallMessageCommand, RemoveConversationMemberCommand,
    ResolveAgentHandoffCommand, RuntimeError, TransferConversationOwnerCommand,
    UpdateReadCursorCommand,
};
use im_domain_core::conversation::{ConversationMember, MembershipRole, MembershipState};
use im_domain_core::message::{ContentPart, MessageBody, MessageType, Sender};
use im_domain_events::CommitEnvelope;
use im_platform_contracts::{CommitJournal, CommitPosition, ContractError};

#[derive(Clone, Default)]
struct InMemoryJournal {
    events: Arc<Mutex<Vec<CommitEnvelope>>>,
}

impl InMemoryJournal {
    fn recorded(&self) -> Vec<CommitEnvelope> {
        self.events.lock().expect("journal should lock").clone()
    }
}

impl CommitJournal for InMemoryJournal {
    fn append(&self, envelope: CommitEnvelope) -> Result<CommitPosition, ContractError> {
        let mut events = self.events.lock().expect("journal should lock");
        events.push(envelope);
        Ok(CommitPosition::new("p0", events.len() as u64))
    }
}

#[test]
fn test_create_conversation_and_post_message_emits_commit_events_in_order() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let conversation = runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_demo".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    assert_eq!(conversation.conversation_id, "c_demo");

    let message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_demo".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: Some("cm_demo".into()),
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_demo".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
            },
        })
        .expect("post message should succeed");

    assert_eq!(message.message_seq, 1);
    assert_eq!(message.message_id, "msg_c_demo_1");

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "message.posted");
    assert_eq!(events[2].ordering_seq, 1);
}

#[test]
fn test_runtime_replays_recorded_conversation_events_after_rebuild() {
    let source_journal = InMemoryJournal::default();
    let source_runtime = ConversationRuntime::new(source_journal.clone());

    source_runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_replay".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    source_runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_replay".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_replay_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("first".into()),
                parts: vec![ContentPart::text("first")],
                render_hints: Default::default(),
            },
        })
        .expect("first post should succeed");

    let replay_journal = InMemoryJournal::default();
    let replay_runtime = ConversationRuntime::new(replay_journal);
    for envelope in source_journal.recorded() {
        replay_runtime
            .apply_recovered_envelope(&envelope)
            .expect("replay should succeed");
    }

    let members = replay_runtime
        .list_members("t_demo", "c_replay")
        .expect("members should exist after replay");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "u_demo");

    let posted = replay_runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_replay".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo_new".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_replay_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("second".into()),
                parts: vec![ContentPart::text("second")],
                render_hints: Default::default(),
            },
        })
        .expect("post after replay should succeed");
    assert_eq!(posted.message_seq, 2);
    assert_eq!(posted.message_id, "msg_c_replay_2");
}

#[test]
fn test_same_conversation_id_is_isolated_per_tenant() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_alpha".into(),
            conversation_id: "c_shared".into(),
            creator_id: "u_alpha".into(),
            conversation_type: "group".into(),
        })
        .expect("tenant alpha conversation should succeed");

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_beta".into(),
            conversation_id: "c_shared".into(),
            creator_id: "u_beta".into(),
            conversation_type: "group".into(),
        })
        .expect("tenant beta conversation should succeed");

    let alpha_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_alpha".into(),
            conversation_id: "c_shared".into(),
            sender: Sender {
                id: "u_alpha".into(),
                kind: "user".into(),
                member_id: Some("cm_alpha".into()),
                device_id: Some("d_alpha".into()),
                session_id: Some("s_alpha".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_alpha".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello alpha".into()),
                parts: vec![ContentPart::text("hello alpha")],
                render_hints: Default::default(),
            },
        })
        .expect("tenant alpha message should succeed");

    let beta_message = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_beta".into(),
            conversation_id: "c_shared".into(),
            sender: Sender {
                id: "u_beta".into(),
                kind: "user".into(),
                member_id: Some("cm_beta".into()),
                device_id: None,
                session_id: Some("s_beta".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_beta".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello beta".into()),
                parts: vec![ContentPart::text("hello beta")],
                render_hints: Default::default(),
            },
        })
        .expect("tenant beta message should succeed");

    assert_eq!(alpha_message.message_seq, 1);
    assert_eq!(beta_message.message_seq, 1);
}

#[test]
fn test_post_message_rejects_sender_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "u_demo".into(),
        })
        .expect("group create should succeed");

    let post = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_kind_guard".into(),
        sender: Sender {
            id: "u_demo".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_kind_mismatch".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
        },
    });

    assert!(matches!(post, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_edit_message_rejects_editor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_edit_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "u_demo".into(),
        })
        .expect("group create should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_edit_kind_guard".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_edit_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("before edit".into()),
                parts: vec![ContentPart::text("before edit")],
                render_hints: Default::default(),
            },
        })
        .expect("post message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id,
        editor: Sender {
            id: "u_demo".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
        },
    });

    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_recall_message_rejects_actor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_recall_kind_guard".into(),
            conversation_type: "group".into(),
            creator_id: "u_demo".into(),
        })
        .expect("group create should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_recall_kind_guard".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_recall_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("before recall".into()),
                parts: vec![ContentPart::text("before recall")],
                render_hints: Default::default(),
            },
        })
        .expect("post message should succeed");

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "u_demo".into(),
            kind: "agent".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_demo".into()),
            metadata: Default::default(),
        },
    });

    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_generic_create_rejects_unknown_and_reserved_special_conversation_types() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    for (conversation_id, conversation_type) in [
        ("c_unknown_type", "workspace"),
        ("c_agent_dialog_type", "agent_dialog"),
        ("c_agent_handoff_type", "agent_handoff"),
        ("c_system_channel_type", "system_channel"),
    ] {
        let create = runtime.create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: conversation_id.into(),
            creator_id: "u_demo".into(),
            conversation_type: conversation_type.into(),
        });

        assert!(
            create.is_err(),
            "conversation type should be rejected: {conversation_type}"
        );
    }
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_agent_dialog_creates_requester_and_agent_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_agent_dialog_with_requester_kind(
            CreateAgentDialogCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_dialog".into(),
                requester_id: "u_demo".into(),
                agent_id: "ag_demo".into(),
            },
            "user",
        )
        .expect("agent dialog create should succeed");

    assert_eq!(created.conversation_id, "c_agent_dialog");

    let members = runtime
        .list_members("t_demo", "c_agent_dialog")
        .expect("agent dialog members should list");
    assert_eq!(members.len(), 2);

    let requester = members
        .iter()
        .find(|member| member.principal_id == "u_demo")
        .expect("requester member should exist");
    assert_eq!(requester.principal_kind, "user");
    assert_eq!(requester.role, MembershipRole::Owner);
    assert_eq!(requester.state, MembershipState::Joined);

    let agent = members
        .iter()
        .find(|member| member.principal_id == "ag_demo")
        .expect("agent member should exist");
    assert_eq!(agent.principal_kind, "agent");
    assert_eq!(agent.role, MembershipRole::Member);
    assert_eq!(agent.state, MembershipState::Joined);
    assert_eq!(agent.invited_by.as_deref(), Some("u_demo"));

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "conversation.member_joined");
}

#[test]
fn test_create_agent_dialog_rejects_non_user_requester_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_agent_dialog_with_requester_kind(
        CreateAgentDialogCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_dialog_invalid".into(),
            requester_id: "svc_ops".into(),
            agent_id: "ag_demo".into(),
        },
        "system",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_system_channel_creates_system_and_subscriber_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_system_channel".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "u_demo".into(),
            },
            "system",
        )
        .expect("system channel create should succeed");

    assert_eq!(created.conversation_id, "c_system_channel");

    let members = runtime
        .list_members("t_demo", "c_system_channel")
        .expect("system channel members should list");
    assert_eq!(members.len(), 2);

    let publisher = members
        .iter()
        .find(|member| member.principal_id == "svc_ops")
        .expect("system publisher should exist");
    assert_eq!(publisher.principal_kind, "system");
    assert_eq!(publisher.role, MembershipRole::Owner);
    assert_eq!(
        publisher.attributes.get("channelRole").map(String::as_str),
        Some("publisher")
    );

    let subscriber = members
        .iter()
        .find(|member| member.principal_id == "u_demo")
        .expect("subscriber should exist");
    assert_eq!(subscriber.principal_kind, "user");
    assert_eq!(subscriber.role, MembershipRole::Member);
    assert_eq!(subscriber.invited_by.as_deref(), Some("svc_ops"));
    assert_eq!(
        subscriber.attributes.get("channelRole").map(String::as_str),
        Some("subscriber")
    );

    let cursor = runtime
        .read_cursor_view("t_demo", "c_system_channel", "u_demo")
        .expect("subscriber read cursor should be initialized");
    assert_eq!(cursor.member_id, subscriber.member_id);
    assert_eq!(cursor.read_seq, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[0].actor.actor_kind, "system");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[1].actor.actor_kind, "system");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[2].actor.actor_kind, "system");
}

#[test]
fn test_create_system_channel_rejects_non_system_requester_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_system_channel_with_requester_kind(
        CreateSystemChannelCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_system_channel_invalid".into(),
            requester_id: "u_demo".into(),
            subscriber_id: "u_subscriber".into(),
        },
        "user",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_create_agent_handoff_creates_source_agent_and_target_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let created = runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff".into(),
                source_id: "ag_source".into(),
                target_id: "u_demo".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_demo".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    assert_eq!(created.conversation_id, "c_agent_handoff");

    let members = runtime
        .list_members("t_demo", "c_agent_handoff")
        .expect("agent handoff members should list");
    assert_eq!(members.len(), 2);

    let source = members
        .iter()
        .find(|member| member.principal_id == "ag_source")
        .expect("source agent should exist");
    assert_eq!(source.principal_kind, "agent");
    assert_eq!(source.role, MembershipRole::Owner);
    assert_eq!(
        source.attributes.get("handoffRole").map(String::as_str),
        Some("source")
    );
    assert_eq!(
        source
            .attributes
            .get("handoffSessionId")
            .map(String::as_str),
        Some("hs_demo")
    );

    let target = members
        .iter()
        .find(|member| member.principal_id == "u_demo")
        .expect("target member should exist");
    assert_eq!(target.principal_kind, "user");
    assert_eq!(target.role, MembershipRole::Member);
    assert_eq!(target.invited_by.as_deref(), Some("ag_source"));
    assert_eq!(
        target.attributes.get("handoffRole").map(String::as_str),
        Some("target")
    );
    assert_eq!(
        target.attributes.get("sourceAgentId").map(String::as_str),
        Some("ag_source")
    );
    assert_eq!(
        target.attributes.get("handoffReason").map(String::as_str),
        Some("manual_escalation")
    );

    let cursor = runtime
        .read_cursor_view("t_demo", "c_agent_handoff", "u_demo")
        .expect("target read cursor should be initialized");
    assert_eq!(cursor.member_id, target.member_id);
    assert_eq!(cursor.read_seq, 0);

    let events = journal.recorded();
    assert_eq!(events.len(), 3);
    assert_eq!(events[0].event_type, "conversation.created");
    assert_eq!(events[0].actor.actor_kind, "agent");
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[1].actor.actor_kind, "agent");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[2].actor.actor_kind, "agent");
}

#[test]
fn test_create_agent_handoff_rejects_non_agent_source_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    let create = runtime.create_agent_handoff_with_source_kind(
        CreateAgentHandoffCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_handoff_invalid".into(),
            source_id: "svc_ops".into(),
            target_id: "u_demo".into(),
            target_kind: "user".into(),
            handoff_session_id: "hs_invalid".into(),
            handoff_reason: Some("manual_escalation".into()),
        },
        "system",
    );

    assert!(matches!(create, Err(RuntimeError::PermissionDenied(_))));
    assert!(journal.recorded().is_empty());
}

#[test]
fn test_agent_handoff_allows_source_and_target_posts() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_post".into(),
                source_id: "ag_source".into(),
                target_id: "u_demo".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_post".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let source_post = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_handoff_post".into(),
            sender: Sender {
                id: "ag_source".into(),
                kind: "agent".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_agent".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_handoff_source".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("source".into()),
                parts: vec![ContentPart::text("source")],
                render_hints: Default::default(),
            },
        })
        .expect("source agent post should succeed");
    assert_eq!(source_post.message_seq, 1);

    let target_post = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_handoff_post".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_target".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_handoff_target".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("target".into()),
                parts: vec![ContentPart::text("target")],
                render_hints: Default::default(),
            },
        })
        .expect("target post should succeed");
    assert_eq!(target_post.message_seq, 2);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.posted")
            .count(),
        2
    );
}

#[test]
fn test_agent_handoff_accept_resolve_close_state_machine_and_closed_handoff_rejects_posts() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_state".into(),
                source_id: "ag_source".into(),
                target_id: "u_demo".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_state".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let opened = runtime
        .get_agent_handoff_state("t_demo", "c_agent_handoff_state", "ag_source")
        .expect("source should read handoff state");
    assert_eq!(opened.status, "open");
    assert!(opened.accepted_at.is_none());
    assert!(opened.resolved_at.is_none());
    assert!(opened.closed_at.is_none());

    let accepted = runtime
        .accept_agent_handoff_with_actor_kind(
            AcceptAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_state".into(),
                accepted_by: "u_demo".into(),
            },
            "user",
        )
        .expect("target should accept handoff");
    assert_eq!(accepted.status, "accepted");
    assert_eq!(
        accepted.accepted_by,
        Some(ChangeAgentHandoffStatusView {
            id: "u_demo".into(),
            kind: "user".into(),
        })
    );
    assert!(accepted.accepted_at.is_some());

    let resolved = runtime
        .resolve_agent_handoff_with_actor_kind(
            ResolveAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_state".into(),
                resolved_by: "u_demo".into(),
            },
            "user",
        )
        .expect("target should resolve handoff");
    assert_eq!(resolved.status, "resolved");
    assert_eq!(
        resolved.resolved_by,
        Some(ChangeAgentHandoffStatusView {
            id: "u_demo".into(),
            kind: "user".into(),
        })
    );
    assert!(resolved.resolved_at.is_some());

    let closed = runtime
        .close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_state".into(),
                closed_by: "ag_source".into(),
            },
            "agent",
        )
        .expect("source should close handoff");
    assert_eq!(closed.status, "closed");
    assert_eq!(
        closed.closed_by,
        Some(ChangeAgentHandoffStatusView {
            id: "ag_source".into(),
            kind: "agent".into(),
        })
    );
    assert!(closed.closed_at.is_some());

    let post_after_close = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_agent_handoff_state".into(),
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_target".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_handoff_closed".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(post_after_close, Err(RuntimeError::Conflict(_))));

    let events = journal.recorded();
    let status_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.agent_handoff_status_changed")
        .collect();
    assert_eq!(status_events.len(), 3);
}

#[test]
fn test_agent_handoff_accept_requires_target_and_resolve_requires_accepted_state() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_handoff_with_source_kind(
            CreateAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_policy".into(),
                source_id: "ag_source".into(),
                target_id: "u_demo".into(),
                target_kind: "user".into(),
                handoff_session_id: "hs_policy".into(),
                handoff_reason: Some("manual_escalation".into()),
            },
            "agent",
        )
        .expect("agent handoff create should succeed");

    let source_accept = runtime.accept_agent_handoff_with_actor_kind(
        AcceptAgentHandoffCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_handoff_policy".into(),
            accepted_by: "ag_source".into(),
        },
        "agent",
    );
    assert!(matches!(
        source_accept,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let resolve_before_accept = runtime.resolve_agent_handoff_with_actor_kind(
        ResolveAgentHandoffCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_handoff_policy".into(),
            resolved_by: "u_demo".into(),
        },
        "user",
    );
    assert!(matches!(
        resolve_before_accept,
        Err(RuntimeError::Conflict(_))
    ));

    let target_close = runtime
        .close_agent_handoff_with_actor_kind(
            CloseAgentHandoffCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_handoff_policy".into(),
                closed_by: "u_demo".into(),
            },
            "user",
        )
        .expect("target should be allowed to close open handoff");
    assert_eq!(target_close.status, "closed");
}

#[test]
fn test_create_group_member_joined_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_group_actor_kind".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member_joined = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.member_joined"
                && event.aggregate_id == "c_group_actor_kind"
        })
        .expect("creator join event should be recorded");
    assert_eq!(member_joined.actor.actor_id, "svc_ops");
    assert_eq!(member_joined.actor.actor_kind, "system");
}

#[test]
fn test_conversation_membership_lifecycle_tracks_creator_and_member_changes() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_members".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let members = runtime
        .list_members("t_demo", "c_members")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].member_id, "cm_c_members_u_owner");
    assert_eq!(members[0].principal_id, "u_owner");
    assert_eq!(members[0].role, MembershipRole::Owner);
    assert_eq!(members[0].state, MembershipState::Joined);

    let added_member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_members".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("add member should succeed");

    assert_eq!(added_member.member_id, "cm_c_members_u_member");
    assert_eq!(added_member.principal_id, "u_member");
    assert_eq!(added_member.role, MembershipRole::Member);
    assert_eq!(added_member.state, MembershipState::Joined);
    assert_eq!(added_member.invited_by.as_deref(), Some("u_owner"));

    let members_after_add = runtime
        .list_members("t_demo", "c_members")
        .expect("list members after add should succeed");
    assert_eq!(members_after_add.len(), 2);

    let removed_member = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_members".into(),
            member_id: added_member.member_id.clone(),
            removed_by: "u_owner".into(),
        })
        .expect("remove member should succeed");

    assert_eq!(removed_member.member_id, "cm_c_members_u_member");
    assert_eq!(removed_member.state, MembershipState::Removed);
    assert!(removed_member.removed_at.is_some());

    let members_after_remove = runtime
        .list_members("t_demo", "c_members")
        .expect("list members after remove should succeed");
    assert_eq!(members_after_remove.len(), 1);
    assert_eq!(members_after_remove[0].member_id, "cm_c_members_u_owner");

    let events = journal.recorded();
    assert_eq!(events.len(), 4);
    assert_eq!(events[1].event_type, "conversation.member_joined");
    assert_eq!(events[2].event_type, "conversation.member_joined");
    assert_eq!(events[3].event_type, "conversation.member_removed");
}

#[test]
fn test_read_cursor_advances_monotonically_for_active_member() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
            },
        })
        .expect("first message should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
            },
        })
        .expect("second message should succeed");

    let cursor = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "u_owner".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_1".into()),
        })
        .expect("read cursor update should succeed");

    assert_eq!(cursor.member_id, "cm_c_cursor_u_owner");
    assert_eq!(cursor.read_seq, 1);
    assert_eq!(
        cursor.last_read_message_id.as_deref(),
        Some("msg_c_cursor_1")
    );

    let regressed = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "u_owner".into(),
            read_seq: 0,
            last_read_message_id: Some("msg_c_cursor_0".into()),
        })
        .expect("regressed read cursor update should be idempotent");

    assert_eq!(regressed.read_seq, 1);
    assert_eq!(
        regressed.last_read_message_id.as_deref(),
        Some("msg_c_cursor_1")
    );

    let advanced = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor".into(),
            principal_id: "u_owner".into(),
            read_seq: 2,
            last_read_message_id: Some("msg_c_cursor_2".into()),
        })
        .expect("advanced read cursor update should succeed");

    assert_eq!(advanced.read_seq, 2);
    assert_eq!(
        advanced.last_read_message_id.as_deref(),
        Some("msg_c_cursor_2")
    );

    let events = journal.recorded();
    let read_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.read_cursor_updated")
        .collect();
    assert_eq!(read_events.len(), 2);
    assert_eq!(read_events[0].ordering_seq, 1);
    assert_eq!(read_events[1].ordering_seq, 2);
}

#[test]
fn test_read_cursor_rejects_actor_kind_mismatch_against_member_principal_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_actor_kind_guard".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
            },
        })
        .expect("message should succeed");

    let update_attempt = runtime.update_read_cursor_with_actor_kind(
        UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_actor_kind_guard".into(),
            principal_id: "u_owner".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_actor_kind_guard_1".into()),
        },
        "agent",
    );

    assert!(matches!(
        update_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_conversation_bound_write_capability_gate_rejects_actor_kind_mismatch() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_write_capability_actor_kind_guard".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let gate_attempt = runtime.ensure_conversation_bound_write_allowed_with_actor_kind(
        "t_demo",
        "c_write_capability_actor_kind_guard",
        "u_owner",
        "agent",
        "stream.append",
    );

    assert!(matches!(
        gate_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_system_channel_requires_dedicated_publish_command_and_allows_only_publisher() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_system_channel_with_requester_kind(
            CreateSystemChannelCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_system_channel_post".into(),
                requester_id: "svc_ops".into(),
                subscriber_id: "u_demo".into(),
            },
            "system",
        )
        .expect("system channel create should succeed");

    let subscriber_post = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_system_channel_post".into(),
        sender: Sender {
            id: "u_demo".into(),
            kind: "user".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_subscriber".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_subscriber_post".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("should fail".into()),
            parts: vec![ContentPart::text("should fail")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(
        subscriber_post,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let system_post = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_system_channel_post".into(),
        sender: Sender {
            id: "svc_ops".into(),
            kind: "system".into(),
            member_id: None,
            device_id: None,
            session_id: Some("s_system".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_system_post".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("system notice".into()),
            parts: vec![ContentPart::text("system notice")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(
        system_post,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let subscriber_publish =
        runtime.publish_system_channel_message(PublishSystemChannelMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_system_channel_post".into(),
            publisher: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_subscriber".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_subscriber_publish".into()),
            body: MessageBody {
                summary: Some("should fail".into()),
                parts: vec![ContentPart::text("should fail")],
                render_hints: Default::default(),
            },
        });
    assert!(matches!(
        subscriber_publish,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let system_publish = runtime
        .publish_system_channel_message(PublishSystemChannelMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_system_channel_post".into(),
            publisher: Sender {
                id: "svc_ops".into(),
                kind: "system".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_system".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_system_publish".into()),
            body: MessageBody {
                summary: Some("system notice".into()),
                parts: vec![ContentPart::text("system notice")],
                render_hints: Default::default(),
            },
        })
        .expect("system publisher dedicated publish should succeed");

    assert_eq!(system_publish.message_seq, 1);
    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "message.posted")
            .count(),
        1
    );
}

#[test]
fn test_read_cursor_event_preserves_agent_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_agent_dialog_with_requester_kind(
            CreateAgentDialogCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_agent_cursor".into(),
                requester_id: "u_requester".into(),
                agent_id: "ag_demo".into(),
            },
            "user",
        )
        .expect("agent dialog create should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_cursor".into(),
            sender: Sender {
                id: "u_requester".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_requester".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_agent_cursor".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("question".into()),
                parts: vec![ContentPart::text("question")],
                render_hints: Default::default(),
            },
        })
        .expect("message should succeed");

    runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_agent_cursor".into(),
            principal_id: "ag_demo".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_agent_cursor_1".into()),
        })
        .expect("agent read cursor update should succeed");

    let read_cursor_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.read_cursor_updated"
                && event.aggregate_id == "c_agent_cursor"
                && event.ordering_seq == 1
        })
        .expect("read cursor update event should exist");
    assert_eq!(read_cursor_event.actor.actor_id, "ag_demo");
    assert_eq!(read_cursor_event.actor.actor_kind, "agent");
}

#[test]
fn test_edit_and_recall_message_emit_mutation_events_without_changing_sequence() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_mutation".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_mutation".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
            },
        })
        .expect("post message should succeed");

    let edited = runtime
        .edit_message(EditMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: posted.message_id.clone(),
            editor: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited".into()),
                parts: vec![ContentPart::text("edited")],
                render_hints: Default::default(),
            },
        })
        .expect("edit message should succeed");

    let recalled = runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: posted.message_id.clone(),
            recalled_by: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("recall message should succeed");

    assert_eq!(edited.message_id, posted.message_id);
    assert_eq!(edited.message_seq, 1);
    assert_eq!(recalled.message_id, posted.message_id);
    assert_eq!(recalled.message_seq, 1);

    let events = journal.recorded();
    assert_eq!(events.len(), 5);
    assert_eq!(events[2].event_type, "message.posted");
    assert_eq!(events[3].event_type, "message.edited");
    assert_eq!(events[3].ordering_seq, 1);
    assert_eq!(events[4].event_type, "message.recalled");
    assert_eq!(events[4].ordering_seq, 1);
}

#[test]
fn test_non_member_cannot_post_message_to_conversation() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_private".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let result = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_private".into(),
        sender: Sender {
            id: "u_intruder".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_intruder".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("unauthorized".into()),
            parts: vec![ContentPart::text("unauthorized")],
            render_hints: Default::default(),
        },
    });

    assert!(matches!(result, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_non_member_cannot_edit_or_recall_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_private_mutation".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_private_mutation".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_owner".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
            },
        })
        .expect("owner message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "u_intruder".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("edited by intruder".into()),
            parts: vec![ContentPart::text("edited by intruder")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "u_intruder".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_intruder".into()),
            session_id: Some("s_intruder".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_member_cannot_edit_or_recall_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_mutation".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_mutation".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("add member should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_mutation".into(),
            sender: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_owner".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("hello".into()),
                parts: vec![ContentPart::text("hello")],
                render_hints: Default::default(),
            },
        })
        .expect("owner message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "u_member".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("edited by member".into()),
            parts: vec![ContentPart::text("edited by member")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "u_member".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_recall_but_not_edit_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_override".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_override".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("add member should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_override".into(),
            sender: Sender {
                id: "u_member".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_member".into()),
                session_id: Some("s_member".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_member".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("member hello".into()),
                parts: vec![ContentPart::text("member hello")],
                render_hints: Default::default(),
            },
        })
        .expect("member message should succeed");

    let edit = runtime.edit_message(EditMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id.clone(),
        editor: Sender {
            id: "u_owner".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
        body: MessageBody {
            summary: Some("owner edit".into()),
            parts: vec![ContentPart::text("owner edit")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(edit, Err(RuntimeError::PermissionDenied(_))));

    let recall = runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: posted.message_id,
            recalled_by: Sender {
                id: "u_owner".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_owner".into()),
                session_id: Some("s_owner".into()),
                metadata: Default::default(),
            },
        })
        .expect("owner should be able to recall member message in group conversation");
    assert_eq!(recall.conversation_id, "c_group_owner_override");
    assert_eq!(recall.message_seq, 1);
}

#[test]
fn test_direct_conversation_owner_cannot_recall_other_members_message() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_mutation".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_mutation".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("add peer should succeed");

    let posted = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_mutation".into(),
            sender: Sender {
                id: "u_peer".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_peer".into()),
                session_id: Some("s_peer".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_peer".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("peer hello".into()),
                parts: vec![ContentPart::text("peer hello")],
                render_hints: Default::default(),
            },
        })
        .expect("peer message should succeed");

    let recall = runtime.recall_message(RecallMessageCommand {
        tenant_id: "t_demo".into(),
        message_id: posted.message_id,
        recalled_by: Sender {
            id: "u_owner".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_owner".into()),
            session_id: Some("s_owner".into()),
            metadata: Default::default(),
        },
    });
    assert!(matches!(recall, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_member_cannot_manage_other_members() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_member_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_member_governance".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add regular member");

    let target = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_member_governance".into(),
            principal_id: "u_target".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add target member");

    let add_attempt = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_member_governance".into(),
        principal_id: "u_extra".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        invited_by: "u_member".into(),
    });
    assert!(matches!(
        add_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let remove_attempt = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_member_governance".into(),
        member_id: target.member_id,
        removed_by: "u_member".into(),
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_governance_writes_reject_actor_kind_mismatch() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let target = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "u_target".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add transfer target");

    let add_attempt = runtime.add_member_with_actor_kind(
        AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "u_extra".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        },
        "agent",
    );
    assert!(matches!(
        add_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let remove_attempt = runtime.remove_member_with_actor_kind(
        RemoveConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            member_id: target.member_id.clone(),
            removed_by: "u_owner".into(),
        },
        "agent",
    );
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let leave_attempt = runtime.leave_conversation_with_actor_kind(
        LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            principal_id: "u_member".into(),
        },
        "agent",
    );
    assert!(matches!(
        leave_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let transfer_attempt = runtime.transfer_conversation_owner_with_actor_kind(
        TransferConversationOwnerCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            target_member_id: target.member_id.clone(),
            transferred_by: "u_owner".into(),
        },
        "agent",
    );
    assert!(matches!(
        transfer_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let role_change_attempt = runtime.change_conversation_member_role_with_actor_kind(
        ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_actor_kind_governance".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "u_owner".into(),
        },
        "agent",
    );
    assert!(matches!(
        role_change_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let members = runtime
        .list_members("t_demo", "c_group_actor_kind_governance")
        .expect("list members should succeed");
    assert_eq!(members.len(), 3);
    let owner = members
        .iter()
        .find(|item| item.principal_id == "u_owner")
        .expect("owner should exist");
    assert_eq!(owner.role, MembershipRole::Owner);
    let member_state = members
        .iter()
        .find(|item| item.principal_id == "u_member")
        .expect("member should exist");
    assert_eq!(member_state.role, MembershipRole::Member);
    let target_state = members
        .iter()
        .find(|item| item.principal_id == "u_target")
        .expect("target should exist");
    assert_eq!(target_state.role, MembershipRole::Member);
}

#[test]
fn test_group_admin_can_manage_regular_members_but_cannot_escalate_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_admin_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "u_admin".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add admin");

    let admin_peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "u_admin_2".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add another admin");

    let joined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_admin_governance".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_admin".into(),
        })
        .expect("admin should be able to add regular member");
    assert_eq!(joined.role, MembershipRole::Member);

    let admin_escalation = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_admin_governance".into(),
        principal_id: "u_admin_3".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Admin,
        invited_by: "u_admin".into(),
    });
    assert!(matches!(
        admin_escalation,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let owner_escalation = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_admin_governance".into(),
        principal_id: "u_owner_2".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        invited_by: "u_admin".into(),
    });
    assert!(matches!(
        owner_escalation,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let admin_remove_admin = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_admin_governance".into(),
        member_id: admin_peer.member_id,
        removed_by: "u_admin".into(),
    });
    assert!(matches!(
        admin_remove_admin,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let removed = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_admin_governance".into(),
            member_id: joined.member_id,
            removed_by: "u_admin".into(),
        })
        .expect("admin should be able to remove regular member");
    assert_eq!(removed.state, MembershipState::Removed);
}

#[test]
fn test_group_owner_cannot_create_second_owner() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let second_owner = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_owner_governance".into(),
        principal_id: "u_owner_2".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Owner,
        invited_by: "u_owner".into(),
    });
    assert!(matches!(
        second_owner,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_direct_conversation_owner_can_add_only_single_non_elevated_peer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_member_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_member_governance".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add the direct conversation peer");
    assert_eq!(peer.role, MembershipRole::Member);

    let third_participant = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_direct_member_governance".into(),
        principal_id: "u_third".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Member,
        invited_by: "u_owner".into(),
    });
    assert!(matches!(
        third_participant,
        Err(RuntimeError::PermissionDenied(_))
    ));

    let elevated_peer = runtime.add_member(AddConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_direct_member_governance".into(),
        principal_id: "u_peer_admin".into(),
        principal_kind: "user".into(),
        role: MembershipRole::Admin,
        invited_by: "u_owner".into(),
    });
    assert!(matches!(
        elevated_peer,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_direct_conversation_rejects_member_removal() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_remove_governance".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_remove_governance".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add the direct conversation peer");

    let remove_attempt = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_direct_remove_governance".into(),
        member_id: peer.member_id,
        removed_by: "u_owner".into(),
    });
    assert!(matches!(
        remove_attempt,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_group_member_can_leave_and_loses_access() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_leave".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_leave".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let left_member = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_leave".into(),
            principal_id: "u_member".into(),
        })
        .expect("member should be able to leave group conversation");
    assert_eq!(left_member.state, MembershipState::Left);
    assert!(left_member.removed_at.is_some());

    let members = runtime
        .list_members("t_demo", "c_group_leave")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "u_owner");

    let post_after_leave = runtime.post_message(PostMessageCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_leave".into(),
        sender: Sender {
            id: "u_member".into(),
            kind: "user".into(),
            member_id: None,
            device_id: Some("d_member".into()),
            session_id: Some("s_member".into()),
            metadata: Default::default(),
        },
        client_msg_id: Some("client_after_leave".into()),
        message_type: MessageType::Standard,
        body: MessageBody {
            summary: Some("after leave".into()),
            parts: vec![ContentPart::text("after leave")],
            render_hints: Default::default(),
        },
    });
    assert!(matches!(
        post_after_leave,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_group_owner_cannot_leave_without_transfer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_leave".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let leave = runtime.leave_conversation(LeaveConversationCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_owner_leave".into(),
        principal_id: "u_owner".into(),
    });
    assert!(matches!(leave, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_direct_conversation_rejects_leave_for_now() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_leave".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_leave".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add direct peer");

    let leave = runtime.leave_conversation(LeaveConversationCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_direct_leave".into(),
        principal_id: "u_peer".into(),
    });
    assert!(matches!(leave, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_transfer_ownership_and_then_leave() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let promoted_member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let transfer = runtime
        .transfer_conversation_owner(TransferConversationOwnerCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner".into(),
            target_member_id: promoted_member.member_id,
            transferred_by: "u_owner".into(),
        })
        .expect("owner transfer should succeed");
    assert_eq!(transfer.previous_owner.role, MembershipRole::Admin);
    assert_eq!(transfer.new_owner.role, MembershipRole::Owner);

    let leave = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner".into(),
            principal_id: "u_owner".into(),
        })
        .expect("previous owner should be able to leave after transfer");
    assert_eq!(leave.state, MembershipState::Left);

    let members = runtime
        .list_members("t_demo", "c_group_transfer_owner")
        .expect("list members should succeed");
    assert_eq!(members.len(), 1);
    assert_eq!(members[0].principal_id, "u_member");
    assert_eq!(members[0].role, MembershipRole::Owner);
}

#[test]
fn test_owner_transfer_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_group_owner_system".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_system".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "svc_ops".into(),
        })
        .expect("system owner should be able to add member");

    runtime
        .transfer_conversation_owner(TransferConversationOwnerCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_owner_system".into(),
            target_member_id: member.member_id,
            transferred_by: "svc_ops".into(),
        })
        .expect("system owner should be able to transfer ownership");

    let transfer_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.owner_transferred"
                && event.aggregate_id == "c_group_owner_system"
        })
        .expect("owner transfer event should exist");
    assert_eq!(transfer_event.actor.actor_id, "svc_ops");
    assert_eq!(transfer_event.actor.actor_kind, "system");
}

#[test]
fn test_group_admin_cannot_transfer_ownership() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let admin = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            principal_id: "u_admin".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add admin");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_transfer_owner_forbidden".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let transfer = runtime.transfer_conversation_owner(TransferConversationOwnerCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_transfer_owner_forbidden".into(),
        target_member_id: member.member_id,
        transferred_by: admin.principal_id,
    });
    assert!(matches!(transfer, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_direct_conversation_rejects_owner_transfer() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_transfer_owner".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_transfer_owner".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add direct peer");

    let transfer = runtime.transfer_conversation_owner(TransferConversationOwnerCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_direct_transfer_owner".into(),
        target_member_id: peer.member_id,
        transferred_by: "u_owner".into(),
    });
    assert!(matches!(transfer, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_owner_can_change_non_owner_member_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let promote = runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "u_owner".into(),
        })
        .expect("owner should be able to promote member");
    assert_eq!(promote.previous_member.role, MembershipRole::Member);
    assert_eq!(promote.updated_member.role, MembershipRole::Admin);

    let demote = runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change".into(),
            target_member_id: member.member_id.clone(),
            new_role: MembershipRole::Guest,
            changed_by: "u_owner".into(),
        })
        .expect("owner should be able to demote admin");
    assert_eq!(demote.previous_member.role, MembershipRole::Admin);
    assert_eq!(demote.updated_member.role, MembershipRole::Guest);
    assert_ne!(demote.event_id, promote.event_id);

    let members = runtime
        .list_members("t_demo", "c_group_role_change")
        .expect("list members should succeed");
    let target = members
        .into_iter()
        .find(|item| item.principal_id == "u_member")
        .expect("target member should exist");
    assert_eq!(target.role, MembershipRole::Guest);

    let events = journal.recorded();
    assert_eq!(
        events
            .iter()
            .filter(|event| event.event_type == "conversation.member_role_changed")
            .count(),
        2
    );
}

#[test]
fn test_member_role_changed_event_preserves_system_actor_kind() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation_with_creator_kind(
            CreateConversationCommand {
                tenant_id: "t_demo".into(),
                conversation_id: "c_group_role_system".into(),
                creator_id: "svc_ops".into(),
                conversation_type: "group".into(),
            },
            "system",
        )
        .expect("system actor should be able to create group conversation");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_system".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "svc_ops".into(),
        })
        .expect("system owner should be able to add member");

    runtime
        .change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_system".into(),
            target_member_id: member.member_id,
            new_role: MembershipRole::Admin,
            changed_by: "svc_ops".into(),
        })
        .expect("system owner should be able to change member role");

    let role_changed_event = journal
        .recorded()
        .into_iter()
        .find(|event| {
            event.event_type == "conversation.member_role_changed"
                && event.aggregate_id == "c_group_role_system"
        })
        .expect("member role changed event should exist");
    assert_eq!(role_changed_event.actor.actor_id, "svc_ops");
    assert_eq!(role_changed_event.actor.actor_kind, "system");
}

#[test]
fn test_group_admin_cannot_change_member_roles() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let admin = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            principal_id: "u_admin".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Admin,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add admin");

    let member = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_forbidden".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let change = runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_role_change_forbidden".into(),
        target_member_id: member.member_id,
        new_role: MembershipRole::Guest,
        changed_by: admin.principal_id,
    });
    assert!(matches!(change, Err(RuntimeError::PermissionDenied(_))));
}

#[test]
fn test_group_role_change_rejects_owner_target_and_direct_conversation() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_owner_target".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("group create conversation should succeed");

    let owner_target =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_owner_target".into(),
            target_member_id: "cm_c_group_role_change_owner_target_u_owner".into(),
            new_role: MembershipRole::Admin,
            changed_by: "u_owner".into(),
        });
    assert!(matches!(
        owner_target,
        Err(RuntimeError::PermissionDenied(_))
    ));

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_role_change".into(),
            creator_id: "u_owner".into(),
            conversation_type: "direct".into(),
        })
        .expect("direct create conversation should succeed");

    let peer = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_role_change".into(),
            principal_id: "u_peer".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add direct peer");

    let direct_change =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_direct_role_change".into(),
            target_member_id: peer.member_id,
            new_role: MembershipRole::Guest,
            changed_by: "u_owner".into(),
        });
    assert!(matches!(
        direct_change,
        Err(RuntimeError::PermissionDenied(_))
    ));
}

#[test]
fn test_stale_member_id_cannot_change_rejoined_member_role() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "u_member".into(),
        })
        .expect("member should be able to leave");

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to re-add left member");
    assert_ne!(rejoined.member_id, first_join.member_id);

    let change_stale =
        runtime.change_conversation_member_role(ChangeConversationMemberRoleCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_role_change_rejoin_guard".into(),
            target_member_id: first_join.member_id.clone(),
            new_role: MembershipRole::Admin,
            changed_by: "u_owner".into(),
        });
    assert!(matches!(
        change_stale,
        Err(RuntimeError::MemberNotFound(member_id)) if member_id == first_join.member_id
    ));

    let members = runtime
        .list_members("t_demo", "c_group_role_change_rejoin_guard")
        .expect("list members should succeed");
    let target = members
        .into_iter()
        .find(|item| item.principal_id == "u_member")
        .expect("target member should exist");
    assert_eq!(target.member_id, rejoined.member_id);
    assert_eq!(target.role, MembershipRole::Member);
}

#[test]
fn test_left_member_rejoin_creates_new_membership_episode() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    let left_member = runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "u_member".into(),
        })
        .expect("member should be able to leave");
    assert_eq!(left_member.member_id, first_join.member_id);
    assert_eq!(left_member.state, MembershipState::Left);

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to re-add left member");

    assert_ne!(rejoined.member_id, first_join.member_id);
    assert_eq!(rejoined.state, MembershipState::Joined);
    assert!(rejoined.removed_at.is_none());

    let view = runtime
        .read_cursor_view("t_demo", "c_group_rejoin", "u_member")
        .expect("rejoined member read cursor view should succeed");
    assert_eq!(view.member_id, rejoined.member_id);
    assert_eq!(view.read_seq, 0);
}

#[test]
fn test_stale_member_id_cannot_remove_rejoined_active_member() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first_join = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to add member");

    runtime
        .leave_conversation(LeaveConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "u_member".into(),
        })
        .expect("member should be able to leave");

    let rejoined = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_group_rejoin_remove_guard".into(),
            principal_id: "u_member".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("owner should be able to re-add left member");
    assert_ne!(rejoined.member_id, first_join.member_id);

    let remove_stale = runtime.remove_member(RemoveConversationMemberCommand {
        tenant_id: "t_demo".into(),
        conversation_id: "c_group_rejoin_remove_guard".into(),
        member_id: first_join.member_id.clone(),
        removed_by: "u_owner".into(),
    });
    assert!(matches!(
        remove_stale,
        Err(RuntimeError::MemberNotFound(member_id)) if member_id == first_join.member_id
    ));

    let members = runtime
        .list_members("t_demo", "c_group_rejoin_remove_guard")
        .expect("list members should succeed");
    assert_eq!(members.len(), 2);
    assert!(
        members
            .iter()
            .any(|member| member.member_id == rejoined.member_id)
    );
    assert!(members.iter().all(ConversationMember::is_active));
}

#[test]
fn test_posted_message_timestamps_advance_between_distinct_messages() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_posted_time".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_posted_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_time_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
            },
        })
        .expect("first message should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_posted_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_time_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
            },
        })
        .expect("second message should succeed");

    let events = journal.recorded();
    let posted_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.posted")
        .collect();
    assert_eq!(posted_events.len(), 2);
    assert_ne!(
        posted_events[0].occurred_at, posted_events[1].occurred_at,
        "separate posted messages must not reuse a fixed occurred_at timestamp"
    );
    assert_ne!(
        posted_events[0].committed_at, posted_events[1].committed_at,
        "separate posted messages must not reuse a fixed committed_at timestamp"
    );
}

#[test]
fn test_read_cursor_timestamps_advance_between_distinct_updates() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_time".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
            },
        })
        .expect("first message should succeed");
    runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: None,
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_cursor_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
            },
        })
        .expect("second message should succeed");

    let first = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_time".into(),
            principal_id: "u_demo".into(),
            read_seq: 1,
            last_read_message_id: Some("msg_c_cursor_time_1".into()),
        })
        .expect("first read cursor update should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .update_read_cursor(UpdateReadCursorCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_cursor_time".into(),
            principal_id: "u_demo".into(),
            read_seq: 2,
            last_read_message_id: Some("msg_c_cursor_time_2".into()),
        })
        .expect("second read cursor update should succeed");

    assert_ne!(
        first.updated_at, second.updated_at,
        "separate read cursor updates must not reuse a fixed updated_at timestamp"
    );

    let events = journal.recorded();
    let cursor_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "conversation.read_cursor_updated")
        .collect();
    assert_eq!(cursor_events.len(), 2);
    assert_ne!(
        cursor_events[0].occurred_at, cursor_events[1].occurred_at,
        "separate read cursor updates must not reuse a fixed envelope occurred_at timestamp"
    );
}

#[test]
fn test_membership_timestamps_advance_between_distinct_join_and_remove_mutations() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal);

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_member_time".into(),
            creator_id: "u_owner".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_member_time".into(),
            principal_id: "u_member_1".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("first add member should succeed");

    sleep(Duration::from_millis(5));

    let second = runtime
        .add_member(AddConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_member_time".into(),
            principal_id: "u_member_2".into(),
            principal_kind: "user".into(),
            role: MembershipRole::Member,
            invited_by: "u_owner".into(),
        })
        .expect("second add member should succeed");

    assert_ne!(
        first.joined_at, second.joined_at,
        "separate joined members must not reuse a fixed joined_at timestamp"
    );

    let removed_first = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_member_time".into(),
            member_id: first.member_id.clone(),
            removed_by: "u_owner".into(),
        })
        .expect("first remove member should succeed");

    sleep(Duration::from_millis(5));

    let removed_second = runtime
        .remove_member(RemoveConversationMemberCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_member_time".into(),
            member_id: second.member_id.clone(),
            removed_by: "u_owner".into(),
        })
        .expect("second remove member should succeed");

    assert_ne!(
        removed_first.removed_at, removed_second.removed_at,
        "separate removed members must not reuse a fixed removed_at timestamp"
    );
}

#[test]
fn test_message_edit_and_recall_timestamps_advance_between_distinct_mutations() {
    let journal = InMemoryJournal::default();
    let runtime = ConversationRuntime::new(journal.clone());

    runtime
        .create_conversation(CreateConversationCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_mutation_time".into(),
            creator_id: "u_demo".into(),
            conversation_type: "group".into(),
        })
        .expect("create conversation should succeed");

    let first = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_mutation_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation_1".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("one".into()),
                parts: vec![ContentPart::text("one")],
                render_hints: Default::default(),
            },
        })
        .expect("first message should succeed");
    let second = runtime
        .post_message(PostMessageCommand {
            tenant_id: "t_demo".into(),
            conversation_id: "c_mutation_time".into(),
            sender: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            client_msg_id: Some("client_mutation_2".into()),
            message_type: MessageType::Standard,
            body: MessageBody {
                summary: Some("two".into()),
                parts: vec![ContentPart::text("two")],
                render_hints: Default::default(),
            },
        })
        .expect("second message should succeed");

    runtime
        .edit_message(EditMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: first.message_id.clone(),
            editor: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited one".into()),
                parts: vec![ContentPart::text("edited one")],
                render_hints: Default::default(),
            },
        })
        .expect("first edit should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .edit_message(EditMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: second.message_id.clone(),
            editor: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
            body: MessageBody {
                summary: Some("edited two".into()),
                parts: vec![ContentPart::text("edited two")],
                render_hints: Default::default(),
            },
        })
        .expect("second edit should succeed");

    runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: first.message_id.clone(),
            recalled_by: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("first recall should succeed");

    sleep(Duration::from_millis(5));

    runtime
        .recall_message(RecallMessageCommand {
            tenant_id: "t_demo".into(),
            message_id: second.message_id.clone(),
            recalled_by: Sender {
                id: "u_demo".into(),
                kind: "user".into(),
                member_id: None,
                device_id: Some("d_demo".into()),
                session_id: Some("s_demo".into()),
                metadata: Default::default(),
            },
        })
        .expect("second recall should succeed");

    let events = journal.recorded();
    let edited_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.edited")
        .collect();
    assert_eq!(edited_events.len(), 2);
    assert_ne!(
        edited_events[0].occurred_at, edited_events[1].occurred_at,
        "separate edits must not reuse a fixed edited_at timestamp"
    );

    let recalled_events: Vec<_> = events
        .iter()
        .filter(|event| event.event_type == "message.recalled")
        .collect();
    assert_eq!(recalled_events.len(), 2);
    assert_ne!(
        recalled_events[0].occurred_at, recalled_events[1].occurred_at,
        "separate recalls must not reuse a fixed recalled_at timestamp"
    );
}
