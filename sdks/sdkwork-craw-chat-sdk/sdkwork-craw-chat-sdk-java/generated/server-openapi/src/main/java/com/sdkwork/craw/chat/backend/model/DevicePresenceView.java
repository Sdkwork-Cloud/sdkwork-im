package com.sdkwork.craw.chat.backend.model;


public class DevicePresenceView {
    private String tenantId;
    private String principalId;
    private String deviceId;
    private String platform;
    private String sessionId;
    private String status;
    private Integer lastSyncSeq;
    private String lastResumeAt;
    private String lastSeenAt;

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

    public String getPlatform() {
        return this.platform;
    }
    
    public void setPlatform(String platform) {
        this.platform = platform;
    }

    public String getSessionId() {
        return this.sessionId;
    }
    
    public void setSessionId(String sessionId) {
        this.sessionId = sessionId;
    }

    public String getStatus() {
        return this.status;
    }
    
    public void setStatus(String status) {
        this.status = status;
    }

    public Integer getLastSyncSeq() {
        return this.lastSyncSeq;
    }
    
    public void setLastSyncSeq(Integer lastSyncSeq) {
        this.lastSyncSeq = lastSyncSeq;
    }

    public String getLastResumeAt() {
        return this.lastResumeAt;
    }
    
    public void setLastResumeAt(String lastResumeAt) {
        this.lastResumeAt = lastResumeAt;
    }

    public String getLastSeenAt() {
        return this.lastSeenAt;
    }
    
    public void setLastSeenAt(String lastSeenAt) {
        this.lastSeenAt = lastSeenAt;
    }
}
