package com.sdkwork.craw.chat.backend.model;


public class MediaAsset {
    private String tenantId;
    private String principalId;
    private String principalKind;
    private String mediaAssetId;
    private String bucket;
    private String objectKey;
    private String storageProvider;
    private String checksum;
    private String processingState;
    private MediaResource resource;
    private String createdAt;
    private String completedAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getPrincipalId() {
        return this.principalId;
    }
    
    public void setPrincipalId(String principalId) {
        this.principalId = principalId;
    }

    public String getPrincipalKind() {
        return this.principalKind;
    }
    
    public void setPrincipalKind(String principalKind) {
        this.principalKind = principalKind;
    }

    public String getMediaAssetId() {
        return this.mediaAssetId;
    }
    
    public void setMediaAssetId(String mediaAssetId) {
        this.mediaAssetId = mediaAssetId;
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

    public String getChecksum() {
        return this.checksum;
    }
    
    public void setChecksum(String checksum) {
        this.checksum = checksum;
    }

    public String getProcessingState() {
        return this.processingState;
    }
    
    public void setProcessingState(String processingState) {
        this.processingState = processingState;
    }

    public MediaResource getResource() {
        return this.resource;
    }
    
    public void setResource(MediaResource resource) {
        this.resource = resource;
    }

    public String getCreatedAt() {
        return this.createdAt;
    }
    
    public void setCreatedAt(String createdAt) {
        this.createdAt = createdAt;
    }

    public String getCompletedAt() {
        return this.completedAt;
    }
    
    public void setCompletedAt(String completedAt) {
        this.completedAt = completedAt;
    }
}
