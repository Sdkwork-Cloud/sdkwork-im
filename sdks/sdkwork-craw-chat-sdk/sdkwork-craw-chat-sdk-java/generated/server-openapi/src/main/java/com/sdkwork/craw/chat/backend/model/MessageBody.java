package com.sdkwork.craw.chat.backend.model;

import java.util.List;
import java.util.Map;

public class MessageBody {
    private String summary;
    private List<ContentPart> parts;
    private Map<String, String> renderHints;

    public String getSummary() {
        return this.summary;
    }
    
    public void setSummary(String summary) {
        this.summary = summary;
    }

    public List<ContentPart> getParts() {
        return this.parts;
    }
    
    public void setParts(List<ContentPart> parts) {
        this.parts = parts;
    }

    public Map<String, String> getRenderHints() {
        return this.renderHints;
    }
    
    public void setRenderHints(Map<String, String> renderHints) {
        this.renderHints = renderHints;
    }
}
