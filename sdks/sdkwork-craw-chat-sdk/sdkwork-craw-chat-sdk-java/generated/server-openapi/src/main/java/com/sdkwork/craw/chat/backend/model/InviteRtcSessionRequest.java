package com.sdkwork.craw.chat.backend.model;


public class InviteRtcSessionRequest {
    private String signalingStreamId;

    public String getSignalingStreamId() {
        return this.signalingStreamId;
    }
    
    public void setSignalingStreamId(String signalingStreamId) {
        this.signalingStreamId = signalingStreamId;
    }
}
