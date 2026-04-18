package com.sdkwork.craw.chat.backend.model;


public class UpdateReadCursorRequest {
    private Integer readSeq;
    private String lastReadMessageId;

    public Integer getReadSeq() {
        return this.readSeq;
    }
    
    public void setReadSeq(Integer readSeq) {
        this.readSeq = readSeq;
    }

    public String getLastReadMessageId() {
        return this.lastReadMessageId;
    }
    
    public void setLastReadMessageId(String lastReadMessageId) {
        this.lastReadMessageId = lastReadMessageId;
    }
}
