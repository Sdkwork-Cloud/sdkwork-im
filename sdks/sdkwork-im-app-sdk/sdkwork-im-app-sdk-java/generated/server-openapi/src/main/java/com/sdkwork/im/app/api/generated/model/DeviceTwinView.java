package com.sdkwork.im.app.api.generated.model;


public class DeviceTwinView {
    private String tenantId;
    private String deviceId;
    private String desiredStateJson;
    private String reportedStateJson;
    private String updatedAt;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public String getDesiredStateJson() {
        return this.desiredStateJson;
    }
    
    public void setDesiredStateJson(String desiredStateJson) {
        this.desiredStateJson = desiredStateJson;
    }

    public String getReportedStateJson() {
        return this.reportedStateJson;
    }
    
    public void setReportedStateJson(String reportedStateJson) {
        this.reportedStateJson = reportedStateJson;
    }

    public String getUpdatedAt() {
        return this.updatedAt;
    }
    
    public void setUpdatedAt(String updatedAt) {
        this.updatedAt = updatedAt;
    }
}
