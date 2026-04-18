package com.sdkwork.craw.chat.backend.model;


public class CompleteUploadRequest {
    private String bucket;
    private String objectKey;
    private String storageProvider;
    private String url;
    private String checksum;

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

    public String getUrl() {
        return this.url;
    }
    
    public void setUrl(String url) {
        this.url = url;
    }

    public String getChecksum() {
        return this.checksum;
    }
    
    public void setChecksum(String checksum) {
        this.checksum = checksum;
    }
}
