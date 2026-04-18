import 'package:craw_chat_admin_backend_sdk/craw_chat_admin_backend_sdk.dart';

import 'context.dart';

class CrawChatAdminSocialRuntimeModule {
  final CrawChatAdminSdkContext context;

  CrawChatAdminSocialRuntimeModule(this.context);

  Future<JsonObject> claimPendingTargeted(JsonObject body) {
    return context.backendClient.socialRuntime
        .claimPendingSharedChannelSyncTargeted(body);
  }

  Future<JsonObject> getDeadLetterInventory() {
    return context.backendClient.socialRuntime
        .getDeadLetterSharedChannelSyncInventory();
  }

  Future<JsonObject> getDeliveredInventory() {
    return context.backendClient.socialRuntime
        .getDeliveredSharedChannelSyncInventory();
  }

  Future<JsonObject> getDeliveryStateInventory() {
    return context.backendClient.socialRuntime
        .getSharedChannelSyncDeliveryStateInventory();
  }

  Future<JsonObject> getPendingInventory() {
    return context.backendClient.socialRuntime
        .getPendingSharedChannelSyncInventory();
  }

  Future<JsonObject> reclaimStalePending() {
    return context.backendClient.socialRuntime
        .reclaimStalePendingSharedChannelSync();
  }

  Future<JsonObject> releasePendingTargeted(JsonObject body) {
    return context.backendClient.socialRuntime
        .releasePendingSharedChannelSyncTargeted(body);
  }

  Future<JsonObject> repairSnapshot() {
    return context.backendClient.socialRuntime.repairSocialRuntimeSnapshot();
  }

  Future<JsonObject> repairSharedChannelSync() {
    return context.backendClient.socialRuntime.repairSharedChannelSync();
  }

  Future<JsonObject> republishPendingTargeted(JsonObject body) {
    return context.backendClient.socialRuntime
        .republishPendingSharedChannelSyncTargeted(body);
  }

  Future<JsonObject> requeueDeadLetters() {
    return context.backendClient.socialRuntime
        .requeueDeadLetterSharedChannelSync();
  }

  Future<JsonObject> requeueDeadLettersTargeted(JsonObject body) {
    return context.backendClient.socialRuntime
        .requeueDeadLetterSharedChannelSyncTargeted(body);
  }

  Future<JsonObject> takeoverPendingTargeted(JsonObject body) {
    return context.backendClient.socialRuntime
        .takeoverPendingSharedChannelSyncTargeted(body);
  }
}
