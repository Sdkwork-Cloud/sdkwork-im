#[test]
fn test_control_plane_social_state_uses_pair_indexes_for_hot_uniqueness_checks() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "active_friendship_pair_index: BTreeMap<SocialPairIndexKey, String>",
        "friendship_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>",
        "active_direct_chat_pair_index: BTreeMap<SocialPairIndexKey, String>",
        "fn rebuild_social_pair_indexes(",
        "fn insert_friendship_record(",
        "fn insert_direct_chat_record(",
        "fn active_friendship_record_for_pair(",
        "fn active_direct_chat_record_for_pair(",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social runtime must maintain indexed pair lookups for hot uniqueness checks: {required_symbol}"
        );
    }

    for forbidden_hot_scan in [
        "next_state.friendships.values().find(|record|",
        "next_state.friendships.values().any(|record|",
        "next_state.direct_chats.values().find(|record|",
        "state\n        .friendships\n        .values()\n        .filter_map(|record|",
        "state\n        .direct_chats\n        .values()\n        .filter_map(|record|",
    ] {
        assert!(
            !source.contains(forbidden_hot_scan),
            "control-plane social pair uniqueness/access helpers must not scan all records: {forbidden_hot_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_pair_archive_paths_update_indexes_incrementally() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    let friendship_removal_replay_body = source
        .split("fn apply_friendship_removed_commit(")
        .nth(1)
        .and_then(|rest| rest.split("fn apply_user_block_commit(").next())
        .expect("friendship removal replay body should be present");
    for required_symbol in [
        "self.insert_friendship_record(friendship.friendship_id.clone(), record);",
        "archive_active_direct_chats_for_pair(",
    ] {
        assert!(
            friendship_removal_replay_body.contains(required_symbol),
            "friendship removal replay must incrementally update derived indexes: {required_symbol}"
        );
    }
    for forbidden_rebuild in [
        "self.rebuild_social_pair_indexes();",
        "self.rebuild_social_committed_event_index();",
    ] {
        assert!(
            !friendship_removal_replay_body.contains(forbidden_rebuild),
            "friendship removal replay must not rebuild all social indexes: {forbidden_rebuild}"
        );
    }

    let archive_body = source
        .split("fn archive_active_direct_chats_for_pair(")
        .nth(1)
        .and_then(|rest| rest.split("async fn list_friend_requests(").next())
        .expect("direct-chat archive helper body should be present");
    assert!(
        archive_body.contains("state.insert_direct_chat_record(direct_chat_id, record);"),
        "direct-chat archive helper must use the unified insert path to maintain derived indexes"
    );
    assert!(
        !archive_body.contains("state.rebuild_social_pair_indexes();"),
        "direct-chat archive helper must not rebuild all pair indexes after targeted archive"
    );
}

#[test]
fn test_control_plane_social_state_uses_user_block_indexes_for_hot_access_checks() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "active_user_block_scope_index: BTreeMap<SocialUserBlockScopeIndexKey, String>",
        "active_friendship_block_pair_index: BTreeMap<SocialPairIndexKey, String>",
        "active_direct_chat_block_pair_index: BTreeMap<SocialPairIndexKey, String>",
        "active_direct_chat_block_chat_index: BTreeMap<SocialDirectChatBlockIndexKey, String>",
        "fn rebuild_social_user_block_indexes(",
        "fn insert_user_block_record(",
        "fn active_user_block_for_scope(",
        "fn active_friendship_scoped_user_block(",
        "fn active_direct_chat_scoped_user_block(",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social runtime must maintain indexed user-block lookups for hot access checks: {required_symbol}"
        );
    }

    for forbidden_hot_scan in [
        "next_state.user_blocks.values().any(|record|",
        "state.user_blocks.values().find_map(|record|",
        ".user_blocks\n        .values()\n        .find_map(|record|",
        "BlockScope::DirectChat => {\n            direct_chat_pair_index.insert(pair_key, record.user_block.block_id.clone());",
    ] {
        assert!(
            !source.contains(forbidden_hot_scan),
            "control-plane social user-block access helpers must not scan all records: {forbidden_hot_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_state_uses_committed_event_index_for_idempotency_checks() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "committed_event_index: BTreeMap<SocialCommittedEventIndexKey, SocialCommittedEventPointer>",
        "enum SocialCommittedEventPointer",
        "fn rebuild_social_committed_event_index(",
        "fn index_social_commits(",
        "fn committed_event(&self, tenant_id: &str, event_id: &str) -> Option<SocialCommittedEvent>",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social runtime must use a derived committed-event index for write idempotency checks: {required_symbol}"
        );
    }

    let committed_event_body = source
        .split("fn committed_event(&self, tenant_id: &str, event_id: &str) -> Option<SocialCommittedEvent>")
        .nth(1)
        .and_then(|rest| rest.split("fn aggregate_counts").next())
        .expect("committed_event body should be present");

    for forbidden_scan in [
        ".values()\n            .find_map(|record|",
        "find_committed_social_event(record.commits.as_slice(), tenant_id, event_id)",
    ] {
        assert!(
            !committed_event_body.contains(forbidden_scan),
            "committed_event must resolve by derived index instead of scanning all aggregate commits: {forbidden_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_state_uses_friend_request_pair_indexes_for_hot_conflict_checks() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "pending_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>",
        "accepted_friend_request_pair_index: BTreeMap<SocialPairIndexKey, BTreeSet<String>>",
        "fn rebuild_social_friend_request_indexes(",
        "fn index_friend_request_record(",
        "fn unindex_friend_request_record(",
        "fn open_friend_request_record_for_pair(",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social runtime must maintain indexed friend-request pair lookups for hot conflict checks: {required_symbol}"
        );
    }

    for forbidden_hot_scan in [
        "next_state.friend_requests.values().find(|record|",
        ".friend_requests\n            .get_mut(",
        "record.friend_request.status == FriendRequestStatus::Pending\n                    || (record.friend_request.status == FriendRequestStatus::Accepted",
    ] {
        assert!(
            !source.contains(forbidden_hot_scan),
            "control-plane friend-request conflict checks must not scan all records: {forbidden_hot_scan}"
        );
    }
}

