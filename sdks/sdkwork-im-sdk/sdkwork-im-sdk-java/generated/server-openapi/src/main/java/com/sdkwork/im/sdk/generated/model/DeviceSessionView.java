package com.sdkwork.im.sdk.generated.model;


public class DeviceSessionView {
    private String tenantId;
    private String principalId;
    private String principalKind;
    private String deviceId;
    private String resumedAt;

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

    public String getPrincipalKind() {
        return this.principalKind;
    }
    
    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public String getResumedAt() {
        return this.resumedAt;
    }
    
    public void setResumedAt(String resumedAt) {
        this.resumedAt = resumedAt;
    }
}
