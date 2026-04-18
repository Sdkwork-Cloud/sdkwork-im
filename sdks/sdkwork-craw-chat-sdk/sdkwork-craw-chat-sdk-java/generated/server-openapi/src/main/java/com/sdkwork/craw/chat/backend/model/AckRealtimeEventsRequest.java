package com.sdkwork.craw.chat.backend.model;


public class AckRealtimeEventsRequest {
    private String deviceId;
    private Integer ackedSeq;

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public Integer getAckedSeq() {
        return this.ackedSeq;
    }
    
    public void setAckedSeq(Integer ackedSeq) {
        this.ackedSeq = ackedSeq;
    }
}
