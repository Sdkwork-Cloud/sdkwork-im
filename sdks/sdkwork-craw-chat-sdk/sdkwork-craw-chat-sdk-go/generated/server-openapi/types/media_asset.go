package types


type MediaAsset struct {
	TenantId string `json:"tenantId"`
	PrincipalId string `json:"principalId"`
	PrincipalKind string `json:"principalKind"`
	MediaAssetId string `json:"mediaAssetId"`
	Bucket string `json:"bucket"`
	ObjectKey string `json:"objectKey"`
	StorageProvider string `json:"storageProvider"`
	Checksum string `json:"checksum"`
	ProcessingState MediaProcessingState `json:"processingState"`
	Resource MediaResource `json:"resource"`
	CreatedAt string `json:"createdAt"`
	CompletedAt string `json:"completedAt"`
}
