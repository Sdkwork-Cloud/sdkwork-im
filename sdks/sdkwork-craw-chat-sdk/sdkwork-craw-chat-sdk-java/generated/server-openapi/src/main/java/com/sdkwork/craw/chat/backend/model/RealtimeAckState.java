package com.sdkwork.craw.chat.backend.model;


public class RealtimeAckState {
    private String tenantId;
    private String principalId;
    private String deviceId;
    private Integer ackedThroughSeq;
    private Integer trimmedThroughSeq;
    private Integer retainedEventCount;
    private String ackedAt;

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

    public Integer getAckedThroughSeq() {
        return this.ackedThroughSeq;
    }
    
    public void setAckedThroughSeq(Integer ackedThroughSeq) {
        this.ackedThroughSeq = ackedThroughSeq;
    }

    public Integer getTrimmedThroughSeq() {
        return this.trimmedThroughSeq;
    }
    
    public void setTrimmedThroughSeq(Integer trimmedThroughSeq) {
        this.trimmedThroughSeq = trimmedThroughSeq;
    }

    public Integer getRetainedEventCount() {
        return this.retainedEventCount;
    }
    
    public void setRetainedEventCount(Integer retainedEventCount) {
        this.retainedEventCount = retainedEventCount;
    }

    public String getAckedAt() {
        return this.ackedAt;
    }
    
    public void setAckedAt(String ackedAt) {
        this.ackedAt = ackedAt;
    }
}
