package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class SyncRealtimeSubscriptionsRequest {
    private String deviceId;
    private List<RealtimeSubscriptionItemInput> items;

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public List<RealtimeSubscriptionItemInput> getItems() {
        return this.items;
    }
    
    public void setItems(List<RealtimeSubscriptionItemInput> items) {
        this.items = items;
    }
}
