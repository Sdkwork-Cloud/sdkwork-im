package com.sdkwork.craw.chat.backend.model;


public class ConversationReadCursorView {
    private String tenantId;
    private String conversationId;
    private String memberId;
    private String principalId;
    private Integer readSeq;
    private String lastReadMessageId;
    private String updatedAt;
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

    public String getMemberId() {
        return this.memberId;
    }
    
    public void setMemberId(String memberId) {
        this.memberId = memberId;
    }

    public String getPrincipalId() {
        return this.principalId;
    }
    
    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
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

    public String getUpdatedAt() {
        return this.updatedAt;
    }
    
    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }

    public Integer getUnreadCount() {
        return this.unreadCount;
    }
    
    public void setUnreadCount(Integer unreadCount) {
        this.unreadCount = unreadCount;
    }
}
