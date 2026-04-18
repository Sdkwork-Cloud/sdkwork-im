package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class RealtimeSubscription {
    private String scopeType;
    private String scopeId;
    private List<String> eventTypes;
    private String subscribedAt;

    public String getScopeType() {
        return this.scopeType;
    }
    
    public void setScopeType(String scopeType) {
        this.scopeType = scopeType;
    }

    public String getScopeId() {
        return this.scopeId;
    }
    
    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public List<String> getEventTypes() {
        return this.eventTypes;
    }
    
    public void setEventTypes(List<String> eventTypes) {
        this.eventTypes = eventTypes;
    }

    public String getSubscribedAt() {
        return this.subscribedAt;
    }
    
    public void setSubscribedAt(String subscribedAt) {
        this.subscribedAt = subscribedAt;
    }
}
