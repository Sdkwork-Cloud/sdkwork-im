package com.sdkwork.craw.chat.backend.model;


public class TransferConversationOwnerResult {
    private String eventId;
    private String transferredAt;
    private ConversationMember previousOwner;
    private ConversationMember newOwner;

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getTransferredAt() {
        return this.transferredAt;
    }
    
    public void setTransferredAt(String transferredAt) {
        this.transferredAt = transferredAt;
    }

    public ConversationMember getPreviousOwner() {
        return this.previousOwner;
    }
    
    public void setPreviousOwner(ConversationMember previousOwner) {
        this.previousOwner = previousOwner;
    }

    public ConversationMember getNewOwner() {
        return this.newOwner;
    }
    
    public void setNewOwner(ConversationMember newOwner) {
        this.newOwner = newOwner;
    }
}
