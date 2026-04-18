package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class DeviceSyncFeedResponse {
    private List<DeviceSyncFeedEntry> items;

    public List<DeviceSyncFeedEntry> getItems() {
        return this.items;
    }
    
    public void setItems(List<DeviceSyncFeedEntry> items) {
        this.items = items;
    }
}
