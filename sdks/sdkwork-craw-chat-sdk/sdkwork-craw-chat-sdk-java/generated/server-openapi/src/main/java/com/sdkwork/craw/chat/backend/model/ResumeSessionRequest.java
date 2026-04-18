package com.sdkwork.craw.chat.backend.model;


public class ResumeSessionRequest {
    private String deviceId;
    private Integer lastSeenSyncSeq;

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public Integer getLastSeenSyncSeq() {
        return this.lastSeenSyncSeq;
    }
    
    public void setLastSeenSyncSeq(Integer lastSeenSyncSeq) {
        this.lastSeenSyncSeq = lastSeenSyncSeq;
    }
}
