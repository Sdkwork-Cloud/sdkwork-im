package com.sdkwork.craw.chat.backend.model;


public class RemoveConversationMemberRequest {
    private String memberId;

    public String getMemberId() {
        return this.memberId;
    }
    
    public void setMemberId(String memberId) {
        this.memberId = memberId;
    }
}
