package com.sdkwork.craw.chat.backend.model;


public class CompleteStreamRequest {
    private Integer frameSeq;
    private String resultMessageId;

    public Integer getFrameSeq() {
        return this.frameSeq;
    }
    
    public void setFrameSeq(Integer frameSeq) {
        this.frameSeq = frameSeq;
    }

    public String getResultMessageId() {
        return this.resultMessageId;
    }
    
    public void setResultMessageId(String resultMessageId) {
        this.resultMessageId = resultMessageId;
    }
}
