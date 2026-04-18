package com.sdkwork.craw.chat.backend.model;


public class PortalLoginResponse {
    private String accessToken;
    private String refreshToken;
    private Integer expiresAt;
    private PortalUserView user;
    private PortalWorkspaceView workspace;

    public String getAccessToken() {
        return this.accessToken;
    }
    
    public void setAccessToken(String accessToken) {
        this.accessToken = accessToken;
    }

    public String getRefreshToken() {
        return this.refreshToken;
    }
    
    public void setRefreshToken(String refreshToken) {
        this.refreshToken = refreshToken;
    }

    public Integer getExpiresAt() {
        return this.expiresAt;
    }
    
    public void setExpiresAt(Integer expiresAt) {
        this.expiresAt = expiresAt;
    }

    public PortalUserView getUser() {
        return this.user;
    }
    
    public void setUser(PortalUserView user) {
        this.user = user;
    }

    public PortalWorkspaceView getWorkspace() {
        return this.workspace;
    }
    
    public void setWorkspace(PortalWorkspaceView workspace) {
        this.workspace = workspace;
    }
}
