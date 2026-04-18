package types


type SessionResumeView struct {
	TenantId string `json:"tenantId"`
	ActorId string `json:"actorId"`
	ActorKind string `json:"actorKind"`
	SessionId string `json:"sessionId"`
	DeviceId string `json:"deviceId"`
	ResumeRequired bool `json:"resumeRequired"`
	ResumeFromSyncSeq int `json:"resumeFromSyncSeq"`
	LatestSyncSeq int `json:"latestSyncSeq"`
	ResumedAt string `json:"resumedAt"`
	Presence PresenceSnapshotView `json:"presence"`
}
