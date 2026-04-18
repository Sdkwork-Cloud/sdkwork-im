package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class InboxResponse {
    private List<ConversationInboxEntry> items;

    public List<ConversationInboxEntry> getItems() {
        return this.items;
    }
    
    public void setItems(List<ConversationInboxEntry> items) {
        this.items = items;
    }
}
