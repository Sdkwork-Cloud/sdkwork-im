package com.sdkwork.craw.chat.backend.model;


public class RealtimeEvent {
    private String tenantId;
    private String principalId;
    private String deviceId;
    private Integer realtimeSeq;
    private String scopeType;
    private String scopeId;
    private String eventType;
    private String deliveryClass;
    private String payload;
    private String occurredAt;

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

    public Integer getRealtimeSeq() {
        return this.realtimeSeq;
    }
    
    public void setRealtimeSeq(Integer realtimeSeq) {
        this.realtimeSeq = realtimeSeq;
    }

    public String getScopeType() {
        return this.scopeType;
    }
    
    public void setScopeType(String scopeType) {
        this.scopeType = scopeType;
    }

    public String getScopeId() {
        return this.scopeId;
    }
    
    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public String getEventType() {
        return this.eventType;
    }
    
    public void setEventType(String eventType) {
        this.eventType = eventType;
    }

    public String getDeliveryClass() {
        return this.deliveryClass;
    }
    
    public void setDeliveryClass(String deliveryClass) {
        this.deliveryClass = deliveryClass;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }
    
    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }
}
