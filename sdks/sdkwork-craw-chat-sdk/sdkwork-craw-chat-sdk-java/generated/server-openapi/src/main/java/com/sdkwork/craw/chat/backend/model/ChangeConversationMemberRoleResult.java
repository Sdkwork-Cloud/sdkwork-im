package com.sdkwork.craw.chat.backend.model;


public class ChangeConversationMemberRoleResult {
    private String eventId;
    private String changedAt;
    private ConversationMember previousMember;
    private ConversationMember updatedMember;

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }

    public String getChangedAt() {
        return this.changedAt;
    }
    
    public void setChangedAt(String changedAt) {
        this.changedAt = changedAt;
    }

    public ConversationMember getPreviousMember() {
        return this.previousMember;
    }
    
    public void setPreviousMember(ConversationMember previousMember) {
        this.previousMember = previousMember;
    }

    public ConversationMember getUpdatedMember() {
        return this.updatedMember;
    }
    
    public void setUpdatedMember(ConversationMember updatedMember) {
        this.updatedMember = updatedMember;
    }
}
