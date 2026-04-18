package types


type CreateUploadRequest struct {
	MediaAssetId string `json:"mediaAssetId"`
	Resource MediaResource `json:"resource"`
}
