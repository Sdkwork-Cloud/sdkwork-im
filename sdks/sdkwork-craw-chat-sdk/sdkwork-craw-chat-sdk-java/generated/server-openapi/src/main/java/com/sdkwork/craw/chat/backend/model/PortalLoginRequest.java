package com.sdkwork.craw.chat.backend.model;


public class PortalLoginRequest {
    private String tenantId;
    private String login;
    private String password;
    private String deviceId;
    private String sessionId;
    private String clientKind;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getLogin() {
        return this.login;
    }
    
    public void setLogin(String login) {
        this.login = login;
    }

    public String getPassword() {
        return this.password;
    }
    
    public void setPassword(String password) {
        this.password = password;
    }

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public String getSessionId() {
        return this.sessionId;
    }
    
    public void setSessionId(String sessionId) {
        this.sessionId = sessionId;
    }

    public String getClientKind() {
        return this.clientKind;
    }
    
    public void setClientKind(String clientKind) {
        this.clientKind = clientKind;
    }
}
