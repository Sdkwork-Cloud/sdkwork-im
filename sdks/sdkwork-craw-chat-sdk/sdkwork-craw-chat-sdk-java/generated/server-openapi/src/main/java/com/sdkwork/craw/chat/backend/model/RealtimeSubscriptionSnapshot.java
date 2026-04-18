package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class RealtimeSubscriptionSnapshot {
    private String tenantId;
    private String principalId;
    private String deviceId;
    private List<RealtimeSubscription> items;
    private String syncedAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getPrincipalId() {
        return this.principalId;
    }
    
    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public List<RealtimeSubscription> getItems() {
        return this.items;
    }
    
    public void setItems(List<RealtimeSubscription> items) {
        this.items = items;
    }

    public String getSyncedAt() {
        return this.syncedAt;
    }
    
    public void setSyncedAt(String syncedAt) {
        this.syncedAt = syncedAt;
    }
}
