package com.sdkwork.craw.chat.backend.model;


public class ConversationSummaryView {
    private String tenantId;
    private String conversationId;
    private Integer messageCount;
    private String lastMessageId;
    private Integer lastMessageSeq;
    private String lastSenderId;
    private String lastSenderKind;
    private SummarySenderView lastSender;
    private String lastSummary;
    private String lastMessageAt;
    private ConversationAgentHandoffView agentHandoff;

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

    public SummarySenderView getLastSender() {
        return this.lastSender;
    }
    
    public void setLastSender(SummarySenderView lastSender) {
        this.lastSender = lastSender;
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

    public ConversationAgentHandoffView getAgentHandoff() {
        return this.agentHandoff;
    }
    
    public void setAgentHandoff(ConversationAgentHandoffView agentHandoff) {
        this.agentHandoff = agentHandoff;
    }
}
