use im_domain_events::{
    AggregateType, CommitEnvelope, EventActor,
    social::{
        DirectChatBoundPayload, ExternalMemberLinkBoundPayload, FriendRequestSubmittedPayload,
        FriendshipActivatedPayload, SocialEventType, UserBlockedPayload, social_commit_envelope,
    },
};
use serde_json::Value;

#[test]
fn test_social_aggregate_types_expose_stable_wire_values() {
    assert_eq!(
        AggregateType::FriendRequest.as_wire_value(),
        "friend_request"
    );
    assert_eq!(AggregateType::Friendship.as_wire_value(), "friendship");
    assert_eq!(AggregateType::UserBlock.as_wire_value(), "user_block");
    assert_eq!(AggregateType::DirectChat.as_wire_value(), "direct_chat");
}

#[test]
fn test_social_event_type_exposes_wire_and_schema_values() {
    assert_eq!(
        SocialEventType::FriendRequestSubmitted.as_wire_value(),
        "friend_request.submitted"
    );
    assert_eq!(
        SocialEventType::UserBlocked.as_wire_value(),
        "user_block.blocked"
    );
    assert_eq!(
        SocialEventType::FriendshipActivated.payload_schema(),
        "social.friendship.activated.v1"
    );
    assert_eq!(
        SocialEventType::DirectChatBound.as_wire_value(),
        "direct_chat.bound"
    );
}

#[test]
fn test_social_payloads_serialize_expected_shape() {
    let request = FriendRequestSubmittedPayload {
        request_id: "fr_demo".into(),
        requester_user_id: "user_a".into(),
        target_user_id: "user_b".into(),
        request_message: None,
        requested_at: "2026-04-10T10:00:00Z".into(),
    };
    let friendship = FriendshipActivatedPayload {
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        initiator_user_id: "user_a".into(),
        direct_chat_id: Some("dc_demo".into()),
        established_at: "2026-04-10T10:01:00Z".into(),
    };
    let blocked = UserBlockedPayload {
        block_id: "blk_demo".into(),
        blocker_user_id: "user_a".into(),
        blocked_user_id: "user_b".into(),
        scope: "all".into(),
        direct_chat_id: Some("dc_demo".into()),
        expires_at: None,
        effective_at: "2026-04-10T10:02:00Z".into(),
    };
    let bound = DirectChatBoundPayload {
        direct_chat_id: "dc_demo".into(),
        conversation_id: "c_demo".into(),
        left_actor_id: "actor_a".into(),
        right_actor_id: "actor_b".into(),
        pair_hash: "actor_a:actor_b".into(),
        bound_at: "2026-04-10T10:03:00Z".into(),
    };
    let external_member_link = ExternalMemberLinkBoundPayload {
        link_id: "eml_demo".into(),
        connection_id: "ec_demo".into(),
        local_actor_id: "actor_a".into(),
        local_actor_kind: Some("user".into()),
        external_member_id: "partner::alice".into(),
        external_display_name: Some("Alice Partner".into()),
        linked_at: "2026-04-10T10:04:00Z".into(),
    };

    let request_value = serde_json::to_value(request).expect("request should serialize");
    let friendship_value = serde_json::to_value(friendship).expect("friendship should serialize");
    let blocked_value = serde_json::to_value(blocked).expect("block should serialize");
    let bound_value = serde_json::to_value(bound).expect("bound should serialize");
    let external_member_link_value =
        serde_json::to_value(external_member_link).expect("external member link should serialize");

    assert_eq!(request_value["requestId"], Value::String("fr_demo".into()));
    assert_eq!(
        friendship_value["directChatId"],
        Value::String("dc_demo".into())
    );
    assert_eq!(blocked_value["scope"], Value::String("all".into()));
    assert_eq!(
        bound_value["conversationId"],
        Value::String("c_demo".into())
    );
    assert_eq!(
        external_member_link_value["localActorKind"],
        Value::String("user".into())
    );
}

#[test]
fn test_social_commit_envelope_builds_social_defaults() {
    let payload = serde_json::to_string(&FriendshipActivatedPayload {
        friendship_id: "fs_demo".into(),
        user_low_id: "user_a".into(),
        user_high_id: "user_b".into(),
        initiator_user_id: "user_a".into(),
        direct_chat_id: Some("dc_demo".into()),
        established_at: "2026-04-10T10:01:00Z".into(),
    })
    .expect("payload should serialize");

    let envelope = social_commit_envelope(
        "evt_demo",
        "t_demo",
        AggregateType::Friendship,
        "fs_demo",
        SocialEventType::FriendshipActivated,
        7,
        EventActor {
            actor_id: "user_a".into(),
            actor_kind: "user".into(),
            actor_session_id: Some("s_demo".into()),
        },
        "2026-04-10T10:01:00Z",
        "2026-04-10T10:01:01Z",
        &payload,
    );

    assert_eq!(envelope.aggregate_type, AggregateType::Friendship);
    assert_eq!(envelope.event_type, "friendship.activated");
    assert_eq!(
        envelope.payload_schema,
        Some("social.friendship.activated.v1".into())
    );
    assert_eq!(envelope.scope_type, "friendship");
    assert_eq!(envelope.scope_id, "fs_demo");
    assert_eq!(
        envelope.ordering_key,
        CommitEnvelope::ordering_key("t_demo", "fs_demo")
    );
    assert_eq!(envelope.audit_class, "social");
    assert_eq!(envelope.retention_class, "standard");
}
