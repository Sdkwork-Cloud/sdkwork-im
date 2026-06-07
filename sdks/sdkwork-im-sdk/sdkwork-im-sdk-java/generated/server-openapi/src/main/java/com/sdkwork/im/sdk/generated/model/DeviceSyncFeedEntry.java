package com.sdkwork.im.sdk.generated.model;


public class DeviceSyncFeedEntry {
    private String tenantId;
    private String principalId;
    private String principalKind;
    private String deviceId;
    private Integer syncSeq;
    private String eventId;
    private String originEventType;
    private String actorId;
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private String payload;
    private Integer readSeq;
    private String summary;
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

    public Integer getSyncSeq() {
        return this.syncSeq;
    }
    
    public void setSyncSeq(Integer syncSeq) {
        this.syncSeq = syncSeq;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getOriginEventType() {
        return this.originEventType;
    }
    
    public void setOriginEventType(String originEventType) {
        this.originEventType = originEventType;
    }

    public String getActorId() {
        return this.actorId;
    }
    
    public void setActorId(String actorId) {
        this.actorId = actorId;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getMessageId() {
        return this.messageId;
    }
    
    public void setMessageId(String messageId) {
        this.messageId = messageId;
    }

    public Integer getMessageSeq() {
        return this.messageSeq;
    }
    
    public void setMessageSeq(Integer messageSeq) {
        this.messageSeq = messageSeq;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }

    public Integer getReadSeq() {
        return this.readSeq;
    }
    
    public void setReadSeq(Integer readSeq) {
        this.readSeq = readSeq;
    }

    public String getSummary() {
        return this.summary;
    }
    
    public void setSummary(String summary) {
        this.summary = summary;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }
    
    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }
}
