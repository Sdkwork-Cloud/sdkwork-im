package com.sdkwork.craw.chat.backend.model;

import java.util.List;
import java.util.Map;

public class MediaResource {
    private Integer id;
    private String uuid;
    private String url;
    private List<Integer> bytes;
    private String localFile;
    private String base64;
    private String type;
    private String mimeType;
    private Integer size;
    private String name;
    private String extension;
    private Map<String, String> tags;
    private Map<String, String> metadata;
    private String prompt;

    public Integer getId() {
        return this.id;
    }
    
    public void setId(Integer id) {
        this.id = id;
    }

    public String getUuid() {
        return this.uuid;
    }
    
    public void setUuid(String uuid) {
        this.uuid = uuid;
    }

    public String getUrl() {
        return this.url;
    }
    
    public void setUrl(String url) {
        this.url = url;
    }

    public List<Integer> getBytes() {
        return this.bytes;
    }
    
    public void setBytes(List<Integer> bytes) {
        this.bytes = bytes;
    }

    public String getLocalFile() {
        return this.localFile;
    }
    
    public void setLocalFile(String localFile) {
        this.localFile = localFile;
    }

    public String getBase64() {
        return this.base64;
    }
    
    public void setBase64(String base64) {
        this.base64 = base64;
    }

    public String getType() {
        return this.type;
    }
    
    public void setType(String type) {
        this.type = type;
    }

    public String getMimeType() {
        return this.mimeType;
    }
    
    public void setMimeType(String mimeType) {
        this.mimeType = mimeType;
    }

    public Integer getSize() {
        return this.size;
    }
    
    public void setSize(Integer size) {
        this.size = size;
    }

    public String getName() {
        return this.name;
    }
    
    public void setName(String name) {
        this.name = name;
    }

    public String getExtension() {
        return this.extension;
    }
    
    public void setExtension(String extension) {
        this.extension = extension;
    }

    public Map<String, String> getTags() {
        return this.tags;
    }
    
    public void setTags(Map<String, String> tags) {
        this.tags = tags;
    }

    public Map<String, String> getMetadata() {
        return this.metadata;
    }
    
    public void setMetadata(Map<String, String> metadata) {
        this.metadata = metadata;
    }

    public String getPrompt() {
        return this.prompt;
    }
    
    public void setPrompt(String prompt) {
        this.prompt = prompt;
    }
}
