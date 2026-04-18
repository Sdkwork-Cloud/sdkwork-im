package com.sdkwork.craw.chat.backend.model;


public class AgentHandoffStateView {
    private String tenantId;
    private String conversationId;
    private String status;
    private ChangeAgentHandoffStatusView source;
    private ChangeAgentHandoffStatusView target;
    private String handoffSessionId;
    private String handoffReason;
    private String acceptedAt;
    private ChangeAgentHandoffStatusView acceptedBy;
    private String resolvedAt;
    private ChangeAgentHandoffStatusView resolvedBy;
    private String closedAt;
    private ChangeAgentHandoffStatusView closedBy;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getStatus() {
        return this.status;
    }
    
    public void setStatus(String status) {
        this.status = status;
    }

    public ChangeAgentHandoffStatusView getSource() {
        return this.source;
    }
    
    public void setSource(ChangeAgentHandoffStatusView source) {
        this.source = source;
    }

    public ChangeAgentHandoffStatusView getTarget() {
        return this.target;
    }
    
    public void setTarget(ChangeAgentHandoffStatusView target) {
        this.target = target;
    }

    public String getHandoffSessionId() {
        return this.handoffSessionId;
    }
    
    public void setHandoffSessionId(String handoffSessionId) {
        this.handoffSessionId = handoffSessionId;
    }

    public String getHandoffReason() {
        return this.handoffReason;
    }
    
    public void setHandoffReason(String handoffReason) {
        this.handoffReason = handoffReason;
    }

    public String getAcceptedAt() {
        return this.acceptedAt;
    }
    
    public void setAcceptedAt(String acceptedAt) {
        this.acceptedAt = acceptedAt;
    }

    public ChangeAgentHandoffStatusView getAcceptedBy() {
        return this.acceptedBy;
    }
    
    public void setAcceptedBy(ChangeAgentHandoffStatusView acceptedBy) {
        this.acceptedBy = acceptedBy;
    }

    public String getResolvedAt() {
        return this.resolvedAt;
    }
    
    public void setResolvedAt(String resolvedAt) {
        this.resolvedAt = resolvedAt;
    }

    public ChangeAgentHandoffStatusView getResolvedBy() {
        return this.resolvedBy;
    }
    
    public void setResolvedBy(ChangeAgentHandoffStatusView resolvedBy) {
        this.resolvedBy = resolvedBy;
    }

    public String getClosedAt() {
        return this.closedAt;
    }
    
    public void setClosedAt(String closedAt) {
        this.closedAt = closedAt;
    }

    public ChangeAgentHandoffStatusView getClosedBy() {
        return this.closedBy;
    }
    
    public void setClosedBy(ChangeAgentHandoffStatusView closedBy) {
        this.closedBy = closedBy;
    }
}
