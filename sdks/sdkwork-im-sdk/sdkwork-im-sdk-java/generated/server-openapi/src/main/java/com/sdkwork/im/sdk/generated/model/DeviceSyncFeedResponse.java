package com.sdkwork.im.sdk.generated.model;

import java.util.List;

public class DeviceSyncFeedResponse {
    private List<DeviceSyncFeedEntry> items;
    private Integer nextAfterSeq;
    private Boolean hasMore;
    private Integer trimmedThroughSeq;

    public List<DeviceSyncFeedEntry> getItems() {
        return this.items;
    }
    
    public void setItems(List<DeviceSyncFeedEntry> items) {
        this.items = items;
    }

    public Integer getNextAfterSeq() {
        return this.nextAfterSeq;
    }
    
    public void setNextAfterSeq(Integer nextAfterSeq) {
        this.nextAfterSeq = nextAfterSeq;
    }

    public Boolean getHasMore() {
        return this.hasMore;
    }
    
    public void setHasMore(Boolean hasMore) {
        this.hasMore = hasMore;
    }

    public Integer getTrimmedThroughSeq() {
        return this.trimmedThroughSeq;
    }
    
    public void setTrimmedThroughSeq(Integer trimmedThroughSeq) {
        this.trimmedThroughSeq = trimmedThroughSeq;
    }
}
