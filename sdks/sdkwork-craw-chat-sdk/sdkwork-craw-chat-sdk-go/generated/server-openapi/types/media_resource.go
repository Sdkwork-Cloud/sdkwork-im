package types


type MediaResource struct {
	Id int `json:"id"`
	Uuid string `json:"uuid"`
	Url string `json:"url"`
	Bytes []int `json:"bytes"`
	LocalFile string `json:"localFile"`
	Base64 string `json:"base64"`
	Type MediaResourceType `json:"type"`
	MimeType string `json:"mimeType"`
	Size int `json:"size"`
	Name string `json:"name"`
	Extension string `json:"extension"`
	Tags StringMap `json:"tags"`
	Metadata StringMap `json:"metadata"`
	Prompt string `json:"prompt"`
}