#[test]
fn test_control_plane_friend_request_inventory_uses_user_index() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "friend_request_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>",
        "fn friend_request_records_for_user(",
    ] {
        assert!(
            source.contains(required_symbol),
            "friend-request inventory must use a user-scoped derived index: {required_symbol}"
        );
    }

    let list_body = source
        .split("fn list_friend_requests(")
        .nth(1)
        .and_then(|rest| rest.split("fn accept_friend_request(").next())
        .expect("friend request inventory body should be present");
    assert!(
        list_body.contains("friend_request_records_for_user("),
        "friend request inventory must read candidate records from the user index"
    );
    for forbidden_scan in [
        ".friend_requests\n            .values()\n            .filter(",
        ".friend_requests.values().filter(",
    ] {
        assert!(
            !list_body.contains(forbidden_scan),
            "friend request inventory must not scan every friend request: {forbidden_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_state_uses_external_collaboration_indexes_for_hot_target_checks() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "active_external_connection_target_index:",
        "BTreeMap<SocialExternalConnectionTargetIndexKey, String>",
        "active_external_member_mapping_index: BTreeMap<SocialExternalMemberMappingIndexKey, String>",
        "active_shared_channel_policy_target_index:",
        "BTreeMap<SocialSharedChannelPolicyTargetIndexKey, String>",
        "struct SocialExternalConnectionTargetIndexKey",
        "struct SocialExternalMemberMappingIndexKey",
        "struct SocialSharedChannelPolicyTargetIndexKey",
        "fn rebuild_social_external_collaboration_indexes(",
        "fn index_external_connection_record(",
        "fn index_external_member_link_record(",
        "fn index_shared_channel_policy_record(",
        "fn active_external_connection_record_for_target(",
        "fn active_external_member_link_record_for_mapping(",
        "fn active_shared_channel_policy_record_for_target(",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social runtime must maintain indexed external-collaboration target lookups for hot uniqueness checks: {required_symbol}"
        );
    }

    for forbidden_hot_scan in [
        "next_state.external_connections.values().any(|record|",
        "next_state.external_member_links.values().any(|record|",
        "next_state.shared_channel_policies.values().any(|record|",
    ] {
        assert!(
            !source.contains(forbidden_hot_scan),
            "control-plane external-collaboration target checks must not scan all records: {forbidden_hot_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_state_uses_connection_indexes_for_shared_channel_fanout() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "active_external_member_connection_index:",
        "BTreeMap<SocialConnectionIndexKey, BTreeSet<String>>",
        "active_shared_channel_policy_connection_index:",
        "struct SocialConnectionIndexKey",
        "fn active_external_member_link_records_for_connection(",
        "fn active_shared_channel_policy_records_for_connection(",
    ] {
        assert!(
            source.contains(required_symbol),
            "shared-channel fan-out must use connection-scoped active indexes: {required_symbol}"
        );
    }

    let member_link_fanout_body = source
        .split("fn shared_channel_sync_requests_for_external_member_link(")
        .nth(1)
        .and_then(|rest| {
            rest.split("fn shared_channel_sync_requests_for_shared_channel_policy(")
                .next()
        })
        .expect("external-member-link fan-out body should be present");
    for forbidden_scan in [
        ".shared_channel_policies\n        .values()\n        .filter_map(",
        ".shared_channel_policies.values().filter_map(",
    ] {
        assert!(
            !member_link_fanout_body.contains(forbidden_scan),
            "external-member-link fan-out must not scan every shared-channel policy: {forbidden_scan}"
        );
    }

    let policy_fanout_body = source
        .split("fn shared_channel_sync_requests_for_shared_channel_policy(")
        .nth(1)
        .and_then(|rest| rest.split("fn shared_channel_sync_request_key(").next())
        .expect("shared-channel-policy fan-out body should be present");
    for forbidden_scan in [
        ".external_member_links\n        .values()\n        .filter_map(",
        ".external_member_links.values().filter_map(",
    ] {
        assert!(
            !policy_fanout_body.contains(forbidden_scan),
            "shared-channel-policy fan-out must not scan every external member link: {forbidden_scan}"
        );
    }
}

