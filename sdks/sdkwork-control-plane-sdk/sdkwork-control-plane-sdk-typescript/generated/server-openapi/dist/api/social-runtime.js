export function createSocialRuntimeApi(httpClient) {
    return {
        claimPendingSharedChannelSyncTargeted(body) {
            return httpClient.post('/api/v1/control/social/runtime/claim-pending-shared-channel-sync-targeted', body);
        },
        getDeadLetterSharedChannelSyncInventory() {
            return httpClient.get('/api/v1/control/social/runtime/dead-letter-shared-channel-sync');
        },
        getDeliveredSharedChannelSyncInventory() {
            return httpClient.get('/api/v1/control/social/runtime/delivered-shared-channel-sync');
        },
        getSharedChannelSyncDeliveryStateInventory() {
            return httpClient.get('/api/v1/control/social/runtime/delivery-state-shared-channel-sync');
        },
        getPendingSharedChannelSyncInventory() {
            return httpClient.get('/api/v1/control/social/runtime/pending-shared-channel-sync');
        },
        reclaimStalePendingSharedChannelSync() {
            return httpClient.post('/api/v1/control/social/runtime/reclaim-stale-pending-shared-channel-sync');
        },
        releasePendingSharedChannelSyncTargeted(body) {
            return httpClient.post('/api/v1/control/social/runtime/release-pending-shared-channel-sync-targeted', body);
        },
        repairSocialRuntimeSnapshot() {
            return httpClient.post('/api/v1/control/social/runtime/repair-derived-snapshot');
        },
        repairSharedChannelSync() {
            return httpClient.post('/api/v1/control/social/runtime/repair-shared-channel-sync');
        },
        republishPendingSharedChannelSyncTargeted(body) {
            return httpClient.post('/api/v1/control/social/runtime/republish-pending-shared-channel-sync-targeted', body);
        },
        requeueDeadLetterSharedChannelSync() {
            return httpClient.post('/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync');
        },
        requeueDeadLetterSharedChannelSyncTargeted(body) {
            return httpClient.post('/api/v1/control/social/runtime/requeue-dead-letter-shared-channel-sync-targeted', body);
        },
        takeoverPendingSharedChannelSyncTargeted(body) {
            return httpClient.post('/api/v1/control/social/runtime/takeover-pending-shared-channel-sync-targeted', body);
        },
    };
}
