package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class TimelineResponse {
    private List<TimelineViewEntry> items;

    public List<TimelineViewEntry> getItems() {
        return this.items;
    }
    
    public void setItems(List<TimelineViewEntry> items) {
        this.items = items;
    }
}
