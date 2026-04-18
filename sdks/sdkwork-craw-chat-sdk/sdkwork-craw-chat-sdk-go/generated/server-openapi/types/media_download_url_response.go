package types


type MediaDownloadUrlResponse struct {
	MediaAssetId string `json:"mediaAssetId"`
	StorageProvider string `json:"storageProvider"`
	DownloadUrl string `json:"downloadUrl"`
	ExpiresInSeconds int `json:"expiresInSeconds"`
}
