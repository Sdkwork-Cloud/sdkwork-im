import 'paths.dart';
import '../http/client.dart';

class SocialApi {
  final HttpClient _client;

  SocialApi(this._client);

  /// get_api_v1_control_social_direct_chats_direct_chat_id
  Future<dynamic> getApiV1ControlSocialDirectChatsDirectChatId(
    Object directChatId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/direct-chats/${Uri.encodeComponent(String(directChatId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_direct_chats_bindings
  Future<dynamic> postApiV1ControlSocialDirectChatsBindings(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/direct-chats/bindings'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_external_connections
  Future<dynamic> postApiV1ControlSocialExternalConnections(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/external-connections'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_external_connections_connection_id
  Future<dynamic> getApiV1ControlSocialExternalConnectionsConnectionId(
    Object connectionId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/external-connections/${Uri.encodeComponent(String(connectionId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_external_member_links
  Future<dynamic> postApiV1ControlSocialExternalMemberLinks(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/external-member-links'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_external_member_links_link_id
  Future<dynamic> getApiV1ControlSocialExternalMemberLinksLinkId(
    Object linkId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/external-member-links/${Uri.encodeComponent(String(linkId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_friend_requests
  Future<dynamic> postApiV1ControlSocialFriendRequests(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/friend-requests'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_friend_requests_request_id
  Future<dynamic> getApiV1ControlSocialFriendRequestsRequestId(
    Object requestId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/friend-requests/${Uri.encodeComponent(String(requestId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_friendships
  Future<dynamic> postApiV1ControlSocialFriendships(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/friendships'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_friendships_friendship_id
  Future<dynamic> getApiV1ControlSocialFriendshipsFriendshipId(
    Object friendshipId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/friendships/${Uri.encodeComponent(String(friendshipId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_runtime_claim_pending_shared_channel_sync_targeted
  Future<dynamic> postApiV1ControlSocialRuntimeClaimPendingSharedChannelSyncTargeted(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_runtime_dead_letter_shared_channel_sync
  Future<dynamic> getApiV1ControlSocialRuntimeDeadLetterSharedChannelSync(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/runtime/dead-letter-shared-channel-sync'),
      params: params,
      headers: headers,
    );
  }

  /// get_api_v1_control_social_runtime_delivered_shared_channel_sync
  Future<dynamic> getApiV1ControlSocialRuntimeDeliveredSharedChannelSync(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/runtime/delivered-shared-channel-sync'),
      params: params,
      headers: headers,
    );
  }

  /// get_api_v1_control_social_runtime_delivery_state_shared_channel_sync
  Future<dynamic> getApiV1ControlSocialRuntimeDeliveryStateSharedChannelSync(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/runtime/delivery-state-shared-channel-sync'),
      params: params,
      headers: headers,
    );
  }

  /// get_api_v1_control_social_runtime_pending_shared_channel_sync
  Future<dynamic> getApiV1ControlSocialRuntimePendingSharedChannelSync(
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/runtime/pending-shared-channel-sync'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_runtime_reclaim_stale_pending_shared_channel_sync
  Future<dynamic> postApiV1ControlSocialRuntimeReclaimStalePendingSharedChannelSync(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_release_pending_shared_channel_sync_targeted
  Future<dynamic> postApiV1ControlSocialRuntimeReleasePendingSharedChannelSyncTargeted(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_repair_derived_snapshot
  Future<dynamic> postApiV1ControlSocialRuntimeRepairDerivedSnapshot(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/repair-derived-snapshot'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_repair_shared_channel_sync
  Future<dynamic> postApiV1ControlSocialRuntimeRepairSharedChannelSync(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/repair-shared-channel-sync'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_republish_pending_shared_channel_sync_targeted
  Future<dynamic> postApiV1ControlSocialRuntimeRepublishPendingSharedChannelSyncTargeted(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_requeue_dead_letter_shared_channel_sync_targeted
  Future<dynamic> postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSyncTargeted(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_requeue_dead_letter_shared_channel_sync
  Future<dynamic> postApiV1ControlSocialRuntimeRequeueDeadLetterSharedChannelSync(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_runtime_takeover_pending_shared_channel_sync_targeted
  Future<dynamic> postApiV1ControlSocialRuntimeTakeoverPendingSharedChannelSyncTargeted(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// post_api_v1_control_social_shared_channel_policies
  Future<dynamic> postApiV1ControlSocialSharedChannelPolicies(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/shared-channel-policies'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_shared_channel_policies_policy_id
  Future<dynamic> getApiV1ControlSocialSharedChannelPoliciesPolicyId(
    Object policyId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/shared-channel-policies/${Uri.encodeComponent(String(policyId))}'),
      params: params,
      headers: headers,
    );
  }

  /// post_api_v1_control_social_user_blocks
  Future<dynamic> postApiV1ControlSocialUserBlocks(
    {
      dynamic body,
      Map<String, dynamic>? params,
      Map<String, String>? headers,
      String? contentType,
    }
  ) {
    return _client.post(
      backendApiPath('/api/v1/control/social/user-blocks'),
      body: body,
      params: params,
      headers: headers,
      contentType: contentType,
    );
  }

  /// get_api_v1_control_social_user_blocks_block_id
  Future<dynamic> getApiV1ControlSocialUserBlocksBlockId(
    Object blockId,
    {
      Map<String, dynamic>? params,
      Map<String, String>? headers,
    }
  ) {
    return _client.get(
      backendApiPath('/api/v1/control/social/user-blocks/${Uri.encodeComponent(String(blockId))}'),
      params: params,
      headers: headers,
    );
  }
}
