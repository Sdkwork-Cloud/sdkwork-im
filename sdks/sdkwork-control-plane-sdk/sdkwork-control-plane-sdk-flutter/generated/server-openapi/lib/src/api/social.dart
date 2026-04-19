import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class SocialApi {
  final AdminHttpClient _httpClient;

  SocialApi(this._httpClient);

  Future<JsonObject> bindDirectChat(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/direct-chats/bindings'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.bindDirectChat',
    );
  }

  Future<JsonObject> getDirectChatSnapshot(Identifier directChatId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/direct-chats/${encodeIdentifier(directChatId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getDirectChatSnapshot',
    );
  }

  Future<JsonObject> establishExternalConnection(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/external-connections'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.establishExternalConnection',
    );
  }

  Future<JsonObject> getExternalConnectionSnapshot(Identifier connectionId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/external-connections/${encodeIdentifier(connectionId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getExternalConnectionSnapshot',
    );
  }

  Future<JsonObject> bindExternalMemberLink(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/external-member-links'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.bindExternalMemberLink',
    );
  }

  Future<JsonObject> getExternalMemberLinkSnapshot(Identifier linkId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/external-member-links/${encodeIdentifier(linkId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getExternalMemberLinkSnapshot',
    );
  }

  Future<JsonObject> submitFriendRequest(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/friend-requests'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.submitFriendRequest',
    );
  }

  Future<JsonObject> getFriendRequestSnapshot(Identifier requestId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/friend-requests/${encodeIdentifier(requestId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getFriendRequestSnapshot',
    );
  }

  Future<JsonObject> activateFriendship(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/friendships'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.activateFriendship',
    );
  }

  Future<JsonObject> getFriendshipSnapshot(Identifier friendshipId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/friendships/${encodeIdentifier(friendshipId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getFriendshipSnapshot',
    );
  }

  Future<JsonObject> applySharedChannelPolicy(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/shared-channel-policies'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.applySharedChannelPolicy',
    );
  }

  Future<JsonObject> getSharedChannelPolicySnapshot(Identifier policyId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/shared-channel-policies/${encodeIdentifier(policyId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getSharedChannelPolicySnapshot',
    );
  }

  Future<JsonObject> blockUser(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/user-blocks'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.blockUser',
    );
  }

  Future<JsonObject> getUserBlockSnapshot(Identifier blockId) async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/user-blocks/${encodeIdentifier(blockId)}'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'social.getUserBlockSnapshot',
    );
  }
}
