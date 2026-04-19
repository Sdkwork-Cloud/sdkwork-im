import 'package:control_plane_backend_sdk/control_plane_backend_sdk.dart';

import 'context.dart';

class ControlPlaneSocialModule {
  final ControlPlaneSdkContext context;

  ControlPlaneSocialModule(this.context);

  Future<JsonObject> bindDirectChat(JsonObject body) {
    return context.backendClient.social.bindDirectChat(body);
  }

  Future<JsonObject> getDirectChat(Identifier id) {
    return context.backendClient.social.getDirectChatSnapshot(id);
  }

  Future<JsonObject> establishExternalConnection(JsonObject body) {
    return context.backendClient.social.establishExternalConnection(body);
  }

  Future<JsonObject> getExternalConnection(Identifier id) {
    return context.backendClient.social.getExternalConnectionSnapshot(id);
  }

  Future<JsonObject> bindExternalMemberLink(JsonObject body) {
    return context.backendClient.social.bindExternalMemberLink(body);
  }

  Future<JsonObject> getExternalMemberLink(Identifier id) {
    return context.backendClient.social.getExternalMemberLinkSnapshot(id);
  }

  Future<JsonObject> submitFriendRequest(JsonObject body) {
    return context.backendClient.social.submitFriendRequest(body);
  }

  Future<JsonObject> getFriendRequest(Identifier id) {
    return context.backendClient.social.getFriendRequestSnapshot(id);
  }

  Future<JsonObject> activateFriendship(JsonObject body) {
    return context.backendClient.social.activateFriendship(body);
  }

  Future<JsonObject> getFriendship(Identifier id) {
    return context.backendClient.social.getFriendshipSnapshot(id);
  }

  Future<JsonObject> applySharedChannelPolicy(JsonObject body) {
    return context.backendClient.social.applySharedChannelPolicy(body);
  }

  Future<JsonObject> getSharedChannelPolicy(Identifier id) {
    return context.backendClient.social.getSharedChannelPolicySnapshot(id);
  }

  Future<JsonObject> blockUser(JsonObject body) {
    return context.backendClient.social.blockUser(body);
  }

  Future<JsonObject> getUserBlock(Identifier id) {
    return context.backendClient.social.getUserBlockSnapshot(id);
  }
}
