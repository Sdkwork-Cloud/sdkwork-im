package com.sdkwork.craw.chat.backend.model;


public class RtcSignalEvent {
    private String tenantId;
    private String rtcSessionId;
    private String conversationId;
    private String rtcMode;
    private String signalType;
    private String schemaRef;
    private String payload;
    private Sender sender;
    private String signalingStreamId;
    private String occurredAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getRtcSessionId() {
        return this.rtcSessionId;
    }
    
    public void setRtcSessionId(String rtcSessionId) {
        this.rtcSessionId = rtcSessionId;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getRtcMode() {
        return this.rtcMode;
    }
    
    public void setRtcMode(String rtcMode) {
        this.rtcMode = rtcMode;
    }

    public String getSignalType() {
        return this.signalType;
    }
    
    public void setSignalType(String signalType) {
        this.signalType = signalType;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }
    
    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }

    public Sender getSender() {
        return this.sender;
    }
    
    public void setSender(Sender sender) {
        this.sender = sender;
    }

    public String getSignalingStreamId() {
        return this.signalingStreamId;
    }
    
    public void setSignalingStreamId(String signalingStreamId) {
        this.signalingStreamId = signalingStreamId;
    }

    public String getOccurredAt() {
        return this.occurredAt;
    }
    
    public void setOccurredAt(String occurredAt) {
        this.occurredAt = occurredAt;
    }
}
