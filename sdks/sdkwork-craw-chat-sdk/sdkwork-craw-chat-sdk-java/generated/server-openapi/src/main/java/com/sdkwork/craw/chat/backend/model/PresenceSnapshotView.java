package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class PresenceSnapshotView {
    private String tenantId;
    private String principalId;
    private String currentDeviceId;
    private List<DevicePresenceView> devices;

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

    public String getCurrentDeviceId() {
        return this.currentDeviceId;
    }
    
    public void setCurrentDeviceId(String currentDeviceId) {
        this.currentDeviceId = currentDeviceId;
    }

    public List<DevicePresenceView> getDevices() {
        return this.devices;
    }
    
    public void setDevices(List<DevicePresenceView> devices) {
        this.devices = devices;
    }
}
