package com.sdkwork.craw.chat.backend.model;


public class AddConversationMemberRequest {
    private String principalId;
    private String principalKind;
    private String role;

    public String getPrincipalId() {
        return this.principalId;
    }
    
    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public String getPrincipalKind() {
        return this.principalKind;
    }
    
    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

    public String getRole() {
        return this.role;
    }
    
    public void setRole(String role) {
        this.role = role;
    }
}
