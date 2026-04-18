package types


type RtcRecordingArtifact struct {
	TenantId string `json:"tenantId"`
	RtcSessionId string `json:"rtcSessionId"`
	Bucket string `json:"bucket"`
	ObjectKey string `json:"objectKey"`
	StorageProvider string `json:"storageProvider"`
	PlaybackUrl string `json:"playbackUrl"`
}
