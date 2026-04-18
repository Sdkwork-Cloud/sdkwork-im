package com.sdkwork.craw.chat.backend.model;


public class TimelineViewEntry {
    private String tenantId;
    private String conversationId;
    private String messageId;
    private Integer messageSeq;
    private String summary;

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

    public String getSummary() {
        return this.summary;
    }
    
    public void setSummary(String summary) {
        this.summary = summary;
    }
}
