package com.sdkwork.craw.chat.backend.model;


public class AbortStreamRequest {
    private Integer frameSeq;
    private String reason;

    public Integer getFrameSeq() {
        return this.frameSeq;
    }
    
    public void setFrameSeq(Integer frameSeq) {
        this.frameSeq = frameSeq;
    }

    public String getReason() {
        return this.reason;
    }
    
    public void setReason(String reason) {
        this.reason = reason;
    }
}