#[test]
fn test_control_plane_social_query_paths_use_indexes_for_authoritative_active_access() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "active_friendship_user_index: BTreeMap<SocialUserIndexKey, BTreeSet<String>>",
        "struct SocialUserIndexKey",
        "fn active_friendship_records_for_user(",
        "fn active_direct_chat_record_for_pair(",
    ] {
        assert!(
            source.contains(required_symbol),
            "control-plane social authoritative query paths must use derived indexes for active access: {required_symbol}"
        );
    }

    let friendship_query_body = source
        .split("fn authoritative_active_friendships_for_user(")
        .nth(1)
        .and_then(|rest| {
            rest.split("fn authoritative_active_direct_chat_for_pair(")
                .next()
        })
        .expect("authoritative_active_friendships_for_user body should be present");
    for forbidden_scan in [
        ".friendships\n            .values()\n            .filter_map(",
        ".friendships.values().filter_map(",
    ] {
        assert!(
            !friendship_query_body.contains(forbidden_scan),
            "authoritative_active_friendships_for_user must read active friendship ids from user index: {forbidden_scan}"
        );
    }

    let direct_chat_query_body = source
        .split("fn authoritative_active_direct_chat_for_pair(")
        .nth(1)
        .and_then(|rest| rest.split("}\n}\n\npub fn configured_runtime_dir").next())
        .expect("authoritative_active_direct_chat_for_pair body should be present");
    for forbidden_scan in [
        ".direct_chats\n            .values()\n            .filter_map(",
        ".direct_chats.values().filter_map(",
    ] {
        assert!(
            !direct_chat_query_body.contains(forbidden_scan),
            "authoritative_active_direct_chat_for_pair must resolve by active pair index: {forbidden_scan}"
        );
    }
}

