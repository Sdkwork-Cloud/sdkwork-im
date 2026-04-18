package com.sdkwork.craw.chat.backend.model;


public class DeviceSyncFeedEntry {
    private String tenantId;
    private String principalId;
    private String deviceId;
    private Integer syncSeq;
    private String originEventId;
    private String originEventType;
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private String memberId;
    private Integer readSeq;
    private String lastReadMessageId;
    private String actorId;
    private String actorKind;
    private String actorDeviceId;
    private String summary;
    private String payloadSchema;
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

    public Integer getSyncSeq() {
        return this.syncSeq;
    }
    
    public void setSyncSeq(Integer syncSeq) {
        this.syncSeq = syncSeq;
    }

    public String getOriginEventId() {
        return this.originEventId;
    }
    
    public void setOriginEventId(String originEventId) {
        this.originEventId = originEventId;
    }

    public String getOriginEventType() {
        return this.originEventType;
    }
    
    public void setOriginEventType(String originEventType) {
        this.originEventType = originEventType;
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

    public String getMemberId() {
        return this.memberId;
    }
    
    public void setMemberId(String memberId) {
        this.memberId = memberId;
    }

    public Integer getReadSeq() {
        return this.readSeq;
    }
    
    public void setReadSeq(Integer readSeq) {
        this.readSeq = readSeq;
    }

    public String getLastReadMessageId() {
        return this.lastReadMessageId;
    }
    
    public void setLastReadMessageId(String lastReadMessageId) {
        this.lastReadMessageId = lastReadMessageId;
    }

    public String getActorId() {
        return this.actorId;
    }
    
    public void setActorId(String actorId) {
        this.actorId = actorId;
    }

    public String getActorKind() {
        return this.actorKind;
    }
    
    public void setActorKind(String actorKind) {
        this.actorKind = actorKind;
    }

    public String getActorDeviceId() {
        return this.actorDeviceId;
    }
    
    public void setActorDeviceId(String actorDeviceId) {
        this.actorDeviceId = actorDeviceId;
    }

    public String getSummary() {
        return this.summary;
    }
    
    public void setSummary(String summary) {
        this.summary = summary;
    }

    public String getPayloadSchema() {
        return this.payloadSchema;
    }
    
    public void setPayloadSchema(String payloadSchema) {
        this.payloadSchema = payloadSchema;
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
