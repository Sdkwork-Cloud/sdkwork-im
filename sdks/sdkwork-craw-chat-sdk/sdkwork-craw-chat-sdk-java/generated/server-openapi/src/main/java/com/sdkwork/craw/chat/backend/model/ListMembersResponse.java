package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class ListMembersResponse {
    private List<ConversationMember> items;

    public List<ConversationMember> getItems() {
        return this.items;
    }
    
    public void setItems(List<ConversationMember> items) {
        this.items = items;
    }
}
