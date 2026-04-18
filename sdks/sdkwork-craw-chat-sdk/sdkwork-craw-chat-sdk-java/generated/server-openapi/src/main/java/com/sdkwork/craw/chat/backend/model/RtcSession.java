package com.sdkwork.craw.chat.backend.model;


public class RtcSession {
    private String tenantId;
    private String rtcSessionId;
    private String conversationId;
    private String rtcMode;
    private String initiatorId;
    private String providerPluginId;
    private String providerSessionId;
    private String accessEndpoint;
    private String providerRegion;
    private String state;
    private String signalingStreamId;
    private String artifactMessageId;
    private String startedAt;
    private String endedAt;

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

    public String getInitiatorId() {
        return this.initiatorId;
    }
    
    public void setInitiatorId(String initiatorId) {
        this.initiatorId = initiatorId;
    }

    public String getProviderPluginId() {
        return this.providerPluginId;
    }
    
    public void setProviderPluginId(String providerPluginId) {
        this.providerPluginId = providerPluginId;
    }

    public String getProviderSessionId() {
        return this.providerSessionId;
    }
    
    public void setProviderSessionId(String providerSessionId) {
        this.providerSessionId = providerSessionId;
    }

    public String getAccessEndpoint() {
        return this.accessEndpoint;
    }
    
    public void setAccessEndpoint(String accessEndpoint) {
        this.accessEndpoint = accessEndpoint;
    }

    public String getProviderRegion() {
        return this.providerRegion;
    }
    
    public void setProviderRegion(String providerRegion) {
        this.providerRegion = providerRegion;
    }

    public String getState() {
        return this.state;
    }
    
    public void setState(String state) {
        this.state = state;
    }

    public String getSignalingStreamId() {
        return this.signalingStreamId;
    }
    
    public void setSignalingStreamId(String signalingStreamId) {
        this.signalingStreamId = signalingStreamId;
    }

    public String getArtifactMessageId() {
        return this.artifactMessageId;
    }
    
    public void setArtifactMessageId(String artifactMessageId) {
        this.artifactMessageId = artifactMessageId;
    }

    public String getStartedAt() {
        return this.startedAt;
    }
    
    public void setStartedAt(String startedAt) {
        this.startedAt = startedAt;
    }

    public String getEndedAt() {
        return this.endedAt;
    }
    
    public void setEndedAt(String endedAt) {
        this.endedAt = endedAt;
    }
}
