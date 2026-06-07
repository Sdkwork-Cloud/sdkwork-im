package com.sdkwork.im.sdk.generated.model;


public class RegisteredDeviceView {
    private String tenantId;
    private String principalId;
    private String principalKind;
    private String deviceId;
    private String registeredAt;

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

    public String getRegisteredAt() {
        return this.registeredAt;
    }
    
    public void setRegisteredAt(String registeredAt) {
        this.registeredAt = registeredAt;
    }
}
