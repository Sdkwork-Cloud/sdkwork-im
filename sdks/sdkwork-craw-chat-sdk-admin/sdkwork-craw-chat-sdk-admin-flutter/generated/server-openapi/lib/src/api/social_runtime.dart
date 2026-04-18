import '../http/client.dart';
import '../models.dart';
import 'paths.dart';
import 'response_helpers.dart';

class SocialRuntimeApi {
  final AdminHttpClient _httpClient;

  SocialRuntimeApi(this._httpClient);

  Future<JsonObject> claimPendingSharedChannelSyncTargeted(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/claim-pending-shared-channel-sync-targeted'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.claimPendingSharedChannelSyncTargeted',
    );
  }

  Future<JsonObject> getDeadLetterSharedChannelSyncInventory() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/runtime/dead-letter-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.getDeadLetterSharedChannelSyncInventory',
    );
  }

  Future<JsonObject> getDeliveredSharedChannelSyncInventory() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/runtime/delivered-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.getDeliveredSharedChannelSyncInventory',
    );
  }

  Future<JsonObject> getSharedChannelSyncDeliveryStateInventory() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/runtime/delivery-state-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.getSharedChannelSyncDeliveryStateInventory',
    );
  }

  Future<JsonObject> getPendingSharedChannelSyncInventory() async {
    final response = await _httpClient.get(
      AdminApiPaths.control('/social/runtime/pending-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.getPendingSharedChannelSyncInventory',
    );
  }

  Future<JsonObject> reclaimStalePendingSharedChannelSync() async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/reclaim-stale-pending-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.reclaimStalePendingSharedChannelSync',
    );
  }

  Future<JsonObject> releasePendingSharedChannelSyncTargeted(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/release-pending-shared-channel-sync-targeted'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.releasePendingSharedChannelSyncTargeted',
    );
  }

  Future<JsonObject> repairSocialRuntimeSnapshot() async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/repair-derived-snapshot'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.repairSocialRuntimeSnapshot',
    );
  }

  Future<JsonObject> repairSharedChannelSync() async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/repair-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.repairSharedChannelSync',
    );
  }

  Future<JsonObject> republishPendingSharedChannelSyncTargeted(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/republish-pending-shared-channel-sync-targeted'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.republishPendingSharedChannelSyncTargeted',
    );
  }

  Future<JsonObject> requeueDeadLetterSharedChannelSync() async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/requeue-dead-letter-shared-channel-sync'),
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.requeueDeadLetterSharedChannelSync',
    );
  }

  Future<JsonObject> requeueDeadLetterSharedChannelSyncTargeted(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/requeue-dead-letter-shared-channel-sync-targeted'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.requeueDeadLetterSharedChannelSyncTargeted',
    );
  }

  Future<JsonObject> takeoverPendingSharedChannelSyncTargeted(JsonObject body) async {
    final response = await _httpClient.post(
      AdminApiPaths.control('/social/runtime/takeover-pending-shared-channel-sync-targeted'),
      body: body,
    );
    return sdkworkRequireJsonObject(
      response,
      operationName: 'socialRuntime.takeoverPendingSharedChannelSyncTargeted',
    );
  }
}
