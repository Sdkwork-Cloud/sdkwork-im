export class ControlPlaneSocialRuntimeModule {
    context;
    constructor(context) {
        this.context = context;
    }
    claimPendingTargeted(body) {
        return this.context.backendClient.socialRuntime.claimPendingSharedChannelSyncTargeted(body);
    }
    getDeadLetterInventory() {
        return this.context.backendClient.socialRuntime.getDeadLetterSharedChannelSyncInventory();
    }
    getDeliveredInventory() {
        return this.context.backendClient.socialRuntime.getDeliveredSharedChannelSyncInventory();
    }
    getDeliveryStateInventory() {
        return this.context.backendClient.socialRuntime.getSharedChannelSyncDeliveryStateInventory();
    }
    getPendingInventory() {
        return this.context.backendClient.socialRuntime.getPendingSharedChannelSyncInventory();
    }
    reclaimStalePending() {
        return this.context.backendClient.socialRuntime.reclaimStalePendingSharedChannelSync();
    }
    releasePendingTargeted(body) {
        return this.context.backendClient.socialRuntime.releasePendingSharedChannelSyncTargeted(body);
    }
    repairSnapshot() {
        return this.context.backendClient.socialRuntime.repairSocialRuntimeSnapshot();
    }
    repairSharedChannelSync() {
        return this.context.backendClient.socialRuntime.repairSharedChannelSync();
    }
    republishPendingTargeted(body) {
        return this.context.backendClient.socialRuntime.republishPendingSharedChannelSyncTargeted(body);
    }
    requeueDeadLetters() {
        return this.context.backendClient.socialRuntime.requeueDeadLetterSharedChannelSync();
    }
    requeueDeadLettersTargeted(body) {
        return this.context.backendClient.socialRuntime.requeueDeadLetterSharedChannelSyncTargeted(body);
    }
    takeoverPendingTargeted(body) {
        return this.context.backendClient.socialRuntime.takeoverPendingSharedChannelSyncTargeted(body);
    }
}
