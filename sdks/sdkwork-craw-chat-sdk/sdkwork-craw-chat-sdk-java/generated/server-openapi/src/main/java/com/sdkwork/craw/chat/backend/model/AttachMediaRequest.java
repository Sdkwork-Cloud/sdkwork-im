package com.sdkwork.craw.chat.backend.model;

import java.util.Map;

public class AttachMediaRequest {
    private String conversationId;
    private String clientMsgId;
    private String summary;
    private String text;
    private Map<String, String> renderHints;

    public String getConversationId() {
        return this.conversationId;
    }
    
    public void setConversationId(String conversationId) {
        this.conversationId = conversationId;
    }

    public String getClientMsgId() {
        return this.clientMsgId;
    }
    
    public void setClientMsgId(String clientMsgId) {
        this.clientMsgId = clientMsgId;
    }

    public String getSummary() {
        return this.summary;
    }
    
    public void setSummary(String summary) {
        this.summary = summary;
    }

    public String getText() {
        return this.text;
    }
    
    public void setText(String text) {
        this.text = text;
    }

    public Map<String, String> getRenderHints() {
        return this.renderHints;
    }
    
    public void setRenderHints(Map<String, String> renderHints) {
        this.renderHints = renderHints;
    }
}
