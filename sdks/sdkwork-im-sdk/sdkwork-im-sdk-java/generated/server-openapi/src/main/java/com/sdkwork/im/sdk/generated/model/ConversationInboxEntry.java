package com.sdkwork.im.sdk.generated.model;


public class ConversationInboxEntry {
    private String tenantId;
    private String conversationId;
    private Boolean agentHandoff;
    private String conversationType;
    private String lastActivityAt;
    private String lastMessageId;
    private String lastSenderId;
    private Integer messageCount;
    private Integer lastMessageSeq;
    private String lastSummary;
    private String lastMessageAt;
    private Integer unreadCount;

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

    public Boolean getAgentHandoff() {
        return this.agentHandoff;
    }
    
    public void setAgentHandoff(Boolean agentHandoff) {
        this.agentHandoff = agentHandoff;
    }

    public String getConversationType() {
        return this.conversationType;
    }
    
    public void setConversationType(String conversationType) {
        this.conversationType = conversationType;
    }

    public String getLastActivityAt() {
        return this.lastActivityAt;
    }
    
    public void setLastActivityAt(String lastActivityAt) {
        this.lastActivityAt = lastActivityAt;
    }

    public String getLastMessageId() {
        return this.lastMessageId;
    }
    
    public void setLastMessageId(String lastMessageId) {
        this.lastMessageId = lastMessageId;
    }

    public String getLastSenderId() {
        return this.lastSenderId;
    }
    
    public void setLastSenderId(String lastSenderId) {
        this.lastSenderId = lastSenderId;
    }

    public Integer getMessageCount() {
        return this.messageCount;
    }
    
    public void setMessageCount(Integer messageCount) {
        this.messageCount = messageCount;
    }

    public Integer getLastMessageSeq() {
        return this.lastMessageSeq;
    }
    
    public void setLastMessageSeq(Integer lastMessageSeq) {
        this.lastMessageSeq = lastMessageSeq;
    }

    public String getLastSummary() {
        return this.lastSummary;
    }
    
    public void setLastSummary(String lastSummary) {
        this.lastSummary = lastSummary;
    }

    public String getLastMessageAt() {
        return this.lastMessageAt;
    }
    
    public void setLastMessageAt(String lastMessageAt) {
        this.lastMessageAt = lastMessageAt;
    }

    public Integer getUnreadCount() {
        return this.unreadCount;
    }
    
    public void setUnreadCount(Integer unreadCount) {
        this.unreadCount = unreadCount;
    }
}
