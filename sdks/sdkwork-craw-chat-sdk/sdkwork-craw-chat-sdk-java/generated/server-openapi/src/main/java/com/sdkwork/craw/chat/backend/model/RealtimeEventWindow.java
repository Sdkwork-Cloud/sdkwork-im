package com.sdkwork.craw.chat.backend.model;

import java.util.List;

public class RealtimeEventWindow {
    private String deviceId;
    private List<RealtimeEvent> items;
    private Integer nextAfterSeq;
    private Boolean hasMore;
    private Integer ackedThroughSeq;
    private Integer trimmedThroughSeq;

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public List<RealtimeEvent> getItems() {
        return this.items;
    }
    
    public void setItems(List<RealtimeEvent> items) {
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

    public Integer getAckedThroughSeq() {
        return this.ackedThroughSeq;
    }
    
    public void setAckedThroughSeq(Integer ackedThroughSeq) {
        this.ackedThroughSeq = ackedThroughSeq;
    }

    public Integer getTrimmedThroughSeq() {
        return this.trimmedThroughSeq;
    }
    
    public void setTrimmedThroughSeq(Integer trimmedThroughSeq) {
        this.trimmedThroughSeq = trimmedThroughSeq;
    }
}
