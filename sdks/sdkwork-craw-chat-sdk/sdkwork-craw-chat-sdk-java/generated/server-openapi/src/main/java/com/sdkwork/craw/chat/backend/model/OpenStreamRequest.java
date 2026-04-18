package com.sdkwork.craw.chat.backend.model;


public class OpenStreamRequest {
    private String streamId;
    private String streamType;
    private String scopeKind;
    private String scopeId;
    private String durabilityClass;
    private String schemaRef;

    public String getStreamId() {
        return this.streamId;
    }
    
    public void setStreamId(String streamId) {
        this.streamId = streamId;
    }

    public String getStreamType() {
        return this.streamType;
    }
    
    public void setStreamType(String streamType) {
        this.streamType = streamType;
    }

    public String getScopeKind() {
        return this.scopeKind;
    }
    
    public void setScopeKind(String scopeKind) {
        this.scopeKind = scopeKind;
    }

    public String getScopeId() {
        return this.scopeId;
    }
    
    public void setScopeId(String scopeId) {
        this.scopeId = scopeId;
    }

    public String getDurabilityClass() {
        return this.durabilityClass;
    }
    
    public void setDurabilityClass(String durabilityClass) {
        this.durabilityClass = durabilityClass;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }
    
    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }
}
