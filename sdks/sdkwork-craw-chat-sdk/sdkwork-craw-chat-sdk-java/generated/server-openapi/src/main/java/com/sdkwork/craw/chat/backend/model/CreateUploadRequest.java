package com.sdkwork.craw.chat.backend.model;


public class CreateUploadRequest {
    private String mediaAssetId;
    private MediaResource resource;

    public String getMediaAssetId() {
        return this.mediaAssetId;
    }
    
    public void setMediaAssetId(String mediaAssetId) {
        this.mediaAssetId = mediaAssetId;
    }

    public MediaResource getResource() {
        return this.resource;
    }
    
    public void setResource(MediaResource resource) {
        this.resource = resource;
    }
}
