package com.sdkwork.craw.chat.backend.model;


public class RtcRecordingArtifact {
    private String tenantId;
    private String rtcSessionId;
    private String bucket;
    private String objectKey;
    private String storageProvider;
    private String playbackUrl;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getRtcSessionId() {
        return this.rtcSessionId;
    }
    
    public void setRtcSessionId(String rtcSessionId) {
        this.rtcSessionId = rtcSessionId;
    }

    public String getBucket() {
        return this.bucket;
    }
    
    public void setBucket(String bucket) {
        this.bucket = bucket;
    }

    public String getObjectKey() {
        return this.objectKey;
    }
    
    public void setObjectKey(String objectKey) {
        this.objectKey = objectKey;
    }

    public String getStorageProvider() {
        return this.storageProvider;
    }
    
    public void setStorageProvider(String storageProvider) {
        this.storageProvider = storageProvider;
    }

    public String getPlaybackUrl() {
        return this.playbackUrl;
    }
    
    public void setPlaybackUrl(String playbackUrl) {
        this.playbackUrl = playbackUrl;
    }
}
