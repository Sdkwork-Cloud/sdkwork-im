package com.sdkwork.craw.chat.backend.model;


public class ConversationInboxEntry {
    private String tenantId;
    private String principalId;
    private String memberId;
    private String conversationId;
    private String conversationType;
    private Integer messageCount;
    private String lastMessageId;
    private Integer lastMessageSeq;
    private String lastSenderId;
    private String lastSenderKind;
    private String lastSummary;
    private Integer unreadCount;
    private String lastActivityAt;
    private ConversationAgentHandoffView agentHandoff;

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

    public String getMemberId() {
        return this.memberId;
    }
    
    public void setMemberId(String memberId) {
        this.memberId = memberId;
    }

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getConversationType() {
        return this.conversationType;
    }
    
    public void setConversationType(String conversationType) {
        this.conversationType = conversationType;
    }

    public Integer getMessageCount() {
        return this.messageCount;
    }
    
    public void setMessageCount(Integer messageCount) {
        this.messageCount = messageCount;
    }

    public String getLastMessageId() {
        return this.lastMessageId;
    }
    
    public void setLastMessageId(String lastMessageId) {
        this.lastMessageId = lastMessageId;
    }

    public Integer getLastMessageSeq() {
        return this.lastMessageSeq;
    }
    
    public void setLastMessageSeq(Integer lastMessageSeq) {
        this.lastMessageSeq = lastMessageSeq;
    }

    public String getLastSenderId() {
        return this.lastSenderId;
    }
    
    public void setLastSenderId(String lastSenderId) {
        this.lastSenderId = lastSenderId;
    }

    public String getLastSenderKind() {
        return this.lastSenderKind;
    }
    
    public void setLastSenderKind(String lastSenderKind) {
        this.lastSenderKind = lastSenderKind;
    }

    public String getLastSummary() {
        return this.lastSummary;
    }
    
    public void setLastSummary(String lastSummary) {
        this.lastSummary = lastSummary;
    }

    public Integer getUnreadCount() {
        return this.unreadCount;
    }
    
    public void setUnreadCount(Integer unreadCount) {
        this.unreadCount = unreadCount;
    }

    public String getLastActivityAt() {
        return this.lastActivityAt;
    }
    
    public void setLastActivityAt(String lastActivityAt) {
        this.lastActivityAt = lastActivityAt;
    }

    public ConversationAgentHandoffView getAgentHandoff() {
        return this.agentHandoff;
    }
    
    public void setAgentHandoff(ConversationAgentHandoffView agentHandoff) {
        this.agentHandoff = agentHandoff;
    }
}
