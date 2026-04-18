import type { JsonObject } from './types.js';
import type { CrawChatAdminSdkContext } from './sdk-context.js';

export class CrawChatAdminSocialRuntimeModule {
  constructor(private readonly context: CrawChatAdminSdkContext) {}

  claimPendingTargeted(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.claimPendingSharedChannelSyncTargeted(body);
  }

  getDeadLetterInventory(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.getDeadLetterSharedChannelSyncInventory();
  }

  getDeliveredInventory(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.getDeliveredSharedChannelSyncInventory();
  }

  getDeliveryStateInventory(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.getSharedChannelSyncDeliveryStateInventory();
  }

  getPendingInventory(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.getPendingSharedChannelSyncInventory();
  }

  reclaimStalePending(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.reclaimStalePendingSharedChannelSync();
  }

  releasePendingTargeted(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.releasePendingSharedChannelSyncTargeted(body);
  }

  repairSnapshot(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.repairSocialRuntimeSnapshot();
  }

  repairSharedChannelSync(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.repairSharedChannelSync();
  }

  republishPendingTargeted(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.republishPendingSharedChannelSyncTargeted(body);
  }

  requeueDeadLetters(): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.requeueDeadLetterSharedChannelSync();
  }

  requeueDeadLettersTargeted(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.requeueDeadLetterSharedChannelSyncTargeted(body);
  }

  takeoverPendingTargeted(body: JsonObject): Promise<JsonObject> {
    return this.context.backendClient.socialRuntime.takeoverPendingSharedChannelSyncTargeted(body);
  }
}
