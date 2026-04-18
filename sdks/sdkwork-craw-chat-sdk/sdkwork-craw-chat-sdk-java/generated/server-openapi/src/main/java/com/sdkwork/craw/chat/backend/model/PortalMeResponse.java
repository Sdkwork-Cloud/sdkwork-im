package com.sdkwork.craw.chat.backend.model;


public class PortalMeResponse {
    private String tenantId;
    private PortalUserView user;
    private PortalWorkspaceView workspace;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
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
