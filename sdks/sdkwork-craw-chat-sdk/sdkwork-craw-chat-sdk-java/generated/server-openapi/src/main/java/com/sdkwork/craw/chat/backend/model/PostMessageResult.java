package com.sdkwork.craw.chat.backend.model;


public class PostMessageResult {
    private String messageId;
    private Integer messageSeq;
    private String eventId;

    public String getMessageId() {
        return this.messageId;
    }
    
    public void setMessageId(String messageId) {
        this.messageId = messageId;
    }

    public Integer getMessageSeq() {
        return this.messageSeq;
    }
    
    public void setMessageSeq(Integer messageSeq) {
        this.messageSeq = messageSeq;
    }

    public String getEventId() {
        return this.eventId;
    }
    
    public void setEventId(String eventId) {
        this.eventId = eventId;
    }
}
