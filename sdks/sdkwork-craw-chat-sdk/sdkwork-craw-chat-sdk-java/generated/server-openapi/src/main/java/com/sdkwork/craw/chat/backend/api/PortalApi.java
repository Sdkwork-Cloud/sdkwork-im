package com.sdkwork.craw.chat.backend.api;

import com.fasterxml.jackson.core.type.TypeReference;
import com.sdkwork.craw.chat.backend.http.HttpClient;
import com.sdkwork.craw.chat.backend.model.*;
import java.util.List;
import java.util.Map;

public class PortalApi {
    private final HttpClient client;
    
    public PortalApi(HttpClient client) {
        this.client = client;
    }

    /** Read the tenant portal home snapshot */
    public Map<String, Object> getHome() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/home"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant portal sign-in snapshot */
    public Map<String, Object> getAuth() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/auth"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the current tenant workspace snapshot */
    public PortalWorkspaceView getWorkspace() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/workspace"));
        return client.convertValue(raw, new TypeReference<PortalWorkspaceView>() {});
    }

    /** Read the tenant dashboard snapshot */
    public Map<String, Object> getDashboard() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/dashboard"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant conversations snapshot */
    public Map<String, Object> getConversations() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/conversations"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant realtime snapshot */
    public Map<String, Object> getRealtime() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/realtime"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant media snapshot */
    public Map<String, Object> getMedia() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/media"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant automation snapshot */
    public Map<String, Object> getAutomation() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/automation"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }

    /** Read the tenant governance snapshot */
    public Map<String, Object> getGovernance() throws Exception {
        Object raw = client.get(ApiPaths.backendPath("/portal/governance"));
        return client.convertValue(raw, new TypeReference<Map<String, Object>>() {});
    }
}