#[test]
fn test_control_plane_shared_channel_targeted_backlog_operations_select_by_key() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "fn selected_pending_shared_channel_sync_requests(",
        "fn selected_undelivered_pending_shared_channel_sync_requests(",
    ] {
        assert!(
            source.contains(required_symbol),
            "targeted shared-channel sync operations must select pending backlog entries by request key: {required_symbol}"
        );
    }

    for function_name in [
        "fn release_pending_shared_channel_sync_targeted(",
        "fn takeover_pending_shared_channel_sync_targeted(",
        "fn republish_pending_shared_channel_sync_targeted(",
    ] {
        let body = source
            .split(function_name)
            .nth(1)
            .and_then(|rest| rest.split("\n    fn ").next())
            .unwrap_or_else(|| panic!("{function_name} body should be present"));
        assert!(
            !body.contains("pending_shared_channel_sync_requests_with_keys()\n            .into_iter()\n            .filter("),
            "{function_name} must not materialize and filter the full pending backlog for targeted requestKeys"
        );
    }
}

#[test]
fn test_control_plane_shared_channel_dispatch_queue_uses_retry_index() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "pending_shared_channel_retry_index:",
        "BTreeMap<SharedChannelRetryIndexKey, BTreeSet<String>>",
        "struct SharedChannelRetryIndexKey",
        "fn rebuild_shared_channel_pending_indexes(",
        "fn upsert_pending_shared_channel_sync_request(",
        "fn remove_pending_shared_channel_sync_request_by_key(",
        "fn retryable_pending_shared_channel_sync_requests(",
    ] {
        assert!(
            source.contains(required_symbol),
            "shared-channel dispatch queue must maintain a retry-time index for pending backlog: {required_symbol}"
        );
    }

    let dispatch_body = source
        .split("fn pending_shared_channel_sync_dispatch_queue(")
        .nth(1)
        .and_then(|rest| {
            rest.split("fn clear_pending_shared_channel_sync_request_and_record_delivery(")
                .next()
        })
        .expect("shared-channel dispatch queue body should be present");
    assert!(
        dispatch_body.contains("retryable_pending_shared_channel_sync_requests("),
        "dispatch queue must read pending backlog candidates from retry index"
    );
    assert!(
        !dispatch_body.contains("pending_shared_channel_sync_requests.values()"),
        "dispatch queue must not scan every pending shared-channel sync request"
    );
}

#[test]
fn test_control_plane_shared_channel_stale_claim_reclaim_uses_lease_index() {
    let source = include_str!("../src/lib.rs").replace("\r\n", "\n");

    for required_symbol in [
        "pending_shared_channel_lease_index:",
        "BTreeMap<SharedChannelLeaseIndexKey, BTreeSet<String>>",
        "struct SharedChannelLeaseIndexKey",
        "fn stale_pending_shared_channel_sync_requests(",
    ] {
        assert!(
            source.contains(required_symbol),
            "shared-channel stale claim reclaim must maintain a lease-expiry index: {required_symbol}"
        );
    }

    let reclaim_body = source
        .split("fn reclaim_stale_pending_shared_channel_sync_claims(")
        .nth(1)
        .and_then(|rest| {
            rest.split("fn release_selected_pending_shared_channel_sync_requests(")
                .next()
        })
        .expect("stale claim reclaim body should be present");
    assert!(
        reclaim_body.contains("stale_pending_shared_channel_sync_requests("),
        "stale claim reclaim must read candidates from the lease index"
    );
    assert!(
        !reclaim_body.contains("pending_shared_channel_sync_requests.values_mut()"),
        "stale claim reclaim must not scan every pending shared-channel sync request"
    );
}
