package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class StreamFrameWindow {
    private List<StreamFrame> items;
    private Integer nextAfterFrameSeq;
    private Boolean hasMore;

    public List<StreamFrame> getItems() {
        return this.items;
    }
    
    public void setItems(List<StreamFrame> items) {
        this.items = items;
    }

    public Integer getNextAfterFrameSeq() {
        return this.nextAfterFrameSeq;
    }
    
    public void setNextAfterFrameSeq(Integer nextAfterFrameSeq) {
        this.nextAfterFrameSeq = nextAfterFrameSeq;
    }

    public Boolean getHasMore() {
        return this.hasMore;
    }
    
    public void setHasMore(Boolean hasMore) {
        this.hasMore = hasMore;
    }
}
