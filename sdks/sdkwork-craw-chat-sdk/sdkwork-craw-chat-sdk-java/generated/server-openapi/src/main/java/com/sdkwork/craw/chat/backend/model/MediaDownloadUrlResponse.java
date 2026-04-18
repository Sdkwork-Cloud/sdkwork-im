package com.sdkwork.craw.chat.backend.model;


public class MediaDownloadUrlResponse {
    private String mediaAssetId;
    private String storageProvider;
    private String downloadUrl;
    private Integer expiresInSeconds;

    public String getMediaAssetId() {
        return this.mediaAssetId;
    }
    
    public void setMediaAssetId(String mediaAssetId) {
        this.mediaAssetId = mediaAssetId;
    }

    public String getStorageProvider() {
        return this.storageProvider;
    }
    
    public void setStorageProvider(String storageProvider) {
        this.storageProvider = storageProvider;
    }

    public String getDownloadUrl() {
        return this.downloadUrl;
    }
    
    public void setDownloadUrl(String downloadUrl) {
        this.downloadUrl = downloadUrl;
    }

    public Integer getExpiresInSeconds() {
        return this.expiresInSeconds;
    }
    
    public void setExpiresInSeconds(Integer expiresInSeconds) {
        this.expiresInSeconds = expiresInSeconds;
    }
}
