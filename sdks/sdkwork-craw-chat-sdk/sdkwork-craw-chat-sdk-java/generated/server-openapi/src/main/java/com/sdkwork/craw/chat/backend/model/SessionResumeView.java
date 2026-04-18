package com.sdkwork.craw.chat.backend.model;


public class SessionResumeView {
    private String tenantId;
    private String actorId;
    private String actorKind;
    private String sessionId;
    private String deviceId;
    private Boolean resumeRequired;
    private Integer resumeFromSyncSeq;
    private Integer latestSyncSeq;
    private String resumedAt;
    private PresenceSnapshotView presence;

    public String getTenantId() {
        return this.tenantId;
    }
    
    public void setTenantId(String tenantId) {
        this.tenantId = tenantId;
    }

    public String getActorId() {
        return this.actorId;
    }
    
    public void setActorId(String actorId) {
        this.actorId = actorId;
    }

    public String getActorKind() {
        return this.actorKind;
    }
    
    public void setActorKind(String actorKind) {
        this.actorKind = actorKind;
    }

    public String getSessionId() {
        return this.sessionId;
    }
    
    public void setSessionId(String sessionId) {
        this.sessionId = sessionId;
    }

    public String getDeviceId() {
        return this.deviceId;
    }
    
    public void setDeviceId(String deviceId) {
        this.deviceId = deviceId;
    }

    public Boolean getResumeRequired() {
        return this.resumeRequired;
    }
    
    public void setResumeRequired(Boolean resumeRequired) {
        this.resumeRequired = resumeRequired;
    }

    public Integer getResumeFromSyncSeq() {
        return this.resumeFromSyncSeq;
    }
    
    public void setResumeFromSyncSeq(Integer resumeFromSyncSeq) {
        this.resumeFromSyncSeq = resumeFromSyncSeq;
    }

    public Integer getLatestSyncSeq() {
        return this.latestSyncSeq;
    }
    
    public void setLatestSyncSeq(Integer latestSyncSeq) {
        this.latestSyncSeq = latestSyncSeq;
    }

    public String getResumedAt() {
        return this.resumedAt;
    }
    
    public void setResumedAt(String resumedAt) {
        this.resumedAt = resumedAt;
    }

    public PresenceSnapshotView getPresence() {
        return this.presence;
    }
    
    public void setPresence(PresenceSnapshotView presence) {
        this.presence = presence;
    }
}
