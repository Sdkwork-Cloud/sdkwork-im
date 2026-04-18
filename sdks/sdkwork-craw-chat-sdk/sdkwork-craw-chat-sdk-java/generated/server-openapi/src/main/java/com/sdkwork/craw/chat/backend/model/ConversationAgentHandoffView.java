package com.sdkwork.craw.chat.backend.model;


public class ConversationAgentHandoffView {
    private String status;
    private ConversationActorView source;
    private ConversationActorView target;
    private String handoffSessionId;
    private String handoffReason;
    private String acceptedAt;
    private ConversationActorView acceptedBy;
    private String resolvedAt;
    private ConversationActorView resolvedBy;
    private String closedAt;
    private ConversationActorView closedBy;

    public String getStatus() {
        return this.status;
    }
    
    public void setStatus(String status) {
        this.status = status;
    }

    public ConversationActorView getSource() {
        return this.source;
    }
    
    public void setSource(ConversationActorView source) {
        this.source = source;
    }

    public ConversationActorView getTarget() {
        return this.target;
    }
    
    public void setTarget(ConversationActorView target) {
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

    public ConversationActorView getAcceptedBy() {
        return this.acceptedBy;
    }
    
    public void setAcceptedBy(ConversationActorView acceptedBy) {
        this.acceptedBy = acceptedBy;
    }

    public String getResolvedAt() {
        return this.resolvedAt;
    }
    
    public void setResolvedAt(String resolvedAt) {
        this.resolvedAt = resolvedAt;
    }

    public ConversationActorView getResolvedBy() {
        return this.resolvedBy;
    }
    
    public void setResolvedBy(ConversationActorView resolvedBy) {
        this.resolvedBy = resolvedBy;
    }

    public String getClosedAt() {
        return this.closedAt;
    }
    
    public void setClosedAt(String closedAt) {
        this.closedAt = closedAt;
    }

    public ConversationActorView getClosedBy() {
        return this.closedBy;
    }
    
    public void setClosedBy(ConversationActorView closedBy) {
        this.closedBy = closedBy;
    }
}
