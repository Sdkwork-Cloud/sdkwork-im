package com.sdkwork.craw.chat.backend.model;


public class CreateAgentDialogRequest {
    private String conversationId;
    private String agentId;

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getAgentId() {
        return this.agentId;
    }
    
    public void setAgentId(String agentId) {
        this.agentId = agentId;
    }
}
