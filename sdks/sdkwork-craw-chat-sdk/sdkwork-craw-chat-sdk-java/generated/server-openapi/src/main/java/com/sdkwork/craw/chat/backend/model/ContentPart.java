package com.sdkwork.craw.chat.backend.model;


public class ContentPart {
    private String kind;
    private String text;
    private String schemaRef;
    private String encoding;
    private String payload;
    private String mediaAssetId;
    private MediaResource resource;
    private String signalType;
    private String streamId;
    private String streamType;
    private String state;

    public String getKind() {
        return this.kind;
    }
    
    public void setKind(String kind) {
        this.kind = kind;
    }

    public String getText() {
        return this.text;
    }
    
    public void setText(String text) {
        this.text = text;
    }

    public String getSchemaRef() {
        return this.schemaRef;
    }
    
    public void setSchemaRef(String schemaRef) {
        this.schemaRef = schemaRef;
    }

    public String getEncoding() {
        return this.encoding;
    }
    
    public void setEncoding(String encoding) {
        this.encoding = encoding;
    }

    public String getPayload() {
        return this.payload;
    }
    
    public void setPayload(String payload) {
        this.payload = payload;
    }

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

    public String getSignalType() {
        return this.signalType;
    }
    
    public void setSignalType(String signalType) {
        this.signalType = signalType;
    }

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

    public String getState() {
        return this.state;
    }
    
    public void setState(String state) {
        this.state = state;
    }
}
