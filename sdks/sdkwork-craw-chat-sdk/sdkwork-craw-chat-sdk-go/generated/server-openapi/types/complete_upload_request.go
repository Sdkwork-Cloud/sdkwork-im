package types


type CompleteUploadRequest struct {
	Bucket string `json:"bucket"`
	ObjectKey string `json:"objectKey"`
	StorageProvider string `json:"storageProvider"`
	Url string `json:"url"`
	Checksum string `json:"checksum"`
}
